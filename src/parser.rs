//! Lecture et désérialisation des fichiers JSON contenant des contacts.

use std::fs;
use std::path::Path;

use crate::contact::Contact;

/// Charge une liste de contacts depuis un fichier JSON.
///
/// Les fichiers fournis dans le dossier `data/` contiennent parfois des
/// virgules terminales (« trailing commas ») qui ne sont pas conformes
/// à la spec JSON. Comme le sujet impose de **ne pas modifier** ces
/// fichiers, on effectue un léger prétraitement avant la
/// désérialisation par `serde_json`.
pub fn load_contacts<P: AsRef<Path>>(path: P) -> Result<Vec<Contact>, ParseError> {
    let raw = fs::read_to_string(path.as_ref())?;
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

        // Dans une string JSON : on copie tel quel et on gère l'échappement.
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

        // Si on tombe sur une virgule en dehors d'une string, on regarde
        // le prochain caractère non-blanc : si c'est `]` ou `}`, on
        // saute la virgule.
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

/// Erreur survenant lors du chargement d'un fichier de contacts.
#[derive(Debug)]
pub enum ParseError {
    /// Erreur d'I/O à la lecture du fichier.
    Io(std::io::Error),
    /// JSON malformé après prétraitement.
    Json(serde_json::Error),
}

impl std::fmt::Display for ParseError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ParseError::Io(e) => write!(f, "erreur d'I/O : {e}"),
            ParseError::Json(e) => write!(f, "JSON invalide : {e}"),
        }
    }
}

impl std::error::Error for ParseError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            ParseError::Io(e) => Some(e),
            ParseError::Json(e) => Some(e),
        }
    }
}

impl From<std::io::Error> for ParseError {
    fn from(e: std::io::Error) -> Self {
        ParseError::Io(e)
    }
}

impl From<serde_json::Error> for ParseError {
    fn from(e: serde_json::Error) -> Self {
        ParseError::Json(e)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    // --- Tests du prétraitement ---

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

    // --- Tests d'intégration sur les fichiers fournis ---

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
        assert!(matches!(err, ParseError::Io(_)));
    }
}
