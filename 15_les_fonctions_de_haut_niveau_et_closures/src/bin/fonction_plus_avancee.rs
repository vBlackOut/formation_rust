fn doubler(x: i32) -> i32 {
    x * 2
}

fn appliquer<F>(f:F, x: i32) -> i32
where
    F: Fn(i32) -> i32,
{
    f(x)
}

fn main() {
    let resultat = appliquer(doubler, 5);
    println!("Résultat : {}", resultat); // renvoie 10
}
