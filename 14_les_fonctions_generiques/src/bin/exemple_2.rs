fn afficher_contenu<T>(vecteur: Vec<T>)
where
    T: std::fmt::Debug,
    // on peut ajouter plusieur contrainte à la suite
    // quand on n'en n'as plusieur.
{
    for element in &vecteur {
        println!("{:?}", element);
    }
}

// on peut aussi l'ecrire comme cela
fn afficher_contenu_2<T: std::fmt::Debug>(vecteur: Vec<T>)
{
    for element in &vecteur {
        println!("{:?}", element);
    }
}

fn main() {
    let vecteur_entiers = vec![1, 2, 3];
    let vecteur_mots = vec!["Hello", "Rust", "World"];

    afficher_contenu(vecteur_entiers);
    afficher_contenu_2(vecteur_mots);
}
