//! Définition d'un contact (numéro de téléphone + nom).

use serde::Deserialize;

/// Représente un contact, tel que stocké dans les fichiers JSON d'entrée.
///
/// Les noms des champs (`nb`, `name`) reflètent exactement les clés du
/// fichier JSON — pas de renommage via `#[serde(rename = ...)]`, ce qui
/// garde la struct simple et évite toute divergence avec les données.
#[derive(Debug, Clone, Deserialize, PartialEq, Eq)]
pub struct Contact {
    /// Numéro de téléphone, sans préfixe international.
    pub nb: String,
    /// Nom (ou label) associé au numéro.
    pub name: String,
}

impl Contact {
    /// Crée un nouveau contact à la main (utile pour les tests).
    pub fn new(nb: impl Into<String>, name: impl Into<String>) -> Self {
        Self {
            nb: nb.into(),
            name: name.into(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn new_creates_contact_with_given_values() {
        let c = Contact::new("0612345678", "Alice");
        assert_eq!(c.nb, "0612345678");
        assert_eq!(c.name, "Alice");
    }
}
