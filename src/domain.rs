

#[derive(Debug,PartialEq,Eq)]
pub struct Condicion {
    expresion_cell: String
}

impl Condicion {
  pub fn new(expresion_cell: &str) -> Self {
        let expresion_cell = expresion_cell.to_owned();
        Self { expresion_cell }
    }
}

impl Evaluable for Condicion {
    fn evaluar(&self)-> Evaluacion {
        todo!()
    }
}


pub type Precedencia = (u8,u8);


#[derive(Debug,PartialEq,Eq)]
pub struct PremisaDatos {
   condiciones : Vec<Condicion>,
   precedencia : Precedencia
}

impl PremisaDatos {
    pub fn new(condiciones: Vec<Condicion>, precedencia: Precedencia) -> Self {
        Self { condiciones, precedencia }
    }
}

impl Evaluable for PremisaDatos {
    fn evaluar(&self)-> Evaluacion {
        todo!()
    }
}

#[derive(Debug,PartialEq,Eq)]
pub enum Premisa {
    Permiso(PremisaDatos),
    Excepcion(PremisaDatos)
}
impl Premisa {
   pub fn permiso(condiciones:Vec<Condicion>, precedencia: Precedencia)  -> Self {
        return Self::Permiso(PremisaDatos::new(condiciones, precedencia))
   }
   pub fn excepcion(condiciones:Vec<Condicion>, precedencia: Precedencia)  -> Self {
        return Self::Excepcion(PremisaDatos::new(condiciones, precedencia))
   }


} 

#[derive(Debug,PartialEq,Eq)]
pub enum Evaluacion {
    Aplica,
    NoAplica,
    Error
}


#[derive(Debug,PartialEq,Eq)]
pub struct Resultado {
    decision: Decision,
    backtrace: String
}

#[derive(Debug,PartialEq,Eq)]
pub enum Decision {
    Allow,
    Deny,
}

impl Resultado {
    pub fn new(decision: Decision, backtrace: &str) -> Self {
        let backtrace = backtrace.to_owned();
        Self { decision,backtrace }
    }
}


trait Evaluable {
    fn evaluar(&self)-> Evaluacion;
}


