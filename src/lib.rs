//! Bibliothèque de gestion de numéros de téléphone via un trie (prefix tree).

#![forbid(unsafe_code)]

pub mod contact;
pub mod error;
pub mod parser;
pub mod plantuml;
pub mod trie;

pub use contact::Contact;
pub use error::{Error, Result};
pub use parser::load_contacts;
pub use trie::{Trie, TrieNode};
