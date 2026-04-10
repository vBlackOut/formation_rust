// cargo run --bin 02_fonction

fn main() {
    let s: String = String::from("hello");

    takes_ownership(some_string: s);

    // ceci génere une erreur de compilation car 's' n'est plus valie apres avoir été déplacée.
    println!("{}", s);
}

fn takes_ownership(some_string: String){
    println!("{}", some_string)
}
