pub struct StateExported {
    pub alias: String,
    pub internal_states: Vec<String>,
    pub internal_states_exported: Option<String>,
    pub has_parent: bool,
    pub export: Option<String>,
}

impl StateExported {
    pub fn new(alias: &str, has_parent: bool) -> Self {
        Self {
            alias: String::from(alias),
            internal_states: vec![],
            internal_states_exported: None,
            has_parent,
            export: None,
        }
    }

    pub fn add_internal_state(&mut self, internal_state: &str) {
        self.internal_states.push(String::from(internal_state));
    }
}
