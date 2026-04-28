//! Démo de l'étape 2 : création de contacts en mémoire.

use phone_trie::Contact;

fn main() {
    println!("=== phone-trie : étape 2 (création de contacts) ===\n");

    let contacts = vec![
        Contact::new("0612345678", "Alice"),
        Contact::new("0698765432", "Bob"),
        Contact::new("112", "Urgences"),
    ];

    println!("Contacts créés en mémoire :");
    for c in &contacts {
        println!("  • {:<10} → {}", c.name, c.nb);
    }

    println!("\nTotal : {} contact(s).", contacts.len());
}
