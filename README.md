# 🎵 BlindTestAuto (Rust)

BlindTestAuto est un outil en ligne de commande écrit en **Rust** permettant de **générer automatiquement une vidéo de blind test** à partir de clips vidéo.

Le montage est entièrement automatisé grâce à **FFmpeg** et un fichier de configuration **JSON**, généré manuellement ou via un assistant interactif.

---

## ✨ Fonctionnalités (V1 + V2)

### 🎬 Génération automatique de blind test
Pour chaque clip :
- **Phase Devinette**
  - écran noir
  - musique du clip
  - minuteur en secondes
- **Phase Révélation**
  - vidéo visible
  - réponse affichée à l’écran

Les clips sont concaténés automatiquement pour produire une seule vidéo finale.

---

### 🎞️ Introduction optionnelle (V2)
Avant le blind test, il est possible d’ajouter une **introduction** :
- image de fond
- titre affiché à l’écran
- musique d’introduction
- durée personnalisée

L’introduction est **optionnelle**.

---

### ⚡ Mode rapide (utilisateur lambda)
À partir d’un simple dossier de vidéos :

```bash
blindtest new --quick ./videos
```

- tous les fichiers `.mp4` sont utilisés
- le nom du fichier devient automatiquement la réponse
- un `montage.json` est généré
- la vidéo finale est rendue directement

Options :
- `--shuffle` : mélange l’ordre des clips
- `--only-json` : génère uniquement le JSON
- `--dry-run` : affiche la commande FFmpeg sans lancer le rendu

---

### 🧙 Mode interactif (assistant guidé)
Un assistant en ligne de commande permet de :
- configurer une intro (optionnelle)
- choisir la sortie vidéo
- définir les durées
- ajouter les clips manuellement

```bash
blindtest new
```

---

### 📄 Format JSON strictement validé

```json
{
  "intro": {
    "background": "assets/intro.png",
    "title": "Blind Test Soirée",
    "music": "assets/intro.mp3",
    "duration": "00:00:05.000"
  },
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
      "start": "00:00:01.000",
      "answer": "Daft Punk - One More Time"
    }
  ]
}
```

---

## 🚀 Utilisation

### Mode rapide
```bash
blindtest new --quick ./videos
```

### Mode rapide sans rendu
```bash
blindtest new --quick ./videos --only-json
```

### Mode interactif
```bash
blindtest new
```

### Rendu depuis un JSON existant
```bash
blindtest render montage.json
```

### Debug FFmpeg
```bash
blindtest render montage.json --dry-run
```

---

## 🧱 Compilation

### Prérequis

- **Rust** (stable)  
  Installation : https://rustup.rs

Vérification :
```bash
rustc --version
cargo --version
```

- **FFmpeg** (obligatoire)

Vérification :
```bash
ffmpeg -version
```

---

### Compilation (développement)

```bash
cargo build
```

Binaire généré :
```text
target/debug/blindtest
```

---

### Compilation optimisée (recommandée)

```bash
cargo build --release
```

Binaire généré :
```text
target/release/blindtest
```

---

### Exécution après compilation

```bash
./target/release/blindtest --help
```

Exemples :
```bash
./target/release/blindtest new --quick ./videos
./target/release/blindtest render montage.json
```

---

### Tests

```bash
cargo test
```

Les tests couvrent :
- parsing JSON
- validation métier
- génération FFmpeg
- assistant interactif
- gestion de l’introduction

---

### Nettoyage

```bash
cargo clean
```

---

## 🎯 Objectifs pédagogiques (ESGI)

- automatiser un montage vidéo répétitif
- rendre l’outil accessible aux non-développeurs
- architecture Rust modulaire et testée
- séparation claire parsing / validation / rendu

---

## 🔮 Évolutions possibles
- transitions (fade, animations)
- interface graphique
- export YouTube / TikTok
- détection BPM / silence

---

## 📄 Licence
Projet pédagogique – ESGI
