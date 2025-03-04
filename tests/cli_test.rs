use std::process::Command;
use std::str;

#[test]
fn test_cli_with_royal_flush_vs_four_of_a_kind() {
    let output = Command::new("cargo")
        .args(&["run", "--", "AS KS QS JS 10S", "AH AD AC AS KH"])
        .output()
        .expect("Failed to execute command");
    
    let stdout = str::from_utf8(&output.stdout).unwrap();
    
    // Afficher la sortie pour le débogage
    println!("Sortie: {}", stdout);
    
    assert!(stdout.contains("Poker Hand Evaluator"));
    assert!(stdout.contains("Hand 1 wins!"));
}

#[test]
fn test_cli_with_pair_vs_pair() {
    let output = Command::new("cargo")
        .args(&["run", "--", "AH AD 2C 3S 4H", "KH KD 2S 3C 4D"])
        .output()
        .expect("Failed to execute command");
    
    let stdout = str::from_utf8(&output.stdout).unwrap();
    
    // Afficher la sortie pour le débogage
    println!("Sortie: {}", stdout);
    
    assert!(stdout.contains("Poker Hand Evaluator"));
    assert!(stdout.contains("Hand 1 wins!"));
}

#[test]
fn test_cli_with_tie() {
    let output = Command::new("cargo")
        .args(&["run", "--", "AH KH QH JH 10H", "AD KD QD JD 10D"])
        .output()
        .expect("Failed to execute command");
    
    let stdout = str::from_utf8(&output.stdout).unwrap();
    
    // Afficher la sortie pour le débogage
    println!("Sortie: {}", stdout);
    
    assert!(stdout.contains("Poker Hand Evaluator"));
    assert!(stdout.contains("It's a tie!"));
}

#[test]
fn test_cli_with_invalid_hand() {
    let output = Command::new("cargo")
        .args(&["run", "--", "AH AD", "KH KD 2S 3C 4D"])
        .output()
        .expect("Failed to execute command");
    
    let stdout = str::from_utf8(&output.stdout).unwrap();
    
    // Afficher la sortie pour le débogage
    println!("Sortie: {}", stdout);
    
    assert!(stdout.contains("Poker Hand Evaluator"));
    assert!(stdout.contains("Error parsing hand 1"));
}

#[test]
fn test_cli_with_no_arguments() {
    let output = Command::new("cargo")
        .args(&["run"])
        .output()
        .expect("Failed to execute command");
    
    let stdout = str::from_utf8(&output.stdout).unwrap();
    
    // Afficher la sortie pour le débogage
    println!("Sortie: {}", stdout);
    
    assert!(stdout.contains("Poker Hand Evaluator"));
    assert!(stdout.contains("No hands provided as arguments"));
} 