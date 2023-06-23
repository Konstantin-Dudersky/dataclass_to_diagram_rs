pub struct Transition<TStates> {
    pub begin: TStates,
    pub end: TStates,
    pub description: Option<String>,
    pub option: TransitionOption,
}

#[derive(Clone, Debug, Default, PartialEq)]
pub enum TransitionOption {
    #[default]
    No,
    History,
    DeepHistory,
}

impl<TStates> Transition<TStates> {
    pub fn new(begin: TStates, end: TStates) -> Self {
        Self {
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

    use super::*;

    #[test]
    fn test() {
        #[derive(Debug, PartialEq)]
        enum States {
            State1,
            State2,
        }

        let mut trans = Transition::new(States::State1, States::State2);

        trans
            .set_description("trans description")
            .set_option(TransitionOption::DeepHistory);

        assert_eq!(trans.begin, States::State1);
        assert_eq!(trans.end, States::State2);
        assert_eq!(trans.description.unwrap(), "trans description");
        assert_eq!(trans.option, TransitionOption::DeepHistory);
    }
}
