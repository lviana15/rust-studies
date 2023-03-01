use crate::direcao::Direcao;

#[derive(PartialEq)]
pub struct Ponto {
    pub x: usize,
    pub y: usize
}

impl Ponto {
    pub fn new(x: usize, y: usize) -> Self {
        Self {
            x,
            y
        }
    }

    pub fn alterar(&mut self, direcao: Direcao) {
        match direcao {
            Direcao::Direita => self.x += 1,
            Direcao::Esquerda=> self.x -= 1,
            Direcao::Cima => self.y -= 1,
            Direcao::Baixo => self.y += 1,
        }
    }
}

impl PartialEq<(usize, usize)> for Ponto {
    fn eq(&self, other: &(usize, usize)) -> bool {
        self.x == other.0 && self.y == other.1
    }
}


