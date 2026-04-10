// définition d'un trait
trait Affichage {
    fn afficher(&self);
}

impl Affichage for i32 {
    fn afficher(&self){
        println!("C'est un nombre : {}", self);
    }
}

fn main() {
    println!("Hello, world!");
}
