// cargo run --bin tirage
use rand::{Rng, thread_rng}; // note we need the `Rng` trait

fn main() {
    let mut rng = thread_rng();

    loop {
        let dice_roll: i32 = rng.gen_range(1, 7);

        if dice_roll == 1 {
            println!("Echec avec un jet de 1 !");
            break;
        } else if dice_roll == 6 {
            println!("Succés avec un jet de 6");
            break;
        } else {
            println!("Jeté : {}", dice_roll);
        }
    }

}
