use std::collections::HashMap;

use crate::domain::{Decision::Deny, Premisa, Resultado};

pub fn evaluar(premisas :Option<Vec<Premisa>>,payload:Option<HashMap<String,String>>) -> Resultado {
    match payload {
        Some(payload) => return evaluar_with_payload(premisas,payload),
        None => return Resultado::new(Deny, "Error: No payload supplyed"), 
    } 
}

fn evaluar_with_payload(premisas: Option<Vec<Premisa>>, payload: HashMap<String, String>) -> Resultado {
    match premisas {
        Some(premisas) => return evaluar_with_valid_action(premisas,payload), 
        None => return Resultado::new(Deny, "Error: Action not provided or not registered"), 
    }
}

fn evaluar_with_valid_action(premisas: Vec<Premisa>, payload: HashMap<String, String>) -> Resultado {
    return Resultado::new(Deny, "No rules apply")
}
