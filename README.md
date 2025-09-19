# Vendly

Vendly es una plataforma de gestión integral para tiendas modernas, que combina inventario, proveedores y control de cartera en un solo lugar.  
Construida con **Rust** y **React (Vite + Tailwind)**, ofrece rapidez, seguridad y una interfaz intuitiva para facilitar la administración de tu negocio, optimizando cada proceso desde la venta hasta la gestión de stock.  

---

## 🚀 Tecnologías principales
- **Rust** → núcleo de negocio (arquitectura hexagonal, sin dependencias pesadas).  
- **React + Vite + Tailwind** → frontend web reutilizado también en Tauri.  
- **Tauri** → aplicación de escritorio ligera que reutiliza el mismo frontend.  
- **Tokio / SeaORM** → asincronía y acceso a base de datos (solo en infraestructura).  

---

## 📂 Estructura del proyecto

```text
/vendly
├── apps/                         
│   ├── frontend/                 # 🌐 React + Vite + Tailwind (UI principal y base de Tauri)
│   ├── backend/                  # ⚙️ API backend en Rust
│   └── desktop/                  # 💻 App Tauri (usa frontend como UI)
│
├── core/                         # 🧠 Dominio + aplicación (Rust puro, hexagonal)
├── infrastructure/               # 🔌 Adaptadores concretos (DB, HTTP, messaging)
├── libs/                         # 📦 Librerías compartidas (UI y utils)
├── tests/                        # 🧪 Pruebas integrales / e2e
├── infra/                        # ☁️ Infraestructura como código (Docker, k8s, etc.)
├── docs/                         # 📖 Documentación
│
├── .eslintignore / .eslintrc.cjs / jest.config.js / tailwind.config.js  
├── .gitignore / .prettierignore / .prettierrc  
├── cspell.json / package.json / Cargo.toml  
├── LICENSE  
└── README.md  
