use std::collections::HashMap;

use rule_evaluation_enigine::{
    domain::{
        Backtrace,
        Decision::Deny,
        Resultado,
        UsageError::{AcionNotRegistered, NoPayload},
    },
    evaluar::evaluar,
};

use crate::situaciones_comunes::situaciones_comunes::*;

mod situaciones_comunes;
#[test]
fn cri_001() {
    let premisas =[un_permiso_que_siempre_aplica()];
    let a: Resultado = evaluar(Some(&premisas), None);
    assert_eq!(
        a,
        Resultado::new(Deny, Backtrace::new(Some(NoPayload), None, None, None)),
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
        Resultado::new(
            Deny,
            Backtrace::new(Some(AcionNotRegistered), None, None, None)
        ),
        "no hay acción en el payload"
    );
    assert_eq!(
        b,
        Resultado::new(
            Deny,
            Backtrace::new(Some(AcionNotRegistered), None, None, None)
        ),
        "la accion no está registrada"
    );
}

#[test]
fn cri_003_sin_errores() {
    
    let premisas =[un_permiso_que_nunca_aplica()];
    let a: Resultado = evaluar(Some(&[]), Some(&un_payload_con_accion_valida()));
    let b: Resultado = evaluar(
        Some(&premisas),
        Some(&un_payload_con_accion_valida()),
    );
    assert_eq!(
        a,
        Resultado::new(Deny, Backtrace::new(None, None, None, None)),
        "no se ha configurado ningún permiso"
    );
    assert_eq!(
        b,
        Resultado::new(Deny, Backtrace::new(None, None, None, None)),
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
        Resultado::new(Deny, Backtrace::new(None, None, None, Some(vec![uuid]))),
        "Hay permisos configurados pero no aplican, y hay errores"
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
            Backtrace::new(None, Some(uuid_excepcion), Some(vec![uuid_permiso1]), None)
        ),
        "Una excepcion a deshabilitado el permiso"
    );

    let uuid_excepcion = premisas_con_fallo[0].uuid();
    let uuid_permiso1 = premisas_con_fallo[1].uuid();
    let uuid_permiso_fallo = premisas_con_fallo[2].uuid();
    assert_eq!(
        b,
        Resultado::new(
            Deny,
            Backtrace::new(
                None,
                Some(uuid_excepcion),
                Some(vec![uuid_permiso1]),
                Some(vec![uuid_permiso_fallo])
            )
        ),
        "Una excepcion ha deshabilitado el permiso y habia un fallo"
    );
}
