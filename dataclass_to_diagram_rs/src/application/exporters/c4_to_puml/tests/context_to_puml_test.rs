use super::*;

#[test]
fn test1() {
    let c1 = Context::new("system").build();
    assert_eq!(
        export_single(c1),
        r#"System($alias = 0, $label = "system")"#
    );
}
