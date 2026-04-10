fn main(){
    let x = 5;
    let doubler = |y: i32 |y * x; // la même choses d'un lambda en python.
                                  // les closures sont strictement typer.
    let resultat = doubler(10);
    println!("résultat {}", resultat);
}
