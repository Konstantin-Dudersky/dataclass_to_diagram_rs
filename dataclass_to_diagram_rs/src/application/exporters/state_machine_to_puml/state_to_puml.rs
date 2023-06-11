use crate::domain::models::state_machine::{State, StateKind};

pub fn export(state: &State) -> String {
    match state.kind {
        StateKind::General => format!(
            "state \"{name}\" as {alias}{description}",
            name = state.name,
            alias = state.alias,
            description =
                export_description(state.alias, state.description.as_deref())
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
    }
}

fn export_description(alias: u32, description: Option<&str>) -> String {
    match description {
        Some(value) => export_description_some(alias, value),
        None => String::from(""),
    }
}

fn export_description_some(alias: u32, description: &str) -> String {
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

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn minimal() {
        let state = State::new("state1").build();
        let puml = format!("state \"state1\" as {}", state.alias);
        assert_eq!(export(&state), puml);
    }

    #[test]
    fn state_start() {
        let state = State::new("state1").set_kind(StateKind::Start).build();
        let puml = format!("state {} <<start>>", state.alias);
        assert_eq!(export(&state), puml);
    }

    #[test]
    fn state_end() {
        let state = State::new("state1").set_kind(StateKind::End).build();
        let puml = format!("state {} <<end>>", state.alias);
        assert_eq!(export(&state), puml);
    }

    #[test]
    fn state_fork() {
        let state = State::new("state1").set_kind(StateKind::Fork).build();
        let puml = format!("state {} <<fork>>", state.alias);
        assert_eq!(export(&state), puml);
    }

    #[test]
    fn state_join() {
        let state = State::new("state1").set_kind(StateKind::Join).build();
        let puml = format!("state {} <<join>>", state.alias);
        assert_eq!(export(&state), puml);
    }

    #[test]
    fn state_choice() {
        let state = State::new("state1").set_kind(StateKind::Choice).build();
        let puml = format!("state {} <<choice>>", state.alias);
        assert_eq!(export(&state), puml);
    }

    #[test]
    fn description() {
        let state = State::new("state1").set_description("description").build();
        let puml = format!(
            "state \"state1\" as {alias}
{alias} : description",
            alias = state.alias,
        );
        assert_eq!(export(&state), puml);
    }

    #[test]
    fn description_multiline() {
        let desc1 = "ex1, desc line 1\ndesc line 2\ndesc line 3";
        let state1 = State::new("state1").set_description(desc1).build();
        let puml1 = format!(
            "state \"state1\" as {alias}
{alias} : ex1, desc line 1
{alias} : desc line 2
{alias} : desc line 3",
            alias = state1.alias
        );
        assert_eq!(export(&state1), puml1);

        let desc2 = "ex2, desc line 1
desc line 2
desc line 3";
        let state2 = State::new("state1").set_description(desc2).build();
        let puml2 = format!(
            "state \"state1\" as {alias}
{alias} : ex2, desc line 1
{alias} : desc line 2
{alias} : desc line 3",
            alias = state2.alias
        );
        assert_eq!(export(&state2), puml2);
    }
}
