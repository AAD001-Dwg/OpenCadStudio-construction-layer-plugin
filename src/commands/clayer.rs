use ocs_plugin_api::host::{
    codec::{
        entities::XLine,
        types::{Color, Vector3},
        EntityType, Handle,
    },
    HostApi, HostSettingValue, LayerConfig,
};

use super::{
    is_construction_layer_name_or_desc, CONSTRUCTION_LAYER_TAG, DEFAULT_CONST_LAYER,
};

/// Ensure that the construction layer exists with non-plottable attribute and guide color.
pub fn setup_construction_layer(host: &mut dyn HostApi, layer_name: &str, set_current: bool) {
    let exists = host.document().layers.contains(layer_name);

    if !exists {
        let desc = format!("{CONSTRUCTION_LAYER_TAG} Capa de guías y construcción");
        host.add_layer(LayerConfig {
            name: layer_name.to_string(),
            color: Some(Color::Index(4)), // Cyan
            plottable: Some(false),       // Non-plottable by default
            linetype: Some("Continuous".to_string()),
            description: Some(desc),
            ..Default::default()
        });
        host.push_info(&format!(
            "Capa '{layer_name}' creada (No-Plot: ACTIVADO, Color: Cyan)."
        ));
    } else {
        let existing_desc = host
            .document()
            .layers
            .get(layer_name)
            .map(|l| l.description.clone())
            .unwrap_or_default();

        let new_desc = if !existing_desc.contains(CONSTRUCTION_LAYER_TAG) {
            format!("{CONSTRUCTION_LAYER_TAG} {existing_desc}").trim().to_string()
        } else {
            existing_desc
        };

        host.modify_layer(LayerConfig {
            name: layer_name.to_string(),
            plottable: Some(false),
            description: Some(new_desc),
            ..Default::default()
        });
        host.push_info(&format!(
            "Capa '{layer_name}' actualizada como construcción (No-Plot: ACTIVADO)."
        ));
    }

    if set_current {
        let _ = host.set_system_variable("CLAYER", HostSettingValue::Text(layer_name.to_string()));
        host.push_info(&format!("Capa actual establecida en '{layer_name}'."));
    }

    host.bump_geometry();
    host.set_dirty();
}

/// Toggle any layer (or active layer / selected layers) as a Construction Layer.
pub fn toggle_layer_as_construction(host: &mut dyn HostApi, layer_arg: Option<&str>) {
    let layers_to_toggle: Vec<String> = match layer_arg {
        Some(name) => vec![name.to_string()],
        None => {
            let selected_handles = host.selection();
            if !selected_handles.is_empty() {
                let mut set = std::collections::BTreeSet::new();
                for h in selected_handles {
                    if let Some(e) = host.document().get_entity(h) {
                        set.insert(e.common().layer.clone());
                    }
                }
                set.into_iter().collect()
            } else {
                let active = match host.system_variable("CLAYER") {
                    Some(HostSettingValue::Text(t)) if !t.is_empty() => t,
                    _ => DEFAULT_CONST_LAYER.to_string(),
                };
                vec![active]
            }
        }
    };

    if layers_to_toggle.is_empty() {
        host.push_error("No se especificó ninguna capa para conmutar.");
        return;
    }

    host.push_undo("Toggle Construction Layer");

    for layer_name in layers_to_toggle {
        let (exists, desc, plottable) = match host.document().layers.get(&layer_name) {
            Some(l) => (true, l.description.clone(), l.is_plottable),
            None => (false, String::new(), true),
        };

        if !exists {
            setup_construction_layer(host, &layer_name, false);
            continue;
        }

        let is_const = is_construction_layer_name_or_desc(&layer_name, &desc);
        let new_is_const = !is_const;

        let new_desc = if new_is_const {
            if !desc.contains(CONSTRUCTION_LAYER_TAG) {
                format!("{CONSTRUCTION_LAYER_TAG} {}", desc.trim())
                    .trim()
                    .to_string()
            } else {
                desc
            }
        } else {
            desc.replace(CONSTRUCTION_LAYER_TAG, "").trim().to_string()
        };

        let new_plottable = if new_is_const { false } else { plottable };

        host.modify_layer(LayerConfig {
            name: layer_name.clone(),
            description: Some(new_desc),
            plottable: Some(new_plottable),
            ..Default::default()
        });

        // Set reference_only on all entities of this layer (API v9 / PR #1747)
        let handles: Vec<Handle> = host
            .document()
            .entities()
            .filter(|e| e.common().layer.eq_ignore_ascii_case(&layer_name))
            .map(|e| e.common().handle)
            .collect();

        for h in handles {
            let _ = host.set_reference_only(h, new_is_const);
        }

        let state_str = if new_is_const {
            "CAPA DE CONSTRUCCIÓN (No-Plot, Reference-Only)"
        } else {
            "CAPA NORMAL (Estándar)"
        };
        host.push_info(&format!("Capa '{layer_name}' ahora es: {state_str}."));
    }

    host.bump_geometry();
    host.set_dirty();
}

