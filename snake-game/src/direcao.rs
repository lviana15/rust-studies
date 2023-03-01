#[derive(PartialEq, Clone, Copy)]
pub enum Direcao {
    Cima,
    Baixo,
    Esquerda,
    Direita,
}

impl Default for Direcao {
    fn default() -> Self {
        Direcao::Direita
    }
}

impl Direcao {
    pub fn direcao_inversa(outro: Self) -> Self {
        match outro {
            Self::Cima => Self::Baixo,
            Self::Baixo => Self::Cima,
            Self::Esquerda => Self::Direita,
            Self::Direita => Self::Esquerda,
        }
    }
}
