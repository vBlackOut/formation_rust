/*
Fonctionnalité

1) l'utilisateur doit etre invité à choisir une operation :
addition (+), soustraction (-), multiplication (*), ou division (/)

2) Ensuite, l'utilisateur doit entrer deux nombres
sur lesquels l'operation sera effectuée, l'un apres l'autre

3) l'application doit vérifier la validité de l'entrée de l'utilisateur

4) le calcul est affichée ainsi que son résultat

5) l'application ne s'eteint pas sauf si l'utilisateur le décide.
*/

fn calculator(input: String, string_operator: String) {

    let premier_nombre: Vec<_> = input.split(&string_operator).collect();
    let deuxieme_nombre: Vec<_> = input.split(&string_operator).collect();

    let premier_nombre: i32 = premier_nombre[0].parse().expect("Erreur lors de la lecture du nombre");
    let deuxieme_nombre: i32 = deuxieme_nombre[1].parse().expect("Erreur lors de la lecture du nombre");
    let mut result = 0;

    //println!("premier nombre : {:?}", premier_nombre);
    //println!("deuxieme nombre : {:?}", deuxieme_nombre);

    if string_operator.eq("+") {
        result = premier_nombre + deuxieme_nombre;
    }

    if string_operator.eq("-") {
        result = premier_nombre - deuxieme_nombre;
    }

    if string_operator.eq("*") {
        result = premier_nombre * deuxieme_nombre;
    }

    if string_operator.eq("/") {
        result = premier_nombre / deuxieme_nombre;
    }

    println!("resultat total: {}", result)
    //println!("{} {} {} = {}", premier_nombre, string_operator, deuxieme_nombre, result)
}

fn main() {
    println!("Bienvenue, dans ma calculatrice :)");

    loop {

        let mut input: String = String::new();
        println!("\ndonner moi votre calcule \n[envoyer x pour terminer]");
        std::io::stdin().read_line(&mut input).expect("Erreur lors de la lecture de la ligne");
        let input = input.trim();
        // vérification multiplication
        let operation = ["*", "/", "+", "-"].iter().find(|op| input.contains(**op) && input.matches(**op).count() == 1);

        match operation {
            Some(op) => calculator((&input).to_string(), (&&op).to_string()),
            None => println!("No operation found"),
        }

        if input.trim().eq("x") {
            break;
        }

    }
}
