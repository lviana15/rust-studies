use crate::direcao::Direcao;

#[cfg(test)]
mod ponto_tests {
    use super::*;

    #[test]
    fn alterar_para_cima() {
        let mut ponto = Ponto::new(1, 1);
        ponto.alterar(Direcao::Cima);
        assert_eq!(Ponto::new(1, 0), ponto);
    }

    #[test]
    fn alterar_para_baixo() {
        let mut ponto = Ponto::new(1, 1);
        ponto.alterar(Direcao::Baixo);
        assert_eq!(Ponto::new(1, 2), ponto);
    }
    
    #[test]
    fn alterar_para_direita() {
        let mut ponto = Ponto::new(1, 1);
        ponto.alterar(Direcao::Direita);
        assert_eq!(Ponto::new(2, 1), ponto);
    }

    #[test]
    fn alterar_para_esquerda() {
        let mut ponto = Ponto::new(1, 1);
        ponto.alterar(Direcao::Esquerda);
        assert_eq!(Ponto::new(0, 1), ponto);
    }
}

#[derive(PartialEq, Clone, Copy)]
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


