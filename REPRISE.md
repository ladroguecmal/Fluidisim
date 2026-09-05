# REPRISE — à lire en premier, en entier

Ce document permet à **n'importe quelle session** — un autre compte Claude, une autre machine, une
personne — de reprendre le projet sans rien connaître de ce qui précède. Il est autoportant : tout
ce qui est nécessaire est ici ou pointé depuis ici.

Il est mis à jour à la fin de chaque session. Si son contenu contredit une mémoire privée ou un
souvenir de conversation, **c'est lui qui fait foi**.

---

## Jeton de session

```
JETON            : libre
Dernière session : S06 — 2026-09-05 — outillage auteur + dispositif de passation
Session suivante : S07 — recroiser les cinq SPEC entre elles (S05 n'a confronté que les ADR)
```

**Convention de passation.** Une seule session travaille à la fois sur le dépôt.

1. En commençant : passer `JETON` à `occupé — S<n>, <date>`, et écrire la ligne dans
   `notes/JOURNAL.md`.
2. En terminant : exécuter le rituel de fin (§6), puis repasser `JETON` à `libre`.
3. Si le jeton est trouvé `occupé` avec une date ancienne, il est présumé abandonné : le signaler
   dans le journal, le reprendre, et ne rien supprimer de ce que la session précédente avait
   commencé.

---

## 1. Ce qu'est ce projet

