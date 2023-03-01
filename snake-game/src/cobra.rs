use crate::ponto::Ponto;
use crate::direcao::Direcao;

pub struct Cobra {
    pub cabeca: Ponto,
    pub corpo: Vec<Ponto>,
    pub direcao: Direcao,
}

impl Default for Cobra {
    fn default() -> Self {
        Self {
            cabeca: Ponto::new(7,7),
            corpo: vec![Ponto::new(6,7), Ponto::new(5,7)],
            direcao: Direcao::default(),
        }
    }
}
