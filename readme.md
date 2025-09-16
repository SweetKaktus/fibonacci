# Fibonacci en Rust

![Work in Progress](https://img.shields.io/badge/Status-Work%20in%20Progress-yellow)
![Rust](https://img.shields.io/badge/Language-Rust-orange)
![License: MIT](https://img.shields.io/badge/License-MIT-blue)

Ce projet est un petit programme écrit en **Rust** qui calcule le `n`-ième nombre de la suite de Fibonacci.  
Il a pour but de montrer comment :

- manipuler les arguments passés en ligne de commande,
- parser une entrée utilisateur,
- implémenter une version itérative de l'algorithme de Fibonacci.

---

## 🚀 Utilisation

### 1. Compiler le programme
```bash
cargo build --release
```

### 2. Lancer le programme

Le programme attend un argument numérique n, représentant la position dans la suite de Fibonacci.

Exemple :
```bash
cargo run -- 10
```
Résultat attendu :
```bash
Le 10ème nombre de la suite de Fibonacci est le 55
```
## 🧮 Exemple de la suite de Fibonacci

n : 0  1  2  3  4  5  6  7  8  9  10 ...
F : 0  1  1  2  3  5  8 13 21 34 55 ...

## 📂 Structure du projet

    src/main.rs → contient la logique de calcul et la gestion des arguments.

## 📜 Licence

Projet libre sous licence MIT.