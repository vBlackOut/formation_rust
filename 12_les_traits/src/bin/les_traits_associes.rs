trait Utilitaires [
    fn ajouter(a: i32, b: i32) -> i32;
]

impl Utilitaires for i32 {
    fn ajouter(a: i32, b:i32) -> i32 {
        a + b
    }
}

fn main() {
    let a = 5;
    let b = 3;

    let somme = i32::ajouter(a, b);
    println!("La somme est : {}", somme);
}
