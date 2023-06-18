pub struct Transition<TTransition, TStates> {
    pub alias: TTransition,
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

impl<TTransition, TStates> Transition<TTransition, TStates> {
    pub fn new(alias: TTransition, begin: TStates, end: TStates) -> Self {
        Self {
            alias,
            begin,
            end,
            description: None,
            option: TransitionOption::No,
        }
    }

    pub fn set_description(&mut self, description: &str) -> &mut Self {
        self.description = Some(String::from(description));
        self
    }

    pub fn set_option(&mut self, option: TransitionOption) -> &mut Self {
        self.option = option;
        self
    }
}

#[cfg(test)]
mod tests {
    use derive_more::Display;

    use super::*;

    #[test]
    fn test() {
        #[derive(Debug, Display, PartialEq, Clone)]
        enum Trans {
            Trans1,
        }

        enum States {
            State1,
            State2,
        }

        let mut trans =
            Transition::new(Trans::Trans1, States::State1, States::State2);

        trans
            .set_description("trans description")
            .set_option(TransitionOption::DeepHistory);

        assert_eq!(trans.alias, Trans::Trans1);
    }
}
