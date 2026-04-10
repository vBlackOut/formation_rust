fn doubler(x: i32) -> i32 {
    x * 2
}

fn main() {
    let ma_fonction = doubler; // cree une fonction dans une variables
                               // qu'on peut ensuite déclarer comme une fonction.
    let resultat = ma_fonction(5);
    println!("Résultat : {}", resultat);
}
