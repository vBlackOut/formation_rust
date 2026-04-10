// cargo run --bin tirage
use rand::{Rng, thread_rng}; // note we need the `Rng` trait

fn main() {
    let mut rng = thread_rng();

    loop {
        let dice_roll: i32 = rng.gen_range(1, 7);

        if dice_roll == 6 {
            println!("Fin du jeu avec un jet de 6 !");
            break;
        } else if dice_roll %2 != 0 { // içi je test si le jet de dé est impair
            continue // Si le résultat est pair, ignor et recommence la boucle
        }
        println!("Mon jet de dé {} est forcément pair et inférieur à 6", dice_roll);
    }

}
