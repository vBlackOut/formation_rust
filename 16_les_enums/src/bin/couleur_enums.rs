#[derive(Debug)]
enum Couleur {
    Rouge(String),
    Vert(String),
    Blanc,
}

fn main() {
    let led = Couleur::Rouge(String::from("#FF0000"));
    println!("{:?}", led);
}
