use std::collections::HashMap;

use uuid::Uuid;

use crate::domain::{
    Evaluacion::{Aplica, Error, NoAplica},
    Premisa::{Excepcion, Permiso},
};

#[derive(Debug, PartialEq, Eq)]
pub struct Condicion {
    expresion_cell: String,
}

impl Condicion {
    pub fn new(expresion_cell: impl Into<String>) -> Self {
        let expresion_cell = expresion_cell.into();
        Self { expresion_cell }
    }
}

impl Evaluable for Condicion {
    fn evaluar(&self,_payload: &HashMap<String,String>) -> Evaluacion {
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
    activo : bool,
    condiciones: Vec<Condicion>,
    precedencia: Precedencia,
}

impl PremisaDatos {
    #[must_use]
    pub fn new(condiciones: Vec<Condicion>, precedencia: Precedencia) -> Self {
        let uuid = Uuid::new_v4();
        Self {
            uuid,
            activo: true,
            condiciones,
            precedencia,
        }
    }
}

impl Evaluable for PremisaDatos {
    fn evaluar(&self,payload: &HashMap<String,String>) -> Evaluacion {
        let mut evaluacion_final = NoAplica;
        for condicion in &self.condiciones {
            let evaluacion = condicion.evaluar(payload);
            match evaluacion_final {
                NoAplica => evaluacion_final = evaluacion,
                Aplica | Error => return evaluacion_final,
            }
        }
        evaluacion_final
    }
}

#[derive(Debug, PartialEq, Eq)]
pub enum Premisa {
    Permiso(PremisaDatos),
    Excepcion(PremisaDatos),
}
impl Premisa {
    #[must_use]
    pub fn permiso(condiciones: Vec<Condicion>, precedencia: Precedencia) -> Self {
        Self::Permiso(PremisaDatos::new(condiciones, precedencia))
    }
    #[must_use]
    pub fn excepcion(condiciones: Vec<Condicion>, precedencia: Precedencia) -> Self {
        Self::Excepcion(PremisaDatos::new(condiciones, precedencia))
    }

    #[must_use]
    pub fn uuid(&self) -> &Uuid {
        match self {
            Permiso(p) | Excepcion(p) => &p.uuid,
        }
    }

    pub fn desactivar(& mut self)  {
       match self {
            Permiso(p) | Excepcion(p) => p.activo = false,
       } 
    }
    #[must_use]
    pub fn esta_activo(&self) -> bool{
        match self {
            Permiso(p) | Excepcion(p) => p.activo
        }
    }
}
impl PartialOrd for Premisa  {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        match (self,other){
            (Permiso(p), Permiso(q))
            | (Permiso(p), Excepcion(q))
            | (Excepcion(p), Permiso(q))
            | (Excepcion(p), Excepcion(q)) => {
                let (pa,pb) = p.precedencia;
                let (qa,qb) = p.precedencia;
                if p == q {return Some(std::cmp::Ordering::Equal);}
                if pa < qa{return Some(std::cmp::Ordering::Less);}
                if pa > qa {return Some(std::cmp::Ordering::Greater);}
                if pa == qa && pb < qb {return Some(std::cmp::Ordering::Less);} 
                return Some(std::cmp::Ordering::Greater)
            }
        }
    }
}

impl Evaluable for Premisa {
    fn evaluar(&self,payload: &HashMap<String,String>) -> Evaluacion {
        match self {
            Permiso(p) | Excepcion(p) => p.evaluar(payload),
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
    pub fn new(decision: Decision, backtrace: impl Into<String>) -> Self {
        let backtrace = backtrace.into();
        Self {
            decision,
            backtrace,
        }
    }
   pub fn backtrace(&self) -> &str{
        return &self.backtrace;
   }
}

pub trait Evaluable {
    fn evaluar(&self,payload: &HashMap<String,String>) -> Evaluacion;
}
