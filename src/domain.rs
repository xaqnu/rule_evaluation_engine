use std::collections::HashMap;

mod condicion;
mod premisa;
pub use condicion::Condicion;
pub use premisa::Premisa;
pub use premisa::datos_premisas_y_precedencia::Precedencia;
use uuid::Uuid;

pub trait Evaluable {
    fn evaluar(&self, payload: &HashMap<String, String>) -> Evaluacion;
}

#[derive(Debug, PartialEq, Eq)]
pub enum Evaluacion {
    Aplica,
    NoAplica,
    Error,
}

#[derive(Debug, PartialEq, Eq)]
pub enum Decision {
    Allow,
    Deny,
}

#[derive(Debug, PartialEq, Eq)]
pub struct Resultado<'a> {
    decision: Decision,
    backtrace: Backtrace<'a>,
}

impl<'a> Resultado<'a> {
    pub fn new(decision: Decision, backtrace: Backtrace<'a>) -> Self {
        Self {
            decision,
            backtrace,
        }
    }
    #[must_use]
    pub fn backtrace(&self) -> &Backtrace {
        &self.backtrace
    }
}

#[derive(Debug, PartialEq, Eq)]
pub struct Backtrace<'a> {
    error: Option<UsageError>,
    premisa_llave: Option<&'a Uuid>,
    premisas_desactivadas: Option<Vec<&'a Uuid>>,
    premisas_con_fallo: Option<Vec<&'a Uuid>>,
}

impl<'a> Backtrace<'a> {
    pub fn new(
        error: Option<UsageError>,
        premisa_llave: Option<&'a Uuid>,
        premisas_desactivadas: Option<Vec<&'a Uuid>>,
        premisas_con_fallo: Option<Vec<&'a Uuid>>,
    ) -> Self {
        Self {
            error,
            premisa_llave,
            premisas_desactivadas,
            premisas_con_fallo,
        }
    }
}

#[derive(Debug, PartialEq, Eq)]
pub enum UsageError {
    NoPayload,
    AcionNotRegistered,
}
