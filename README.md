# 🎵 Blindtest Video Builder

Outil en ligne de commande écrit en **Rust** permettant de générer automatiquement des **vidéos de blindtest** à partir d’un fichier **JSON**.

Le programme utilise **FFmpeg** pour extraire des clips, masquer la vidéo pendant la phase de devinette, afficher la réponse lors de la révélation, puis concaténer l’ensemble en une vidéo finale.

---

## ✨ Fonctionnalités (V1)

- Entrée unique via un fichier JSON
- Phase devinette (audio seul)
- Phase révélation (vidéo + réponse)
- Durées globales identiques pour tous les clips
- Overlay texte automatique
- Binaire rapide et portable

---

## 📦 Prérequis

- Rust
- FFmpeg (dans le `PATH`)

---

## 🚀 Utilisation

```bash
blindtest render montage.json
```

---

## 🧾 Format du JSON

```json
{
  "output": {
    "path": "render/blindtest.mp4",
    "resolution": "1920x1080",
    "fps": 30
  },
  "timings": {
    "guess_duration": "00:00:10.000",
    "reveal_duration": "00:00:05.000"
  },
  "clips": [
    {
      "video": "videos/source1.mp4",
      "start": "00:01:23.500",
      "answer": "Daft Punk - One More Time"
    }
  ]
}
```

---

## 🎬 Règles de montage

Pour chaque clip :
- audio seul pendant la phase devinette
- vidéo + réponse pendant la phase révélation  

Les clips sont concaténés dans l’ordre du JSON.

---

## 📄 Licence

MIT
