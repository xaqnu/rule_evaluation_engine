use std::{collections::HashMap};


use crate::domain::{
    Decision::{Allow, Deny},
    Evaluable,
    Evaluacion::{Aplica, Error},
    Premisa::{self, Excepcion, Permiso},
    Resultado,
};

#[must_use]
pub fn evaluar(
    premisas: Option<&[Premisa]>,
    payload: Option<&HashMap<String, String>>,
) -> Resultado {
    match payload {
        Some(payload) => evaluar_with_payload(premisas, payload),
        None => Resultado::new(Deny, "Error: No payload supplyed"),
    }
}

fn evaluar_with_payload(
    premisas: Option<&[Premisa]>,
    payload: &HashMap<String, String>,
) -> Resultado {
    match premisas {
        Some(premisas) => evaluar_with_valid_action(premisas, payload),
        None => Resultado::new(Deny, "Error: Action not provided or not registered"),
    }
}

fn evaluar_with_valid_action(premisas: &[Premisa], payload: &HashMap<String, String>) -> Resultado {
    let mut premisas_que_aplican: Vec<&Premisa> = premisas
        .iter()
        .filter(|r| matches!(r.evaluar(payload), Aplica))
        .collect();
    let premisas_que_fallan: Vec<&Premisa> = premisas
        .iter()
        .filter(|r| matches!(r.evaluar(payload), Error))
        .collect();

    let mut traza = String::new();
    if !premisas_que_fallan.is_empty() {
        traza.push_str("\nSe han detectado los siguientes errores:");
        premisas_que_fallan.iter().for_each(|p| {
            traza.push_str(&format!("\n\t {} con id: {}", p.variante(), p.uuid()));
        });
    }
    let mut log = String::new();
    if premisas_que_aplican.is_empty() {
        log.push_str("No rules apply");
        log.push_str(&traza);
        return Resultado::new(Deny, log);
    }

    premisas_que_aplican.sort_unstable_by(|a, b| b.cmp(a));
    let premisa_directriz = premisas_que_aplican.first().unwrap();
    let uuid = premisa_directriz.uuid();

    match premisa_directriz {
        Permiso(_) => Resultado::new(Allow, format!("Aplica la premisa {uuid}\n {traza}")),
        Excepcion(_) => {
            log.push_str("No active rules apply:");
            if premisas_que_aplican.len() > 1 {
                log.push_str(&format!(
                    "\nla restricción {uuid} ha desactivado las siguientes premisas aplicables:"
                ));
            }
            premisas_que_aplican.iter().skip(1).for_each(|p| {
                log.push_str(&format!("\n\t {} con id: {}", p.variante(), p.uuid()));
            });
            log.push_str(&traza);
            Resultado::new(Deny, log)
        }
    }
}
