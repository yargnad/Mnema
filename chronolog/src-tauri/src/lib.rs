use std::fs;
use std::path::PathBuf;
use std::time::Duration;
use tauri::{Emitter, Manager};
use screenshots::Screen;
use std::io::Cursor;
use image::{ImageOutputFormat, GenericImageView};
use base64::Engine;
use chrono::Local;
use rusqlite::{params, Connection};

// ORT Imports
use ort::session::Session;
use ort::session::builder::GraphOptimizationLevel; 
use ort::value::Tensor;
use ort::execution_providers::DirectMLExecutionProvider;
use ndarray::Array4;

// --- DATABASE IMPORTS (Corrected for LanceDB v0.5.0) ---
use lancedb::Table; // FIX: TableRef is now just 'Table'
use arrow::array::{RecordBatch, RecordBatchIterator, StringArray, FixedSizeListArray, Float32Array};
use arrow::datatypes::{DataType, Field, Schema};
use std::sync::Arc; // Needed for Arrow Schemas

// --- HELPER: Preprocess Image for CLIP ---
fn preprocess_for_clip(img: &image::DynamicImage) -> Array4<f32> {
    let resized = img.resize_exact(224, 224, image::imageops::FilterType::Triangle);
    let mut input = Array4::zeros((1, 3, 224, 224));
    let mean = [0.48145466, 0.4578275, 0.40821073];
    let std = [0.26862954, 0.26130258, 0.27577711];

    for (x, y, pixel) in resized.pixels() {
        let r = (pixel[0] as f32 / 255.0 - mean[0]) / std[0];
        let g = (pixel[1] as f32 / 255.0 - mean[1]) / std[1];
        let b = (pixel[2] as f32 / 255.0 - mean[2]) / std[2];
        input[[0, 0, y as usize, x as usize]] = r;
        input[[0, 1, y as usize, x as usize]] = g;
        input[[0, 2, y as usize, x as usize]] = b;
    }
    input
}

// --- HELPER: Initialize LanceDB ---
// FIX: Return Type is now 'Table', not 'TableRef'
async fn init_lancedb(app_handle: &tauri::AppHandle) -> Result<(PathBuf, Table), String> {
    let app_dir = app_handle.path().app_data_dir().map_err(|e| e.to_string())?;
    let lancedb_path = app_dir.join("lancedb_store");
    
    if !lancedb_path.exists() {
        fs::create_dir_all(&lancedb_path).map_err(|e| e.to_string())?;
    }

    let db = lancedb::connect(lancedb_path.to_str().unwrap())
        .execute()
        .await
        .map_err(|e| e.to_string())?;

    const VECTOR_SIZE: i32 = 512;
    let schema = Arc::new(Schema::new(vec![
        Field::new("id", DataType::Utf8, false),
        Field::new("timestamp", DataType::Utf8, false),
        Field::new("file_path", DataType::Utf8, false),
        Field::new(
            "vector",
            DataType::FixedSizeList(
                Arc::new(Field::new("item", DataType::Float32, true)),
                VECTOR_SIZE,
            ),
            true,
        ),
    ]));

    let table = db
        .create_table("memories", RecordBatchIterator::new(vec![], schema.clone()))
        .execute()
        .await;

    let table = match table {
        Ok(t) => t,
        Err(_) => db.open_table("memories").execute().await.map_err(|e| e.to_string())?,
    };

    println!("DEBUG: LanceDB initialized at {:?}", lancedb_path);
    Ok((app_dir, table))
}

// --- HELPER: Save Memory to LanceDB ---
// FIX: Input type is &Table
async fn add_memory(table: &Table, id: String, timestamp: String, path: String, embedding: Vec<f32>) {
    const VECTOR_SIZE: i32 = 512;
    
    let id_array = StringArray::from(vec![id]);
    let ts_array = StringArray::from(vec![timestamp]);
    let path_array = StringArray::from(vec![path]);
    
    let vector_values = Float32Array::from(embedding);
    let fixed_size_list = FixedSizeListArray::new(
        Arc::new(Field::new("item", DataType::Float32, true)),
        VECTOR_SIZE,
        Arc::new(vector_values),
        None,
    );

    let schema = Arc::new(Schema::new(vec![
        Field::new("id", DataType::Utf8, false),
        Field::new("timestamp", DataType::Utf8, false),
        Field::new("file_path", DataType::Utf8, false),
        Field::new("vector", DataType::FixedSizeList(Arc::new(Field::new("item", DataType::Float32, true)), VECTOR_SIZE), true),
    ]));

    let batch = RecordBatch::try_new(
        schema,
        vec![
            Arc::new(id_array),
            Arc::new(ts_array),
            Arc::new(path_array),
            Arc::new(fixed_size_list),
        ],
    ).unwrap();

// Get the schema first
    let schema = table.schema().await.unwrap();
    let _ = table.add(RecordBatchIterator::new(vec![Ok(batch)], schema.clone())).execute().await;}

