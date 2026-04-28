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
//!   termine ici), sinon `None`. C'est plus expressif que d'avoir un
//!   simple booléen + un champ de nom séparé.
//! - La racine du [`Trie`] est privée. L'extérieur passe par les méthodes
//!   [`Trie::root`], [`Trie::insert`] et [`Trie::insert_contact`].

use std::collections::BTreeMap;

use crate::contact::Contact;

/// Un nœud du trie.
///
/// Un nœud peut être *terminal* (il porte alors le nom du contact dont le
/// numéro se termine ici), ou simplement un nœud de passage vers ses enfants.
#[derive(Debug, Default)]
pub struct TrieNode {
    /// Enfants du nœud, indexés par leur caractère (chiffre).
    /// Visibilité `pub(crate)` : les autres modules de la lib (notamment
    /// le futur module `plantuml`) y ont accès, mais pas l'API publique.
    pub(crate) children: BTreeMap<char, TrieNode>,
    /// Nom du contact si ce nœud termine un numéro, `None` sinon.
    pub(crate) terminal: Option<String>,
}

impl TrieNode {
    /// Crée un nouveau nœud vide.
    pub fn new() -> Self {
        Self::default()
    }

    /// Indique si ce nœud termine un numéro complet.
    pub fn is_terminal(&self) -> bool {
        self.terminal.is_some()
    }

    /// Retourne une référence vers les enfants du nœud.
    /// Les enfants sont ordonnés par caractère (grâce au [`BTreeMap`]).
    pub fn children(&self) -> &BTreeMap<char, TrieNode> {
        &self.children
    }

    /// Retourne le nom associé si ce nœud est terminal.
    pub fn terminal(&self) -> Option<&str> {
        self.terminal.as_deref()
    }
}

/// Trie de numéros de téléphone.
///
/// Chaque numéro est représenté par un chemin allant de la racine à un nœud
/// terminal. Les numéros qui partagent un préfixe partagent aussi le chemin
/// correspondant — c'est tout l'intérêt d'un prefix tree.
#[derive(Debug, Default)]
pub struct Trie {
    /// Racine du trie. Volontairement privée pour forcer l'usage de l'API.
    root: TrieNode,
}

impl Trie {
    /// Construit un trie vide.
    pub fn new() -> Self {
        Self::default()
    }

    /// Indique si le trie ne contient aucun numéro.
    pub fn is_empty(&self) -> bool {
        self.root.children.is_empty()
    }

    /// Insère un contact dans le trie.
    ///
    /// C'est l'API recommandée : elle est type-safe (on ne risque pas
    /// d'inverser numéro et nom) et reflète le modèle métier.
    pub fn insert_contact(&mut self, contact: &Contact) {
        self.insert(&contact.nb, &contact.name);
    }

    /// Insère un numéro et son nom dans le trie.
    ///
    /// Si le numéro existe déjà, le nom précédent est écrasé.
    pub fn insert(&mut self, number: &str, name: &str) {
        let mut current = &mut self.root;
        for digit in number.chars() {
            current = current.children.entry(digit).or_default();
        }
        current.terminal = Some(name.to_string());
    }

    /// Donne accès à la racine du trie en lecture seule.
    /// Utile pour les modules qui parcourent l'arbre (ex : sérialisation).
    pub fn root(&self) -> &TrieNode {
        &self.root
    }
}

#[cfg(test)]
mod tests {
    use super::*;

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

        // On suit le chemin 1 → 2 → 3.
        let n1 = trie.root().children().get(&'1').expect("1 manquant");
        let n2 = n1.children().get(&'2').expect("2 manquant");
        let n3 = n2.children().get(&'3').expect("3 manquant");

        // Seul le dernier nœud doit être terminal.
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

        // On suit le numéro complet et on vérifie qu'il termine bien sur "Alice".
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
}
