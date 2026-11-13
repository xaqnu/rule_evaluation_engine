use std::{cmp::Ordering, collections::HashMap};

use uuid::Uuid;

use crate::domain::{
    Decision::Deny, Evaluable, Evaluacion::{self, Aplica, Error, NoAplica}, Premisa::{self, Excepcion, Permiso}, Resultado,
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
    let excepciones_que_aplican: Vec<&Premisa>= premisas.iter()
                                          .filter(|r| matches!(r,Excepcion(_)) && matches!(r.evaluar(payload), Aplica))
                                          .filter(|r| !premisas.iter().any(|b| (*r).partial_cmp(b) == Some(Ordering::Less)))
                                          .collect();



    let mut excepciones_string = "Se aplicaro las siguientes excepciones".into();
    let mut traza_string = "Las siguientes premisas fallaron:".into();
    let mut conclusion_string = "No aplica ninguna premisa".into();
    let mut excepciones: HashMap<&Uuid, Vec<&Uuid>> = HashMap::new();

    'bucle_premisas : for premisa in premisas {
        for excepcion in 
            &excepciones_que_aplican{
            if premisa < excepcion {
                match excepciones.get_mut(excepcion.uuid()){
                    Some(v) => v.push(premisa.uuid()),
                    None => {excepciones.insert(excepcion.uuid(),vec![premisa.uuid()]);}

                };
                continue 'bucle_premisas;
            }
        }
        if let Permiso(_) = premisa {
            let uuid = premisa.uuid();
            let evaluacion: Evaluacion = premisa.evaluar(payload);
            match evaluacion {
                NoAplica => (),
                Aplica => {conclusion_string = format!("Aplica el permiso {uuid}"); break;},
                Error => traza_string = format!("{traza_string}\n\t permiso con id {uuid}"),
            }
        }
        
    }
    for (excepcion,v)  in excepciones{
        excepciones_string = format!("{excepciones_string}\n\tla excepcion {excepcion} desactivo:");
        for premisa in v {
            excepciones_string = format!("{excepciones_string}\n\t\tla premisa {premisa}");
        }
    }
    Resultado::new(Deny, format!("{conclusion_string}\n{excepciones_string}\n{traza_string}"))
}
