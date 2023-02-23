use snake_game::ponto::Ponto;

fn main() {
    let ponto = Ponto::new(7,7);
    let (x, y) = (15,15);
    for x in 0..x {
        for y in 0..y {
            if ponto == (x, y) {
                print!("# ");
            } else {
                print!("- ");
            }
        } 
        println!();
    }
}
