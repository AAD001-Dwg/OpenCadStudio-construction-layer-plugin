//! Construction Layers & Drafting Guides Plugin for Open CAD Studio (API v9).
//!
//! Inspired by LibreCAD and FreeCAD:
//! - Provides infinite construction lines (CLINE / XLINE) with real-time viewport preview.
//! - Direct presets for Horizontal (CLINE H) and Vertical (CLINE V) construction lines.
//! - Provides semi-infinite rays (CRAY) and reference circles (CCIRCLE).
//! - Designates ANY layer as a Construction Layer (CLAYER / CAPACONST).
//! - Marks construction geometry with `reference_only` (API v9) so clicks pass through
//!   without interfering with drafting while preserving 100% OSNAP precision.
//! - Non-blocking asynchronous worker thread integration (via `PluginRequestSender`):
//!   * Reacts to `HostNotification::DocumentOpened` to reactivate reference-only state.
//!   * Reacts to `HostNotification::DrawingChanged` to auto-convert normal lines drawn
//!     on construction layers into infinite construction lines automatically!
//! - Ensures non-plottable attributes (No-Plot) and DWG/DXF persistence via standard XDATA and layer metadata.
//! - Quick toggle, convert, and purge commands.

pub mod commands;
pub mod ribbon;

use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::{Arc, Mutex};

use ocs_plugin_api::host::{
    codec::{
        entities::XLine,
        types::{Color, Vector3},
        EntityType, Handle,
    },
    BuiltinPlugin, HostApi, HostNotification, PluginRequestSender,
};
use ocs_plugin_api::ipc::protocol::{PluginRequest, PluginResponse};
use ocs_plugin_api::manifest::{ApiVersion, EntityClasses, PluginManifest};
use ocs_plugin_api::ribbon::CadModule;

use crate::commands::{
    ccircle::ConstructionCircleCommand,
    clayer::{
        convert_selection_to_construction, purge_construction_guides, setup_construction_layer,
        sync_document_construction_reference_only, toggle_construction_layer,
        toggle_layer_as_construction,
    },
    cline::ConstructionLineCommand,
    cray::ConstructionRayCommand,
    is_construction_layer_name_or_desc, CONSTRUCTION_APPID, DEFAULT_CONST_LAYER,
};
use crate::ribbon::ConstructionModule;

pub(crate) const PLUGIN_ID: &str = "opencad.construction";

static MANIFEST: PluginManifest = PluginManifest {
    id: PLUGIN_ID,
    name: "Construction Layers & Guides",
    version: env!("CARGO_PKG_VERSION"),
    description: "Construction layers, infinite lines, rays, and reference guides inspired by LibreCAD.",
    api_version: ApiVersion { major: 14 },
    ribbon_order: 55,
    xdata_apps: &[CONSTRUCTION_APPID],
    command_prefixes: &[
        "CONST_", "CLINE", "CRAY", "CCIRCLE", "CLAYER", "CAPACONST", "CAPAC",
        "CL", "CH", "CV", "CR", "CC", "CY", "CT",
    ],
};

static ENTITY_CLASSES: EntityClasses = EntityClasses {
    app_id: CONSTRUCTION_APPID,
    class_prefix: "CONST-CLASS:",
    labels: &[
        ("XLINE", "Línea de Construcción Infinita"),
        ("RAY", "Rayo Auxiliar de Referencia"),
        ("CCIRCLE", "Círculo Guía de Referencia"),
        ("LINE", "Guía de Construcción"),
    ],
};

pub struct ConstructionPlugin {
    sender: Arc<Mutex<Option<Arc<dyn PluginRequestSender>>>>,
    processing_change: Arc<AtomicBool>,
}

impl Default for ConstructionPlugin {
    fn default() -> Self {
        Self {
            sender: Arc::new(Mutex::new(None)),
            processing_change: Arc::new(AtomicBool::new(false)),
        }
    }
}

