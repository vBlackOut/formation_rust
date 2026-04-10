fn parse_int(text: &str) -> Result<i32, std::num::ParseIntError> {
    match text.parse() {
        Ok(num) => Ok(num),
        Err(err) => Err(err),
    }
}


fn main() {
    let input = "42gg";

    match parse_int(input) {
        Ok(num) => println!("Enter parsé: {}", num),
        Err(err) => println!("Erreur de conversion : {:?}", err),
    }
}
