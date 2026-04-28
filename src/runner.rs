//! Pipeline qui parcourt le dossier `data/` et génère les fichiers PlantUML
//! correspondants dans `graph/`.
//!
//! Ce module fournit deux fonctions utilitaires pour cette étape :
//! - [`list_json_files`] : liste les `.json` d'un dossier (triés par nom) ;
//! - [`output_path_for`] : calcule le chemin de sortie `.puml` pour un
//!   `.json` donné (`data/01_simple.json` → `graph/01_simple.puml`).
//!
//! La fonction `run_all` qui orchestre le pipeline complet sera ajoutée
//! à l'étape suivante.

use std::fs;
use std::path::{Path, PathBuf};

use crate::error::{Error, Result};

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
}
