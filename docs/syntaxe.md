# Guide de syntaxe EDL

Bienvenue dans EDL, un langage inspiré de Python, TypeScript et Rust.  
Ce guide présente la syntaxe de base pour bien débuter.

---

## 1. Variables et types de base

```edl
let x = 42;
let y = 3.14;
let nom = "EDL";
let actif = true;
```

## 2. Fonctions

```edl
fn carre(n) {
    return n * n;
}

let resultat = carre(5);
```

## 3. Structures de contrôle

### Conditionnelles

```edl
if x > 10 {
    print("x est grand");
} else {
    print("x est petit");
}
```

### Boucles

```edl
while x > 0 {
    print(x);
    x = x - 1;
}

for i in 0..5 {
    print(i);
}
```

## 4. Impression et import

```edl
print("Hello, EDL!");

import "mon_module.edl";
```

## 5. Déclaration de types (structs/classes)

```edl
type Point {
    x: 0,
    y: 0
}

let p = Point { x: 10, y: 20 };
```

## 6. Fonctions anonymes et appels

```edl
let double = fn(x) { return x * 2; };
print(double(4));
```

## 7. Bloc d’instructions

```edl
{
    let temp = 5;
    print(temp);
}
```

## 8. Mots-clés réservés

- `let`, `fn`, `return`, `if`, `else`, `while`, `for`, `in`, `import`, `type`, `const`, `match`, `as`, `pub`, `mod`, `struct`, `enum`, `break`, `continue`, `yield`, `print`

---

## 9. Commentaires

Les commentaires ne sont pas encore supportés, mais cette fonctionnalité est prévue.

---

## 10. Exécution

- Pour exécuter un fichier :  
  `edl run mon_script.edl`
- Pour lancer le REPL :  
  `edl repl`

---

Ce guide évoluera avec le langage.  
N’hésitez pas à proposer des ajouts ou des exemples !