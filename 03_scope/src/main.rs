fn main() {
    let x: i32 = 5;

    {
        let y: i32 = 10;
        println!("Inside, x: {}, y: {}", x, y);
    }

    println!("Outside, x: {}", x);
    //println!("Outside, y: {}", y);

}
