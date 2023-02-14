fn main() {
    let mut string = String::new();
    println!("Digite um número até 255: ");
    std::io::stdin().read_line(&mut string).unwrap();
    
    let number = string.trim().parse::<u8>().unwrap();

    match number {
        0..=50 => println!("1° grau"),
        51..=120 => println!("2° grau"),
        121..=200 => println!("3° grau"),
        _ => println!("4° grau"),
    }
}
