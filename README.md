# 🎵 BlindTestAuto (Rust)

BlindTestAuto est un outil en ligne de commande écrit en **Rust** qui permet de **générer automatiquement une vidéo de blind test** à partir de clips vidéo.

Le montage est entièrement automatisé grâce à **FFmpeg** et un fichier de configuration **JSON** (généré à la main ou via un assistant interactif).

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

Les clips sont ensuite concaténés automatiquement.

---

### 🎞️ Introduction optionnelle (V2)
Avant le blind test, il est possible d’ajouter une **intro** :
- image de fond
- titre centré à l’écran
- musique d’introduction
- durée personnalisée

L’intro est **optionnelle** et n’est ajoutée que si elle est définie.

---

### ⚡ Mode rapide (pour utilisateurs non techniques)
Un seul dossier de vidéos suffit :

```bash
blindtest new --quick ./videos
```

- tous les fichiers `.mp4` sont utilisés
- le nom du fichier devient la réponse
- un `montage.json` est généré automatiquement
- la vidéo finale est rendue directement

Options disponibles :
- `--shuffle` : mélange l’ordre des clips
- `--only-json` : génère uniquement le JSON (pas de rendu)
- `--dry-run` : affiche la commande FFmpeg sans lancer le rendu

---

### 🧙 Mode interactif (assistant guidé)
Un assistant en ligne de commande permet de :
- définir une intro (optionnelle)
- configurer la sortie vidéo
- choisir les durées
- ajouter les clips un par un

```bash
blindtest new
```

---

### 📄 Format JSON clair et validé
Le projet repose sur un fichier JSON strictement validé (types, champs obligatoires, timecodes).

---

## 🚀 Utilisation

### Mode rapide (recommandé)
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

---

## 📄 Licence
Projet pédagogique – ESGI
