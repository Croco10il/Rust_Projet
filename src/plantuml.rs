

use std::fmt::{self, Display, Write};

use crate::trie::{Trie, TrieNode};

impl Display for Trie {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        
        f.write_str("@startmindmap\n")?;


        write_node(self.root(), 1, f)?;

        f.write_str("@endmindmap\n")
    }
}

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


    fn trie_from_file(path: &str) -> Trie {
        let contacts = load_contacts(path).expect("fichier de test manquant");
        let mut trie = Trie::new();
        for c in &contacts {
            trie.insert_contact(c);
        }
        trie
    }

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

    #[test]
    fn output_for_simple_file() {
        let trie = trie_from_file("data/01_simple.json");
        let output = trie.to_string();

        assert!(output.starts_with("@startmindmap\n"));
        assert!(output.ends_with("@endmindmap\n"));
        assert!(output.contains("* 0"));
        assert!(output.contains("Alice"));
    }

    #[test]
    fn output_for_different_roots_has_two_top_branches() {
        let trie = trie_from_file("data/02_different_roots.json");
        let output = trie.to_string();

        let level1_lines = output
            .lines()
            .filter(|line| line.starts_with("* ") && !line.starts_with("**"))
            .count();
        assert_eq!(level1_lines, 2, "attendu 2 racines, sortie : {output}");
    }

    #[test]
    fn output_for_one_in_another_keeps_both_names() {
        let trie = trie_from_file("data/03_one_in_another.json");
        let output = trie.to_string();

        assert!(output.contains("Alice"));
        assert!(output.contains("Bob"));
    }

    #[test]
    fn output_for_common_parts_contains_all_five_names() {
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
