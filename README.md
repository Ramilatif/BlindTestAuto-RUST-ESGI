# BlindTestAuto (V1)

BlindTestAuto est un outil en **Rust** permettant de générer automatiquement une vidéo de **blind test musical** à partir d’un fichier **JSON** descriptif.

La V1 se concentre sur une pipeline simple, robuste et testée, basée sur **FFmpeg**.

---

## Fonctionnalités (V1)

- Lecture d’un fichier JSON décrivant le blind test
- Découpage automatique des clips vidéo à partir de timecodes
- Deux phases par clip :
  - **Phase devinette** : écran noir + musique + minuteur
  - **Phase révélation** : affichage de la vidéo + réponse à l’écran
- Concaténation automatique de plusieurs clips
- Génération d’une **seule commande FFmpeg** (`filter_complex`)
- Mode `--dry-run` pour afficher la commande sans exécuter FFmpeg
- Validation stricte des données (JSON + règles métier)
- Tests unitaires (parsing, validation, génération de commande)

---

## Prérequis

- **Rust** (stable)
- **FFmpeg** accessible dans le `PATH`

---

## Utilisation

### 1. Exemple de fichier JSON

```json
{
  "output": {
    "path": "render/blindtest.mp4",
    "resolution": "1280x720",
    "fps": 30
  },
  "timings": {
    "guess_duration": "00:00:10.000",
    "reveal_duration": "00:00:05.000"
  },
  "clips": [
    {
      "video": "videos/clip1.mp4",
      "start": "00:01:00.000",
      "answer": "Daft Punk - One More Time"
    },
    {
      "video": "videos/clip2.mp4",
      "start": "00:00:30.500",
      "answer": "Nirvana - Smells Like Teen Spirit"
    }
  ]
}
```

---

### 2. Générer la commande FFmpeg (dry-run)

```bash
cargo run -- render montage.json --dry-run
```

---

### 3. Générer la vidéo finale

```bash
cargo run -- render montage.json
```

---

## Fonctionnement interne (V1)

Pour chaque clip :

1. Découpe de la vidéo source à partir du `start`
2. Séparation audio en deux segments :
   - devinette
   - révélation
3. Génération d’un écran noir pour la phase devinette
4. Affichage du minuteur
5. Affichage de la réponse pendant la phase révélation
6. Concaténation des segments
7. Concaténation finale de tous les clips

Tout le montage est réalisé via un **unique appel FFmpeg**.

---

## Limitations connues (V1)

- Les fichiers vidéo doivent contenir une piste audio
- Pas de vérification de l’existence des fichiers avant l’appel à FFmpeg

---

## Tests

```bash
cargo test
```

---

## Statut du projet

- ✅ Version : **V1 stable**
- 🎯 Objectif atteint : génération automatique de blind tests vidéo
- 🔒 API et format JSON considérés comme stables pour la V1

---

## Licence

Projet pédagogique / expérimental.
