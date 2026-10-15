use ocs_plugin_api::host::{
    codec::{
        entities::Circle,
        types::Vector3,
        EntityType,
    },
    CommandStep, InteractiveCommand, PreviewWire,
};

/// Interactive command to draw construction circles on the
/// dedicated construction layer.
pub struct ConstructionCircleCommand {
    layer: String,
    center_point: Option<[f64; 3]>,
}

impl ConstructionCircleCommand {
    pub fn new(layer: String) -> Self {
        Self {
            layer,
            center_point: None,
        }
    }
}

impl InteractiveCommand for ConstructionCircleCommand {
    fn prompt(&self) -> String {
        if self.center_point.is_none() {
            "CCIRCLE: Especifique centro del círculo guía / Specify center point:".to_string()
        } else {
            "CCIRCLE: Especifique radio / Specify radius point:".to_string()
        }
    }

    fn on_point(&mut self, pt: [f64; 3]) -> CommandStep {
        match self.center_point {
            None => {
                self.center_point = Some(pt);
                CommandStep::NeedPoint
            }
            Some(center) => {
                let dx = pt[0] - center[0];
                let dy = pt[1] - center[1];
                let radius = (dx * dx + dy * dy).sqrt();
                if radius < 1e-9 {
                    return CommandStep::NeedPoint;
                }

                let mut circle = Circle::new();
                circle.center = Vector3::new(center[0], center[1], center[2]);
                circle.radius = radius;
                circle.common.layer = self.layer.clone();

                CommandStep::CommitAndEnd(EntityType::Circle(circle))
            }
        }
    }

    fn on_enter(&mut self) -> CommandStep {
        CommandStep::Cancel
    }

    fn on_cursor_move(&mut self, pt: [f64; 3]) -> Vec<PreviewWire> {
        let center = match self.center_point {
            Some(c) => c,
            None => return Vec::new(),
        };

        let dx = pt[0] - center[0];
        let dy = pt[1] - center[1];
        let radius = (dx * dx + dy * dy).sqrt();
        if radius < 1e-9 {
            return Vec::new();
        }

        vec![PreviewWire::Circle {
            center,
            radius,
            color: Some([0.43, 0.72, 0.93, 0.85]), // Native OCS blue
        }]
    }
}