/// Toggle visibility (ON/OFF) of the construction layer.
pub fn toggle_construction_layer(host: &mut dyn HostApi, layer_name: &str) {
    let is_off = host
        .document()
        .layers
        .get(layer_name)
        .map(|l| l.flags.off)
        .unwrap_or(false);

    let new_state = !is_off;
    let modified = host.modify_layer(LayerConfig {
        name: layer_name.to_string(),
        off: Some(new_state),
        ..Default::default()
    });

    if modified {
        let status = if new_state {
            "OCULTA (OFF)"
        } else {
            "VISIBLE (ON)"
        };
        host.push_info(&format!("Capa '{layer_name}' ahora está {status}."));
    } else {
        host.push_error(&format!(
            "La capa '{layer_name}' no existe. Ejecuta CONST_SETUP primero."
        ));
    }

    host.bump_geometry();
    host.set_dirty();
}

/// Convert selected normal LINE entities into infinite XLINES on the construction layer.
pub fn convert_selection_to_construction(host: &mut dyn HostApi, target_layer: &str) {
    let selected_handles = host.selection();
    if selected_handles.is_empty() {
        host.push_error("No hay entidades seleccionadas. Selecciona líneas para convertirlas.");
        return;
    }

    let mut to_replace = Vec::new();

    for handle in selected_handles {
        if let Some(EntityType::Line(line)) = host.document().get_entity(handle) {
            let dx = line.end.x - line.start.x;
            let dy = line.end.y - line.start.y;
            let dz = line.end.z - line.start.z;
            let len = (dx * dx + dy * dy + dz * dz).sqrt();
            if len > 1e-9 {
                let dir = Vector3::new(dx / len, dy / len, dz / len);
                let bp = Vector3::new(line.start.x, line.start.y, line.start.z);

                let mut xline = XLine::new(bp, dir);
                xline.common.layer = target_layer.to_string();

                to_replace.push((handle, EntityType::XLine(xline)));
            }
        }
    }

    if to_replace.is_empty() {
        host.push_info("Ninguna de las entidades seleccionadas era una línea convertible.");
        return;
    }

    host.push_undo("Convert Lines to Construction Lines");

    let mut converted = 0usize;
    for (old_handle, new_entity) in to_replace {
        host.remove_entity(old_handle);
        let new_handle = host.add_entity(new_entity);
        let _ = host.set_reference_only(new_handle, true);
        converted += 1;
    }

    host.push_info(&format!(
        "Convertidas {converted} línea(s) en Líneas Infinitas de Construcción."
    ));
    host.bump_geometry();
    host.set_dirty();
}

/// Delete all construction entities on the construction layer.
pub fn purge_construction_guides(host: &mut dyn HostApi, layer_name: &str) {
    let handles_to_remove: Vec<_> = host
        .document()
        .entities()
        .filter(|e| e.common().layer.eq_ignore_ascii_case(layer_name))
        .map(|e| e.common().handle)
        .collect();

    let count = handles_to_remove.len();
    if count == 0 {
        host.push_info(&format!("No se encontraron entidades en la capa '{layer_name}'."));
        return;
    }

    host.push_undo("Purge Construction Guides");
    for handle in handles_to_remove {
        host.remove_entity(handle);
    }

    host.push_info(&format!(
        "Eliminadas {count} geometrías guía de la capa '{layer_name}'."
    ));
    host.bump_geometry();
    host.set_dirty();
}

/// Scan the document and apply `set_reference_only(handle, true)` on all entities
/// belonging to any construction layer upon DocumentOpened (API v9).
pub fn sync_document_construction_reference_only(host: &mut dyn HostApi) {
    let const_layers: std::collections::BTreeSet<String> = host
        .document()
        .layers
        .iter()
        .filter(|layer| is_construction_layer_name_or_desc(&layer.name, &layer.description))
        .map(|layer| layer.name.clone())
        .collect();

    if const_layers.is_empty() {
        return;
    }

    let entities_to_mark: Vec<Handle> = host
        .document()
        .entities()
        .filter(|e| const_layers.contains(&e.common().layer))
        .map(|e| e.common().handle)
        .collect();

    let count = entities_to_mark.len();
    for h in entities_to_mark {
        let _ = host.set_reference_only(h, true);
    }

    if count > 0 {
        host.push_info(&format!(
            "Complemento de Construcción: {count} entidades de referencia reactivadas."
        ));
    }
}
