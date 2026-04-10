/*
1) Demande initialiser un compote bancaire avec
un solde de 1000€

2) tanter d'ajouter de l'argent au compte

3) tenter de retirer de l'argent du compte

4) afficher le solde par emprunt

5) appliquer des frais bancaires de retrait gérés par shadowing

6) Afficher le solde final
*/

fn main() {
    // message d'accueil
    println!("Bienvenue sur votre compte bancaire !");

    // initialisation du compte
    let mut balance = 1000.0;
    println!("Votre solde initial: {}€", balance);

    // depot sur le compte
    let mut input = String::new();
    std::io::stdin().read_line(&mut input).expect("Erreur lors de la lecture de la ligne");

    // conversion d'un String vers un f64
    let deposit: f64 = input.trim().parse().expect("Veuillez entrer un nombre valide");

    // ajout du depot sur le solde
    balance += deposit;
    println!("Après un dépôt de {}: {}", deposit, balance);

    // retrait sur le compte
    println!("Combien voulez-vous retirer?");
    let mut input = String::new();
    std::io::stdin().read_line(&mut input).expect("Erreur lors de la lecture de la ligne");

    let withdrawal: f64 = input.trim().parse().expect("Veuillez entrer un nombre valide");

    if balance - withdrawal < 0.0 {
        println!("Retrait impossible, solde insuffisant !");
    } else {
        balance -= withdrawal;
        println!("Après un retrait de {}: {}", withdrawal, balance);
    }

    // affichage du solde par une reference
    let borrowed_balance = &balance;
    println!("Solde emprunté (sans modification): {}€", borrowed_balance);

    // application des frais de gestion
    let withdrawal_fee = 10.0;
    let balance = balance - withdrawal_fee;  // shadowing

    // affichage du solde final
    println!("Solde après: {}€", balance);
}
