#[derive(Clone, Copy)]
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
