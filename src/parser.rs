//! Lecture et désérialisation des fichiers JSON contenant des contacts.

use std::fs;
use std::path::Path;

use crate::contact::Contact;
use crate::error::Result;

/// Charge une liste de contacts depuis un fichier JSON.
///
/// Les fichiers fournis dans le dossier `data/` contiennent parfois des
/// virgules terminales (« trailing commas ») qui ne sont pas conformes
/// à la spec JSON. Comme le sujet impose de **ne pas modifier** ces
/// fichiers, on effectue un léger prétraitement avant la
/// désérialisation par `serde_json`.
///
/// Un fichier qui ne contient qu'un tableau vide (`[]`) ou qui contient
/// uniquement des espaces / sauts de ligne est accepté et retourne un
/// `Vec` vide, sans erreur.
pub fn load_contacts<P: AsRef<Path>>(path: P) -> Result<Vec<Contact>> {
    let raw = fs::read_to_string(path.as_ref())?;

    // Cas particulier : fichier vide ou ne contenant que des espaces.
    // serde_json planterait avec une erreur peu explicite, on retourne
    // simplement une liste vide (cohérent avec un tableau JSON vide).
    if raw.trim().is_empty() {
        return Ok(Vec::new());
    }

    let cleaned = strip_trailing_commas(&raw);
    let contacts: Vec<Contact> = serde_json::from_str(&cleaned)?;
    Ok(contacts)
}

/// Retire les trailing commas dans un texte JSON (virgule placée juste
/// avant `]` ou `}`, éventuellement séparée par des espaces ou sauts
/// de ligne).
///
/// L'implémentation est volontairement simple : on parcourt le texte
/// caractère par caractère en gardant trace de si on est dans une
/// chaîne JSON (auquel cas on ne touche à rien).
fn strip_trailing_commas(input: &str) -> String {
    let mut out = String::with_capacity(input.len());
    let mut in_string = false;
    let mut escaped = false;

    let chars: Vec<char> = input.chars().collect();
    let mut i = 0;
    while i < chars.len() {
        let c = chars[i];

        if in_string {
            out.push(c);
            if escaped {
                escaped = false;
            } else if c == '\\' {
                escaped = true;
            } else if c == '"' {
                in_string = false;
            }
            i += 1;
            continue;
        }

        if c == '"' {
            in_string = true;
            out.push(c);
            i += 1;
            continue;
        }

        if c == ',' {
            let mut j = i + 1;
            while j < chars.len() && chars[j].is_whitespace() {
                j += 1;
            }
            if j < chars.len() && (chars[j] == ']' || chars[j] == '}') {
                i += 1;
                continue;
            }
        }

        out.push(c);
        i += 1;
    }

    out
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::error::Error;

    #[test]
    fn strip_trailing_comma_in_array() {
        let input = "[1, 2, 3,]";
        assert_eq!(strip_trailing_commas(input), "[1, 2, 3]");
    }

    #[test]
    fn strip_trailing_comma_in_object() {
        let input = r#"{"a": 1, "b": 2,}"#;
        assert_eq!(strip_trailing_commas(input), r#"{"a": 1, "b": 2}"#);
    }

    #[test]
    fn does_not_touch_commas_inside_strings() {
        let input = r#"["hello, world", "foo,"]"#;
        assert_eq!(strip_trailing_commas(input), input);
    }

    #[test]
    fn handles_whitespace_before_bracket() {
        let input = "[1, 2, 3,\n  ]";
        assert_eq!(strip_trailing_commas(input), "[1, 2, 3\n  ]");
    }

    #[test]
    fn no_op_on_valid_json() {
        let input = r#"{"a": [1, 2, 3], "b": "ok"}"#;
        assert_eq!(strip_trailing_commas(input), input);
    }

    #[test]
    fn load_simple_file() {
        let contacts = load_contacts("data/01_simple.json").expect("should parse");
        assert_eq!(contacts.len(), 1);
        assert_eq!(contacts[0], Contact::new("0467123456", "Alice"));
    }

    #[test]
    fn load_different_roots() {
        let contacts = load_contacts("data/02_different_roots.json").expect("should parse");
        assert_eq!(contacts.len(), 2);
    }

    #[test]
    fn load_one_in_another() {
        let contacts = load_contacts("data/03_one_in_another.json").expect("should parse");
        assert_eq!(contacts.len(), 2);
    }

    #[test]
    fn load_common_parts() {
        let contacts = load_contacts("data/04_common_parts.json").expect("should parse");
        assert_eq!(contacts.len(), 5);
    }

    #[test]
    fn load_missing_file_returns_io_error() {
        let err = load_contacts("data/does_not_exist.json").unwrap_err();
        assert!(matches!(err, Error::Io(_)));
    }

    // ===== Étape 12 : fix empty JSON =====

    #[test]
    fn empty_file_returns_empty_vec() {
        // Crée un fichier temporaire vide.
        let path = std::env::temp_dir().join("phone_trie_empty.json");
        std::fs::write(&path, "").unwrap();

        let contacts = load_contacts(&path).expect("doit accepter fichier vide");
        assert!(contacts.is_empty());

        std::fs::remove_file(&path).ok();
    }

    #[test]
    fn whitespace_only_file_returns_empty_vec() {
        let path = std::env::temp_dir().join("phone_trie_whitespace.json");
        std::fs::write(&path, "   \n\n  \t\n").unwrap();

        let contacts = load_contacts(&path).expect("doit accepter fichier blanc");
        assert!(contacts.is_empty());

        std::fs::remove_file(&path).ok();
    }

    #[test]
    fn empty_json_array_returns_empty_vec() {
        let path = std::env::temp_dir().join("phone_trie_empty_array.json");
        std::fs::write(&path, "[]").unwrap();

        let contacts = load_contacts(&path).expect("doit accepter []");
        assert!(contacts.is_empty());

        std::fs::remove_file(&path).ok();
    }
}
