use snake_game::ponto::Ponto;
use snake_game::cobra::Cobra;

fn main() {
    print_board(&Cobra::default())
}

fn print_board(cobra: &Cobra) {
    let (x, y) = (15,15);
    for y in 0..y {
        for x in 0..x {
            if cobra.cabeca == (x, y) {
                print!("0 ");
            } else if cobra.corpo.contains(&Ponto::new(x, y)){
                print!("# ");
            } else {
                print!("- ");
            }
        } 
        println!();
    }
}