impl BuiltinPlugin for ConstructionPlugin {
    fn manifest(&self) -> &'static PluginManifest {
        &MANIFEST
    }

    fn ribbon(&self) -> Box<dyn CadModule> {
        Box::new(ConstructionModule)
    }

    fn entity_classes(&self) -> Option<&'static EntityClasses> {
        Some(&ENTITY_CLASSES)
    }

    fn dispatch(&self, host: &mut dyn HostApi, cmd: &str) -> bool {
        let parts: Vec<&str> = cmd.trim().split_whitespace().collect();
        if parts.is_empty() {
            return false;
        }

        let cmd_name = parts[0].to_uppercase();
        let arg = parts.get(1).copied();

        match cmd_name.as_str() {
            "CLINE" | "CONST_LINE" | "XLINE" | "LINEACONST" | "CL" => {
                setup_construction_layer(host, DEFAULT_CONST_LAYER, false);
                let command = match arg.map(|s| s.to_uppercase()).as_deref() {
                    Some("H") | Some("HOR") | Some("HORIZONTAL") => {
                        ConstructionLineCommand::horizontal(DEFAULT_CONST_LAYER.to_string())
                    }
                    Some("V") | Some("VER") | Some("VERTICAL") => {
                        ConstructionLineCommand::vertical(DEFAULT_CONST_LAYER.to_string())
                    }
                    Some(val) => {
                        if let Ok(angle) = val.parse::<f64>() {
                            ConstructionLineCommand::with_angle(DEFAULT_CONST_LAYER.to_string(), angle)
                        } else {
                            ConstructionLineCommand::new(DEFAULT_CONST_LAYER.to_string())
                        }
                    }
                    None => ConstructionLineCommand::new(DEFAULT_CONST_LAYER.to_string()),
                };
                host.start_interactive(Box::new(command));
                true
            }
            "CLINE_H" | "XLINE_H" | "CH" => {
                setup_construction_layer(host, DEFAULT_CONST_LAYER, false);
                host.start_interactive(Box::new(ConstructionLineCommand::horizontal(
                    DEFAULT_CONST_LAYER.to_string(),
                )));
                true
            }
            "CLINE_V" | "XLINE_V" | "CV" => {
                setup_construction_layer(host, DEFAULT_CONST_LAYER, false);
                host.start_interactive(Box::new(ConstructionLineCommand::vertical(
                    DEFAULT_CONST_LAYER.to_string(),
                )));
                true
            }
            "CRAY" | "CONST_RAY" | "RAYO" | "CR" => {
                setup_construction_layer(host, DEFAULT_CONST_LAYER, false);
                host.start_interactive(Box::new(ConstructionRayCommand::new(
                    DEFAULT_CONST_LAYER.to_string(),
                )));
                true
            }
            "CCIRCLE" | "CONST_CIRCLE" | "CIRCULOCONST" | "CC" => {
                setup_construction_layer(host, DEFAULT_CONST_LAYER, false);
                host.start_interactive(Box::new(ConstructionCircleCommand::new(
                    DEFAULT_CONST_LAYER.to_string(),
                )));
                true
            }
            "CLAYER" | "CONST_LAYER" | "CAPACONST" | "CAPAC" | "CY" => {
                toggle_layer_as_construction(host, arg);
                true
            }
            "CONST_SETUP" => {
                setup_construction_layer(host, DEFAULT_CONST_LAYER, true);
                true
            }
            "CONST_TOGGLE" | "CT" => {
                toggle_construction_layer(host, DEFAULT_CONST_LAYER);
                true
            }
            "CONST_CONVERT" => {
                setup_construction_layer(host, DEFAULT_CONST_LAYER, false);
                convert_selection_to_construction(host, DEFAULT_CONST_LAYER);
                true
            }
            "CONST_PURGE" => {
                purge_construction_guides(host, DEFAULT_CONST_LAYER);
                true
            }
            "CONST_SYNC" => {
                sync_document_construction_reference_only(host);
                true
            }
            _ => false,
        }
    }

    fn on_load(&mut self, host: &mut dyn HostApi) {
        if let Some(s) = host.plugin_request_sender() {
            let mut guard = self.sender.lock().unwrap();
            *guard = Some(Arc::from(s));
        }
    }

    fn on_notification(&mut self, _command_id: Option<u64>, notification: HostNotification) {
        match notification {
            HostNotification::DocumentOpened { tab_id: _ } => {
                let sender_opt = self.sender.lock().unwrap().clone();
                if let Some(sender) = sender_opt {
                    std::thread::spawn(move || {
                        handle_document_opened_async(&*sender);
                    });
                }
            }
            HostNotification::DocumentChangedRegions {
                tab_id: _,
                version: _,
                entities,
                records: _,
            } => {
                if !entities.is_empty() {
                    let sender_opt = self.sender.lock().unwrap().clone();
                    let busy = Arc::clone(&self.processing_change);
                    if let Some(sender) = sender_opt {
                        std::thread::spawn(move || {
                            handle_changed_regions_async(&*sender, busy, entities);
                        });
                    }
                }
            }
            HostNotification::DrawingChanged { tab_id: _, epoch: _ } => {
                let sender_opt = self.sender.lock().unwrap().clone();
                let busy = Arc::clone(&self.processing_change);
                if let Some(sender) = sender_opt {
                    std::thread::spawn(move || {
                        handle_drawing_changed_async(&*sender, busy);
                    });
                }
            }
            _ => {}
        }
    }
}

