Mnema large resources
=====================

This project excludes large model binaries and runtime DLLs from the repository. To run Mnema locally you may need to obtain the following files and place them in the indicated locations (these are kept out of git to avoid large history and platform binaries):

- ONNX Runtime DLL (Windows): place under `.cargo/resources/onnxruntime/onnxruntime.dll`
- TroCR SentencePiece model: place under `.cargo/resources/trocr-base/sentencepiece.bpe.model` or `resources/trocr-base/`
- Any large `.onnx` or `.gguf` model files: place under `resources/` (ignored by git)

Recommended steps:

1. Download the appropriate ONNX Runtime release for your platform from the official Microsoft ONNX Runtime releases and extract the required runtime DLL into `.cargo/resources/onnxruntime/`.
2. Obtain the TroCR model artifacts you need (from the original model provider) and place the `sentencepiece.bpe.model` under `.cargo/resources/trocr-base/`.
3. Keep any additional models in `resources/` and do not commit them.

If you want these files tracked in the repo history, consider using Git LFS and adding the relevant patterns to `.gitattributes` instead.

If you want, I can add example download links and commands for your preferred platform.
