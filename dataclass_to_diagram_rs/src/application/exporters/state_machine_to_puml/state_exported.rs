pub struct StateExported {
    pub alias: String,
    pub internal_states: Vec<StateExported>,
    pub export_complete: bool,
}
