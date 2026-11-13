use std::collections::HashMap;

use rule_evaluation_enigine::domain::{Condicion, Premisa};

pub fn un_permiso_que_siempre_falla() -> Premisa {
    Premisa::permiso(vec![Condicion::new("falla")], (1, 1))
}
pub fn un_permiso_que_siempre_aplica() -> Premisa {
    Premisa::permiso(vec![Condicion::new("true")], (1, 1))
}
pub fn un_permiso_que_nunca_aplica() -> Premisa {
    Premisa::permiso(vec![Condicion::new("false")], (1, 1))
}

pub fn una_excepcion_que_siempre_aplica() -> Premisa {
    Premisa::excepcion(vec![Condicion::new("true")], (1, 2))
}

pub fn un_payload_con_accion_valida() -> HashMap<String,String>{
    HashMap::from([(
            "accion".to_owned(),
            "registered".to_owned(),
        )])
}
