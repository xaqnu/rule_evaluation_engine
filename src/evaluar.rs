use std::collections::HashMap;

use crate::domain::{Evaluacion::NoAplica, Premisa, Resultado};

pub fn evaluar(premisas :Vec<Premisa>,variables:HashMap<String,String>) -> Resultado {
   return Resultado::new(NoAplica, "Error: No payload supplyed"); 
}
