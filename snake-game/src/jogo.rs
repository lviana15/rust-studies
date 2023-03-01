use crate::ponto::Ponto;
use crate::cobra::Cobra;
use crate::direcao::Direcao;

use std::{
    io::{self, Write},
    thread,
    time::Duration,
};

use termion::event::Key;
use termion::input::TermRead;  
use termion::raw::IntoRawMode;
use rand::Rng;

pub struct Jogo;

impl Jogo {
    pub fn run() -> Result<(), &'static str> {
        let mut cobra: Cobra = Default::default();
        let tabuleiro = (15,15);
        let mut petisco = Self::gerar_petisco(&cobra, &tabuleiro);
        let mut stdin = termion::async_stdin().keys();
        loop {
            if cobra.cabeca == petisco {
                cobra.aumentar_tamanho();
                petisco = Self::gerar_petisco(&cobra, &tabuleiro); 
            } else if cobra.corpo.contains(&cobra.cabeca) {
                return Err("fim de jogo, a cobra bateu nela mesma");
            }
            let tabuleiro_jogo = Self::gerar_tabuleiro(&cobra, &petisco, &tabuleiro);
            print!(
                "{}{}{}",
                termion::clear::All,
                termion::cursor::Goto(1,1),
                termion::cursor::Hide
            );
            println!("{}", tabuleiro_jogo);
            let stdout = io::stdout().into_raw_mode().unwrap();
            let input = stdin.next();
            if let Some(Ok(key)) = input {
                match key {
                    Key::Char('a') | Key::Left => cobra.alterar_direcao(Direcao::Esquerda),
                    Key::Char('d') | Key::Right => cobra.alterar_direcao(Direcao::Direita),
                    Key::Char('w') | Key::Up => cobra.alterar_direcao(Direcao::Cima),
                    Key::Char('s') | Key::Down => cobra.alterar_direcao(Direcao::Baixo),
                    _ => {},
                }
            }
            stdout.lock().flush().unwrap();
            thread::sleep(Duration::from_millis(500));
            cobra.passo(tabuleiro)?;
        }
    }

    fn gerar_tabuleiro(cobra: &Cobra, petisco: &Ponto, tabuleiro: &(usize,usize)) -> String {
        let mut buffer = String::new();
        for y in 0..tabuleiro.1 {
            for x in 0..tabuleiro.0 {
                if cobra.cabeca == (x, y) {
                    buffer.push_str("0 ");
                } else if cobra.corpo.contains(&Ponto::new(x, y)){
                    buffer.push_str("# ");
                } else if *petisco == (x,y) {
                    buffer.push_str("+ ");
                } else {
                    buffer.push_str("- ");
                }
            } 
            buffer.push('\n');
        }
        buffer
    }

    fn gerar_petisco(cobra: &Cobra, tabuleiro: &(usize, usize)) -> Ponto {
        let mut petisco;
        loop {
            let x = rand::thread_rng().gen_range(0..tabuleiro.0 - 1);
            let y = rand::thread_rng().gen_range(0..tabuleiro.1 - 1);
            petisco = Ponto::new(x, y);
            if cobra.cabeca != petisco && !cobra.corpo.contains(&petisco) {
                break;
            }
        }
        petisco
    }
}
