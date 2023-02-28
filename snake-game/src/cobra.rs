use crate::ponto::Ponto;

pub struct Cobra {
    pub cabeca: Ponto,
    pub corpo: Vec<Ponto>,
}

impl Default for Cobra {
    fn default() -> Self {
        Self {
            cabeca: Ponto::new(7,7),
            corpo: vec![Ponto::new(6,7), Ponto::new(5,7)]
        }
    }
}
