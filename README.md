Alright — let’s give **Aurora** a strong identity right now. Here’s a polished **README banner + tagline draft** you can drop straight into `resolutis/Aurora`:

---

# Aurora3D

**Rust + WASM Engine for Real-Time 3D and Edge AI**
*Built by [Resolutis](https://resolutis.com)*

Aurora3D is an open-source engine that combines **modern 3D rendering** with **edge-ready data pipelines**.
It’s designed for **retail, supply chain, and industrial applications**, but open to anyone who wants a high-performance, Rust-first platform for interactive graphics.

---

## ✨ Features (early roadmap)

* **Rust + WGPU core** — safe, modern, GPU-powered rendering.
* **WebAssembly build** — run natively in browsers with React/Vite integration.
* **glTF 2.0 loader** — support for PBR materials, textures, and scenes.
* **Interactive camera** — orbit, pan, zoom.
* **WebSocket bridge** — stream live product or AI inference data into the scene.
* **Extensible scene graph** — plug in metadata, BOMs, or annotations.

---

## Why Aurora3D?

Most 3D engines solve for *games*. Aurora3D is built for the next wave:

* **Digital product twins** — visualize products with their live metadata.
* **Edge AI visualization** — fuse Jetson/AI inference with 3D overlays.
* **Industrial + retail use cases** — product scanning, shelf analytics, BOM exploration.

Our mission: *make 3D + Edge AI accessible as a foundation, not just a demo.*

---

## Getting Started

```bash
git clone https://github.com/resolutis/Aurora
cd Aurora
npm install
npm run dev   # starts Vite + builds Rust → WASM
```

Open `http://localhost:5173` to see the demo.

---

## Roadmap

* [ ] Load & render glTF models
* [ ] Orbit/pan/zoom camera
* [ ] PBR shaders (WGSL)
* [ ] WebSocket live data overlays
* [ ] React metadata panel linked to scene
* [ ] NPM release of the Aurora3D viewer

---

## 📸 Preview

*coming!*

---

## 📜 License

Dual licensed under [MIT](LICENSE-MIT) and [Apache 2.0](LICENSE-APACHE).

---

This way you:

* Keep **Aurora** as the short repo name.
* Brand it as **Aurora3D** in docs → clears up Autodesk confusion.
* Plant the flag that this is about **3D + Edge AI**, not just “yet another renderer.”