/// Asynchronous document handler for DocumentOpened (no runner deadlocks).
fn handle_document_opened_async(sender: &dyn PluginRequestSender) {
    let Ok(PluginResponse::Document(doc)) = sender.request(PluginRequest::DocumentSnapshot) else {
        return;
    };

    let const_layers: std::collections::BTreeSet<String> = doc
        .layers
        .iter()
        .filter(|layer| is_construction_layer_name_or_desc(&layer.name, &layer.description))
        .map(|layer| layer.name.clone())
        .collect();

    if const_layers.is_empty() {
        return;
    }

    let mut marked = 0usize;
    for e in doc.entities() {
        if const_layers.contains(&e.common().layer) {
            let _ = sender.request(PluginRequest::SetReferenceOnly {
                handle: e.common().handle,
                reference_only: true,
            });
            marked += 1;
        }
    }

    if marked > 0 {
        let _ = sender.request(PluginRequest::BumpGeometry);
        let _ = sender.request(PluginRequest::PushInfo(format!(
            "Guías de Construcción: {marked} entidad(es) de referencia reactivadas."
        )));
    }
}

/// Reactive auto-conversion on DrawingChanged (LibreCAD-style drafting ergonomics).
/// When finite LINEs are drawn on a construction layer, auto-convert to infinite lines
/// and mark them reference-only immediately.
fn handle_drawing_changed_async(sender: &dyn PluginRequestSender, busy: Arc<AtomicBool>) {
    // Re-entrancy guard to avoid recursive feedback loops
    if busy.compare_exchange(false, true, Ordering::SeqCst, Ordering::SeqCst).is_err() {
        return;
    }

    struct Guard(Arc<AtomicBool>);
    impl Drop for Guard {
        fn drop(&mut self) {
            self.0.store(false, Ordering::SeqCst);
        }
    }
    let _guard = Guard(Arc::clone(&busy));

    let Ok(PluginResponse::Document(doc)) = sender.request(PluginRequest::DocumentSnapshot) else {
        return;
    };

    let const_layers: std::collections::BTreeSet<String> = doc
        .layers
        .iter()
        .filter(|layer| is_construction_layer_name_or_desc(&layer.name, &layer.description))
        .map(|layer| layer.name.clone())
        .collect();

    if const_layers.is_empty() {
        return;
    }

    let mut to_convert = Vec::new();
    for e in doc.entities() {
        if let EntityType::Line(line) = e {
            if const_layers.contains(&line.common.layer) {
                let dx = line.end.x - line.start.x;
                let dy = line.end.y - line.start.y;
                let dz = line.end.z - line.start.z;
                let len = (dx * dx + dy * dy + dz * dz).sqrt();

                // Standard finite user-drawn line (< 500k units) on construction layer
                if len > 1e-4 && len < 500_000.0 {
                    let dir = Vector3::new(dx / len, dy / len, dz / len);
                    let bp = Vector3::new(line.start.x, line.start.y, line.start.z);

                    let mut xline = XLine::new(bp, dir);
                    xline.common.layer = line.common.layer.clone();
                    xline.common.color = Color::Rgb { r: 109, g: 183, b: 237 };

                    to_convert.push((line.common.handle, EntityType::XLine(xline)));
                }
            }
        }
    }

    if to_convert.is_empty() {
        return;
    }

    let _ = sender.request(PluginRequest::PushUndo {
        label: "Auto-Convert Construction Lines".to_string(),
    });

    let count = to_convert.len();
    for (old_handle, new_entity) in to_convert {
        let _ = sender.request(PluginRequest::RemoveEntity { handle: old_handle });
        if let Ok(PluginResponse::Handle(new_handle)) = sender.request(PluginRequest::AddEntity(new_entity)) {
            let _ = sender.request(PluginRequest::SetReferenceOnly {
                handle: new_handle,
                reference_only: true,
            });
        }
    }

    let _ = sender.request(PluginRequest::BumpGeometry);
    let _ = sender.request(PluginRequest::SetDirty);
    let _ = sender.request(PluginRequest::PushInfo(format!(
        "Guías de Construcción: {count} línea(s) convertidas automáticamente a guías infinitas."
    )));
}

