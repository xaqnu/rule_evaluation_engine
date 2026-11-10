use uuid::Uuid;

use crate::domain::{
    Decision::Deny,
    Evaluacion::{Aplica, Error, NoAplica},
    Premisa::{Excepcion, Permiso},
};

#[derive(Debug, PartialEq, Eq)]
pub struct Condicion {
    expresion_cell: String,
}

impl Condicion {
    pub fn new(expresion_cell: &str) -> Self {
        let expresion_cell = expresion_cell.to_owned();
        Self { expresion_cell }
    }
}

impl Evaluable for Condicion {
    fn evaluar(&self) -> Evaluacion {
        match self.expresion_cell.as_str() {
            "true" => Aplica,
            "false" => NoAplica,
            _ => Error,
        }
    }
}

pub type Precedencia = (u8, u8);

#[derive(Debug, PartialEq, Eq)]
pub struct PremisaDatos {
    uuid: Uuid,
    condiciones: Vec<Condicion>,
    precedencia: Precedencia,
}

impl PremisaDatos {
    pub fn new(condiciones: Vec<Condicion>, precedencia: Precedencia) -> Self {
        let uuid = Uuid::new_v4();
        Self {
            uuid,
            condiciones,
            precedencia,
        }
    }
}

impl Evaluable for PremisaDatos {
    fn evaluar(&self) -> Evaluacion {
        let mut evaluacion_final = NoAplica;
        for condicion in &self.condiciones {
            let evaluacion = condicion.evaluar();
            match evaluacion_final {
                NoAplica => evaluacion_final = evaluacion,
                Aplica | Error => return evaluacion_final,
            }
        }
        return evaluacion_final;
    }
}

#[derive(Debug, PartialEq, Eq)]
pub enum Premisa {
    Permiso(PremisaDatos),
    Excepcion(PremisaDatos),
}
impl Premisa {
    pub fn permiso(condiciones: Vec<Condicion>, precedencia: Precedencia) -> Self {
        return Self::Permiso(PremisaDatos::new(condiciones, precedencia));
    }
    pub fn excepcion(condiciones: Vec<Condicion>, precedencia: Precedencia) -> Self {
        return Self::Excepcion(PremisaDatos::new(condiciones, precedencia));
    }

    pub fn uuid(&self) -> &Uuid {
        match self {
            Permiso(p) => return &p.uuid,
            Excepcion(p) => return &p.uuid,
        }
    }
}

impl Evaluable for Premisa {
    fn evaluar(&self) -> Evaluacion {
        match self {
            Permiso(p) => p.evaluar(),
            Excepcion(p) => p.evaluar(),
        }
    }
}

#[derive(Debug, PartialEq, Eq)]
pub enum Evaluacion {
    Aplica,
    NoAplica,
    Error,
}

#[derive(Debug, PartialEq, Eq)]
pub struct Resultado {
    decision: Decision,
    backtrace: String,
}

#[derive(Debug, PartialEq, Eq)]
pub enum Decision {
    Allow,
    Deny,
}

impl Resultado {
    pub fn new(decision: Decision, backtrace: &str) -> Self {
        let backtrace = backtrace.to_owned();
        Self {
            decision,
            backtrace,
        }
    }
}

pub trait Evaluable {
    fn evaluar(&self) -> Evaluacion;
}
