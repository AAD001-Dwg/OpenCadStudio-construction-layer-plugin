use ocs_plugin_api::host::{
    codec::{
        entities::XLine,
        types::Vector3,
        EntityType,
    },
    CommandStep, InteractiveCommand, PreviewWire,
};

/// Interactive command to draw infinite construction lines (XLINE / CLINE)
/// on a dedicated construction or auxiliary layer.
pub struct ConstructionLineCommand {
    layer: String,
    base_point: Option<[f64; 3]>,
}

impl ConstructionLineCommand {
    pub fn new(layer: String) -> Self {
        Self {
            layer,
            base_point: None,
        }
    }
}

impl InteractiveCommand for ConstructionLineCommand {
    fn prompt(&self) -> String {
        if self.base_point.is_none() {
            "CLINE: Especifique punto de origen / Specify root point:".to_string()
        } else {
            "CLINE: Especifique dirección / Specify through point (Enter para finalizar):".to_string()
        }
    }

    fn on_point(&mut self, pt: [f64; 3]) -> CommandStep {
        match self.base_point {
            None => {
                self.base_point = Some(pt);
                CommandStep::NeedPoint
            }
            Some(base) => {
                let dx = pt[0] - base[0];
                let dy = pt[1] - base[1];
                let dz = pt[2] - base[2];
                let len = (dx * dx + dy * dy + dz * dz).sqrt();
                if len < 1e-9 {
                    return CommandStep::NeedPoint;
                }
                let dir = Vector3::new(dx / len, dy / len, dz / len);
                let bp = Vector3::new(base[0], base[1], base[2]);

                let mut xline = XLine::new(bp, dir);
                xline.common.layer = self.layer.clone();

                // Commit the construction line and keep the base point active
                // so the user can easily place multiple radiating guide lines.
                CommandStep::Commit(EntityType::XLine(xline))
            }
        }
    }

    fn on_enter(&mut self) -> CommandStep {
        CommandStep::Done
    }

    fn on_cursor_move(&mut self, pt: [f64; 3]) -> Vec<PreviewWire> {
        let base = match self.base_point {
            Some(b) => b,
            None => return Vec::new(),
        };

        let dx = pt[0] - base[0];
        let dy = pt[1] - base[1];
        let dz = pt[2] - base[2];
        let len = (dx * dx + dy * dy + dz * dz).sqrt();
        if len < 1e-9 {
            return Vec::new();
        }

        let dir = [dx / len, dy / len, dz / len];
        let extent = 1_000_000.0;
        let p_far_pos = [
            base[0] + dir[0] * extent,
            base[1] + dir[1] * extent,
            base[2] + dir[2] * extent,
        ];
        let p_far_neg = [
            base[0] - dir[0] * extent,
            base[1] - dir[1] * extent,
            base[2] - dir[2] * extent,
        ];

        vec![PreviewWire::Polyline {
            points: vec![p_far_neg, p_far_pos],
            color: Some([0.43, 0.72, 0.93, 0.85]), // Native OCS blue #6DB7ED
        }]
    }
}
