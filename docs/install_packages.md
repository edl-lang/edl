# Installer un paquet EDL

Ce guide explique comment ajouter un module/paquet EDL à votre projet, soit automatiquement via le gestionnaire de paquets EDL, soit manuellement à partir d’un fichier source.

---

## 1. Installation via le gestionnaire de paquets EDL

La méthode recommandée pour installer un paquet est d’utiliser la commande CLI :

```sh
edl install nom_du_paquet
```

- Le gestionnaire télécharge automatiquement le module depuis le dépôt officiel.
- Le fichier du module est placé dans le dossier `edl_modules/` de votre projet.
- Le nom du paquet est ajouté à la section `"dependencies"` de votre fichier `package.edl.json`.

**Exemple :**

```sh
edl install math_utils
```

Après installation, votre `package.edl.json` contiendra :

```json
"dependencies": {
    "math_utils": "latest"
}
```

---

## 2. Installation manuelle depuis un fichier source

Vous pouvez aussi ajouter un module localement, par exemple si vous avez reçu un fichier `.edl` ou téléchargé un module depuis une autre source.

### Étapes :

1. **Copiez le fichier du module**  
   Placez le fichier source (ex : `foo.edl`) dans le dossier `edl_modules/` à la racine de votre projet.

2. **Ajoutez la dépendance dans `package.edl.json`**  
   Ouvrez le fichier `package.edl.json` et ajoutez le nom du module dans la section `"dependencies"` :

   ```json
   "dependencies": {
       "foo": "local"
   }
   ```

   Vous pouvez utiliser `"local"` ou une version personnalisée.

---

## 3. Utiliser le module dans votre code

Une fois le module installé (par l’une ou l’autre méthode), vous pouvez l’importer dans vos fichiers EDL :

```edl
import "foo";
```

---

## Résumé

- **Automatique** : `edl install nom_du_paquet` (recommandé)
- **Manuel** : copier le fichier dans `edl_modules/` et éditer `package.edl.json`
- **Import** : utilisez `import "nom_du_paquet";` dans votre code

N’hésitez pas à consulter la documentation du gestionnaire de paquets EDL pour plus d’options !