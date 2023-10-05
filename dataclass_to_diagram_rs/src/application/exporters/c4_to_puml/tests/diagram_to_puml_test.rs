use super::*;

#[test]
fn empty_diagram() {
    let dia = Diagram::new();
    let exported = export(&dia);
    assert_eq!(
        exported,
        "@startuml
!include C4_Dynamic.puml
@enduml
"
    );
}
