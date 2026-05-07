# 📞 phone-trie

> Gestionnaire de numéros de téléphone basé sur un **trie** (prefix tree),
> avec export au format **PlantUML MindMap** pour la visualisation.

[![Rust](https://img.shields.io/badge/Rust-1.95-orange?logo=rust)](https://www.rust-lang.org/)
[![License](https://img.shields.io/badge/license-MIT-blue.svg)](LICENSE)

Projet réalisé dans le cadre du module **Rust** — ESGI B3 SRC.

---

## 📑 Sommaire

- [✨ Présentation](#-présentation)
- [🚀 Démarrage rapide](#-démarrage-rapide)
- [🔧 Environnement de développement](#-environnement-de-développement)
- [⚡ Commandes & sorties attendues](#-commandes--sorties-attendues)
- [📂 Structure du projet](#-structure-du-projet)
- [🏗️ Architecture](#️-architecture)
- [🔬 Choix techniques](#-choix-techniques)
- [🧪 Tests](#-tests)
- [✅ Qualité du code](#-qualité-du-code)
- [🎨 Visualiser les graphes PlantUML](#-visualiser-les-graphes-plantuml)
- [📋 Exemple complet](#-exemple-complet)
- [🤖 Outils utilisés](#-outils-utilisés)
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

- [Rust](https://rustup.rs/) — édition 2021, version stable récente
- `cargo` (fourni avec rustup)

### Lancement

```bash
# Cloner le repo
git clone https://github.com/Croco10il/Rust_Projet.git
cd Rust_Projet

# Compiler et lancer en mode optimisé
cargo run --release
```

---

## 🔧 Environnement de développement

Ce projet a été développé et testé avec les versions suivantes.
Pour reproduire le build à l'identique, il est recommandé d'utiliser
des versions compatibles.

### Rust toolchain

| Composant | Version utilisée |
|---|---|
| `rustc` | **1.95.0** (59807616e — 2026-04-14) |
| `cargo` | **1.95.0** (f2d3ce0bd — 2026-03-21) |
| Édition | **2021** |
| Toolchain par défaut | `stable-x86_64-pc-windows-msvc` |
| Target installée | `x86_64-pc-windows-msvc` |

### Système de test

| Composant | Valeur |
|---|---|
| OS | **Windows 11** — version 10.0.26200.8246 |
| Architecture | x86_64 |

### Dépendances (`Cargo.toml`)

```toml
[dependencies]
serde = { version = "1.0", features = ["derive"] }
serde_json = "1.0"
```

Versions résolues (depuis `Cargo.lock`) :

| Crate | Version |
|---|---|
| `serde` | 1.0.228 |
| `serde_core` | 1.0.228 |
| `serde_derive` | 1.0.228 |
| `serde_json` | 1.0.149 |
| `proc-macro2` | 1.0.106 |
| `quote` | 1.0.45 |
| `syn` | 2.0.117 |
| `unicode-ident` | 1.0.24 |
| `itoa` | 1.0.18 |
| `memchr` | 2.8.0 |
| `zmij` | 1.0.21 |

> ℹ️ Le fichier `Cargo.lock` du projet est en **format version 4**.
> Si vous utilisez une version de cargo antérieure à **1.78**, il sera
> peut-être nécessaire de supprimer `Cargo.lock` avant le build pour
> qu'il soit régénéré au format compatible avec votre version.

### Compatibilité

Le projet doit fonctionner avec n'importe quelle version stable de Rust
en édition 2021 (≥ 1.78 recommandé). Si vous utilisez une version
plus récente que 1.95, il n'y aura pas d'incompatibilité.

⚠️ **Note Windows** : si vous rencontrez une erreur `link.exe not found`
au build, installez les
[Visual Studio Build Tools](https://visualstudio.microsoft.com/fr/visual-cpp-build-tools/)
avec le workload *« Développement Desktop en C++ »*. Le linker
Microsoft est nécessaire pour la toolchain `msvc` utilisée par défaut
sur Windows.

---

## ⚡ Commandes & sorties attendues

Voici toutes les commandes utiles pour vérifier le bon fonctionnement
du projet, avec ce que vous devez voir s'afficher.

### 1. Compilation

```bash
cargo build
```

**Sortie attendue :**

```
   Compiling phone-trie v0.1.0 (...)
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.84s
```

### 2. Exécution du programme

```bash
cargo run --release
```

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

Les 4 fichiers `.puml` sont alors disponibles dans le dossier `graph/`.

### 3. Tests (unitaires + intégration)

```bash
cargo test
```

**Sortie attendue :**

```
running 42 tests
test contact::tests::new_creates_contact_with_given_values ... ok
test parser::tests::does_not_touch_commas_inside_strings ... ok
test parser::tests::handles_whitespace_before_bracket ... ok
test parser::tests::load_common_parts ... ok
test parser::tests::load_different_roots ... ok
test parser::tests::empty_json_array_returns_empty_vec ... ok
test parser::tests::empty_file_returns_empty_vec ... ok
test parser::tests::load_missing_file_returns_io_error ... ok
test parser::tests::load_one_in_another ... ok
test parser::tests::load_simple_file ... ok
test parser::tests::no_op_on_valid_json ... ok
test parser::tests::strip_trailing_comma_in_array ... ok
test parser::tests::strip_trailing_comma_in_object ... ok
test plantuml::tests::empty_trie_outputs_only_markers ... ok
test plantuml::tests::output_for_common_parts_contains_all_five_names ... ok
test plantuml::tests::output_for_different_roots_has_two_top_branches ... ok
test plantuml::tests::output_for_one_in_another_keeps_both_names ... ok
test plantuml::tests::output_for_simple_file ... ok
test parser::tests::whitespace_only_file_returns_empty_vec ... ok
test plantuml::tests::output_starts_and_ends_with_required_markers ... ok
test plantuml::tests::single_number_outputs_chain_with_terminal ... ok
test runner::tests::build_trie_from_simple_file ... ok
test runner::tests::list_json_files_finds_all_data_files ... ok
test runner::tests::list_json_files_rejects_missing_directory ... ok
test runner::tests::output_path_for_replaces_extension ... ok
test runner::tests::list_json_files_returns_sorted_paths ... ok
test trie::tests::contains_returns_false_for_missing_number ... ok
test trie::tests::contains_returns_false_for_prefix_only ... ok
test trie::tests::contains_returns_true_for_inserted_number ... ok
test trie::tests::contains_works_when_prefix_is_also_a_full_number ... ok
test trie::tests::insert_creates_path_of_nodes ... ok
test trie::tests::insert_via_contact_works_like_insert ... ok
test trie::tests::inserting_same_number_twice_overwrites_name ... ok
test trie::tests::len_counts_unique_numbers ... ok
test trie::tests::len_does_not_double_count_overwrites ... ok
test trie::tests::len_handles_prefix_collision ... ok
test trie::tests::len_is_zero_for_empty_trie ... ok
test trie::tests::multiple_disjoint_roots ... ok
test trie::tests::new_trie_is_empty ... ok
test trie::tests::order_of_insertion_does_not_matter ... ok
test trie::tests::prefix_collision_one_number_is_prefix_of_another ... ok
test trie::tests::shared_prefix_creates_branching ... ok

test result: ok. 42 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out

     Running tests/pipeline.rs (...)

running 5 tests
test pipeline_fails_on_missing_data_directory ... ok
test pipeline_creates_output_directory_if_missing ... ok
test common_parts_output_contains_all_names ... ok
test pipeline_produces_one_puml_per_json ... ok
test produced_files_have_mindmap_markers ... ok

test result: ok. 5 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out
```

→ **Total : 47 tests passent**, 0 échec.

### 4. Linter Clippy (qualité du code)

```bash
cargo clippy -- -D warnings
```

**Sortie attendue :**

```
    Checking phone-trie v0.1.0 (...)
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.34s
```

→ **0 warning**. Le `-D warnings` transforme tout warning en erreur,
donc si la commande passe, c'est qu'il n'y a aucun problème de style.

### 5. Formatage

```bash
cargo fmt --check
```

**Sortie attendue :** *(rien — le silence est bon signe)*

Si le code n'est pas formaté correctement, la commande affiche les
diffs à appliquer. Dans ce cas, lancer simplement :

```bash
cargo fmt
```

### 6. Workflow recommandé avant chaque commit

```bash
cargo build && cargo test && cargo clippy -- -D warnings && cargo fmt --check
```

Si toutes ces vérifications passent, le code est prêt à être commit.

---

## 📂 Structure du projet

```
phone-trie/
├── 📄 Cargo.toml          # Manifest Rust
├── 📄 Cargo.lock          # Versions exactes des dépendances
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

## 🤖 Outils utilisés

Dans une démarche de transparence, nous précisons que pour ce projet
nous nous sommes aidés de **Claude Opus 4.7** (Anthropic) à différentes
étapes :

- **Conception de l'architecture** : découpage en modules, choix des
  structures de données (`BTreeMap` vs `HashMap`), pattern de
  sérialisation via le trait `Display`.
- **Aide à la rédaction** : commentaires de doc, structure du README,
  messages de commits suivant la convention *Conventional Commits*.
- **Débogage** : identification d'un bug de typage générique sur la
  fonction `run_all`, résolution de soucis d'environnement Windows
  (Defender, MinGW, MSVC).
- **Aide aux tests** : suggestion de cas limites à couvrir (collision
  de préfixes, fichiers vides, etc.).

L'IA a été utilisée comme **assistant** : chaque ligne de code a été
relue, comprise et validée par les membres du groupe. Les choix
d'implémentation finaux et la responsabilité du code restent les
nôtres.

---

## 👥 Auteurs

Projet réalisé en groupe (3 personnes) dans le cadre du module
**Rust** en B3 SRC à l'**ESGI**.

| Nom         |
|-------------|
| **Gael**    |
| **Eyup**    |
| **Oskar**   |
| **Claude**  |

---

## 📜 Licence

Distribué sous licence [MIT](LICENSE).
