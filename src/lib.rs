//! Bibliothèque de gestion de numéros de téléphone via un trie (prefix tree).
//!
//! Cette bibliothèque permet de :
//! - désérialiser un fichier JSON contenant des contacts (`nb`, `name`)
//! - construire un trie en mémoire à partir de ces contacts
//! - sérialiser ce trie au format PlantUML MindMap
//!
//! # Contraintes du projet
//! - Aucun code `unsafe` n'est autorisé (`#![forbid(unsafe_code)]`).
//! - Le trie est implémenté from scratch, sans crate dédié.

#![forbid(unsafe_code)]

pub mod contact;
pub mod parser;
pub mod plantuml;
pub mod trie;

pub use contact::Contact;
pub use parser::{load_contacts, ParseError};
pub use trie::{Trie, TrieNode};
