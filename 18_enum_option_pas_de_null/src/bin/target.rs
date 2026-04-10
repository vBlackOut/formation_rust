fn find_element(arr: &Vec<i32>,  target: i32) -> Option<usize> {
    for (index, element) in arr.iter().enumerate() {
        if element == &target {
            return Some(index);
        }
    }
    None
}

fn main() {
    let number: Vec<i32> = vec![1, 2, 3, 4, 5];
    let target_value: i32 = 3;

    match find_element(&number, target_value) {
        Some(index) => println!("Valeur trouvée à l'indice : {}", index),
        None => println!("Valeur non trouvée"),
    }
}
