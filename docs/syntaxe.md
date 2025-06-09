# Guide de syntaxe EDL – Pour bien débuter

Bienvenue !  
Ce guide t’accompagne pas à pas pour apprendre à écrire tes premiers programmes en EDL, un langage moderne inspiré de Python, TypeScript et Rust.

---

## 1. Déclarer des variables

En EDL, tu déclares une variable avec `let` :

```edl
let age = 25;
let nom = "Alice";
let actif = true;
let pi = 3.14;
```
- Les types sont déduits automatiquement.
- Les chaînes de caractères sont entre guillemets `"..."`.
- Les booléens sont `true` ou `false`.

---

## 2. Écrire et utiliser des fonctions

Une fonction se déclare avec `fn` :

```edl
fn carre(n) {
    return n * n;
}

let resultat = carre(5); // resultat vaut 25
print(resultat);
```
- `return` permet de renvoyer une valeur.
- Les paramètres n’ont pas besoin de type explicite (pour l’instant).

---

## 3. Contrôler le flux du programme

### Conditionnelles (`if` / `else`)

```edl
if age >= 18 {
    print("Majeur");
} else {
    print("Mineur");
}
```

### Boucles

#### Boucle `while`

```edl
let compteur = 3;
while compteur > 0 {
    print(compteur);
    compteur = compteur - 1;
}
```

#### Boucle `for`

```edl
for i in 0..5 {
    print(i); // Affiche 0, 1, 2, 3, 4
}
```

---

## 4. Listes (arrays)

EDL gère les listes :

```edl
let nombres = [1, 2, 3];
print(nombres); // Affiche [1, 2, 3]

let vide = [];
print(vide); // Affiche []
```

---

## 5. Impression et import

- Pour afficher une valeur :
  ```edl
  print("Bonjour, EDL !");
  ```
- Pour importer un autre fichier EDL :
  ```edl
  import "mon_module.edl";
  ```

---

## 6. Définir des types personnalisés (structs)

```edl
type Point {
    x: 0,
    y: 0
}

let p = Point { x: 10, y: 20 };
```
*(Cette fonctionnalité peut évoluer)*

---

## 7. Fonctions anonymes (lambdas)

```edl
let double = fn(x) { return x * 2; };
print(double(4)); // Affiche 8
```

---

## 8. Blocs d’instructions

Tu peux regrouper des instructions dans un bloc :

```edl
{
    let temp = 42;
    print(temp);
}
```

---

## 9. Mots-clés réservés

- `let`, `fn`, `return`, `if`, `else`, `while`, `for`, `in`, `import`, `type`, `const`, `match`, `as`, `pub`, `mod`, `struct`, `enum`, `break`, `continue`, `yield`, `print`

---

## 10. Commentaires

Pour l’instant, les commentaires ne sont pas encore supportés, mais cette fonctionnalité est prévue.

---

## 11. Exécution

- Pour exécuter un fichier :  
  ```sh
  edl run mon_script.edl
  ```
- Pour lancer le REPL interactif :  
  ```sh
  edl repl
  ```

---

**Astuce** :  
Teste chaque exemple dans un fichier `.edl` ou dans le REPL pour voir le résultat.

---

Ce guide évoluera avec le langage.  
N’hésite pas à proposer des ajouts ou à poser tes questions !