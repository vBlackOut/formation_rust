
fn plus_grand<T>(x: T, y: T) -> T
where
    T: std::cmp::PartialOrd,
{
    if x > y {
        x
    } else {
        y
    }
}

fn main() {
    let a = 5;
    let b = 10;
    let max = plus_grand(a, b);
    println!("le plus grand nombre est : {}", max);

    let c = 3.5;
    let d = 2.7;
    let max_float = plus_grand(c, d);

    println!("le plus grand nombre flottant est : {}", max_float);
}
