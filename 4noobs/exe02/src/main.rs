fn main() {
    let mut hearts = String::from("Rust4Noobs");
    let mut string = String::new();
    println!("Digite um texto: ");
    std::io::stdin().read_line(&mut string).unwrap();
    hearts.push_str(&mut string);
    println!("{}", hearts);
}
