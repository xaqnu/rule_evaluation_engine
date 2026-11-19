use std::collections::HashMap;

use uuid::Uuid;

use crate::domain::{
    Backtrace,
    Decision::{Allow, Deny},
    Evaluable,
    Evaluacion::{Aplica, Error},
    Premisa::{self, Excepcion, Permiso},
    Resultado,
    UsageError::{AcionNotRegistered, NoPayload},
};

#[must_use]
pub fn evaluar<'a>(
    premisas: Option<&'a [Premisa]>,
    payload: Option<&HashMap<String, String>>,
) -> Resultado<'a> {
    match payload {
        Some(payload) => evaluar_with_payload(premisas, payload),
        None => Resultado::new(Deny, Backtrace::new(Some(NoPayload), None, None, None)),
    }
}

fn evaluar_with_payload<'a>(
    premisas: Option<&'a [Premisa]>,
    payload: &HashMap<String, String>,
) -> Resultado<'a> {
    match premisas {
        Some(premisas) => evaluar_with_valid_action(premisas, payload),
        None => Resultado::new(
            Deny,
            Backtrace::new(Some(AcionNotRegistered), None, None, None),
        ),
    }
}

fn evaluar_with_valid_action<'a>(
    premisas: &'a [Premisa],
    payload: &HashMap<String, String>,
) -> Resultado<'a> {
    let mut premisas_que_aplican: Vec<&Premisa> = premisas
        .iter()
        .filter(|r| matches!(r.evaluar(payload), Aplica))
        .collect();
    let premisas_que_fallan: Vec<&Premisa> = premisas
        .iter()
        .filter(|r| matches!(r.evaluar(payload), Error))
        .collect();
    let mut fallos: Option<Vec<&Uuid>> = None;
    if !premisas_que_fallan.is_empty() {
        fallos = Some(premisas_que_fallan.iter().map(|r| r.uuid()).collect())
    }
    if premisas_que_aplican.is_empty() {
        return Resultado::new(Deny, Backtrace::new(None, None, None, fallos));
    }

    premisas_que_aplican.sort_unstable_by(|a, b| b.cmp(a));
    let premisa_directriz = premisas_que_aplican.first().expect("Aquí no está vacio");
    let uuid = premisa_directriz.uuid();

    match premisa_directriz {
        Permiso(_) => Resultado::new(Allow, Backtrace::new(None, Some(uuid), None, fallos)),
        Excepcion(_) => {
            let mut desactivados: Option<Vec<&Uuid>> = None;
            if premisas_que_aplican.len() > 1 {
                desactivados = Some(
                    premisas_que_aplican
                        .iter()
                        .skip(1)
                        .map(|r| r.uuid())
                        .collect(),
                );
            }
            Resultado::new(Deny, Backtrace::new(None, Some(uuid), desactivados, fallos))
        }
    }
}
