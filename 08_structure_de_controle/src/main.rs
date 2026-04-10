fn main() {

    let is_weather_nice: bool = true;

    // conditioon vrai et else si faux
    if is_weather_nice {
        println!("Je vais prendre mes lunettes de soleil");
    } else {
        println!("Je ne vais rien prendre");
    }

    // non sugereer d'utiliser se choix si plusieur condition.
    let is_weather_nice: bool = false;
    let is_it_raining: bool = true;

    if is_weather_nice {
        println!("Je vais prendre mes lunettes de soleil");
    } else if is_it_raining {
        println!("Je vais prendre mon parapluie");
    } else {
        println!("Je ne vais rien prendre");
    }

    // permet d'asigner une variabkle
    // avec des conditions preetablie
    let is_it_raining: bool = true;
    let is_it_cold: bool = false;

    let declaration: String = if is_it_cold && is_it_raining {
        String::from("je vais prendre un manteau et un parapluie")
    } else if is_it_cold || is_it_raining {
        String::from("il fait mauvais dehors")
    } else if !is_it_cold && !is_it_raining {
        String::from("il fait beau dehors")
    } else {
        String::from("le climat me semble incertain")
    }

    println!("{}", declaration);

}
