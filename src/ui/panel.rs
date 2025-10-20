#[derive(Clone)]
pub enum Panel {
    Canvas,
    Tools,
    Properties,
    Library,
    Timeline,
}

impl Panel {
    pub(crate) fn label(&self) -> &str {
        match self {
            Panel::Canvas => "Canvas",
            Panel::Tools => "Tools",
            Panel::Properties => "Properties",
            Panel::Library => "Library",
            Panel::Timeline => "Timeline",
        }
    }
}
