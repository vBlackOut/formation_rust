fn main() {
    let some_value: Option<i32> = Some(42);
    let no_value: Option<i32> = None;

    println!("{:?}", some_value); // Affcihe "Some(42)""
    println!("{:?}", no_value); // Affcihe "None"

    //let _unwrapped_value = some_value.unwrap();
    //let _unwrapped_no_value = no_value.unwrap();  // methode non  sérisuer de recuperer une valeur

    //let _expected_value = some_value.expect("la valeur était attendue");
    //let _expected_no_value = no_value.expect("la valeur était attendue"); // methode non  sérisuer de recuperer une valeur


    // utiliser un match pour seriser la methode de recuperation des valeurs

    match some_value {
        Some(value) => println!("Valeur présente : {}", value),
        None => println!("Aucune valeur présente"),
    }

    match no_value {
        Some(value) => println!("Valeur présente: {}", value),
        None => println!("Aucune Valeur présente"),
    }

}
