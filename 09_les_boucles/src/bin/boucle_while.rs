/*
Dois etre delcarer avec une condition si la condition de départ
est false elle ne sera pas executer.
*/


// exemple 1
fn ex1(){
    println!("\nExercice 1");

    let mut i: i32 = 0;

    while i < 5 {
        println!("{}", i);
        i += 1;
    }
}

/*
on peu utilise continue
pour passer la prochaine iteration
*/

// exemple 2
fn ex2(){
    println!("\nExercice 2");

    let mut n: i32 = 0;

    while n < 10 {
        n += 1;
        if n == 5 { // permet de sauter la valeur 5 et ne pas l'afficher
            continue;
        }
        println!("{}", n);
    }
}


// exemple 3
fn ex3(){
    println!("\nExercice 3");

    let mut n: i32 = 0;

    while n < 100 {
        n += 1;
        if n % 5 == 0 { // permet d'arreter la boucle meme
                        // si la condition de la boucle n'ai pas atteinte.
            break;
        }
        println!("{}", n);
    }
}

// il n'ai pas possible d'utiliser while pour une assignation de varable
// car le retour d'une boucle while ne permet pas de faire de retour.


// exemple 4 corriger par le cours voir le programme
// execercies.rs pour un programme plus optimiser
fn ex4(){
    println!("\nExercice 4");

    let mut table_en_cours: i32 = 1;
    let table_maximum: i32 = 10;
    
    // boucle externe - creation des tables
    while table_en_cours <= table_maximum {
        let mut nombre_en_cours: i32 = 1;
        let nombre_maximum = 10;

        // boucle interne
        while nombre_en_cours <= nombre_maximum {
            println!("{} X {} = {}", table_en_cours, nombre_en_cours, table_en_cours * nombre_en_cours);
            nombre_en_cours += 1;
        }
        table_en_cours += 1;
    }

}

fn main(){
    ex1();
    ex2();
    ex3();
    ex4();
}
