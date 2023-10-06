use std::rc::Rc;

use crate::domain::models::c4_model::Rel;

pub fn export_single(relation: Rc<Rel>) -> String {
    let technology = relation.technology.clone().unwrap_or_default();
    format!(
        r#"{kind}($from = {from}, $to = {to}, $label = "{label}", $techn = "{technology}")"#,
        kind = relation.kind,
        from = relation.from,
        to = relation.to,
        label = relation.label,
        technology = technology
    )
}

pub fn export_several(relations: Vec<Rc<Rel>>) -> String {
    let mut res = vec![];
    for rel in &relations {
        let container = export_single(rel.clone());
        res.push(container);
    }
    let mut res = res.join("\n");
    if res.len() > 0 {
        res = format!("\n{}", res);
    }
    res
}
