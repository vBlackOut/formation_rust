enum Couleur {
    Rouge,
    Vert,
    Blanc,
}


fn main() {
    let ma_couleur = Couleur::Rouge;

    match ma_couleur {
        Couleur::Rouge => println!("C'est rouge !"),
        Couleur::Vert => println!("C'est vert !"),
        Couleur::Blanc => println!("C'est bleu !"),
        _ => println!("Ce n'est pas de la couleur"),
    }

    let valeur = 42;

    match valeur {
        0 => println!("la vlauer est zéro"),
        1..10 => println!("La valeur est entre 1 et 10"),
        _ => println!("La valeur n'apportient à aucue valeur"),
    }


    let teste = "Rust";

    match texte {
        "Rust" => println!("C'est Rust !"),
        "Python" | "Java" => println!("C'est un language de programmation popuplaire")
        _ => println!("C'est quelque chose d'autre."),
    }
}
