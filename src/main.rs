//! Démo de l'étape 5 : trie chargé depuis un fichier JSON réel.

use phone_trie::{load_contacts, Trie};
use std::error::Error;

fn main() -> Result<(), Box<dyn Error>> {
    println!("=== phone-trie : étape 5 (trie depuis fichier JSON) ===\n");

    let path = "data/04_common_parts.json";
    let contacts = load_contacts(path)?;

    let mut trie = Trie::new();
    for contact in &contacts {
        trie.insert_contact(contact);
    }

    println!("Fichier : {path}");
    println!("Contacts insérés : {}", contacts.len());
    println!("\nReprésentation interne du trie :\n");
    println!("{:#?}", trie);

    Ok(())
}
