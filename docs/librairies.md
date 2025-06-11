# Développer une librairie externe EDL – Standards et bonnes pratiques

Ce guide présente les conventions et recommandations pour créer, structurer et publier des librairies externes pour EDL.  
Respecter ces standards facilite la réutilisation, la maintenance et l’intégration de vos modules dans d’autres projets.

---

## 1. Structure recommandée d’une librairie

```
ma_librairie/
├── foo.edl
├── bar.edl
├── README.md
├── package.edl.json
└── tests/
    └── test_foo.edl
```

- **Un fichier principal** (`foo.edl`) ou plusieurs modules.
- **Un fichier `package.edl.json`** décrivant la librairie (nom, version, description, dépendances, etc.).
- **Un dossier `tests/`** pour vos tests unitaires ou d’intégration.
- **Un fichier `README.md`** pour la documentation d’utilisation.

---

## 2. Exemple de `package.edl.json` pour une librairie

```json
{
    "name": "ma_librairie",
    "version": "1.0.0",
    "authors": ["Votre Nom"],
    "description": "Une librairie EDL pour ...",
    "scripts": {},
    "dependencies": {}
}
```

---

## 3. Bonnes pratiques de développement

- **Nommez vos fonctions et variables clairement** (en anglais ou en français, mais soyez cohérent).
- **Documentez chaque fonction** avec des commentaires ou des blocs de documentation.
- **Évitez les effets de bord** : privilégiez les fonctions pures quand c’est possible.
- **Exposez explicitement l’API** de votre librairie (fonctions, types, etc.).
- **Ajoutez des tests** dans le dossier `tests/` pour garantir la fiabilité de votre code.
- **Versionnez votre librairie** et mettez à jour le champ `version` à chaque modification majeure.

---

## 4. Publication et partage

- **Publiez votre librairie** sur le dépôt officiel EDL ou partagez le dossier compressé.
- **Incluez un `README.md`** avec :
  - Un résumé de la librairie
  - Des exemples d’utilisation
  - Les instructions d’installation

---

## 5. Exemple minimal de librairie

**math_utils.edl** :
```edl
fn add(a, b) {
    return a + b;
}

fn sub(a, b) {
    return a - b;
}
```

**package.edl.json** :
```json
{
    "name": "math_utils",
    "version": "1.0.0",
    "authors": ["Alice"],
    "description": "Fonctions mathématiques de base",
    "scripts": {},
    "dependencies": {}
}
```

---

## 6. Importer et utiliser une librairie

Dans un autre projet :

```edl
import "math_utils";
print(math_utils.add(2, 3)); // Affiche 5
```

---

## 7. Respect des conventions

- Utilisez le format JSON pour `package.edl.json`.
- Placez vos modules dans des fichiers `.edl` dans le dossier racine de la librairie.
- Respectez la casse et évitez les caractères spéciaux dans les noms de modules.

---

## 8. (Optionnel) Fournir un script `install.sh` pour l’installation manuelle

Pour faciliter l’installation manuelle de votre librairie, vous pouvez fournir un script `install.sh` à la racine du projet.  
Ce script peut automatiser la copie du module dans le dossier `edl_modules/` du projet utilisateur et mettre à jour le fichier `package.edl.json`.

**Exemple de `install.sh` :**

```sh
#!/bin/sh
# Script d'installation pour une librairie EDL

MODULE="math_utils"
TARGET_DIR="../edl_modules"

echo "📦 Installation du module $MODULE..."

# Crée le dossier cible si besoin
mkdir -p "$TARGET_DIR"

# Copie le fichier .edl
cp "$MODULE.edl" "$TARGET_DIR/"

# Ajoute la dépendance dans package.edl.json (si jq est installé)
if [ -f "../package.edl.json" ]; then
    if command -v jq >/dev/null 2>&1; then
        tmp=$(mktemp)
        jq ".dependencies[\"$MODULE\"] = \"local\"" ../package.edl.json > "$tmp" && mv "$tmp" ../package.edl.json
        echo "🔗 Dépendance ajoutée dans package.edl.json"
    else
        echo "⚠️  Installez 'jq' pour ajouter automatiquement la dépendance dans package.edl.json"
    fi
else
    echo "⚠️  package.edl.json non trouvé dans le dossier parent."
fi

echo "✅ Installation terminée."
```

> **Astuce** : Expliquez dans votre `README.md` comment utiliser ce script :
> ```sh
> cd ma_librairie
> sh install.sh
> ```

---

**En résumé** :  
- Fournir un `install.sh` est recommandé pour les utilisateurs qui installent manuellement des librairies.
- Ce script peut automatiser la copie et la déclaration de dépendance, rendant l’expérience plus simple et fiable.

---

**En suivant ces standards, vos librairies seront faciles à utiliser, à partager et à maintenir dans l’écosystème EDL !**