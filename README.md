# HomemadeDrive
Drive fait maison, partagé via un simple lien (Rust / Dioxus 0.7 fullstack).

## Fonctionnalités
- Explorateur de dossiers avec fil d'Ariane (les fichiers sont stockés dans `uploads/`)
- Créer / renommer / supprimer des dossiers (la suppression est récursive)
- Envoyer des fichiers de tout type dans le dossier affiché (noms conservés, `photo (1).jpg` en cas de doublon)
- Ouvrir : images et vidéos dans la visionneuse (navigation ‹ ›), autres fichiers dans un nouvel onglet
- Télécharger, renommer et supprimer chaque fichier
- Conversion automatique HEIC/HEIF → JPEG si `heif-convert` est installé (l'original est conservé)

## Lancer
```sh
dx serve --platform web
```

## ⚠️ Sécurité
Toute personne ayant le lien peut supprimer et renommer. À protéger avant un usage public.
