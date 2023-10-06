use std::fmt;

#[derive(Clone, Default)]
pub enum RelKind {
    #[default]
    Rel,
    BiRel,
    RelUp,
    RelDown,
    RelLeft,
    RelRight,
    RelBack,
}

impl fmt::Display for RelKind {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let text = match self {
            RelKind::Rel => "Rel",
            RelKind::BiRel => "BiRel",
            RelKind::RelUp => "Rel_Up",
            RelKind::RelDown => "Rel_Down",
            RelKind::RelLeft => "Rel_Left",
            RelKind::RelRight => "Rel_Right",
            RelKind::RelBack => "Rel_Back",
        };
        write!(f, "{}", text)
    }
}
