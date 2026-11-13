use std::collections::HashMap;

use rule_evaluation_enigine::{
    domain::{Decision::Deny, Resultado},
    evaluar::evaluar,
};

use crate::situaciones_comunes::situaciones_comunes::*;

mod situaciones_comunes;
#[test]
fn cri_001() {
    let a: Resultado = evaluar(Some(&[un_permiso_que_siempre_aplica()]), None);

    assert_eq!(
        a,
        Resultado::new(Deny, "Error: No payload supplyed"),
        "el payload esta vacio"
    );
}

#[test]
fn cri_002() {
    let a: Resultado = evaluar(None, Some(&HashMap::new()));
    let b: Resultado = evaluar(
        None,
        Some(&HashMap::from([(
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
    let a: Resultado = evaluar(Some(&[]), Some(&un_payload_con_accion_valida()));
    let b: Resultado = evaluar(
        Some(&[un_permiso_que_nunca_aplica()]),
        Some(&un_payload_con_accion_valida()),
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

#[test]
fn cri_003_con_errores() {
    let premisas = [
        un_permiso_que_nunca_aplica(),
        un_permiso_que_siempre_falla(),
    ];
    let uuid = premisas[1].uuid();
    let a: Resultado = evaluar(Some(&premisas), Some(&un_payload_con_accion_valida()));

    assert_eq!(
        a,
        Resultado::new(
            Deny,
            format!("No rules apply\nError  en premisa con id {uuid}")
        )
    );
}
#[test]
fn cri_004() {
    let premisas = [
        un_permiso_que_siempre_aplica(),
        una_excepcion_que_siempre_aplica(),
    ];
    let premisas_con_fallo = [
        una_excepcion_que_siempre_aplica(),
        un_permiso_que_siempre_aplica(),
        un_permiso_que_siempre_falla(),
    ];
    let a: Resultado = evaluar(Some(&premisas), Some(&un_payload_con_accion_valida()));
    let b: Resultado = evaluar(
        Some(&premisas_con_fallo),
        Some(&un_payload_con_accion_valida()),
    );
    let uuid_excepcion = premisas[1].uuid();
    let uuid_permiso1 = premisas[0].uuid();
    assert_eq!(
        a,
        Resultado::new(
            Deny,
            format!(
                "Excepcion {uuid_excepcion} ha desactivado:\n\
                \tpermiso : {uuid_permiso1}"
            )
        )
    );

    let uuid_excepcion = premisas_con_fallo[0].uuid();
    let uuid_permiso1 = premisas_con_fallo[1].uuid();
    let uuid_permiso_fallo = premisas_con_fallo[2].uuid();
    assert_eq!(
        b,
        Resultado::new(
            Deny,
            format!(
                "Excepcion {uuid_excepcion} ha desactivado:\n\
                \tpermiso : {uuid_permiso1}\n\
                Error : uuid {uuid_permiso_fallo} no compila 
                "
            )
        )
    );
}
