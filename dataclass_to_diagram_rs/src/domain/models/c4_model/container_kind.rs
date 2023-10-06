use std::fmt;

#[derive(Clone, Default)]
pub enum ContainerKind {
    #[default]
    Container,
    ContainerDb,
    ContainerQueue,
    ContainerExt,
    ContainerDbExt,
    ContainerQueueExt,
    ContainerBoundary,
}

impl fmt::Display for ContainerKind {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let text = match self {
            ContainerKind::Container => "Container",
            ContainerKind::ContainerDb => "ContainerDb",
            ContainerKind::ContainerQueue => "ContainerQueue",
            ContainerKind::ContainerExt => "Container_Ext",
            ContainerKind::ContainerDbExt => "ContainerDb_Ext",
            ContainerKind::ContainerQueueExt => "ContainerQueue_Ext",
            ContainerKind::ContainerBoundary => "Container_Boundary",
        };
        write!(f, "{}", text)
    }
}
