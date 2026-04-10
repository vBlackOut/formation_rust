// cargo run --bin 03_emprunt_immuable



fn main(){
    let s: String = String::from("hello");
    let len: usize = calculate_length(&s); // &s est un emprunt immuable de s. (lecture seul)

    println!("{}", len);
    println!("{}", s);
}

/*
                       Permet de définir une variabnle lisible à la fonction tout en autorisant
                       la proprieter de lecture uniquement.
                       ^                 */
fn calculate_length(s: &String) -> usize {
    s.len()
}
