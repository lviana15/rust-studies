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

impl Cobra {
    pub fn passo(&mut self, tabuleiro: (usize, usize)) -> Result<(), &'static str> {
        let posicao_anterior_cabeca = self.cabeca;
        self.mover_cabeca(&tabuleiro)?;
        self.mover_corpo(posicao_anterior_cabeca);
        Ok(())
    } 

    pub fn alterar_direcao(&mut self, direcao: Direcao) {
        if direcao == Direcao::direcao_inversa(self.direcao) {
            self.direcao;
        } else {
            self.direcao = direcao;
        }
    }

    fn mover_cabeca(&mut self, tabuleiro: &(usize, usize)) -> Result<(), &'static str> {
        match self.direcao {
           Direcao::Cima if self.cabeca.y == 0 => Err("fim de jogo, esbarrou na parede"),
           Direcao::Baixo if self.cabeca.y >= tabuleiro.1 => Err("fim de jogo, esbarrou na parede"),
           Direcao::Esquerda if self.cabeca.x == 0 => Err("fim de jogo, esbarrou na parede"),
           Direcao::Direita if self.cabeca.x >= tabuleiro.0 => Err("fim de jogo, esbarrou na parede"),
           _ => {
               self.cabeca.alterar(self.direcao);
               Ok(())
           }
        }
    }

    fn mover_corpo(&mut self, posicao_anterior_cabeca: Ponto) {
        let corpo = &mut self.corpo;
        let mut posicao_anterior = posicao_anterior_cabeca;
        for ponto in corpo.iter_mut() {
            std::mem::swap(&mut posicao_anterior, ponto);
        }
    }

    pub fn aumentar_tamanho(&mut self) {
        let ultimo = self.corpo.last().unwrap().clone();
            self.corpo.push(ultimo);
    }
}
