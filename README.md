# 📞 phone-trie

> Gestionnaire de numéros de téléphone basé sur un **trie** (prefix tree),
> avec export au format **PlantUML MindMap** pour la visualisation.

[![Rust](https://img.shields.io/badge/Rust-2021-orange?logo=rust)](https://www.rust-lang.org/)
[![License](https://img.shields.io/badge/license-MIT-blue.svg)](LICENSE)

Projet réalisé dans le cadre du module **Rust** — ESGI B3 SRC.

---

## 📑 Sommaire

- [✨ Présentation](#-présentation)
- [🚀 Démarrage rapide](#-démarrage-rapide)
- [📂 Structure du projet](#-structure-du-projet)
- [🏗️ Architecture](#️-architecture)
- [🔬 Choix techniques](#-choix-techniques)
- [🧪 Tests](#-tests)
- [✅ Qualité du code](#-qualité-du-code)
- [🎨 Visualiser les graphes PlantUML](#-visualiser-les-graphes-plantuml)
- [📋 Exemple complet](#-exemple-complet)
- [👥 Auteurs](#-auteurs)

---

## ✨ Présentation

Un **trie** (aussi appelé *prefix tree*) est une structure de données
spécialisée dans le stockage de chaînes partageant des préfixes communs.
C'est la structure idéale pour :

- l'autocomplétion (saisie de numéros, mots, etc.)
- la correction orthographique
- la déduplication implicite des préfixes

Dans ce projet, on l'utilise pour stocker des **numéros de téléphone**
associés à des noms de contacts, puis pour visualiser cette structure
sous forme de **MindMap PlantUML**.

### Pipeline du programme

```
┌─────────────────┐     ┌──────────────┐     ┌──────────┐     ┌────────────────┐
│  data/*.json    │────▶│ Désérialise  │────▶│  Trie    │────▶│ graph/*.puml   │
│  (contacts)     │     │  (serde)     │     │ (memory) │     │ (PlantUML)     │
└─────────────────┘     └──────────────┘     └──────────┘     └────────────────┘
```

---

## 🚀 Démarrage rapide

### Prérequis

- [Rust](https://rustup.rs/) (édition 2021 minimum)
- `cargo` (fourni avec rustup)

### Lancement

```bash
# Cloner le repo
git clone https://github.com/<votre-org>/phone-trie.git
cd phone-trie

# Compiler et lancer en mode optimisé
cargo run --release
```

Le programme parcourt automatiquement le dossier `data/`, traite chaque
fichier `.json` et écrit un fichier `.puml` correspondant dans `graph/`.

**Sortie attendue :**

```
=== phone-trie : pipeline complet ===

Pipeline terminé. 4 fichier(s) généré(s) :

  ✓ graph/01_simple.puml
  ✓ graph/02_different_roots.puml
  ✓ graph/03_one_in_another.puml
  ✓ graph/04_common_parts.puml

Le contenu PlantUML est prêt à être visualisé.
Voir le README pour les instructions PlantUML.
```

---

## 📂 Structure du projet

```
phone-trie/
├── 📄 Cargo.toml          # Manifest Rust
├── 📄 README.md           # Ce fichier
├── 📄 LICENSE             # Licence MIT
├── 📄 rustfmt.toml        # Configuration de cargo fmt
├── 📄 .gitignore
│
├── 📁 data/               # ⚠️ Fichiers JSON d'entrée (ne pas modifier)
│   ├── 01_simple.json
│   ├── 02_different_roots.json
│   ├── 03_one_in_another.json
│   └── 04_common_parts.json
│
├── 📁 graph/              # 🎨 Fichiers PlantUML générés (sortie)
│   └── *.puml
│
├── 📁 src/
│   ├── lib.rs             # Point d'entrée de la bibliothèque
│   ├── main.rs            # Programme exécutable
│   ├── contact.rs         # Modèle Contact
│   ├── parser.rs          # Désérialisation JSON
│   ├── trie.rs            # 🌳 Le trie (from scratch)
│   ├── plantuml.rs        # Sérialisation PlantUML
│   ├── error.rs           # Type d'erreur unifié
│   └── runner.rs          # Pipeline complet
│
└── 📁 tests/
    └── pipeline.rs        # Tests d'intégration end-to-end
```

---

## 🏗️ Architecture

Le projet est organisé en **modules clairs**, chacun avec une
responsabilité unique. Cette séparation rend le code testable
indépendamment et facile à faire évoluer.

| Module       | Rôle                                                                     |
|--------------|--------------------------------------------------------------------------|
| `contact`    | Modèle de données : struct `Contact { nb, name }` + dérive Deserialize   |
| `parser`     | Charge un fichier JSON en `Vec<Contact>` (avec gestion trailing commas)  |
| `trie`       | 🌳 **Cœur du projet** : `TrieNode` et `Trie` avec insertion              |
| `plantuml`   | Sérialisation MindMap via `impl Display for Trie`                        |
| `error`      | Type d'erreur unifié `Error` avec préservation des sources               |
| `runner`     | Orchestration : parcourt `data/`, écrit dans `graph/`                    |

### API publique du Trie

```rust
let mut trie = Trie::new();
trie.insert("0612345678", "Alice");
trie.insert_contact(&Contact::new("0699999999", "Bob"));

assert_eq!(trie.len(), 2);
assert!(trie.contains("0612345678"));
assert!(!trie.contains("0000000000"));
```

---

## 🔬 Choix techniques

### `#![forbid(unsafe_code)]`

Présent en tête de `lib.rs` — **aucune ligne de `unsafe`** n'est tolérée
dans tout le projet, conformément aux contraintes du sujet. C'est plus
strict qu'un simple `#![deny(unsafe_code)]` car la directive ne peut pas
être désactivée localement.

### `BTreeMap<char, TrieNode>` plutôt que `HashMap`

Les enfants de chaque nœud sont stockés dans un `BTreeMap` ordonné par
caractère. Avantages :

- ✅ La sortie PlantUML est **déterministe** : pas besoin de trier
  manuellement à chaque parcours.
- ✅ Les tests sont plus faciles à écrire (ordre prédictible).
- ✅ La performance reste excellente avec un alphabet de seulement 10
  chiffres.

### Trie *from scratch*

Le sujet impose d'implémenter le trie sans utiliser de crate dédié.
L'insertion est faite **itérativement** (pas récursivement) avec
`BTreeMap::entry().or_default()`.

### `impl Display for Trie`

Plutôt qu'une fonction libre `generate_plantuml(&trie) -> String`, on
implémente le trait `Display`. Cela permet :

```rust
println!("{trie}");                    // ✓
let s = format!("{trie}");             // ✓
file.write_all(format!("{trie}").as_bytes())?;  // ✓
```

### Trailing commas dans le JSON

Les fichiers `data/*.json` fournis contiennent des **virgules en trop**
(ex. `[1, 2, 3,]`) qui ne sont pas conformes à la spec JSON. Le sujet
interdit de modifier ces fichiers. La solution : un **prétraitement
maison** en pure Rust qui retire ces virgules avant de passer le texte
à `serde_json`. Le prétraitement gère correctement les virgules **à
l'intérieur des strings JSON**.

### Type d'erreur avec `source()`

Le type `Error` de la bibliothèque conserve les erreurs originales
(`std::io::Error`, `serde_json::Error`) au lieu de les convertir en
`String`. Cela permet à un appelant de remonter la chaîne complète
des causes via `e.source()`.

---

## 🧪 Tests

Le projet contient **deux niveaux de tests** :

- **Tests unitaires** dans chaque module (via `#[cfg(test)] mod tests`).
- **Tests d'intégration** dans `tests/pipeline.rs` qui exercent le
  pipeline complet (JSON → trie → PlantUML).

### Lancer les tests

```bash
# Tous les tests (unitaires + intégration)
cargo test

# Uniquement les tests unitaires
cargo test --lib

# Uniquement les tests d'intégration
cargo test --test pipeline

# Tests d'un module précis
cargo test --lib trie::

# Tests d'une fonction précise
cargo test new_trie_is_empty
```

### Couverture

| Module     | Cas testés                                                                                                |
|------------|-----------------------------------------------------------------------------------------------------------|
| `contact`  | Construction par `new()`                                                                                  |
| `parser`   | Trailing commas, fichier vide, espaces seuls, chargement des 4 fichiers, fichier inexistant               |
| `trie`     | Trie vide, insertion, préfixes partagés, collision, racines multiples, `len()`, `contains()`              |
| `plantuml` | Markers de début/fin, chaîne simple, sortie sur les 4 fichiers réels                                      |
| `runner`   | Listage des JSON, ordre alphabétique, dossier manquant, conversion de chemins                             |
| `pipeline` | End-to-end sur les 4 fichiers, création de répertoire, gestion d'erreurs                                  |

---

## ✅ Qualité du code

### Formatage

```bash
cargo fmt --check       # vérifie sans rien modifier
cargo fmt               # applique le formatage
```

### Linter (Clippy)

```bash
cargo clippy -- -D warnings
```

> ⚠️ La commande échoue si **un seul** warning est présent. C'est
> exactement ce que la consigne du sujet exige.

### Workflow recommandé avant chaque commit

```bash
cargo build && cargo test && cargo clippy -- -D warnings && cargo fmt --check
```

---

## 🎨 Visualiser les graphes PlantUML

Le programme produit des fichiers `.puml` dans `graph/`. Pour les
visualiser, le plus simple est d'utiliser un serveur PlantUML local
via Docker :

```bash
# Télécharger l'image PlantUML
docker pull plantuml/plantuml-server:jetty

# Lancer le serveur (port 8080)
docker run -d -p 8080:8080 plantuml/plantuml-server:jetty
```

Ouvrir <http://localhost:8080/> dans un navigateur, puis copier-coller
le contenu d'un fichier `.puml` dans la zone de texte.

📚 La syntaxe MindMap est documentée
[ici](https://plantuml.com/fr/mindmap-diagram).

---

## 📋 Exemple complet

### Entrée (`data/04_common_parts.json`)

```json
[
    { "nb": "0412578440", "name": "Alice" },
    { "nb": "0412199803", "name": "Bob" },
    { "nb": "0468892011", "name": "patate" },
    { "nb": "112", "name": "Urgences" },
    { "nb": "15", "name": "SAMU" }
]
```

### Sortie (`graph/04_common_parts.puml`)

```
@startmindmap
* 0
** 4
*** 1
**** 2
***** 5
****** 7
******* 8
******** 4
********* 4
********** 0
*********** Alice
***** 1
****** 9
******* 9
******** 8
********* 0
********** 3
*********** Bob
*** 6
**** 8
***** 8
****** 9
******* 2
******** 0
********* 1
********** 1
*********** patate
* 1
** 1
*** 2
**** Urgences
** 5
*** SAMU
@endmindmap
```

Le rendu MindMap montre clairement comment les numéros qui partagent
le préfixe `04` partagent les mêmes nœuds dans l'arbre, ce qui
illustre l'efficacité du trie pour le stockage.

---

## 👥 Auteurs

Projet réalisé en groupe (3 personnes) dans le cadre du module
**Rust** en B3 SRC à l'**ESGI**.

| Nom         | Rôle / Responsabilités                                                |
|-------------|-----------------------------------------------------------------------|
| **Nom 1**   | Setup projet, module PlantUML, intégration finale, doc                |
| **Nom 2**   | Modèle de données, parser JSON, trie de base                          |
| **Nom 3**   | Type d'erreur, runner, tests d'intégration, qualité                   |

---

## 📜 Licence

Distribué sous licence [MIT](LICENSE).
