# Gestionnaire de numéros de téléphone (phone-trie)

Projet ESGI B3 SRC — implémentation d'un **trie** (prefix tree) en Rust pour le stockage et l'accès efficace de numéros de téléphone.

## Lancement

```bash
cargo run --release
```

Le programme lit tous les fichiers JSON présents dans le dossier `data/` et génère les fichiers PlantUML MindMap correspondants dans `graph/`.

## Structure du projet

```
phone-trie/
├── Cargo.toml
├── README.md
├── data/         # Fichiers JSON de test (ne pas modifier)
├── graph/        # Sortie PlantUML (.puml) — généré
└── src/
    ├── lib.rs
    └── main.rs
```

## Statut

🚧 En cours de développement — voir l'historique git pour le détail des étapes.
