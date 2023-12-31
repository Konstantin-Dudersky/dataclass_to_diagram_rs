use itertools::Itertools;

use crate::domain::models::state_machine::StateKind;

use super::super::utils::increase_indent::increase_indent;
use super::state_export::{InternalStateExport, StateExport};

pub fn export<TStates>(state: &mut StateExport<TStates>) -> String {
    let exported = match state.kind {
        StateKind::General => format!(
            "state \"{name}\" as {alias}{internal_states}{description}",
            name = state.name.replace("\n", "\\n"),
            alias = state.alias,
            internal_states =
                export_internal_states(&mut state.internal_states_exported),
            description =
                export_description(&state.alias, state.description.as_deref())
        ),
        StateKind::Start => {
            format!("state {alias} <<start>>", alias = state.alias)
        }
        StateKind::End => {
            format!("state {alias} <<end>>", alias = state.alias)
        }
        StateKind::Fork => {
            format!("state {alias} <<fork>>", alias = state.alias)
        }
        StateKind::Join => {
            format!("state {alias} <<join>>", alias = state.alias)
        }
        StateKind::Choice => {
            format!("state {alias} <<choice>>", alias = state.alias)
        }
        StateKind::History => {
            format!("state {alias} <<history>>", alias = state.alias)
        }
        StateKind::DeepHistory => {
            format!("state {alias} <<history*>>", alias = state.alias)
        }
    };
    state.exported = Some(exported.clone());
    exported
}

fn export_description(alias: &str, description: Option<&str>) -> String {
    match description {
        Some(value) => export_description_some(alias, value),
        None => String::from(""),
    }
}

fn export_description_some(alias: &str, description: &str) -> String {
    let desc_lines = description.split('\n');
    let mut desc_lines_format: Vec<String> = vec![];
    for line in desc_lines {
        let line_format = format!(
            "{alias} : {description}",
            alias = alias,
            description = line
        );
        desc_lines_format.push(line_format);
    }
    let format = desc_lines_format.join("\n");
    format!("\n{}", format)
}

fn export_internal_states(
    internal_states_exported: &mut Vec<InternalStateExport>,
) -> String {
    if internal_states_exported.len() == 0 {
        return String::from("");
    }
    let internal_states_string = internal_states_exported
        .iter()
        .sorted_by(|a, b| a.order.cmp(&b.order))
        .map(|ise| ise.exported.clone())
        .join("\n");
    let internal_states_string = increase_indent(&internal_states_string);
    format!(" {{\n{}\n}}", internal_states_string)
}

// tests -----------------------------------------------------------------------

#[cfg(test)]
mod test {
    use derive_more::Display;

    use crate::domain::models::state_machine::State;

    use super::*;

    #[derive(Clone, Display)]
    enum States {
        State1,
    }

    #[test]
    fn minimal() {
        let state = State::new(States::State1, 0);
        let mut state_exported = StateExport::from(&state);
        let puml = format!("state \"State1\" as {}", state.alias);
        assert_eq!(export(&mut state_exported), puml);
    }

    #[test]
    fn state_start() {
        let mut state = State::new(States::State1, 0);
        state.set_kind(StateKind::Start);
        let mut state_exported = StateExport::from(&state);
        let puml = format!("state {} <<start>>", state.alias);
        assert_eq!(export(&mut state_exported), puml);
    }

    #[test]
    fn state_end() {
        let mut state = State::new(States::State1, 0);
        state.set_kind(StateKind::End);
        let mut state_exported = StateExport::from(&state);
        let puml = format!("state {} <<end>>", state.alias);
        assert_eq!(export(&mut state_exported), puml);
    }

    #[test]
    fn state_fork() {
        let mut state = State::new(States::State1, 0);
        state.set_kind(StateKind::Fork);
        let mut state_exported = StateExport::from(&state);
        let puml = format!("state {} <<fork>>", state.alias);
        assert_eq!(export(&mut state_exported), puml);
    }

    #[test]
    fn state_join() {
        let mut state = State::new(States::State1, 0);
        state.set_kind(StateKind::Join);
        let mut state_exported = StateExport::from(&state);
        let puml = format!("state {} <<join>>", state.alias);
        assert_eq!(export(&mut state_exported), puml);
    }

    #[test]
    fn state_choice() {
        let mut state = State::new(States::State1, 0);
        state.set_kind(StateKind::Choice);
        let mut state_exported = StateExport::from(&state);
        let puml = format!("state {} <<choice>>", state.alias);
        assert_eq!(export(&mut state_exported), puml);
    }

    #[test]
    fn description() {
        let mut state = State::new(States::State1, 0);
        state.set_description("description");
        let mut state_exported = StateExport::from(&state);
        let puml = format!(
            "state \"State1\" as {alias}
{alias} : description",
            alias = state.alias,
        );
        assert_eq!(export(&mut state_exported), puml);
    }

    #[test]
    fn description_multiline() {
        let desc1 = "ex1, desc line 1\ndesc line 2\ndesc line 3";
        let mut state1 = State::new(States::State1, 0);
        state1.set_description(desc1);
        let puml1 = format!(
            "state \"{alias}\" as {alias}
{alias} : ex1, desc line 1
{alias} : desc line 2
{alias} : desc line 3",
            alias = state1.alias
        );
        let mut state_exported = StateExport::from(&state1);
        assert_eq!(export(&mut state_exported), puml1);

        let desc2 = "ex2, desc line 1
desc line 2
desc line 3";
        let mut state2 = State::new(States::State1, 0);
        state2.set_description(desc2);
        let puml2 = format!(
            "state \"{alias}\" as {alias}
{alias} : ex2, desc line 1
{alias} : desc line 2
{alias} : desc line 3",
            alias = state2.alias
        );
        let mut state_exported = StateExport::from(&state2);
        assert_eq!(export(&mut state_exported), puml2);
    }
}
