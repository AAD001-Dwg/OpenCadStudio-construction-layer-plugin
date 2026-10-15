pub mod ccircle;
pub mod clayer;
pub mod cline;
pub mod cray;

pub const DEFAULT_CONST_LAYER: &str = "CONSTRUCTION";
pub const CONSTRUCTION_APPID: &str = "OPENCAD_CONSTRUCTION";
pub const CONSTRUCTION_LAYER_TAG: &str = "[CONSTRUCTION]";

/// Check whether a layer name or description marks it as a construction layer.
pub fn is_construction_layer_name_or_desc(name: &str, desc: &str) -> bool {
    let name_upper = name.to_ascii_uppercase();
    desc.contains(CONSTRUCTION_LAYER_TAG)
        || name_upper == "CONSTRUCTION"
        || name_upper == "CONSTRUCCION"
        || name_upper == "DEFPOINTS_CONST"
        || name_upper.starts_with("CONST_")
        || name_upper.starts_with("CONST-")
        || name_upper.starts_with("GUIA_")
        || name_upper.starts_with("GUIA-")
        || name_upper.starts_with("GUÍA_")
        || name_upper.starts_with("GUÍA-")
        || name_upper.starts_with("AUX_")
        || name_upper.starts_with("AUX-")
        || name_upper.starts_with("REF_")
        || name_upper.starts_with("REF-")
}
