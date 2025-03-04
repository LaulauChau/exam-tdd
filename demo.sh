#!/bin/bash

# Couleurs pour une meilleure lisibilité
GREEN='\033[0;32m'
BLUE='\033[0;34m'
YELLOW='\033[1;33m'
NC='\033[0m' # No Color

echo -e "${BLUE}=== Poker Hand Evaluator Demo ===${NC}\n"

# Fonction pour exécuter une comparaison et afficher un séparateur
run_comparison() {
    local hand1="$1"
    local hand2="$2"
    local description="$3"
    
    echo -e "${YELLOW}$description${NC}"
    echo -e "Main 1: ${GREEN}$hand1${NC}"
    echo -e "Main 2: ${GREEN}$hand2${NC}"
    echo -e "${BLUE}Résultat:${NC}"
    cargo run -- "$hand1" "$hand2"
    echo -e "\n${BLUE}----------------------------------------${NC}\n"
}

# Compilation du programme
echo -e "${BLUE}Compilation du programme...${NC}"
cargo build --quiet
echo -e "${GREEN}Compilation terminée.${NC}\n"

# Exemple 1: Quinte Flush Royale vs Carré
run_comparison "AS KS QS JS 10S" "AH AD AC AS KH" "Exemple 1: Quinte Flush Royale vs Carré"

# Exemple 2: Paire d'As vs Paire de Rois
run_comparison "AH AD 2C 3S 4H" "KH KD 2S 3C 4D" "Exemple 2: Paire d'As vs Paire de Rois"

# Exemple 3: Full aux As par les Rois vs Full aux Rois par les As
run_comparison "AH AD AC KH KD" "KS KC KD AS AD" "Exemple 3: Full aux As par les Rois vs Full aux Rois par les As"

# Exemple 4: Couleur vs Suite
run_comparison "2H 4H 6H 8H KH" "9C 10D JS QH KS" "Exemple 4: Couleur vs Suite"

# Exemple 5: Deux paires identiques, kicker différent
run_comparison "AH AD KH KD QH" "AS AC KS KC JS" "Exemple 5: Deux paires identiques, kicker différent"

# Exemple 6: Égalité parfaite
run_comparison "AH KH QH JH 10H" "AD KD QD JD 10D" "Exemple 6: Égalité parfaite (Quintes Flush Royales de couleurs différentes)"

# Exemple 7: Suite basse vs Suite haute
run_comparison "2C 3D 4H 5S 6C" "10D JS QH KS AC" "Exemple 7: Suite basse vs Suite haute"

# Exemple 8: Brelan vs Deux paires
run_comparison "AH AD AC 2H 3D" "KH KD QH QD 2S" "Exemple 8: Brelan vs Deux paires"

echo -e "${BLUE}=== Fin de la démo ===${NC}" 