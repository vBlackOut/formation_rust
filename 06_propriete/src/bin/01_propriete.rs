// cargo run --bin 01_propriete

fn main() {
    let s: String = String::from("hello"); // s devient proprietaire
    // s.push_str(" word") ne fonctionnerait pas car n'est pas mutable.

    {
        let mut t: String = s; // t devient proprietaire, s n'est plus valide
        t.push_str(" world");
    } // t sort de la portée, "hello" est supprimé

    // içi, s ne peut plus être utilisée car elle n'est plus proprietaire

    println!("{}", s);
    //   Error:    ^ value borrowed here after move

    /*
    Résolution du problème 1:
        Gérer le problème de proprieter
        le clonage
        faire un s.clone() à la ligne 8.
    */

    /*
    Résolution du problème 2:
        Gérer le problème de proprieter
        Faire une copie si possible.

        **
        les valeuir type int sont automatiquement copier et
        pas les types string, vecteurs ou les structures.
        # lien : https://jenniferchukwu.com/posts/copyclone
        **

    */

}
