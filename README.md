# ChronoLog

> **The antidote to corporate surveillance.** > *Part of [The Authentic Rebellion] suite of projects.*

![Status](https://img.shields.io/badge/Status-Pre--Alpha-red)
![License](https://img.shields.io/badge/License-AGPLv3-blue)
![Platform](https://img.shields.io/badge/Platform-Windows%2011-0078D6)

**ChronoLog** is a privacy-first, open-source alternative to Microsoft Recall. It is designed to act as your digital memory without compromising your dignity or data sovereignty. 

Unlike proprietary solutions that treat your data as a commodity, ChronoLog runs entirely **offline**. It leverages modern local hardware (NPUs and GPUs) to capture, index, and analyze your screen activity, keeping 100% of the processing and storage on your own device.

## 🌟 Core Philosophy

We believe the world has become too motivated by capitalist gains at the expense of human dignity. ChronoLog is built on the following pillars:
* **Privacy is a Human Right:** No data leaves your machine. Ever.
* **Local-First:** We prioritize local processing power (NPU/GPU) over cloud dependency.
* **User Agency:** The interface is yours to mold. The frontend is fully scriptable and customizable.

## 🚀 Key Features

* **Hybrid Architecture:** A high-performance **Rust** backend handles the heavy lifting, while a lightweight web-based frontend provides a flexible user interface.
* **Hardware Acceleration:** Built on the **ONNX Runtime** and **DirectML**, ChronoLog specifically targets dedicated **NPUs** (Neural Processing Units) for efficient, low-power background processing, while leveraging **GPUs** for heavy on-demand tasks.
* **Hackable UI:** The frontend is designed for scripters. Users can modify the interface, create custom overlays, and build widgets using standard HTML, CSS, and JavaScript.
* **Semantic Search:** "Recall" past actions using natural language (e.g., *"Show me the article about pottery I was reading last Tuesday"*).

## 🛠️ Tech Stack

* **Core:** Rust 🦀
* **App Framework:** Tauri v2
* **AI/ML Runtime:** ONNX Runtime (with DirectML execution provider)
* **Database:** LanceDB (Local vector database for semantic search)
* **Frontend:** HTML/JS/CSS (Framework agnostic, designed for hackability)

## 📦 Installation & Development

*Note: ChronoLog is currently in active pre-alpha development.*

### Prerequisites
* **Windows 11** (Current primary target)
* [Rust](https://www.rust-lang.org/tools/install) (`rustup`)
* [Node.js](https://nodejs.org/) (LTS)
* Microsoft C++ Build Tools (via Visual Studio Installer)

### Setting up the Environment
1.  Clone the repository:
    ```bash
    git clone [https://github.com/YOUR_USERNAME/chronolog.git](https://github.com/YOUR_USERNAME/chronolog.git)
    cd chronolog
    ```

2.  Install frontend dependencies:
    ```bash
    npm install
    ```

3.  Run the application in development mode:
    ```bash
    npm run tauri dev
    ```

## 🗺️ Roadmap

- [ ] **Phase 1: Foundation (Windows 11)**
    - [ ] Basic Rust backend for efficient screen capture.
    - [ ] Initial Tauri setup with basic web frontend.
    - [ ] Database integration (LanceDB).
- [ ] **Phase 2: The Eye (AI Integration)**
    - [ ] ONNX Runtime integration.
    - [ ] OCR and Image Captioning pipelines (NPU target).
- [ ] **Phase 3: The Canvas (UI Scripting)**
    - [ ] API hooks for frontend scripting.
    - [ ] Custom overlay support.
- [ ] **Phase 4: Distribution**
    - [ ] `winget` package submission.

## 🤝 Contributing

We welcome fellow travelers! Whether you are a Rustacean, a frontend wizard, or an AI enthusiast, your help is needed. 

**Specific Help Needed:**
* **Rust/Systems Programming:** We are looking for assistance in optimizing the screen capture pipeline and NPU hardware abstraction layers.
* **AI/ML:** Help optimizing ONNX models for DirectML on Windows.

Please read [CONTRIBUTING.md](CONTRIBUTING.md) for details on our code of conduct and the process for submitting pull requests.

## 📜 License

This project is licensed under the **GNU Affero General Public License v3.0 (AGPL-3.0)** - ensuring that all modifications and network deployments remain open source. See the `LICENSE` file for details.