Conception du **système général de gestion de l'eau** d'un jeu vidéo de très grande échelle
(référence citée par l'équipe : Star Citizen, en plus grand). Le dépôt contient la conception, pas
le code : aucune ligne n'a encore été écrite.

Point de départ historique : deux documents d'intention, conservés non modifiés dans
`docs/sources/`. Tout le reste a été produit depuis.

## 2. Ton rôle et la manière de travailler

Tu es l'ordinateur central de l'équipe de développement. Les développeurs suivent le projet mais
n'interviennent pas dans la conception.

| Attendu | Précision |
|---|---|
| **Autonomie complète** | tu suis ton raisonnement jusqu'au bout, y compris les chemins sinueux ; tu ne t'arrêtes pas à la solution simple |
| **Chercher ce qui n'a pas été anticipé** | c'est la valeur principale attendue ; voir `docs/registres/ANGLES-MORTS.md` |
| **Publier dans le dépôt** | une réponse conversationnelle non archivée est une perte sèche |
| **Markdown uniquement** | pas de page HTML publiée, pas d'artefact — demandé explicitement après S01 |
| **Français, concis, factuel** | pas de reformulation, pas de remplissage ; le fond va dans les fichiers, pas dans le message |
| **Signaler, ne pas trancher** | les décisions qui engagent le design ou une autre équipe remontent à l'humain (§5) |

Le protocole de conception détaillé est dans `notes/METHODE.md`. Les enseignements accumulés sont
dans `notes/LECONS.md` — les lire avant de commencer fait gagner du temps, plusieurs y sont des
pièges déjà payés.

## 3. Où est la connaissance

```
docs/00_INDEX.md          ← point d'entrée, état d'avancement, arbitrages en attente
docs/01_INVARIANTS.md     ← 16 règles non négociables, à connaître avant toute proposition
docs/adr/                 ← 21 décisions d'architecture, numérotées, jamais réécrites
docs/specs/               ← SPEC-001 hydrodynamique · 002 phénomènes secondaires
                            004 interfaces · 005 outillage auteur
docs/validation/          ← SPEC-003 harnais · CAS-CANONIQUES · PLAN-BENCHMARK
docs/registres/           ← angles morts · questions ouvertes · revue croisée
docs/sources/             ← documents d'intention d'origine, non modifiés
notes/                    ← METHODE · LECONS · JOURNAL
```

**Ordre de lecture pour reprendre** : ce document → `notes/JOURNAL.md` (dernière entrée) →
`docs/00_INDEX.md` → `docs/01_INVARIANTS.md` → `docs/adr/ADR-001`.

Une reprise complète demande une vingtaine de minutes de lecture. Ne pas la sauter : plusieurs
décisions ne se comprennent que par leur motif, et refaire un raisonnement déjà fait est le
gaspillage le plus fréquent d'un projet de ce type.

## 4. Où en est le projet

Six sessions, 21 ADR, cinq spécifications, trois registres. Les 30 sections du document de
questions ouvertes d'origine sont traitées. Les vingt premiers ADR ont été confrontés les uns aux
autres en S05 : douze écarts trouvés, dont deux de gravité 1, tous résolus.

**Il n'y a plus de document bloquant.** Ce qui reste est du code, des mesures et des réunions.

Chemin critique : `ADR-020 acté → H1 (cœur du harnais) → (C01, C02 → λ_cut → B2) et (H4 → B3) → B4`.
**H1 doit précéder la première ligne du solveur** — c'est le seul élément du plan qui ne se
rattrape pas.

Détail à jour : `docs/00_INDEX.md`, section « État d'avancement ».

## 5. Ce qui n'est pas à toi de décider

Trois arbitrages de design et quatre interfaces inter-équipes attendent une réponse humaine. Ils
sont listés dans `docs/00_INDEX.md`, section « Ce qui attend une réponse humaine ».

Les rappeler en fin de session tant qu'ils sont ouverts. Ne pas les trancher, ne pas les contourner
par une hypothèse implicite.

## 6. Rituel de fin de session — obligatoire

Avant de rendre la main, dans cet ordre :

1. **Écrire l'entrée de journal** dans `notes/JOURNAL.md` : entrées, sorties, décision
   structurante, chiffres qui ont orienté la conception, ce qui n'a pas été fait, session suivante
   recommandée, arbitrages en attente.
2. **Enregistrer les angles morts trouvés** dans `docs/registres/ANGLES-MORTS.md`, avec sévérité.
3. **Enregistrer les leçons généralisables** dans `notes/LECONS.md` — une leçon qui ne sert que
   dans son cas d'origine n'y a pas sa place.
4. **Mettre à jour `docs/00_INDEX.md`** : nouveaux documents, avancement, arbitrages.
5. **Corriger ce qui a été invalidé** : un ADR n'est jamais réécrit, mais une erreur factuelle
   reçoit une note corrective visible et datée, et une décision changée fait l'objet d'un nouvel
   ADR qui remplace explicitement l'ancien.
6. **Mettre à jour ce document** : jeton, numéro de session, état, session suivante.

Une session qui n'exécute pas ce rituel laisse le projet dans un état où la suivante devra
reconstituer ce qu'elle a fait — c'est-à-dire perdre l'essentiel de son apport.

## 7. Règles de tenue du dépôt

- **Un ADR n'est jamais réécrit.** L'historique du raisonnement a autant de valeur que la
  conclusion. Une décision qui change fait l'objet d'un nouvel ADR.
- **Aucun nombre sans provenance** (invariant I-14) : formule citée dans SPEC-001 ou SPEC-002, ou
  étiquette « à calibrer » avec le banc qui le fixera.
- **Les documents de `docs/sources/` ne sont pas modifiés.** Leur relecture critique vit dans
  `docs/registres/`.
- **Distinguer les statuts** : résolu · dissous · partiel · ouvert par décision. « Ouvert par
  décision » est un statut légitime et doit être dit.

## 8. Limites connues de ce dispositif

À signaler à l'humain plutôt qu'à contourner :

- **Le dépôt n'est pas sous gestion de version.** Pour une passation entre comptes, c'est un risque
  réel : pas d'historique, pas de fusion, pas de récupération après écrasement. Un `git init` et un
  dépôt distant partagé résoudraient les trois. **À proposer, pas à faire sans accord.**
- **Le partage des fichiers entre comptes relève de l'infrastructure de l'utilisateur** (dossier
  synchronisé, dépôt distant). Ce document ne peut pas y suppléer : si un autre compte ne voit pas
  ces fichiers, il ne peut pas reprendre le projet, quelle que soit la qualité de la passation.
- **Les mémoires privées d'un compte ne voyagent pas.** Rien d'important ne doit vivre uniquement
  là. En cas de contradiction, ce document et le journal font foi.
- **Aucune session ne doit supposer que la précédente était la sienne.** Vérifier le journal plutôt
  que se fier à un souvenir.
