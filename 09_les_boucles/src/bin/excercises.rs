/*
Exercce :

- tenter d'afficher dans le trminale les tables de multiplicatin allant de 1 a 10.
- utiliser le prioncipe de loop et de priontln! pour afficher vis opération et résultat

le mot clé break vous permettra de sortir de la boucle.
*/

fn main(){
    let mut table: i32 = 1;
    let nombre_maximum: i32 = 10;
    let table_maximum: i32 = 10;

    loop {
        println!("\nTable de multiplication de  {}", table);

        for multiplicateur in 1..=nombre_maximum {
            let resultat = table * multiplicateur;
            println!("la multiplication de {} x {} = {}", table, multiplicateur, resultat);
        }

        table += 1;

        if table > table_maximum {
            break;
        }
    }

}
