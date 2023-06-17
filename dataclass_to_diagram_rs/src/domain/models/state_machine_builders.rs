use super::state_machine::{State, Transition, TransitionOption};

// Transition ------------------------------------------------------------------

pub struct TransitionBuilder<'a> {
    begin: &'a State,
    end: &'a State,
    description: Option<&'a str>,
    option: TransitionOption,
}

impl<'a> TransitionBuilder<'a> {
    pub fn new(begin: &'a State, end: &'a State) -> Self {
        Self {
            begin,
            end,
            description: None,
            option: TransitionOption::No,
        }
    }

    pub fn set_description(self, description: &'a str) -> Self {
        Self {
            description: Some(description),
            ..self
        }
    }

    pub fn set_option(self, option: TransitionOption) -> Self {
        Self { option, ..self }
    }

    pub fn build(self) -> Transition<'a> {
        Transition {
            begin: self.begin,
            end: self.end,
            description: self.description.map(String::from),
            option: self.option,
        }
    }
}

// Diagram ---------------------------------------------------------------------
#[cfg(test)]
mod tests {

    //     #[test]
    //     fn test_transition() {
    //         let state1 = State::new("state1").build();
    //         let state2 = State::new("state2").build();
    //         let trans = Transition::new(&state1, &state2)
    //             .set_description("transition description")
    //             .set_option(TransitionOption::DeepHistory)
    //             .build();

    //         assert_eq!(trans.begin.name, "state1");
    //         assert_eq!(trans.end.name, "state2");
    //         assert_eq!(trans.description.unwrap(), "transition description");
    //         assert_eq!(trans.option, TransitionOption::DeepHistory);
    //     }

    //     #[test]
    //     fn test_diagram() {
    //         let state1 = State::new("state1").build();
    //         let state2 = State::new("state2").build();
    //         let trans1 = Transition::new(&state1, &state2).build();
    //         let diagram = Diagram::new()
    //             .add_state(&state1)
    //             .add_state(&state2)
    //             .add_transition(&trans1);

    //         assert!(diagram.states.contains(&&state1));
    //         assert!(diagram.states.contains(&&state2));
    //     }
}
