//! Implémentation d'un trie (prefix tree) pour stocker des numéros de téléphone.
//!
//! Le trie est implémenté entièrement « from scratch » comme imposé par le
//! sujet : aucun crate dédié n'est utilisé pour cette structure de données.
//!
//! # Choix d'implémentation
//!
//! - Les enfants d'un nœud sont stockés dans un [`BTreeMap`] plutôt qu'une
//!   [`std::collections::HashMap`]. Avantages :
//!     - les clés (les chiffres) sont naturellement triées par ordre
//!       croissant, ce qui rend la sortie PlantUML déterministe sans avoir
//!       à trier manuellement à chaque parcours ;
//!     - l'ordre déterministe simplifie aussi l'écriture des tests.
//! - Chaque nœud porte un champ [`TrieNode::terminal`] : c'est `Some(nom)`
//!   uniquement si le nœud termine un numéro complet (= un contact se
//!   termine ici), sinon `None`.
//! - La racine du [`Trie`] est privée. L'extérieur passe par les méthodes
//!   [`Trie::root`], [`Trie::insert`] et [`Trie::insert_contact`].

use std::collections::BTreeMap;

use crate::contact::Contact;

/// Un nœud du trie.
#[derive(Debug, Default)]
pub struct TrieNode {
    pub(crate) children: BTreeMap<char, TrieNode>,
    pub(crate) terminal: Option<String>,
}

impl TrieNode {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn is_terminal(&self) -> bool {
        self.terminal.is_some()
    }

    pub fn children(&self) -> &BTreeMap<char, TrieNode> {
        &self.children
    }

    pub fn terminal(&self) -> Option<&str> {
        self.terminal.as_deref()
    }
}

/// Trie de numéros de téléphone.
#[derive(Debug, Default)]
pub struct Trie {
    root: TrieNode,
}

impl Trie {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn is_empty(&self) -> bool {
        self.root.children.is_empty()
    }

    pub fn insert_contact(&mut self, contact: &Contact) {
        self.insert(&contact.nb, &contact.name);
    }

    pub fn insert(&mut self, number: &str, name: &str) {
        let mut current = &mut self.root;
        for digit in number.chars() {
            current = current.children.entry(digit).or_default();
        }
        current.terminal = Some(name.to_string());
    }

    pub fn root(&self) -> &TrieNode {
        &self.root
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    // ===== Tests basiques (étape 4) =====

    #[test]
    fn new_trie_is_empty() {
        let trie = Trie::new();
        assert!(trie.is_empty());
        assert!(trie.root().children().is_empty());
    }

    #[test]
    fn insert_creates_path_of_nodes() {
        let mut trie = Trie::new();
        trie.insert("123", "Alice");

        assert!(!trie.is_empty());

        let n1 = trie.root().children().get(&'1').expect("1 manquant");
        let n2 = n1.children().get(&'2').expect("2 manquant");
        let n3 = n2.children().get(&'3').expect("3 manquant");

        assert!(!n1.is_terminal());
        assert!(!n2.is_terminal());
        assert!(n3.is_terminal());
        assert_eq!(n3.terminal(), Some("Alice"));
    }

    #[test]
    fn insert_via_contact_works_like_insert() {
        let contact = Contact::new("0612345678", "Alice");
        let mut trie = Trie::new();
        trie.insert_contact(&contact);

        let mut node = trie.root();
        for c in "0612345678".chars() {
            node = node.children().get(&c).expect("chemin tronqué");
        }
        assert_eq!(node.terminal(), Some("Alice"));
    }

    #[test]
    fn inserting_same_number_twice_overwrites_name() {
        let mut trie = Trie::new();
        trie.insert("42", "Alice");
        trie.insert("42", "Bob");

        let n4 = trie.root().children().get(&'4').unwrap();
        let n2 = n4.children().get(&'2').unwrap();
        assert_eq!(n2.terminal(), Some("Bob"));
    }

    // ===== Tests des cas complexes (étape 5) =====

    #[test]
    fn shared_prefix_creates_branching() {
        // Deux numéros qui partagent le préfixe "12" : la branche commune
        // doit être partagée jusqu'au point de divergence.
        let mut trie = Trie::new();
        trie.insert("123", "Alice");
        trie.insert("124", "Bob");

        let n1 = trie.root().children().get(&'1').unwrap();
        let n2 = n1.children().get(&'2').unwrap();

        assert_eq!(n2.children().len(), 2);
        assert_eq!(n2.children().get(&'3').unwrap().terminal(), Some("Alice"));
        assert_eq!(n2.children().get(&'4').unwrap().terminal(), Some("Bob"));
    }

    #[test]
    fn prefix_collision_one_number_is_prefix_of_another() {
        // Cas du fichier 03_one_in_another.json : "0123" est préfixe
        // de "0123456789". Le nœud à la position "0123" doit être
        // simultanément terminal (pour Bob) ET avoir des enfants
        // (pour continuer vers Alice).
        let mut trie = Trie::new();
        trie.insert("0123", "Bob");
        trie.insert("0123456789", "Alice");

        let mut node = trie.root();
        for c in "0123".chars() {
            node = node.children().get(&c).unwrap();
        }
        assert_eq!(node.terminal(), Some("Bob"));
        assert!(node.children().contains_key(&'4'));

        let mut node = trie.root();
        for c in "0123456789".chars() {
            node = node.children().get(&c).unwrap();
        }
        assert_eq!(node.terminal(), Some("Alice"));
    }

    #[test]
    fn multiple_disjoint_roots() {
        let mut trie = Trie::new();
        trie.insert("0123", "Alice");
        trie.insert("1123", "Bob");

        assert_eq!(trie.root().children().len(), 2);
        assert!(trie.root().children().contains_key(&'0'));
        assert!(trie.root().children().contains_key(&'1'));
    }

    #[test]
    fn order_of_insertion_does_not_matter() {
        // BTreeMap garantit l'ordre par clé, pas par ordre d'insertion.
        let mut a = Trie::new();
        a.insert("123", "Alice");
        a.insert("124", "Bob");

        let mut b = Trie::new();
        b.insert("124", "Bob");
        b.insert("123", "Alice");

        assert_eq!(format!("{:?}", a), format!("{:?}", b));
    }
}
