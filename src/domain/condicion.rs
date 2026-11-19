use std::collections::HashMap;

use crate::domain::{
    Evaluable, Evaluacion::{self, Aplica, Error, NoAplica},
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
    fn evaluar(&self, _payload: &HashMap<String, String>) -> Evaluacion {
        match self.expresion_cell.as_str() {
            "true" => Aplica,
            "false" => NoAplica,
            _ => Error,
        }
    }
}
