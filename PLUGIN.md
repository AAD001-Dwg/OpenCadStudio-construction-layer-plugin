# Construction Layers & Guides (`opencad.construction`)

A plugin for Open CAD Studio providing infinite construction lines, semi-infinite rays, reference circles, and dedicated construction layers inspired by LibreCAD and FreeCAD.

## Commands

| Command | English Alias | Spanish Alias | Description |
|---------|---------------|---------------|-------------|
| `CLINE` | `CONST_LINE`, `XLINE` | `LINEACONST` | Interactive infinite line in both directions with real-time viewport preview. |
| `CRAY` | `CONST_RAY` | `RAYO` | Interactive semi-infinite ray starting at a point and extending along a direction. |
| `CCIRCLE` | `CONST_CIRCLE` | `CIRCULOCONST` | Interactive auxiliary guide circle for bolt circles and reference radii. |
| `CLAYER` | `CONST_LAYER` | `CAPACONST`, `CAPAC` | Toggles construction layer mode (No-Plot, Reference-Only) on active, selected or named layer. |
| `CONST_SETUP` | | | Creates and activates the default `CONSTRUCTION` layer with Cyan color and No-Plot. |
| `CONST_TOGGLE` | | | Toggles visibility (ON / OFF) of the construction layer. |
| `CONST_CONVERT` | | | Converts selected normal `LINE` entities into infinite construction `XLINE`s. |
| `CONST_PURGE` | | | Purges and deletes all construction guide entities on the construction layer. |
| `CONST_SYNC` | | | Re-evaluates open document layers and reactivates reference-only mode on construction entities. |

## XDATA & DWG Compatibility (API v14)

- **APPID:** `OPENCAD_CONSTRUCTION`
- **Entity Classes (API v10):** Integrated display classes (`EntityClasses`) identifying `XLINE` as *"Línea de Construcción Infinita"*, `RAY` as *"Rayo Auxiliar de Referencia"*, `CCIRCLE` as *"Círculo Guía de Referencia"* and `LINE` as *"Guía de Construcción"* in the Open CAD Studio Properties inspector.
- **Layer Marker:** Tag `[CONSTRUCTION]` inside standard layer description (`AcDbLayerTableRecord.description`).
- **Plottable State:** Automatically marks layers as non-plottable (`is_plottable = false`).
- **Reference-Only Geometry (API v9+):** Marking entities as reference-only ensures mouse clicks pass through without accidental selection or grip disruption, while maintaining 100% OSNAP precision (endpoints, midpoints, intersections).
- **Batch Dirty Regions (API v13/v14):** Reacts to `HostNotification::DocumentChangedRegions` using `GetEntitiesBatch` for sub-millisecond $O(\text{changed})$ auto-conversion without cloning the document.
- **Toolchain Independent (API v14):** Uses `export_plugin!` with native `ocs_plugin_run` runner loop; no compiler lock-in to the host executable.
