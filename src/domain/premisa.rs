
pub mod  datos_premisas_y_precedencia;
use std::collections::HashMap;



use uuid::Uuid;

use crate::domain::{Condicion, Evaluable, Evaluacion, Premisa::{Excepcion, Permiso}, premisa::datos_premisas_y_precedencia::{PremisaDatos,Precedencia}};

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
            Permiso(p) | Excepcion(p) => p.uuid(),
        }
    }
    #[must_use]
    pub fn variante(&self) -> String {
        match self {
            Permiso(_) => "Permiso".to_string(),
            Excepcion(_) => "Restricción".to_string(),
        }
    }
}
impl Ord for Premisa {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        match (self, other) {
            (Permiso(p) | Excepcion(p), Permiso(q) | Excepcion(q)) => p.precedencia().cmp(&q.precedencia()),
        }
    }
}

impl PartialOrd for Premisa {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl Evaluable for Premisa {
    fn evaluar(&self, payload: &HashMap<String, String>) -> Evaluacion {
        match self {
            Permiso(p) | Excepcion(p) => p.evaluar(payload),
        }
    }
}


