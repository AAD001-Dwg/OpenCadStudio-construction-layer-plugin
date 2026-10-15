use ocs_plugin_api::host::{
    codec::{
        entities::XLine,
        types::{Color, Vector3},
        EntityType,
    },
    CommandStep, InteractiveCommand, PreviewWire,
};

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum LineOrientation {
    TwoPoints,
    Horizontal,
    Vertical,
    Angle(f64),
}

/// Interactive command to draw infinite construction lines (XLINE / CLINE)
/// on a dedicated construction or auxiliary layer.
pub struct ConstructionLineCommand {
    layer: String,
    orientation: LineOrientation,
    base_point: Option<[f64; 3]>,
}

impl ConstructionLineCommand {
    pub fn new(layer: String) -> Self {
        Self {
            layer,
            orientation: LineOrientation::TwoPoints,
            base_point: None,
        }
    }

    pub fn horizontal(layer: String) -> Self {
        Self {
            layer,
            orientation: LineOrientation::Horizontal,
            base_point: None,
        }
    }

    pub fn vertical(layer: String) -> Self {
        Self {
            layer,
            orientation: LineOrientation::Vertical,
            base_point: None,
        }
    }

    pub fn with_angle(layer: String, angle_deg: f64) -> Self {
        Self {
            layer,
            orientation: LineOrientation::Angle(angle_deg),
            base_point: None,
        }
    }
}

impl InteractiveCommand for ConstructionLineCommand {
    fn prompt(&self) -> String {
        match self.orientation {
            LineOrientation::TwoPoints => {
                if self.base_point.is_none() {
                    "CLINE: Especifique punto de origen / Specify root point:".to_string()
                } else {
                    "CLINE: Especifique dirección / Specify through point (Enter para finalizar):".to_string()
                }
            }
            LineOrientation::Horizontal => {
                "CLINE [Horizontal]: Especifique punto de paso / Specify through point (Enter para salir):".to_string()
            }
            LineOrientation::Vertical => {
                "CLINE [Vertical]: Especifique punto de paso / Specify through point (Enter para salir):".to_string()
            }
            LineOrientation::Angle(deg) => {
                format!("CLINE [Ángulo {deg}°]: Especifique punto de paso / Specify through point (Enter para salir):")
            }
        }
    }

    fn on_point(&mut self, pt: [f64; 3]) -> CommandStep {
        match self.orientation {
            LineOrientation::TwoPoints => match self.base_point {
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
                    xline.common.color = Color::Rgb { r: 109, g: 183, b: 237 };

                    CommandStep::Commit(EntityType::XLine(xline))
                }
            },
            LineOrientation::Horizontal => {
                let bp = Vector3::new(pt[0], pt[1], pt[2]);
                let dir = Vector3::new(1.0, 0.0, 0.0);
                let mut xline = XLine::new(bp, dir);
                xline.common.layer = self.layer.clone();
                xline.common.color = Color::Rgb { r: 109, g: 183, b: 237 };
                CommandStep::Commit(EntityType::XLine(xline))
            }
            LineOrientation::Vertical => {
                let bp = Vector3::new(pt[0], pt[1], pt[2]);
                let dir = Vector3::new(0.0, 1.0, 0.0);
                let mut xline = XLine::new(bp, dir);
                xline.common.layer = self.layer.clone();
                xline.common.color = Color::Rgb { r: 109, g: 183, b: 237 };
                CommandStep::Commit(EntityType::XLine(xline))
            }
            LineOrientation::Angle(deg) => {
                let rad = deg.to_radians();
                let bp = Vector3::new(pt[0], pt[1], pt[2]);
                let dir = Vector3::new(rad.cos(), rad.sin(), 0.0);
                let mut xline = XLine::new(bp, dir);
                xline.common.layer = self.layer.clone();
                xline.common.color = Color::Rgb { r: 109, g: 183, b: 237 };
                CommandStep::Commit(EntityType::XLine(xline))
            }
        }
    }

    fn on_enter(&mut self) -> CommandStep {
        CommandStep::Done
    }

    fn on_cursor_move(&mut self, pt: [f64; 3]) -> Vec<PreviewWire> {
        let extent = 1_000_000.0;
        let color = Some([0.43, 0.72, 0.93, 0.85]); // Native OCS blue #6DB7ED

        match self.orientation {
            LineOrientation::TwoPoints => {
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
                    color,
                }]
            }
            LineOrientation::Horizontal => vec![PreviewWire::Polyline {
                points: vec![
                    [pt[0] - extent, pt[1], pt[2]],
                    [pt[0] + extent, pt[1], pt[2]],
                ],
                color,
            }],
            LineOrientation::Vertical => vec![PreviewWire::Polyline {
                points: vec![
                    [pt[0], pt[1] - extent, pt[2]],
                    [pt[0], pt[1] + extent, pt[2]],
                ],
                color,
            }],
            LineOrientation::Angle(deg) => {
                let rad = deg.to_radians();
                let dx = rad.cos() * extent;
                let dy = rad.sin() * extent;
                vec![PreviewWire::Polyline {
                    points: vec![
                        [pt[0] - dx, pt[1] - dy, pt[2]],
                        [pt[0] + dx, pt[1] + dy, pt[2]],
                    ],
                    color,
                }]
            }
        }
    }
}
