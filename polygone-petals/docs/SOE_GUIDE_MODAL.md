# SOE-ORRET dLLM — Guide Complet de Construction (Modal Edition)

Intelligence Artificielle Symbiotique · Infrastructure Modal · 2026

> "Ce qui est maintenant prouvé n’était autrefois qu’imaginé." — William Blake

## Ce que ce guide construit

SOE-Orret est un **dLLM (Diffusion Language Model)** basé sur Qwen2.5-7B converti via **A2D**, avec mémoire hiérarchique, agent autonome RAG, et autocritique intégrée. 

Ce guide remplace l'utilisation de Google Colab par **Modal** pour l'entraînement et l'inférence.

---

## 1. Philosophie et Architecture

### 1.1 Pourquoi un dLLM?
Contrairement aux modèles autoregressifs (AR) classiques (token par token), un dLLM (Diffusion LLM) remplit toute la séquence en parallèle via débruitage itératif. 

**Avantages :** 
- Contexte bidirectionnel natif.
- Génération parallèle massive.
- Résilience au "Reversal Curse".

### 1.2 La recette A2D (Autoregressive-to-Diffusion)
La conversion s'effectue via un **Simple Continual Pretraining (SCP)** :
1. Partir d'un checkpoint AR fort (Qwen2.5-7B).
2. Remplacer le masque causal par un masque bidirectionnel.
3. Entraîner avec l'objectif de démasquage MDLM.
4. Appliquer des learning rates différentiés.

---

## 2. Préparation Infrastructure (Modal)

### 2.1 Installation
Assurez-vous d'avoir Modal configuré sur votre machine :

```bash
pip install modal
modal setup
```

### 2.2 Environnement Python
Le script `modal_app.py` gère automatiquement l'image Docker avec les dépendances suivantes :
- `torch`, `transformers`, `accelerate`, `datasets`
- `vLLM` ou `llm-diffusion`
- `faiss-cpu`, `chromadb` (pour la mémoire)

---

## 3. Conversion A2D — Entraînement

L'entraînement nécessite un GPU de classe **A100 (40GB)** ou **H100**.

### 3.1 Lancement de l'entraînement
Utilisez le script `train_modal.py` fourni dans le dépôt :

```bash
modal run train_modal.py --data ./datasets/train.jsonl --steps 6000
```

Le script va :
- Monter un `modal.NetworkFileSystem` pour les checkpoints.
- Allouer un GPU A100 dynamiquement.
- Exécuter la boucle d'entraînement MDLM.

---

## 4. Inférence et Architecture Mémoire

### 4.1 Mémoire ARIA (5 couches)
- **L1 Working** : RAM contextuelle.
- **L2 Episodic** : Historique avec scoring de cohérence.
- **L3 Semantic** : Connaissances stockées via FAISS.
- **L4 Procedural** : Patterns d'actions.
- **L5 World Model** : Modèle interne de soi.

### 4.2 Lancement de l'API
Pour servir le modèle via une API compatible OpenAI sur Modal :

```bash
modal serve inference_modal.py
```

---

## 5. Intégration POLYGONE P2P

SOE-ORRET utilise le cœur **Polygone** pour fragmenter ses sessions de réflexion et les distribuer sur le réseau éphémère.

**Usage :**
Dans l'agent Orret, activez la modalité P2P :
```python
orret = OrretComplete(enable_p2p=True)
```

---

*SOE-Orret — MIT License*
*Construit par Lévy, 14 ans, France, 2026*
