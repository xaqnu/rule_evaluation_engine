use std::collections::HashMap;

use rule_evaluation_enigine::{
    domain::{Condicion, Decision::Deny, Premisa, Resultado},
    evaluar::evaluar,
};

fn un_permiso_que_siempre_aplica() -> Premisa {
    return Premisa::permiso(vec![Condicion::new("true")], (1, 1));
}
fn un_permiso_que_nunca_aplica() -> Premisa {
    return Premisa::permiso(vec![Condicion::new("false")], (1, 1));
}
#[test]
fn cri_001() {
    let a: Resultado = evaluar(Some(vec![un_permiso_que_siempre_aplica()]), None);

    assert_eq!(
        a,
        Resultado::new(Deny, "Error: No payload supplyed"),
        "el payload esta vacio"
    );
}

#[test]
fn cri_002() {
    let a: Resultado = evaluar(None, Some(HashMap::new()));
    let b: Resultado = evaluar(
        None,
        Some(HashMap::from([(
            "accion".to_owned(),
            "no existe".to_owned(),
        )])),
    );
    assert_eq!(
        a,
        Resultado::new(Deny, "Error: Action not provided or not registered"),
        "no hay acción en el payload"
    );
    assert_eq!(
        b,
        Resultado::new(Deny, "Error: Action not provided or not registered"),
        "la accion no está registrada"
    );
}

#[test]
fn cri_003_sin_errores() {
    let a: Resultado = evaluar(
        Some(vec![]),
        Some(HashMap::from([(
            "accion".to_owned(),
            "registered".to_owned(),
        )])),
    );
    let b: Resultado = evaluar(
        Some(vec![un_permiso_que_nunca_aplica()]),
        Some(HashMap::from([(
            "accion".to_owned(),
            "registered".to_owned(),
        )])),
    );
    assert_eq!(
        a,
        Resultado::new(Deny, "No rules apply"),
        "no se ha configurado ningún permiso"
    );
    assert_eq!(
        b,
        Resultado::new(Deny, "No rules apply"),
        "Hay permisos configurados pero no aplican"
    );
}
