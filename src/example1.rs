use crate::inv_objs::item_type::{ItemType, ItemTyped};



fn main(){
    println!("--- Démarrage de la démonstration des Items ---");

    // 1. we create items but in a real use case we NEVER
    // insticiate those types inside variables, we group them instead in 
    // a larger structure to use them as a group of data, 
    // (we see identifiers later for reuse in real objects)
    let weapon = ItemTyped::new(String::from("weapon"));
    // sous types de weapons
        let firearm = ItemTyped::new(String::from("firearm"));
            let handgun = ItemTyped::new(String::from("handgun"));
            let smg = ItemTyped::new(String::from("smg"));
            let sr = ItemTyped::new(String::from("sr"));
            let ar = ItemTyped::new(String::from("ar"));
        let melee = ItemTyped::new(String::from("melee"));
            // sous type melee
            let sword = ItemTyped::new(String::from("sword"));
    let heal = ItemTyped::new(String::from("heal"));
    let gear = ItemTyped::new(String::from("gear"));

    // 2. Collecte dans un vecteur (Vector). Le type doit être le pointeur vers la trait,
    // pour que tous les items soient traités comme des "ItemType" génériques.
    let inventory: Vec<&dyn ItemType> = vec![&weapon, &heal, &gear];

    println!("\n--- Inventaire Actuel ---");

    // 3. Itération et utilisation du trait (Le but de la démonstration!)
    // Grâce au Trait, nous n'avons pas besoin de savoir si c'est une épée ou un bouclier.
    for item in inventory {
        // Nous appelons get_name(), mais Rust sait grâce à `&dyn ItemType` 
        // quelle vraie implémentation appeler (ItemTyped). C'est la polymorphisme!
        println!("-> Élément trouvé : {}", item.get_name());
    }

    println!("\n--- Fin de la démonstration ---");
}


