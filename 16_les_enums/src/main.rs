enum Forme {
    Rectangle,
    Cercle,
}

fn main() {
    let forme = Forme::Rectangle;

    if let Forme::Rectangle = forme {
        println!("c'est un rectangle.")
    } else if let Forme::Cercle = forme {
        println!("c'est un cercle.")
    } else {
        println!("Forme Inconnue.")
    }
}
