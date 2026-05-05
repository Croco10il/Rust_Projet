use std::fs;
use std::path::PathBuf;

use phone_trie::runner;

/// Crée un dossier temporaire propre pour un test (isolé entre tests).
fn temp_output_dir(test_name: &str) -> PathBuf {
    let dir = std::env::temp_dir().join(format!("phone_trie_test_{test_name}"));
    if dir.exists() {
        fs::remove_dir_all(&dir).expect("nettoyage dossier temp");
    }
    fs::create_dir_all(&dir).expect("création dossier temp");
    dir
}

#[test]
fn pipeline_produces_one_puml_per_json() {
    let out = temp_output_dir("pipeline_basic");
    let outputs = runner::run_all("data", &out).expect("pipeline a planté");

    assert_eq!(outputs.len(), 4, "4 fichiers attendus, vu : {outputs:?}");

    for path in &outputs {
        assert!(path.exists(), "{} n'a pas été créé", path.display());
        assert_eq!(
            path.extension().unwrap(),
            "puml",
            "extension invalide pour {}",
            path.display()
        );
    }
}

#[test]
fn produced_files_have_mindmap_markers() {
    let out = temp_output_dir("pipeline_markers");
    let outputs = runner::run_all("data", &out).expect("pipeline a planté");

    for path in &outputs {
        let content = fs::read_to_string(path).expect("lecture du .puml");
        assert!(
            content.starts_with("@startmindmap\n"),
            "{} ne commence pas par @startmindmap",
            path.display()
        );
        assert!(
            content.ends_with("@endmindmap\n"),
            "{} ne finit pas par @endmindmap",
            path.display()
        );
    }
}

#[test]
fn common_parts_output_contains_all_names() {
    let out = temp_output_dir("pipeline_common");
    runner::run_all("data", &out).unwrap();

    let content = fs::read_to_string(out.join("04_common_parts.puml")).unwrap();
    for name in ["Alice", "Bob", "patate", "Urgences", "SAMU"] {
        assert!(
            content.contains(name),
            "le nom '{name}' manque du fichier 04_common_parts.puml"
        );
    }
}

#[test]
fn pipeline_creates_output_directory_if_missing() {
    let out = std::env::temp_dir().join("phone_trie_test_create_dir");
    if out.exists() {
        fs::remove_dir_all(&out).unwrap();
    }
    // Le dossier n'existe pas — run_all doit le créer.
    runner::run_all("data", &out).expect("doit créer le dossier de sortie");
    assert!(out.is_dir());
}

#[test]
fn pipeline_fails_on_missing_data_directory() {
    let out = temp_output_dir("pipeline_missing_data");
    let err = runner::run_all("does_not_exist", &out).unwrap_err();

    use phone_trie::Error;
    assert!(matches!(err, Error::InvalidDataDirectory(_)));
}
