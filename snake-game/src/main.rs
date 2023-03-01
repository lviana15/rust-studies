use snake_game::jogo::Jogo;

fn main() {
    if let Err(msg) = Jogo::run() {
        println!("{}",msg);
    }
}

