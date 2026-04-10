/*
launch this programme : cargo run --bin heap_tas
*/

fn main() {
    /*
    'b' est un pointeur sur la pile, 42 est sur le tas
    */
    let var_tas: Box<i32> = Box::new(42); // alloue un entiee sur le var_tas
    println!("{}", var_tas);
} // 'b' sort de sa portée içi


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
