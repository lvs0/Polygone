# Community Onboarding Flow — Polygone
## Telegram / Discord

---

## Vue d'ensemble du funnel

```
Discover → Welcome → Educate → Engage → Invest/Contribute → Retain
   ↓          ↓          ↓         ↓           ↓               ↓
 Post TG   Auto msg   Doc link  Bot quiz   #opportunities   Roles
 Reddit    Rules      Video 1   Role pick  Manual intro      Raffles
 Share     FAQ        AMA       Channels   Contract template Peer refs
```

---

## ÉTAPE 1 — Découverte (Public)

### Canaux
- Reddit (r/crypto, r/privacy, r/france)
- Twitter/X (threads éducatifs)
- Hacker News / Lobsters
- Liens directs depuis le site

### Message template (à publier sur Reddit/Twitter) :

```
Polygone : la cryptographie post-quantique en Rust
=================================================

On construit la couche d'infrastructure qui va protéger tes données quand les ordis quantiques casseraient RSA.

- NIST-finalized algorithms (ML-KEM, ML-DSA)
- 100% Rust (mémoire = sécurité)
- Testnet en cours, démos disponibles

Pas de promesse de token qui va x100. Juste du code sérieux sur un problème réel.

Questions ? AMA dans ce thread.
Lien : [site]
Repo : [github]
```

---

## ÉTAPE 2 — Welcome (Auto-message Telegram)

**Déclencheur :** Nouveau membre rejoint le groupe

**Message automatique (envoyé par bot après 2 min) :**

```
Bienvenue sur Polygone 👋

Avant de participer, voici l'essentiel :

🔒 **Qu'est-ce que Polygone ?**
On construit une bibliothèque cryptographique post-quantique en Rust — le truc qui va protéger tes données quand les ordis quantiques seront assez puissants pour casser les chiffrements actuels (RSA, ECC).

Pas de hype. Pas de promesses de gains. Juste de l'ingénierie sérieuse sur un problème avec une date d'échéance.

📚 **Par où commencer ?**
→ [doc link] — Présentation une page du projet
→ [whitepaper summary] — Le whitepaper résumé
→ [github] — Le code

❓ **FAQ Rapide :**

**C'est quoi le "problème quantique" ?**
Les chiffrements qui protègent tes paiements, messages, mots de passe seront cassables par un ordi quantique. La migration est obligatoire d'ici 2030 (NIS2, directives gouvernementales).

**C'est quoi le token POLY ?**
Jeton utilitaire du protocole. Sert à staker, voter, accéder aux services premium. Pas un "investment opportunity".

**Comment contribuer ?**
On cherche : devs Rust, marketing, community, bizdev. Voir #opportunities.

⚠️ **Règles :**
1. Pas de financial advice
2. Pas de pump-and-dump talk
3. Questions ouvertes encouragées
4. L'honnêteté > la positivité

💬 **Maintenant, dis-nous :**
Qu'est-ce qui t'a mené ici ? (1 phrase)
Tes centres d'intérêt : crypto, privacy, développement, ou autre ?
```

---

## ÉTAPE 3 — Educate

### Channel dédié : #📚-learning

**Système de progression (rôles Discord/Telegram) :**

```
Role : 🔰 Newcomer
→ A accès à : #welcome, #📚-learning, #💬-general

Role : 📖 Learner  
→ Condition : A lu la présentation + répondu au quiz
→ A accès à : + #🔬-tech, #📢-updates

Role : 🏛️ Contributor
→ Condition : A contribué (code, docs, com)
→ A accès à : + #💡-proposals, #investors-talk

Role : 🌟 OG Member
→ Condition : 3+ mois actif + contributions régulières
→ A accès à : + salon privé avec l'équipe
```

### Quiz d'onboarding (bot interactif)

```
QUIZ POLYGONE — 5 questions

Q1. Quel algorithme NIST post-quantique est utilisé pour l'échange de clés ?
A) RSA-2048  B) ML-KEM  C) AES-256

Q2. En quelle année NIST a-t-il finalisé les standards post-quantiques ?
A) 2020  B) 2023  C) 2024

Q3. Quel langage de programmation est utilisé pour Polygone ?
A) Python  B) Rust  C) Go

Q4. NIS2 est une directive de quel organisme ?
A) USA  B) ONU  C) UE

Q5. Qu'est-ce que "harvest now, decrypt later" ?
A) Un protocole de sécurité
B) Une attaque où on stocke la data pour plus tard
C) Un algo de chiffrement

Réponses : Q1=B, Q2=C, Q3=B, Q4=C, Q5=B
Score 4/5+ → accès #🔬-tech
```

---

## ÉTAPE 4 — Engage

### Programme d'ambassadeur informel

| Action | Récompense |
|---|---|
| Partage d'un post Polygone (capture écran) | Accès anticipé au role #📖-Learner |
| Question utile dans #AMA | Mention dans le weekly update |
| Bug report accepté sur GitHub | NFT Discord (non transférable) |
| Traduction doc (FR→EN ou EN→FR) | Allocation tokens (à discuter) |
| 10 helpful answers dans #💬 | Accès salon privé avec équipe |

### Weekly engagement thread (chaque lundi)

```
📊 Weekly Polygone Update — [Date]

1. Avancement de la semaine écoulée (2-3 lignes, honnête)
2. Objectifs de la semaine القادمة
3. Questions ouvertes au comunidad
4. Shoutouts aux contributeurs

Pas de hype. Juste du factuel.
```

### AMA bimensuel (1er et 3e jeudi du mois)

Format :
- 30 min questions ouvertes
- Pas de questions sur "le prix du token" ou "when lambo"
- L'équipe répond honnêtement, y compris "je ne sais pas"

---

