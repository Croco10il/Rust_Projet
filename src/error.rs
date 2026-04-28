//! Type d'erreur unifié pour la bibliothèque phone-trie.
//!
//! On regroupe ici toutes les erreurs susceptibles de remonter de la lib :
//! - I/O (lecture / écriture de fichier)
//! - JSON (désérialisation)
//! - dossier de données invalide
//!
//! # Choix d'implémentation
//!
//! Plutôt qu'un enum dont les variantes ne portent que des `String`,
//! on conserve les erreurs originales (`std::io::Error`,
//! `serde_json::Error`) et on implémente
//! [`std::error::Error::source`] pour les exposer à l'utilisateur. Cela
//! permet à un appelant de remonter la chaîne complète des causes
//! avec `e.source()`, ce qui est très utile au debug.

use std::fmt;

/// Erreur produite par la bibliothèque phone-trie.
#[derive(Debug)]
pub enum Error {
    /// Erreur d'I/O (lecture ou écriture de fichier).
    Io(std::io::Error),
    /// JSON malformé après prétraitement.
    Json(serde_json::Error),
    /// Le dossier de données est invalide ou inexistant.
    InvalidDataDirectory(String),
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Error::Io(e) => write!(f, "erreur d'I/O : {e}"),
            Error::Json(e) => write!(f, "JSON invalide : {e}"),
            Error::InvalidDataDirectory(msg) => {
                write!(f, "dossier de données invalide : {msg}")
            }
        }
    }
}

impl std::error::Error for Error {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            Error::Io(e) => Some(e),
            Error::Json(e) => Some(e),
            Error::InvalidDataDirectory(_) => None,
        }
    }
}

impl From<std::io::Error> for Error {
    fn from(e: std::io::Error) -> Self {
        Error::Io(e)
    }
}

impl From<serde_json::Error> for Error {
    fn from(e: serde_json::Error) -> Self {
        Error::Json(e)
    }
}

/// Alias pratique : la majorité des fonctions de la lib retournent
/// `Result<T, Error>`, on raccourcit en `Result<T>`.
pub type Result<T> = std::result::Result<T, Error>;
