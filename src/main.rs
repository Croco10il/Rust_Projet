//! Démo de l'étape 7 : sortie PlantUML + tests étendus sur tous les fichiers.

use phone_trie::{load_contacts, Trie};
use std::error::Error;

fn main() -> Result<(), Box<dyn Error>> {
    println!("=== phone-trie : étape 7 (PlantUML + tests étendus) ===\n");

    let path = "data/04_common_parts.json";
    let contacts = load_contacts(path)?;

    let mut trie = Trie::new();
    for contact in &contacts {
        trie.insert_contact(contact);
    }

    println!("Fichier : {path}");
    println!("Contacts insérés : {}\n", contacts.len());
    println!("Sortie PlantUML MindMap :\n");
    println!("{trie}");

    Ok(())
}