/// O(changed) reactive auto-conversion using API v13/v14 dirty regions.
fn handle_changed_regions_async(
    sender: &dyn PluginRequestSender,
    busy: Arc<AtomicBool>,
    handles: Vec<Handle>,
) {
    if handles.is_empty() {
        return;
    }

    if busy.compare_exchange(false, true, Ordering::SeqCst, Ordering::SeqCst).is_err() {
        return;
    }

    struct Guard(Arc<AtomicBool>);
    impl Drop for Guard {
        fn drop(&mut self) {
            self.0.store(false, Ordering::SeqCst);
        }
    }
    let _guard = Guard(Arc::clone(&busy));

    let Ok(PluginResponse::GetEntitiesBatchResult(Ok(entities_opt))) =
        sender.request(PluginRequest::GetEntitiesBatch {
            handles: handles.clone(),
        })
    else {
        return;
    };

    let mut to_convert: Vec<(Handle, EntityType)> = Vec::new();

    for (handle, maybe_entity) in handles.into_iter().zip(entities_opt.into_iter()) {
        let Some(EntityType::Line(line)) = maybe_entity else {
            continue;
        };

        if !is_construction_layer_name_or_desc(&line.common.layer, "") {
            continue;
        }

        let p1 = line.start;
        let p2 = line.end;
        let dx = p2.x - p1.x;
        let dy = p2.y - p1.y;
        let dz = p2.z - p1.z;
        let len = (dx * dx + dy * dy + dz * dz).sqrt();

        if len < 1e-6 {
            continue;
        }

        let dir = Vector3::new(dx / len, dy / len, dz / len);
        let mut xline = XLine::new(p1, dir);
        xline.common.layer = line.common.layer.clone();
        xline.common.color = if line.common.color == Color::ByLayer {
            Color::Rgb {
                r: 109,
                g: 183,
                b: 237,
            }
        } else {
            line.common.color
        };

        to_convert.push((handle, EntityType::XLine(xline)));
    }

    if to_convert.is_empty() {
        return;
    }

    let _ = sender.request(PluginRequest::PushUndo {
        label: "Auto-Convert Construction Lines (Regions)".to_string(),
    });

    let count = to_convert.len();
    for (old_handle, new_entity) in to_convert {
        let _ = sender.request(PluginRequest::RemoveEntity { handle: old_handle });
        if let Ok(PluginResponse::Handle(new_handle)) =
            sender.request(PluginRequest::AddEntity(new_entity))
        {
            let _ = sender.request(PluginRequest::SetReferenceOnly {
                handle: new_handle,
                reference_only: true,
            });
        }
    }

    let _ = sender.request(PluginRequest::BumpGeometry);
    let _ = sender.request(PluginRequest::SetDirty);
    let _ = sender.request(PluginRequest::PushInfo(format!(
        "Guías de Construcción: {count} línea(s) convertidas automáticamente a guías infinitas (región rápida)."
    )));
}

// Export the C-ABI symbols expected by Open CAD Studio plugin loader for API v14.
ocs_plugin_api::export_plugin!(ConstructionPlugin::default());
