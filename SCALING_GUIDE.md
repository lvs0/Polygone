# ⬢ POLYGONE : GUIDE DE PASSAGE À L'ÉCHELLE (SCALING)

Ce document décrit comment ajouter des **nœuds relais** et renforcer la résilience du réseau.  
Le message public du projet reste la **cryptographie post-quantique** et le **protocole** — les choix concrets d’hébergement (fournisseurs, budgets, relais) relèvent des **opérateurs** et ne font pas partie du discours public de **Hope**.

## 1. Stratégie multi-nœuds

Déploie plusieurs relais sur des machines **distinctes**, chacun avec une **identité réseau unique**. Relie-les via une adresse de **bootstrap** connue du maillage.

## 2. Variables et déploiement (référence)

Les noms exacts évoluent avec le dépôt ; consulte le `Dockerfile` et le dépôt **Polygone-Server** pour la liste à jour. En pratique :

| Concept | Rôle |
| :--- | :--- |
| Identité / graine | Une clé par nœud — ne jamais réutiliser la même entre instances |
| Bootstrap | Multiaddr vers un pair déjà dans le réseau |
| Santé / exposition HTTP | Selon ton orchestration (healthcheck, shim, etc.) |

L’image conteneur et le registre utilisés par les mainteneurs ne sont pas un argument « marketing » : l’accent public reste le **post-quantique** et le **protocole**.

## 3. Résilience

Le modèle vise une **multi-localisation** des fragments : la perte d’un relais ne doit pas détruire le message si le quorum Shamir est atteint.

## 4. Karma (vision)

Le prêt de ressources peut être assorti d’un système de **Karma** / vouchers — voir le code et la documentation du core.

---

**L'information n'existe pas. Elle traverse.** ⬡  
**l-vs** · **Hope** (*by Hope*)