// --- MAIN LOOP ---
// FIX: Input type is Table
async fn start_screen_capture_loop(app_handle: tauri::AppHandle, app_dir: PathBuf, mut session: Session, table: Table) {
    println!("DEBUG: Starting ChronoLog Memory Core...");
    let images_dir = app_dir.join("snapshots");
    if !images_dir.exists() { let _ = fs::create_dir_all(&images_dir); }

    loop {
        let screens = Screen::all().unwrap_or_default();

        if let Some(screen) = screens.first() {
            if let Ok(image_buffer) = screen.capture() {
                // UI Stream
                let mut buffer = Vec::new();
                let mut cursor = Cursor::new(&mut buffer);
                let dynamic_image = image::DynamicImage::ImageRgba8(image_buffer);
                
                let thumbnail = dynamic_image.thumbnail(800, 600);
                if thumbnail.write_to(&mut cursor, ImageOutputFormat::Png).is_ok() {
                     let base64_string = base64::engine::general_purpose::STANDARD.encode(&buffer);
                     let data_url = format!("data:image/png;base64,{}", base64_string);
                     let _ = app_handle.emit("new-screenshot", data_url);
                }

                // AI Inference
                let input_tensor_values = preprocess_for_clip(&dynamic_image);
                let input_tensor = Tensor::from_array(input_tensor_values).unwrap();
                let input_name = session.inputs[0].name.clone();
                
                let mut embedding_vec: Vec<f32> = Vec::new();

                match session.run(ort::inputs![input_name => input_tensor]) {
                    Ok(outputs) => {
                        let (_, output_value) = outputs.iter().next().unwrap();
                        let output_tensor = output_value.try_extract_tensor::<f32>().unwrap();
                        let (_, embedding) = output_tensor; 
                        
                        embedding_vec = embedding.to_vec();
                        
                        // println!("DEBUG: Memory Vector Generated (Size: {})", embedding_vec.len());
                    },
                    Err(e) => println!("Inferencing Error: {}", e),
                }

                // Persistence
                if !embedding_vec.is_empty() {
                    let id = uuid::Uuid::new_v4().to_string();
                    let now = Local::now();
                    let timestamp_str = now.format("%Y-%m-%d %H:%M:%S").to_string();
                    let filename = format!("{}.png", id);
                    let file_path = images_dir.join(&filename);

                    if dynamic_image.save(&file_path).is_ok() {
                        let path_string = file_path.to_string_lossy().to_string();
                        
                        // SAVE TO VECTOR DB
                        add_memory(&table, id, timestamp_str.clone(), path_string, embedding_vec).await;
                        println!("DEBUG: Memory Encoded & Saved. [{}]", timestamp_str);
                    }
                }
            }
        }
        tokio::time::sleep(Duration::from_secs(5)).await;
    }
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    ort::init().with_name("ChronoLog").commit().expect("Failed to init ONNX");

    tauri::Builder::default()
        .setup(|app| {
            let handle = app.handle().clone();

            let resource_path = app.path().resolve("resources/clip-vision.onnx", tauri::path::BaseDirectory::Resource)
                .expect("failed to resolve resource");

            println!("DEBUG: Loading CLIP Model...");
            let session = Session::builder()? 
                .with_execution_providers([DirectMLExecutionProvider::default().build()])?
                .with_optimization_level(GraphOptimizationLevel::Level3)?
                .with_intra_threads(4)?
                .commit_from_file(resource_path)?;

            println!("DEBUG: CLIP Brain loaded.");
            
            tauri::async_runtime::spawn(async move {
                match init_lancedb(&handle).await {
                    Ok((app_dir, table)) => {
                        start_screen_capture_loop(handle, app_dir, session, table).await;
                    },
                    Err(e) => println!("CRITICAL ERROR: LanceDB Init failed: {}", e),
                }
            });

            Ok(())
        })
        .plugin(tauri_plugin_shell::init())
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}