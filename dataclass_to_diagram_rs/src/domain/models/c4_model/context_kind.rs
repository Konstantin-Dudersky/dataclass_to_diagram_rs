use std::fmt;

#[derive(Clone, Default)]
pub enum ContextKind {
    Person,
    PersonExt,
    #[default]
    System,
    SystemDb,
    SystemQueue,
    SystemExt,
    SystemDbExt,
    SystemQueueExt,
    Boundary,
    EnterpriseBoundary,
    SystemBoundary,
}

impl fmt::Display for ContextKind {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ContextKind::Person => write!(f, "Person"),
            ContextKind::PersonExt => write!(f, "Person_Ext"),
            ContextKind::System => write!(f, "System"),
            ContextKind::SystemDb => write!(f, "SystemDb"),
            ContextKind::SystemQueue => write!(f, "SystemQueue"),
            ContextKind::SystemExt => write!(f, "System_Ext"),
            ContextKind::SystemDbExt => write!(f, "SystemDb_Ext"),
            ContextKind::SystemQueueExt => write!(f, "SystemQueue_Ext"),
            ContextKind::Boundary => write!(f, "Boundary"),
            ContextKind::EnterpriseBoundary => write!(f, "Enterprise_Boundary"),
            ContextKind::SystemBoundary => write!(f, "System_Boundary"),
        }
    }
}
