fn addition(a: i32, b: i32) -> i32 {
    a + b
}

fn incrementer (nombre: &mut i32) {
    *nombre +=1
}


fn main() {
    let mut x: i32 = 5;
    incrementer(&mut x);
    println!("x après incrémentation : {}", x);
}