## ÉTAPE 5 — Invest / Contribute

### Channel : #💼-opportunities

**Message template :**

```
🤝 Contribuer à Polygone — Comment ça marche

On ne demande pas juste de l'argent. On cherche des co-équipiers.

💻 **Développement Rust**
Vous êtes : dév Rust intermédiaire ou plus
Vous faites : implémentation ML-KEM/ML-DSA, optimisations SIMD, SDK
En retour : allocation tokens + co-authorship
Niveau d'engagement : 10h/semaine minimum

📢 **Marketing / Comms**
Vous êtes : à l'aise avec Twitter, Reddit, creation de contenu
Vous faites : threads éducatifs, stratégie de marque, outreach
En retour : allocation tokens + accès anticipé aux features
Engagement : 5h/semaine minimum

👥 **Community**
Vous êtes : disponible, patient, bon communiquant
Vous faites : onboarding de nouveaux membres, Q&A, feedback
En retour : role OG + tokens pour engagement long terme
Engagement : 3h/semaine minimum

💰 **Investissement**
Montant : 500 € — 10 000 € (ce que tu peux perdre)
Format : prêt d'honneur, equity SARL, ou allocation tokens
En retour : selon le format choisi (voir #partnership-template)
Horizon : 3-5 ans minimum

📩 **Pour postuler :**
DM à @[handle] ou email [adresse]
Précise : ce qui t'intéresse + tes compétences/ressources disponibles
```

### Pour les investisseurs (amis/famille) uniquement

**Channel séparé (sur invitation) : #🔐-trusted-investors**

```
Ce salon est reservé aux personnes qui ont signé ou sont en discussion pour le partnership template Polygone.

Accès : sur invitation manuelle par @[handle]

Ici on partage :
- Updates financiers détaillés (trimestriels)
- Avancement technique réel (y compris les problèmes)
- Documents légaux (partnership template, clauses)
- Questions directes à l'équipe
```

---

## ÉTAPE 6 — Retain

### Rôles et privilèges

| Rôle | Condition | Accès | Badge |
|---|---|---|---|
| 🔰 Newcomer | Default on join | #welcome, #general | — |
| 📖 Learner | Quiz 4/5 | #tech, #updates | ✓ |
| 🏛️ Contributor | 1+ contribution | #proposals, #investors | ✓ |
| 🌟 OG Member | 3 mois + 10+ contributions | Salon privé équipe | ✓ |
| 💎 Early Backer | A investi pre-TGE | Salon privé + calls | ✓ |

### Offres réservées aux membres actifs

```
 Membres avec rôle 📖 Learner ou plus :

1. Accès anticipé aux testnets (2 semaines avant public)
2. Nommage d'une feature (parmi une liste pre-définie)
3. Accès aux tokens de gouvernance (une fois DAO lancé)
4. Possibilité de rejoindre l'équipe à temps plein (si fit)

 Membres OG uniquement :

1. Calls trimestriels avec le fondateur
2.输入 dans les décisions produit (via #💡-proposals)
3. Accès anticipé aux audits de sécurité ( NDA )
4. Early adopter pricing si service enterprise un jour
```

### Offboarding

Si un membre est inactif depuis 30+ jours :
- Message amical : "On t'a manqué ! Des nouvelles ?"
- Pas de kick automatique
- Rôle rétrogradé (Contributor → Learner) après 60 jours sans réponse

---

## Bot Commands (Telegram)

```
/start — Affiche le message de bienvenue
/quiz — Lance le quiz d'onboarding  
/status — Affiche ton rôle actuel
/roles — Liste des rôles disponibles
/contribute — Affiche les opportunités
/partnership — Lien vers le template d'investissement
/doc — Liens vers la documentation
/help — Liste des commandes
```

---

## KPI Onboarding

| Métrique | Cible |
|---|---|
| Taux de completion quiz | > 60% |
| Conversion Welcome → Learner | > 30% |
| Conversion Learner → Contributor | > 10% |
| Taux de rétention 30 jours | > 50% |
| Nouveaux membres / semaine | Non ciblé (quality > quantity) |

---

## Templates de messages

### Welcome (DM automatique après join)

```
Hey [Name] ! Bienvenue chez Polygone 🌐

Tu viens de rejoindre notre communauté. On est en train de construire la bibliothèque cryptographique post-quantique en Rust — le truc qui va protéger les données de tout le monde quand les ordi quantiques seront là.

C'est un projet technique, sérieux, sans promesses de gains miraculeux. Si c'est le genre de truc qui te parle, tu es au bon endroit.

Par où commencer :
→ Lis la présentation : [link]
→ Rejoins #📚-learning et fais le quiz pour débloquer les autres channels
→ Présente-toi dans #💬-general !

Questions ? DM moi.
```

### Weekly Update (posté chaque lundi)

```
📊 POLYGONE — Semaine du [date]

🔧 TECH
[Bref résumé de ce qui a avancé — 2-3 bullets max, sans langue de bois]

📅 SEMAINE PROCHAINE
[Ce qu'on prévoit de faire]

❓ OUVERT
[Problème non résolu ou question ouverte — invitation à contribuer]

🌟 CONTRIBUTEURS DE LA SEMAINE
[@user] — [ce qu'il/elle a fait]
[@user] — [ce qu'il/elle a fait]

C'est tout pour cette semaine. Questions bienvenues.
```

### Offboarding message (inactivité 30j)

```
Hey ! Ça fait un moment qu'on t'a pas vu par ici 👋

J'espère que tout va bien de ton côté. On voulait juste prendre de tes nouvelles — le projet avance bien, et on se dit que tu as peut-être des questions ou des choses à partager.

Sinon, pas de pression. On reste là si jamais tu veux revenir ou discuter.

À+
```
