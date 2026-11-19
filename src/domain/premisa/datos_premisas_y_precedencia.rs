use std::collections::HashMap;

use uuid::Uuid;

use crate::domain::{Condicion, Evaluable, Evaluacion::{self, Aplica, Error, NoAplica}};

#[derive(Debug, PartialEq, Eq)]
pub struct Precedencia(pub u8, pub u8);

#[derive(Debug, PartialEq, Eq)]
pub struct PremisaDatos {
    uuid: Uuid,
    condiciones: Vec<Condicion>,
    precedencia: Precedencia,
}

impl PremisaDatos {
    #[must_use]
    pub fn new(condiciones: Vec<Condicion>, precedencia: Precedencia) -> Self {
        let uuid = Uuid::new_v4();
        Self {
            uuid,
            condiciones,
            precedencia,
        }
    }

    pub fn uuid(&self) -> &Uuid {
        &self.uuid
    }

    pub fn precedencia(&self) -> &Precedencia {
        &self.precedencia
    }
}
impl Evaluable for PremisaDatos {
    fn evaluar(&self, payload: &HashMap<String, String>) -> Evaluacion {
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


impl PartialOrd for Precedencia {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}
impl Ord for Precedencia {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        let (pa, pb) = (self.0, self.1);
        let (qa, qb) = (other.0, other.1);
        if self == other {
            return std::cmp::Ordering::Equal;
        }
        if pa > qa {
            return std::cmp::Ordering::Less;
        }
        if pa < qa {
            return std::cmp::Ordering::Greater;
        }
        if pa == qa && pb < qb {
            return std::cmp::Ordering::Less;
        }
        std::cmp::Ordering::Greater
    }
}


