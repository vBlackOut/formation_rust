#[derive(Debug)]
enum Taches {
    Encours(String),
    Terminer(String),
    Afaire(String),
}

fn cree_taches(liste_taches: &mut Vec<Taches>) {
    println!("{:?}", liste_taches);
}

fn supprimer_taches(liste_taches: &mut Vec<Taches>) {
    println!("{:?}", liste_taches);
}


fn main() {
    let mut liste_taches: Vec<Taches> = vec![];
    let tache = Taches::Encours("description 1".to_string());

    loop {

        let mut input: String = String::new();
        println!("\nc - Crée une taches\nd - supprimer une taches");

        println!("\ndonner moi votre actions \n[envoyer x pour terminer]");
        std::io::stdin().read_line(&mut input).expect("Erreur lors de la lecture de la ligne");
        let input = input.trim();

        match input {
            "c" => cree_taches(&mut liste_taches),
            "d" => supprimer_taches(&mut liste_taches),
            "x" => break,
            _ => println!("command no found")
        };
    }
}
