use std::ops::Range;
/*
Exemple de boucle for
les .. est designer type range en rust.

0..5 designe de 0 à 4.

les :? permet de faire un print en mode debug
*/

fn ex1(){
    println!("\nExercice 1");

    for i in 0..5 {
        println!("{}", i);
    }
}

fn ex2(){
    println!("\nExercice 2");
    let range: Range<i32> = 0..5;

    println!("{:?}", range);
    for i in range.clone() { // utiliser la fonction .clone pour faire une copie ou une
                            // reference si cela est dans un autre type
                            // car par defaut le range se detruit lorsque la boucle consomme la variable.
        println!("{}", i);
    }
    println!("{:?}", range);
}

fn ex3(){
    // un trait en rust ( les table_maximumux implement automatiquement une
    // proprieter copie ce qui ne vas pas detruire le tableaux
    println!("\nExercice 3");

    let a = [10, 20, 30, 40, 50];

    for element in a {
        println!("la valeur est : {}", element);
    }
    println!("{:?}", a);  // permet d'afficher car c'est un iterateur une listes
}

fn ex4() {
    println!("\nExercice 4");

    let vec = vec!["Rust", "C", "Python", "Javascript"];

    for lang in &vec {
        println!("{}", lang)
    }

    println!("{:?}", vec)

}

fn ex5() {
    println!("\nExercice 5");
    let a: [i32; 5] = [10, 20, 30, 40, 50];
    for (index, value) in a.iter().enumerate() {
        println!("a[{}] = {}", index, value);
    }

    println!("{:?}", a); // permet d'afficher car c'est un iterateur une listes

}

fn ex6(){

    println!("\nExercice 6");
    // un tuple en rust est une collection qui peux
    // contenir plusieur données differentes int, str, ect...
    // il sont une taille fix ni aggrandir ni retressir
    let mon_tuple = (1, "deux", 3.0);
    println!("{:?}", mon_tuple);
}

fn ex7(){

    println!("\nExercice 7");
    let pairs = vec![(1, "one"), (2, "two"), (3, "three")];

    // permet directement de destructuré directement les tuples.
    for (number, name) in pairs {
        println!("Number: {}, Name : {}", number, name);
    }
}


fn main() {
    ex1();
    ex2();
    ex3();
    ex4();
    ex5();
    ex6();
    ex7();
}
