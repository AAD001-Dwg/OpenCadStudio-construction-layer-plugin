# Open CAD Studio — Construction Layers & Guides Plugin

[![Open CAD Studio](https://img.shields.io/badge/Open_CAD_Studio-Add--on-blue.svg)](https://github.com/HakanSeven12/OpenCADStudio)
[![License: GPL v3](https://img.shields.io/badge/License-GPLv3-blue.svg)](LICENSE)
[![GitHub release](https://img.shields.io/github/v/release/alan0018ad/opencad-construction-plugin)](https://github.com/alan0018ad/opencad-construction-plugin/releases)

Complemento para **Open CAD Studio** que introduce **Capas de Construcción y Líneas Guía Infinitas**, inspirado en las mejores características de **LibreCAD** y **FreeCAD**.

An add-on for **Open CAD Studio** providing **Construction Layers and Infinite Drafting Guides**, bringing LibreCAD and FreeCAD drafting ergonomics into Open CAD Studio.

---

## ✨ Características / Features

- ⟷ **Líneas Infinitas (`CLINE` / `XLINE`):** Trazado de líneas de construcción infinitas con previsualización en tiempo real (*real-time GPU preview*). Permite proyectar múltiples líneas radiales desde un mismo punto de origen.
- ⟶ **Rayos Guía (`CRAY`):** Semirrectas infinitas a partir de un punto base hacia una dirección.
- ○ **Círculos de Referencia (`CCIRCLE`):** Circunferencias auxiliares ideales para círculos de centros de agujeros y radios de referencia.
- ✦ **Modo Referencia Inteligente (*Reference-Only* - API v9):** Las geometrías de construcción permiten el 100% de captura y precisión (*OSNAP*: extremos, centros, intersecciones), pero los clics ordinarios del mouse las atraviesan (*click-through*), evitando selecciones accidentales mientras diseñas.
- 🖨 **100% No-Imprimible (*No-Plot*):** Las capas de construcción se configuran automáticamente como no ploteables, garantizando que nunca aparezcan en planos impresos ni en exportaciones PDF.
- 🔄 **Auto-Conversión Reactiva (`HostNotification::DrawingChanged`):** Al dibujar con el comando normal `LINE` (`L`) sobre una capa de construcción activa, el plugin convierte automáticamente la línea en una guía infinita y la marca como referencia sin que tengas que cambiar de comando.
- 📐 **Presets Rápidos Horizontal / Vertical:** Trazado de guías horizontales (`CLINE H`) y verticales (`CLINE V`) con un solo clic.

---

## 🛠 Comandos Disponibles / Commands

| Comando / Command | Alias en Español | Alias en Inglés | Descripción / Description |
|---|---|---|---|
| `CLINE` | `LINEACONST` | `XLINE`, `CONST_LINE` | Dibuja líneas infinitas interactivas con preview en tiempo real. |
| `CLINE H` / `CLINE_H` | `LINEACONST H` | `XLINE H` | Trazado rápido de líneas infinitas puramente **Horizontales** (1 clic). |
| `CLINE V` / `CLINE_V` | `LINEACONST V` | `XLINE V` | Trazado rápido de líneas infinitas puramente **Verticales** (1 clic). |
| `CLINE <ángulo>` | `LINEACONST <ang>` | `XLINE <ang>` | Trazado de líneas infinitas con un ángulo fijo (ej. `CLINE 45`). |
| `CRAY` | `RAYO` | `CONST_RAY` | Dibuja semirrectas infinitas desde un punto base. |
| `CCIRCLE` | `CIRCULOCONST` | `CONST_CIRCLE` | Dibuja círculos auxiliares de referencia. |
| `CLAYER` | `CAPACONST`, `CAPAC` | `CONST_LAYER` | Alterna cualquier capa (activa, seleccionada o por nombre) como capa de construcción. |
| `CONST_SETUP` | — | — | Crea y activa la capa de construcción por defecto (`CONSTRUCTION`). |
| `CONST_TOGGLE`| — | — | Oculta o muestra la capa de construcción (`ON / OFF`). |
| `CONST_CONVERT`| — | — | Convierte las líneas estándar seleccionadas en líneas infinitas de construcción. |
| `CONST_PURGE` | — | — | Elimina todas las geometrías auxiliares de la capa de construcción. |
| `CONST_SYNC`  | — | — | Sincroniza y reactiva el modo referencia en documentos abiertos. |

---

## 📦 Instalación / Installation

### Método 1: Desde el catálogo de Open CAD Studio (Recomendado)
1. Abre **Open CAD Studio**.
2. Ve al menú superior o ejecuta el comando de complementos: **Opciones → Administrador de Complementos (Marketplace)**.
3. Busca **Construction Layers & Guides** y haz clic en **Instalar**.
4. Reinicia Open CAD Studio para comenzar a utilizar la nueva pestaña en la Cinta.

### Método 2: Instalación Manual
1. Descarga la versión correspondiente a tu sistema operativo desde [Releases](https://github.com/alan0018ad/opencad-construction-plugin/releases):
   * Windows: `opencad.construction-windows-x86_64.dll`
   * Linux: `opencad.construction-linux-x86_64.so`
   * macOS: `opencad.construction-macos-aarch64.dylib`
   * Y el archivo `plugin.toml`.
2. Coloca ambos archivos dentro de tu carpeta de plugins de Open CAD Studio:
   * **Windows:** `%APPDATA%\OpenCADStudio\plugins\opencad.construction\`
   * **Linux:** `~/.config/OpenCADStudio/plugins/opencad.construction/`
   * **macOS:** `~/Library/Application Support/OpenCADStudio/plugins/opencad.construction/`

---

## 🏗 Compilación desde el código fuente / Build from source

Requisitos: [Rust y Cargo](https://rustup.rs/) instalados.

```bash
git clone https://github.com/alan0018ad/opencad-construction-plugin.git
cd opencad-construction-plugin
cargo build --release
```

El archivo binario compilado estará en `target/release/`.

---

## 📄 Licencia / License

Distribuido bajo la licencia **GPL-3.0**. Consulta el archivo `LICENSE` para más detalles.
