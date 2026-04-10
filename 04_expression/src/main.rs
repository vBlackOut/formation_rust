fn main() {

    let y: &str = {
        let x: i32 = 3;
        let x = "test";
        x
    };

    println!("y: {}", y);
}
