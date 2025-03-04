# Poker Hand Evaluator

## Prérequis

- [Rust](https://www.rust-lang.org/tools/install) (version 1.70.0 ou supérieure)
- [Cargo](https://doc.rust-lang.org/cargo/getting-started/installation.html) (généralement installé avec Rust)
- [Make](https://www.gnu.org/software/make/) (optionnel, pour utiliser le Makefile)

## Installation

Clonez le dépôt et naviguez dans le répertoire du projet :

```bash
git clone https://github.com/LaulauChau/exam-tdd.git
cd exam-tdd
```

## Compilation

### Avec Cargo

Pour compiler le programme, exécutez :

```bash
cargo build
```

Pour une version optimisée :

```bash
cargo build --release
```

### Avec Make

Pour compiler le programme, exécutez :

```bash
make build
```

Pour une version optimisée :

```bash
make build-release
```

## Utilisation

Le programme permet de comparer deux mains de poker et de déterminer laquelle est la plus forte.

### Exécution avec Cargo

Pour exécuter le programme, utilisez la commande suivante :

```bash
cargo run -- "<main1>" "<main2>"
```

Où `<main1>` et `<main2>` sont les mains de poker à comparer.

### Exécution avec Make

Pour exécuter le programme avec les mains par défaut (quinte flush royale vs carré) :

```bash
make run
```

Pour exécuter le programme avec des mains personnalisées :

```bash
make run HAND1='"AH AD 2C 3S 4H"' HAND2='"KH KD 2S 3C 4D"'
```

Notez l'utilisation des guillemets simples et doubles pour les arguments.

### Format des mains

Chaque main doit contenir exactement 5 cartes, séparées par des espaces. Chaque carte est représentée par son rang suivi de sa couleur :

- Rangs : 2, 3, 4, 5, 6, 7, 8, 9, 10, J, Q, K, A
- Couleurs : H (Hearts/Cœurs), D (Diamonds/Carreaux), C (Clubs/Trèfles), S (Spades/Piques)

Exemple : "AS KS QS JS 10S" représente une quinte flush royale à pique.

### Exemples

```bash
# Comparer une quinte flush royale à un carré
cargo run -- "AS KS QS JS 10S" "AH AD AC AS KH"

# Comparer une paire d'as à une paire de rois
cargo run -- "AH AD 2C 3S 4H" "KH KD 2S 3C 4D"
```

### Script de démonstration

Un script de démonstration est inclus pour montrer différents exemples de comparaisons de mains :

```bash
# Rendre le script exécutable
chmod +x demo.sh

# Exécuter la démo
./demo.sh
```

Ou avec Make :

```bash
make demo
```

Ce script exécute automatiquement plusieurs comparaisons de mains différentes, illustrant les différentes combinaisons et règles de départage.

### Sortie

Le programme affichera les deux mains, leur type (paire, brelan, etc.) et indiquera quelle main gagne ou s'il y a égalité.

## Tests

### Tests unitaires

Pour exécuter les tests unitaires :

```bash
cargo test
```

Ou avec Make :

```bash
make test-unit
```

### Tests d'intégration

Le projet comprend deux types de tests d'intégration :

1. **Tests de la bibliothèque** : Vérifient que les fonctions de parsing et d'évaluation fonctionnent correctement ensemble.

```bash
cargo test --test integration_test
```

Ou avec Make :

```bash
make test-integration
```

2. **Tests de l'interface en ligne de commande** : Vérifient que le programme principal fonctionne correctement avec différentes entrées.

```bash
cargo test --test cli_test
```

Ou avec Make :

```bash
make test-cli
```

Pour exécuter tous les tests (unitaires et d'intégration) :

```bash
cargo test
```

Ou avec Make :

```bash
make test
```

## Structure du projet

- `src/poker/card.rs` : Définitions des cartes, rangs, couleurs et types de mains
- `src/poker/hand.rs` : Logique d'évaluation et de comparaison des mains
- `src/poker/parser.rs` : Fonctions pour analyser les chaînes de caractères en cartes et mains
- `src/main.rs` : Point d'entrée du programme
- `demo.sh` : Script de démonstration
- `tests/` : Tests d'intégration
- `Makefile` : Automatisation des tâches courantes