//! Construction Layers & Drafting Guides Plugin for Open CAD Studio.
//!
//! Inspired by LibreCAD and FreeCAD:
//! - Provides infinite construction lines (CLINE / XLINE) with real-time viewport preview.
//! - Provides semi-infinite rays (CRAY) and reference circles (CCIRCLE).
//! - Allows designating ANY layer as a Construction Layer (CLAYER / CAPACONST).
//! - Marks construction geometry with `reference_only` (API v9) so clicks pass through
//!   without interfering with drafting while preserving 100% OSNAP precision.
//! - Ensures non-plottable attributes (No-Plot) and DWG/DXF persistence via standard XDATA and layer metadata.
//! - Quick toggle, convert, and purge commands.

pub mod commands;
pub mod ribbon;

use ocs_plugin_api::host::{BuiltinPlugin, HostApi, HostNotification};
use ocs_plugin_api::manifest::{ApiVersion, PluginManifest};
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
    CONSTRUCTION_APPID, DEFAULT_CONST_LAYER,
};
use crate::ribbon::ConstructionModule;

pub(crate) const PLUGIN_ID: &str = "opencad.construction";

static MANIFEST: PluginManifest = PluginManifest {
    id: PLUGIN_ID,
    name: "Construction Layers & Guides",
    version: env!("CARGO_PKG_VERSION"),
    description: "Construction layers, infinite lines, and reference guides inspired by LibreCAD.",
    api_version: ApiVersion { major: 9 },
    ribbon_order: 55,
    xdata_apps: &[CONSTRUCTION_APPID],
    command_prefixes: &["CONST_", "CLINE", "CRAY", "CCIRCLE", "CLAYER", "CAPACONST", "CAPAC"],
};

#[derive(Default)]
struct ConstructionPlugin;

impl BuiltinPlugin for ConstructionPlugin {
    fn manifest(&self) -> &'static PluginManifest {
        &MANIFEST
    }

    fn ribbon(&self) -> Box<dyn CadModule> {
        Box::new(ConstructionModule)
    }

    fn dispatch(&self, host: &mut dyn HostApi, cmd: &str) -> bool {
        let parts: Vec<&str> = cmd.trim().split_whitespace().collect();
        if parts.is_empty() {
            return false;
        }

        let cmd_name = parts[0].to_uppercase();
        let arg = parts.get(1).copied();

        match cmd_name.as_str() {
            "CLINE" | "CONST_LINE" | "XLINE" | "LINEACONST" => {
                setup_construction_layer(host, DEFAULT_CONST_LAYER, false);
                host.start_interactive(Box::new(ConstructionLineCommand::new(
                    DEFAULT_CONST_LAYER.to_string(),
                )));
                true
            }
            "CRAY" | "CONST_RAY" | "RAYO" => {
                setup_construction_layer(host, DEFAULT_CONST_LAYER, false);
                host.start_interactive(Box::new(ConstructionRayCommand::new(
                    DEFAULT_CONST_LAYER.to_string(),
                )));
                true
            }
            "CCIRCLE" | "CONST_CIRCLE" | "CIRCULOCONST" => {
                setup_construction_layer(host, DEFAULT_CONST_LAYER, false);
                host.start_interactive(Box::new(ConstructionCircleCommand::new(
                    DEFAULT_CONST_LAYER.to_string(),
                )));
                true
            }
            "CLAYER" | "CONST_LAYER" | "CAPACONST" | "CAPAC" => {
                toggle_layer_as_construction(host, arg);
                true
            }
            "CONST_SETUP" => {
                setup_construction_layer(host, DEFAULT_CONST_LAYER, true);
                true
            }
            "CONST_TOGGLE" => {
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

    fn on_notification(&mut self, _command_id: Option<u64>, _notification: HostNotification) {
        // Notification channel available for future IPC expansion
    }
}

// Export the C-ABI symbols expected by Open CAD Studio plugin loader for API v9.
#[no_mangle]
pub extern "C" fn ocs_plugin_api_version() -> u32 {
    9
}

#[no_mangle]
pub extern "C" fn ocs_plugin_register() -> *mut ::std::boxed::Box<dyn BuiltinPlugin> {
    match ::std::panic::catch_unwind(::std::panic::AssertUnwindSafe(|| {
        let plugin: ::std::boxed::Box<dyn BuiltinPlugin> =
            ::std::boxed::Box::new(ConstructionPlugin::default());
        ::std::boxed::Box::into_raw(::std::boxed::Box::new(plugin))
    })) {
        ::std::result::Result::Ok(ptr) => ptr,
        ::std::result::Result::Err(_) => ::std::ptr::null_mut(),
    }
}
