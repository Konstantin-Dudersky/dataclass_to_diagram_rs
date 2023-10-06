use crate::domain::models::c4_model::ContextKind;

use super::*;

#[test]
fn test1() {
    let c1 = Context::new(ContextKind::System, "system").build();
    assert_eq!(export(c1), r#"System($alias = 0, $label = "system")"#);
}
