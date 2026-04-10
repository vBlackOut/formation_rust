/*
1) Ajouter une tache : permet à l'utilisateur d'ajouter une nouvelle tache avec une description
2) Affciher la liste de tache : Affcihe la liste de toutes les taches. Indiquant si elle sont completes ou non.
3) Marquer une taches comme compléte : Permet à l'utilisateur de marquer une tache spécifique comme compléte
4) supprimer une taches : permet à l'utilisateur de supprimer une taches spécifique de la utilisateur
5) Quitter l'application : permet a l'utilisateur de quitter l'application.
*/
#[derive(Debug)]
enum Taches {
    Encours(String),
    Terminer(String),
}

fn cree_taches(liste_taches: &mut Vec<Taches>) {
    println!("Entrez la description de la tâche :");
    let mut description = String::new();
    std::io::stdin().read_line(&mut description).expect("Erreur lors de la lecture de la ligne");
    let tache = Taches::Encours(description);
    liste_taches.push(tache)
}

fn voir_lestaches(liste_taches: &mut Vec<Taches>) {
    print!("\n");
    for (index, tache) in liste_taches.iter().enumerate() {
        match tache {
            Taches::Encours(desc) => print!("{}: Encours - {}", index, desc),
            Taches::Terminer(desc) => print!("{}: Terminé - {}", index, desc),
        }
    }
}


fn main() {
    let mut liste_taches: Vec<Taches> = vec![];

    loop {

        let mut input: String = String::new();
        println!("\nc - Crée une taches\nd - supprimer une taches\nv - voir les taches");

        println!("\ndonner moi votre actions \n[envoyer x pour terminer]");
        std::io::stdin().read_line(&mut input).expect("Erreur lors de la lecture de la ligne");
        let input = input.trim();

        match input {
            "c" => cree_taches(&mut liste_taches),
            "v" => voir_lestaches(&mut liste_taches),
            "x" => break,
             _ => println!("Commande invalide"),
        };
    }
}
