use std::collections::HashMap;

use rule_evaluation_enigine::{
    domain::{Condicion, Evaluacion::NoAplica, Premisa, Resultado},
    evaluar::evaluar,
};

fn un_clausure_verdadero() -> Premisa {
    return Premisa::permiso(vec![Condicion::new("true")], (1, 1));
}
#[test]
fn cri_001() {
    let a: Resultado = evaluar(vec![un_clausure_verdadero()], HashMap::new());
    assert_eq!(
        a,
        Resultado::new(NoAplica, "Error: No payload supplyed"),
        "el payload es vacio"
    );
}
