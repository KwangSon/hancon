# 📝 HanCon: High-Performance HWP to ODT Converter (Wasm-based)

**HanCon** is a privacy-focused, open-source document converter that transforms Hancom Office (.hwp, .hwpx) files into the OpenDocument Text (.odt) format. 

Built with **Rust** and **WebAssembly**, HanCon processes everything **locally in your browser**. Your sensitive documents never leave your computer—no server uploads, no privacy risks.



## ✨ Key Features
- **Privacy-First (Client-Side)**: 100% local processing. No server-side storage or data harvesting.
- **Blazing Fast**: Powered by Rust's performance, handling complex parses in milliseconds.
- **Serverless**: Hosted on GitHub Pages for high availability and zero latency.
- **Modern UI**: Clean, responsive interface built with Tailwind CSS and HTMX.
- **Open Source**: Licensed under AGPL-3.0 to ensure transparency and community growth.

## 🛠 Tech Stack
- **Engine**: [Rust](https://www.rust-lang.org/) (for robust binary/XML parsing)
- **Runtime**: [WebAssembly](https://webassembly.org/) (cross-platform browser execution)
- **Frontend**: [Tailwind CSS](https://tailwindcss.com/) & [HTMX](https://htmx.org/)
- **Distribution**: GitHub Pages

## 🚀 Getting Started

### Prerequisites
- Install Rust & Cargo
- Install `wasm-pack`: `cargo install wasm-pack`

### Build from Source
```bash
# Compile Rust to WebAssembly
wasm-pack build --target web --out-dir www/pkg
