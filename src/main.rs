//! Démo de l'étape 4 : construction d'un trie depuis des contacts en mémoire.

use phone_trie::{Contact, Trie};

fn main() {
    println!("=== phone-trie : étape 4 (trie de base) ===\n");

    // Quelques contacts en dur — pas encore de chargement JSON depuis main
    // (ça arrivera quand on branchera le pipeline complet).
    let contacts = vec![
        Contact::new("0612345678", "Alice"),
        Contact::new("0698765432", "Bob"),
        Contact::new("112", "Urgences"),
    ];

    // On construit le trie en y insérant chaque contact.
    let mut trie = Trie::new();
    for contact in &contacts {
        trie.insert_contact(contact);
    }

    println!("{} contact(s) insérés dans le trie.", contacts.len());
    println!("Trie vide ? {}", trie.is_empty());
    println!("\nReprésentation interne (debug-print) :\n");
    println!("{:#?}", trie);
}
