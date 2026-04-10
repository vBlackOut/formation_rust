// cargo run --bin 04_dangling


fn dangling() -> &String {
    let s: String = String::from("hello");
    &s // ceci causera une erreur !
//  ^^ returns a reference to data owned by the current function
// la poprietaire s est définie içi. et non pas à l'exterieur
// durée de vie de s
}

fn main() {
    let r: &String = dangling();
    println!("{}", r);
}

/*
Rust assure que les références ne deviennent jamais des références "dangling" ou pendantes.
Cela signifie que vous ne pouvez jamais accidentellement créer une situation où vous avez
une référence vers une mémoire qui pourrait être nettoyée
*/
