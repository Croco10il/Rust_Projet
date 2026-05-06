//! Point d'entrée du programme phone-trie.
//!
//! Le main est volontairement minimal : toute la logique est dans la
//! bibliothèque (`lib.rs`), ce qui permet de la tester indépendamment.

use phone_trie::{runner, Result};

fn main() -> Result<()> {
    println!("=== phone-trie : pipeline complet ===\n");

    let outputs = runner::run_all("data", "graph")?;

    println!("Pipeline terminé. {} fichier(s) généré(s) :\n", outputs.len());
    for path in &outputs {
        println!("  ✓ {}", path.display());
    }
    println!(
        "\nLe contenu PlantUML est prêt à être visualisé.\n\
         Voir le README pour les instructions PlantUML."
    );

    Ok(())
}
