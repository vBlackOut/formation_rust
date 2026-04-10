// cargo run --bin tirage

fn main() {
    let mut numero: i32 = 17;

    let _resultat: i32 = loop {

        if numero % 5 == 0 {
            break numero;
        }
        numero += 1;
    };

println!("le premier multiple de 5 supérieur ou égal à 17 est : {}", _resultat)
}
