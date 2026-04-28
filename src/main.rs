//! Démo de l'étape 9 : détection automatique des fichiers JSON dans data/.

use phone_trie::{runner, Result};

fn main() -> Result<()> {
    println!("=== phone-trie : étape 9 (détection des fichiers) ===\n");

    let files = runner::list_json_files("data")?;
    println!("Fichiers JSON détectés dans data/ ({}) :\n", files.len());
    for path in &files {
        let output = runner::output_path_for(path, std::path::Path::new("graph"))?;
        println!("  • {} → {}", path.display(), output.display());
    }

    Ok(())
}
