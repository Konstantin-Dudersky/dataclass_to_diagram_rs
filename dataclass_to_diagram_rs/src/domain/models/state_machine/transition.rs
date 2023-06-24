pub struct Transition<TStates> {
    pub order: u32,
    pub begin: TStates,
    pub end: TStates,
    pub description: Option<String>,
}

impl<TStates> Transition<TStates> {
    pub fn new(begin: TStates, end: TStates, order: u32) -> Self {
        Self {
            order,
            begin,
            end,
            description: None,
        }
    }

    pub fn set_description(&mut self, description: &str) -> &mut Self {
        self.description = Some(String::from(description));
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

        let mut trans = Transition::new(States::State1, States::State2, 0);

        trans.set_description("trans description");

        assert_eq!(trans.begin, States::State1);
        assert_eq!(trans.end, States::State2);
        assert_eq!(trans.description.unwrap(), "trans description");
    }
}
