fn parse_int_and_check_value(text: &str) -> Result<Option<i32>, std::num::ParseIntError> {
    match text.parse() {
        Ok(num) => {
            if num > 50 {
                Ok(Some(num))
            } else {
                Ok(None)
            }
        }
        Err(err) => Err(err),
    }
}

// resttructuration avec le ? qui permet de faire automatiquement
// le Match en cas d'erreur et de retourner directement l'erreur
// ne s'utilise que dans les fonctions qui renvois un result. 

fn parse_int_and_check_value2(text: &str) -> Result<Option<i32>, std::num::ParseIntError> {
    let num = text.parse()?;

    if num > 50 {
        Ok(Some(num))
    } else {
        Ok(None)
    }
}


fn main() {
    let input = "40";

    match parse_int_and_check_value2(input) {
        Ok(Some(num)) => println!("Entier parsé : {}", num),
        Ok(None) => println!("Nombre inférieur à 50"),
        Err(err) => println!("Erreur de conversion : {:?}", err),
    }
}
