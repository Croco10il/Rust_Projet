//! Sérialisation d'un trie au format PlantUML MindMap.
//!
//! # Format
//!
//! Le format MindMap utilise des étoiles pour marquer la profondeur :
//! - `* X` pour le niveau 1
//! - `** X` pour le niveau 2
//! - etc.
//!
//! Le document doit être encadré par les balises `@startmindmap` et
//! `@endmindmap`. Le nom d'un contact apparaît comme un nœud feuille
//! supplémentaire un niveau plus bas que le dernier chiffre de son numéro.
//!
//! # Choix d'implémentation
//!
//! On implémente le trait [`Display`] pour [`Trie`] plutôt qu'une fonction
//! libre `generate_plantuml(trie)`. Avantages :
//! - syntaxe naturelle : `format!("{trie}")` ou `println!("{trie}")` ;
//! - intégration native avec `write!`, `writeln!`, et tout l'écosystème
//!   Rust qui s'appuie sur `Display` ;
//! - on évite d'allouer une [`String`] intermédiaire si on écrit
//!   directement dans un fichier ou un buffer.

use std::fmt::{self, Display, Write};

use crate::trie::{Trie, TrieNode};

impl Display for Trie {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str("@startmindmap\n")?;
        write_node(self.root(), 1, f)?;
        f.write_str("@endmindmap\n")
    }
}

/// Écrit récursivement les enfants d'un nœud dans le formatter, en
/// utilisant `level` étoiles pour marquer la profondeur courante.
fn write_node(node: &TrieNode, level: usize, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    // Le `BTreeMap` garantit que les enfants sont parcourus dans l'ordre
    // croissant de leur caractère — pas besoin de trier manuellement.
    for (digit, child) in node.children() {
        // Écrit la ligne pour le chiffre courant : "*** 5" par exemple.
        write_stars(level, f)?;
        writeln!(f, " {digit}")?;

        // Si l'enfant est terminal, on écrit le nom comme un nœud feuille
        // supplémentaire un niveau plus bas. C'est ce que demande l'exemple
        // du sujet pour le format PlantUML MindMap.
        if let Some(name) = child.terminal() {
            write_stars(level + 1, f)?;
            writeln!(f, " {name}")?;
        }

        // Récursion sur les enfants de cet enfant.
        write_node(child, level + 1, f)?;
    }
    Ok(())
}

/// Écrit `count` étoiles dans le formatter.
fn write_stars(count: usize, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    for _ in 0..count {
        f.write_char('*')?;
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn empty_trie_outputs_only_markers() {
        let trie = Trie::new();
        let output = trie.to_string();
        assert_eq!(output, "@startmindmap\n@endmindmap\n");
    }

    #[test]
    fn single_number_outputs_chain_with_terminal() {
        let mut trie = Trie::new();
        trie.insert("123", "Alice");
        let output = trie.to_string();

        let expected = "\
@startmindmap
* 1
** 2
*** 3
**** Alice
@endmindmap
";
        assert_eq!(output, expected);
    }

    #[test]
    fn output_starts_and_ends_with_required_markers() {
        let mut trie = Trie::new();
        trie.insert("042", "Alice");
        trie.insert("142", "Bob");

        let output = trie.to_string();
        assert!(output.starts_with("@startmindmap\n"));
        assert!(output.ends_with("@endmindmap\n"));
    }
}
