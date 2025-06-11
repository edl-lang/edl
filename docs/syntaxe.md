# Guide de syntaxe EDL – Premiers pas

Bienvenue !  
Ce guide va t’apprendre à programmer en EDL, étape par étape, comme si tu découvrais JavaScript ou Python. Prends un éditeur, lance le REPL ou crée un fichier `.edl` et suis les exemples !

---

## 1. Ton premier programme : Hello World

En EDL, pour afficher du texte, utilise la fonction `print` :

```edl
print("Hello, World!");
```

---

## 2. Déclarer des variables

On utilise `let` pour créer une variable :

```edl
let nom = "Alice";
let age = 25;
let actif = true;
let pi = 3.14;
```
- Les types sont déduits automatiquement.
- Les chaînes sont entre guillemets `"..."`.
- Les booléens sont `true` ou `false`.

---

## 3. Les commentaires

Pour expliquer ton code ou le rendre plus lisible :

```edl
// Ceci est un commentaire sur une ligne
# Ceci aussi (style Python)
/* Ceci est
   un commentaire multi-ligne */
```

---

## 4. Fonctions : définir et utiliser

Déclare une fonction avec `fn` :

```edl
fn carre(n) {
    return n * n;
}

let resultat = carre(5);
print(resultat); // Affiche 25
```

---

## 5. Les conditions : if / else if / else

Pour exécuter du code selon une condition :

```edl
print("What is your age?:");
let age = input();
let age = to_number(age);

if age < 13 {
    print("You are a child.");
} else if age < 18 {
    print("You are a teenager.");
} else {
    print("You are an adult.");
}
```
- Tu peux chaîner autant de `else if` que tu veux.
- Les blocs sont toujours délimités par `{ ... }`.

---

## 6. Les boucles

### Boucle while

```edl
let compteur = 3;
while compteur > 0 {
    print(compteur);
    compteur = compteur - 1;
}
```

### Boucle for

```edl
for i in 0..5 {
    print(i); // Affiche 0, 1, 2, 3, 4
}
```

---

## 7. Les listes (arrays)

EDL gère les listes :

```edl
let nombres = [1, 2, 3];
print(nombres); // Affiche [1, 2, 3]

let vide = [];
print(vide); // Affiche []
```

Accès et modification :

```edl
print(nombres[0]); // 1
nombres[1] = 42;
print(nombres); // [1, 42, 3]
```

---

## 8. Fonctions anonymes (lambdas)

Tu peux créer une fonction sans nom et la stocker dans une variable :

```edl
let double = fn(x) { return x * 2; };
print(double(4)); // Affiche 8
```

---

## 9. Définir ses propres types (structs/classes)

EDL permet de créer des types personnalisés, similaires à des **classes** ou des **structs** dans d’autres langages, grâce au mot-clé `type` :

```edl
type Point {
    x: 0,
    y: 0,
    fn norm(self) {
        return (self.x * self.x + self.y * self.y) ** 0.5;
    }
}

let p = Point { x: 3, y: 4 };
print(p.x);      // 3
print(p.norm()); // 5
```

- Le mot-clé `type` permet de déclarer une **classe** avec des champs et des méthodes.
- Les méthodes sont définies à l’intérieur du bloc, avec `fn nom(self, ...)`.
- L’instanciation se fait avec `Type { champ: valeur, ... }`.

---

## 10. Importer du code

Pour réutiliser du code d’un autre fichier :

```edl
import "mon_module.edl";
```

---

## 11. Les mots-clés réservés

- `let`, `fn`, `return`, `if`, `else`, `while`, `for`, `in`, `import`, `type`, `const`, `match`, `break`, `continue`, `yield`, `print`, etc.

---

## 12. Exécuter ton code

- Pour exécuter un fichier :
  ```sh
  edl run mon_script.edl
  ```
- Pour lancer le REPL interactif :
  ```sh
  edl repl
  ```

---

## 13. Aller plus loin

- **Structs avancés, modules, packages, pattern matching, etc.** : consulte la documentation complète ou expérimente dans le REPL !
- **Astuces** : teste chaque exemple, modifie-le, observe le résultat.

---

Ce guide évoluera avec le langage.  
N’hésite pas à proposer des ajouts ou à poser tes questions !