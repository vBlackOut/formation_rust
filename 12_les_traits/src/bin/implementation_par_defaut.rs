trait Affichage {
    fn affocher(éself) where Self: std::fmt::Display {
        println!("C'est un nombre : {}", self);
    }
}

impl Affichage for i32 {}

fn main() {
    let number = 14;
    number.afficher();
}
