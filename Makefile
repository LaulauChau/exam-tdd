.PHONY: build test run demo

help:
	@echo "Poker Hand Evaluator - Commandes disponibles"
	@echo "make build       - Compile le programme"
	@echo "make build-release - Compile le programme en mode optimisé"
	@echo "make run         - Exécute le programme avec les mains par défaut"
	@echo "make run HAND1=\"...\" HAND2=\"...\" - Exécute avec des mains spécifiques"
	@echo "make test        - Exécute tous les tests"
	@echo "make test-unit   - Exécute uniquement les tests unitaires"
	@echo "make test-integration - Exécute uniquement les tests d'intégration"
	@echo "make test-cli    - Exécute uniquement les tests CLI"
	@echo "make demo        - Exécute le script de démonstration"

# Valeurs par défaut pour les arguments
HAND1 ?= "AS KS QS JS 10S"
HAND2 ?= "AH AD AC AS KH"

build:
	@echo "Compilation du programme..."
	@cargo build
	@echo "Compilation terminée."

build-release:
	@echo "Compilation du programme en mode optimisé..."
	@cargo build --release
	@echo "Compilation terminée."

run: build
	@echo "Exécution du programme..."
	@cargo run -- $(HAND1) $(HAND2)

test:
	@echo "Exécution de tous les tests..."
	@cargo test
	@echo "Tests terminés."

test-unit:
	@echo "Exécution des tests unitaires..."
	@cargo test --lib
	@echo "Tests unitaires terminés."

test-integration:
	@echo "Exécution des tests d'intégration..."
	@cargo test --test integration_test
	@echo "Tests d'intégration terminés."

test-cli:
	@echo "Exécution des tests CLI..."
	@cargo test --test cli_test
	@echo "Tests CLI terminés."

demo:
	@echo "Exécution de la démo..."
	@./demo.sh