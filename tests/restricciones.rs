mod situaciones_comunes;

use rule_evaluation_enigine::{domain::Resultado, evaluar::evaluar};

use crate::situaciones_comunes::situaciones_comunes::*;

#[test]
fn res_001() {
    let premisas = [
        un_permiso_que_siempre_aplica(),
        una_excepcion_que_siempre_aplica(),
    ];

    let uuid_excepcion = premisas[1].uuid();
    let uuid_permiso1 = premisas[0].uuid();

    let backtrace = format!(
        "No active rules apply:\n\
                la restricción {uuid_excepcion} ha desactivado las siguientes premisas aplicables:\n\
                \t Permiso con id: {uuid_permiso1}"
    );
    let resultado: Resultado = evaluar(Some(&premisas), Some(&un_payload_con_accion_valida()));
    assert_eq!(
        resultado.backtrace(),
        backtrace,
        "la excepcion deshabilita el permiso"
    )
}
