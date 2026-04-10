#[derive(Debug)]
enum Cables {
    None,
    Types(Types),
    Statues(String),
    Maintenance(Maintenance),
}

#[derive(Debug)]
enum Types {
    CTR,
    CDI,
    CDA,
}

#[derive(Debug)]
enum Maintenance {
    EnCours,
    Terminer,
    Afaire,
}

impl Cables {
    // Fonction "new" qui retourne une variante vide (None)
    fn new() -> Self {
        Cables::None  // Retourne la variante vide
    }

}

fn main() {
    // Création d'une instance vide avec `new()`
    let mut cable1 = Cables::new();
    cable1 =  Cables::Maintenance(Maintenance::EnCours);

    // Affichage de l'instance
    dbg!("{:?}", cable1);  // Affiche: None
}
