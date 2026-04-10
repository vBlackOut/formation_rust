fn get_bigger_number<'a>(num1: &'a i32, num2: &'a i32) -> &'a i32 {
    if num1 > num2 {
        num1
    } else {
        num2
    }
}


fn get_bigger_number2<'a, 'b: 'a>(num1: &'a i32, num2: &'b i32) -> &'a i32 {
    if num1 > num2 {
        num1
    } else {
        num2
    }
}

fn main() {
    let number1 = 14;
    let number2 = 15;
    let resultat = get_bigger_number(&number1, &number2);
    let resultat2 = get_bigger_number2(&number1, &number2);
    println!("{resultat} {resultat2}");

}
