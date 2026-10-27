use GCRT_API_Inventory::inv_objs::item_type_registry::ItemTypeRegistryBuilder;

fn main(){
    let dname = "demo1";
    println!("--- Start {dname} ---");

    // 1. we create a register for all known types (for sorting later)
    let mut itrb = ItemTypeRegistryBuilder::new();
    // 2. we add all the types we want, and we get the identifier after adding them
    let drink = itrb.add("drink", &[]).unwrap();
    let hot = itrb.add("hot drink", &[drink]).unwrap();
    let cold = itrb.add("cold drink", &[drink]).unwrap();
    let milk = itrb.add("with milk", &[]).unwrap();
    let sugar_free = itrb.add("sugar free", &[]).unwrap();
    let sparkling = itrb.add("sparkling", &[cold]).unwrap();
    let still = itrb.add("still", &[cold]).unwrap();
    let tea = itrb.add("tea", &[hot]).unwrap();
    let latte = itrb.add("latte", &[hot, milk]).unwrap();
    let hot_chocolate = itrb.add("hot chocolate", &[hot, milk]).unwrap();
    let soda = itrb.add("soda", &[sparkling]).unwrap();
    let water = itrb.add("water", &[still, sugar_free]).unwrap();
    let syrup = itrb.add("fruit syrup", &[cold]).unwrap();
    // 4. don't forget to build to get the final result 
    let itrb = itrb.build();
    // 3. we add real items to the inventory
    let contents = [tea, latte, soda, water, hot_chocolate, syrup]; // slots 0..5
    // du coup je fais quoi ici ?

    //...
    //...
    //...

    println!("\n--- {dname} ending ---");
}


/**
 * question :
 * - du coup let itrb = itrb.build() c'est sencé être valide mais c'est bizarre je trouve...
 * de plus je viens d'aller voir dans le fichier et ça renvoie : "Arc<ItemTypeRegistry>" 
 * qui n'est pas le même type car c'est entouré d'un Arc
 * - du coup je mets qoui a la ligne "let mut f=...." ? et cette ligne 
 * servait a quoi avant (c'est pour filtrer c'est bien ça ? mais c'est bizarre de 
 * l'initialiser avec la taille du contenu et du registre...)
 * - je dois mettre quoi ensuite pour filtrer par catégorie (par exemple les "hot") ?
 * - je fais comment si je veux filtrer par "hot" et par "sugar_free" en même temps?
 * - ça fait quoi si j'essaie de filtrer par "cold" et par "hot" en même temps ?
 * 
 */

/**
 * - j'aimerais éviter les "mouvement" inutiles, par exemple dans
 * let drink =.... il y a "&[]" qui est inutile ici et j'aimerais
 * éviter de saisi des choses inutiles car ça alourdi l'affectation
 * et l'écriture du code, on ne devrait jamais écrire du code qui 
 * ne sers a rien, ici le &[] ne sers a rien et on devrait juste
 * écrire 1 seul argument :
 * let drink = itrb.add("drink").unwrap();
 * Donc il faut modifier le code en conséquence pour que rien ne
 * soit initialisé/construit si on ne met rien, et ça ne doit
 * pas causer d'erreurs ou autre... c'est vrai pour cet exemple 
 * mais c'est aussi vrai pour d'autres extraits de code, c'est pour
 * ça que je généralise ici en disant de ne pas faire de code 
 * inutile dans ces cas la.
 */


