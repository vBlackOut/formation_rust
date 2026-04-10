// cargo run --bin 03_emprunt_mutable

fn main() {
    let mut s: String = String::from("hello");
    let r1: &mut String = &mut s;
    r1.push_str(", world!"); // pezrmet de modifier la valeur avec la valeur mut
    println!("{}", s);
}

/*
Attention toute fois un emprunt mutable dois etre exclusif
qu'un seul simultané vers la même variablke(valeur) ceci pour eviter des dangerd d'écriture.
et de lecture simultaner.

et jamais un emprunt_immuable et empruint mutable en même temps pendant
la même durée de vie d'une variable.
le compilateur vous sdecurisera car il gere
la detecter de se genre de choses
*/
