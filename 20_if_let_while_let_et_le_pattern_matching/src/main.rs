enum Shape {
    Circle(f64),
    Rectangle(f64, f64),
    Triangle(f64, f64),
}

fn main() {
    let shape = Shape::Rectangle(5.0, 10.0);

    if let Shape::Rectangle(width, height) = shape {
        let area = width * height;
        println!("La forme est un rectangle avec une aire de {} unités carrées.", area);
    } else {
        println!("La forme n'est pas un rectangle ou n'est pas prise en charge.");
    }
}
