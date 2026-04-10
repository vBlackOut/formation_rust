fn main() {
    let s: String = String::from("toto"); // s debient propriétaure
    {
        let t: String = s; // t debient proprietaire, s n'est polus valide
    } // t sort de la portée, "hello" est supprimé

    println!("{}", s);
    /* içi, s ne peut plus être utilisée car elle ,'est plus proprietaire
       s devient invalide erreur de compilation
    */
}
