fn main() {
    let mut compteur: i32 = 0;

    loop {
        compteur += 1;
        println!("Compteur = {}", compteur);

        if compteur == 0 {
            println!("la boucle s'arrete ! ");
            break;
        }
    }
}
