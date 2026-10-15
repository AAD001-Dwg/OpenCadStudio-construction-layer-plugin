use ocs_plugin_api::ribbon::{
    CadModule, IconKind, ModuleEvent, RibbonGroup, RibbonItem, ToolDef,
};

pub struct ConstructionModule;

impl CadModule for ConstructionModule {
    fn id(&self) -> &'static str {
        "opencad_construction"
    }

    fn title(&self) -> &'static str {
        "Construcción"
    }

    fn ribbon_groups(&self) -> &[RibbonGroup] {
        static GROUPS: std::sync::OnceLock<Vec<RibbonGroup>> = std::sync::OnceLock::new();
        GROUPS.get_or_init(|| {
            vec![
                RibbonGroup {
                    title: "Trazado de Guías",
                    tools: vec![
                        RibbonItem::LargeTool(ToolDef {
                            id: "CLINE",
                            label: "Línea Infinita",
                            icon: IconKind::Svg(include_bytes!("../icons/cline.svg")),
                            event: ModuleEvent::Command("CLINE".to_string()),
                        }),
                        RibbonItem::Tool(ToolDef {
                            id: "CLINE_H",
                            label: "Horizontal",
                            icon: IconKind::Glyph("―"),
                            event: ModuleEvent::Command("CLINE H".to_string()),
                        }),
                        RibbonItem::Tool(ToolDef {
                            id: "CLINE_V",
                            label: "Vertical",
                            icon: IconKind::Glyph("|"),
                            event: ModuleEvent::Command("CLINE V".to_string()),
                        }),
                        RibbonItem::LargeTool(ToolDef {
                            id: "CRAY",
                            label: "Rayo Guía",
                            icon: IconKind::Svg(include_bytes!("../icons/cray.svg")),
                            event: ModuleEvent::Command("CRAY".to_string()),
                        }),
                        RibbonItem::LargeTool(ToolDef {
                            id: "CCIRCLE",
                            label: "Círculo Guía",
                            icon: IconKind::Svg(include_bytes!("../icons/ccircle.svg")),
                            event: ModuleEvent::Command("CCIRCLE".to_string()),
                        }),
                    ],
                },
                RibbonGroup {
                    title: "Capas de Construcción",
                    tools: vec![
                        RibbonItem::LargeTool(ToolDef {
                            id: "CLAYER",
                            label: "Alternar Capa",
                            icon: IconKind::Svg(include_bytes!("../icons/clayer.svg")),
                            event: ModuleEvent::Command("CLAYER".to_string()),
                        }),
                        RibbonItem::LargeTool(ToolDef {
                            id: "CONST_TOGGLE",
                            label: "Ocultar/Ver",
                            icon: IconKind::Svg(include_bytes!("../icons/toggle.svg")),
                            event: ModuleEvent::Command("CONST_TOGGLE".to_string()),
                        }),
                        RibbonItem::LargeTool(ToolDef {
                            id: "CONST_CONVERT",
                            label: "A Guías",
                            icon: IconKind::Svg(include_bytes!("../icons/convert.svg")),
                            event: ModuleEvent::Command("CONST_CONVERT".to_string()),
                        }),
                        RibbonItem::LargeTool(ToolDef {
                            id: "CONST_PURGE",
                            label: "Limpiar",
                            icon: IconKind::Svg(include_bytes!("../icons/purge.svg")),
                            event: ModuleEvent::Command("CONST_PURGE".to_string()),
                        }),
                    ],
                },
            ]
        })
    }
}
