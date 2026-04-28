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
    for (digit, child) in node.children() {
        write_stars(level, f)?;
        writeln!(f, " {digit}")?;

        if let Some(name) = child.terminal() {
            write_stars(level + 1, f)?;
            writeln!(f, " {name}")?;
        }

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
    use crate::load_contacts;

    /// Construit un trie à partir d'un fichier JSON, helper de tests.
    fn trie_from_file(path: &str) -> Trie {
        let contacts = load_contacts(path).expect("fichier de test manquant");
        let mut trie = Trie::new();
        for c in &contacts {
            trie.insert_contact(c);
        }
        trie
    }

    // ===== Tests basiques (étape 6) =====

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

    // ===== Tests sur les fichiers de test fournis (étape 7) =====

    #[test]
    fn output_for_simple_file() {
        // 01_simple : un seul contact "Alice" au numéro "0467123456".
        let trie = trie_from_file("data/01_simple.json");
        let output = trie.to_string();

        assert!(output.starts_with("@startmindmap\n"));
        assert!(output.ends_with("@endmindmap\n"));
        assert!(output.contains("* 0"));
        assert!(output.contains("Alice"));
    }

    #[test]
    fn output_for_different_roots_has_two_top_branches() {
        // 02_different_roots : "0123456789" et "1123456789".
        // → deux branches au niveau 1 ('0' et '1').
        let trie = trie_from_file("data/02_different_roots.json");
        let output = trie.to_string();

        // Compte les lignes qui ont exactement 1 étoile suivie d'un espace.
        let level1_lines = output
            .lines()
            .filter(|line| line.starts_with("* ") && !line.starts_with("**"))
            .count();
        assert_eq!(level1_lines, 2, "attendu 2 racines, sortie : {output}");
    }

    #[test]
    fn output_for_one_in_another_keeps_both_names() {
        // 03_one_in_another : "0123" préfixe de "0123456789".
        // Les deux noms doivent apparaître dans la sortie.
        let trie = trie_from_file("data/03_one_in_another.json");
        let output = trie.to_string();

        assert!(output.contains("Alice"));
        assert!(output.contains("Bob"));
    }

    #[test]
    fn output_for_common_parts_contains_all_five_names() {
        // 04_common_parts : 5 contacts (Alice, Bob, patate, Urgences, SAMU).
        let trie = trie_from_file("data/04_common_parts.json");
        let output = trie.to_string();

        for expected_name in ["Alice", "Bob", "patate", "Urgences", "SAMU"] {
            assert!(
                output.contains(expected_name),
                "le nom '{expected_name}' manque dans la sortie : {output}"
            );
        }
    }
}
