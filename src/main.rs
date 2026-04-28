//! Démo de l'étape 8 : type d'erreur unifié.

use phone_trie::{load_contacts, Result, Trie};

fn main() -> Result<()> {
    println!("=== phone-trie : étape 8 (erreurs unifiées) ===\n");

    let path = "data/04_common_parts.json";
    let contacts = load_contacts(path)?;

    let mut trie = Trie::new();
    for contact in &contacts {
        trie.insert_contact(contact);
    }

    println!("Fichier : {path}");
    println!("Contacts insérés : {}\n", contacts.len());
    println!("Sortie PlantUML :\n");
    println!("{trie}");

    Ok(())
}
