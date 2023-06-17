pub struct Transition<TStates> {
    pub begin: TStates,
    pub end: TStates,
    pub description: Option<String>,
    pub option: TransitionOption,
}

#[derive(Debug, Default, PartialEq)]
pub enum TransitionOption {
    #[default]
    No,
    History,
    DeepHistory,
}

impl<TStates> Transition<TStates> {
    pub fn new(
        begin: TStates,
        end: TStates,
        description: Option<&str>,
    ) -> Self {
    }
}
