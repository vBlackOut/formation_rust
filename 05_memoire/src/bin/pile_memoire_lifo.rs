/*
launch this programme : cargo run --bin pile_memoire_lifo
*/

fn fonction_b() {
    let y: i32 = 5;
    println!("Dans fonctionB y = {}", y);
}

fn fonction_a() {
    let x: i32 = 10;
    println!("Dans fonctionA x = {}", x);

    fonction_b();
    println!("Retour dans fonctionA");
}

fn main() {
    fonction_a();
}


/*

Différence d'une heap (tas) ou d'un lifo (pile memoire)

                              PILE || TAS
Performance                    ++      -
Taille de l'allocation          +     +++
Durée de vie                    +     ++
Risques associées               +     ++

conclusion :

le tas est plus lent ( génération de blocs liberation frequente plus complexe...)
la heap est plus performante. (plus facile a utiliser)
 - pile probleme
 - stackoverflow si débordement utilisation de la ram

 pas besoin de stategie nécessaire au dzbut du programme car le compiulateur le gerer
 il faut gerer au mieux les deux zone le plus optimiser lors de votre programmation.
*/
