//! Pipeline qui parcourt le dossier `data/` et génère les fichiers PlantUML
//! correspondants dans `graph/`.
//!
//! C'est le module qui orchestre tout le projet :
//!
//! ```text
//!   data/*.json  ──[load_contacts]──▶  Vec<Contact>
//!                                          │
//!                                          ▼
//!                              ──[Trie::insert_contact]──▶  Trie
//!                                                            │
//!                                                            ▼
//!                                                  ──[Display]──▶  graph/*.puml
//! ```
//!
//! L'utilisateur final n'a qu'à appeler [`run_all`] qui s'occupe de tout.

use std::fs;
use std::path::{Path, PathBuf};

use crate::error::{Error, Result};
use crate::Trie;

/// Liste tous les fichiers JSON présents dans un répertoire, triés
/// par ordre alphabétique pour avoir une sortie déterministe.
pub fn list_json_files<P: AsRef<Path>>(dir: P) -> Result<Vec<PathBuf>> {
    let dir = dir.as_ref();
    if !dir.is_dir() {
        return Err(Error::InvalidDataDirectory(format!(
            "{} n'est pas un dossier valide",
            dir.display()
        )));
    }

    let mut files: Vec<PathBuf> = fs::read_dir(dir)?
        .filter_map(|entry| entry.ok())
        .map(|entry| entry.path())
        .filter(|path| {
            path.is_file() && path.extension().is_some_and(|ext| ext == "json")
        })
        .collect();

    files.sort();
    Ok(files)
}

/// Calcule le chemin de sortie `.puml` pour un fichier `.json` donné.
///
/// Exemple : `data/01_simple.json` → `graph/01_simple.puml`
pub fn output_path_for(json_path: &Path, output_dir: &Path) -> Result<PathBuf> {
    let stem = json_path
        .file_stem()
        .and_then(|s| s.to_str())
        .ok_or_else(|| {
            Error::InvalidDataDirectory(format!(
                "nom de fichier invalide : {}",
                json_path.display()
            ))
        })?;
    Ok(output_dir.join(format!("{stem}.puml")))
}

/// Construit un trie à partir d'un fichier JSON.
pub fn build_trie<P: AsRef<Path>>(json_path: P) -> Result<Trie> {
    let contacts = crate::parser::load_contacts(json_path)?;
    let mut trie = Trie::new();
    for contact in &contacts {
        trie.insert_contact(contact);
    }
    Ok(trie)
}

/// Traite un fichier JSON : le charge, construit le trie, écrit le `.puml`.
/// Retourne le chemin du fichier de sortie.
pub fn process_file(json_path: &Path, output_dir: &Path) -> Result<PathBuf> {
    let trie = build_trie(json_path)?;
    let output = output_path_for(json_path, output_dir)?;
    fs::write(&output, trie.to_string())?;
    Ok(output)
}

/// Pipeline complet : parcourt `data_dir`, traite chaque fichier JSON,
/// écrit le résultat PlantUML dans `output_dir`.
///
/// Si `output_dir` n'existe pas, il est créé.
pub fn run_all<P: AsRef<Path>>(data_dir: P, output_dir: P) -> Result<Vec<PathBuf>> {
    let data_dir = data_dir.as_ref();
    let output_dir = output_dir.as_ref();

    if !output_dir.exists() {
        fs::create_dir_all(output_dir)?;
    }

    let files = list_json_files(data_dir)?;
    let mut outputs = Vec::with_capacity(files.len());

    for json_path in files {
        let output = process_file(&json_path, output_dir)?;
        outputs.push(output);
    }

    Ok(outputs)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn list_json_files_finds_all_data_files() {
        let files = list_json_files("data").expect("dossier data manquant");
        assert_eq!(files.len(), 4);
    }

    #[test]
    fn list_json_files_returns_sorted_paths() {
        let files = list_json_files("data").unwrap();
        let names: Vec<String> = files
            .iter()
            .map(|p| p.file_name().unwrap().to_string_lossy().into_owned())
            .collect();

        assert_eq!(names[0], "01_simple.json");
        assert_eq!(names[1], "02_different_roots.json");
        assert_eq!(names[2], "03_one_in_another.json");
        assert_eq!(names[3], "04_common_parts.json");
    }

    #[test]
    fn list_json_files_rejects_missing_directory() {
        let err = list_json_files("does_not_exist").unwrap_err();
        assert!(matches!(err, Error::InvalidDataDirectory(_)));
    }

    #[test]
    fn output_path_for_replaces_extension() {
        let p = Path::new("data/01_simple.json");
        let out = output_path_for(p, Path::new("graph")).unwrap();
        assert_eq!(out, Path::new("graph/01_simple.puml"));
    }

    #[test]
    fn build_trie_from_simple_file() {
        let trie = build_trie("data/01_simple.json").unwrap();
        assert!(!trie.is_empty());
    }
}
