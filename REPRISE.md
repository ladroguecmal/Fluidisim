# REPRISE — à lire en premier, en entier

Ce document permet à **n'importe quelle session** — un autre compte Claude, une autre machine, une
personne — de reprendre le projet sans rien connaître de ce qui précède. Il est autoportant : tout
ce qui est nécessaire est ici ou pointé depuis ici.

Il est mis à jour à la fin de chaque session. Si son contenu contredit une mémoire privée ou un
souvenir de conversation, **c'est lui qui fait foi**.

---

## Jeton de session

```
JETON            : occupé
Battement        : 2026-09-06
Session en cours : S22 — C01, le repos hydrostatique sur pente, et le premier δ
Dernière session : S21 — 2026-09-05 — H3 : le cas analytique trouve le premier vrai bug
Session suivante : S23 — C02 sur δ et λ_cut, ou H2
```

**Une seule session travaille à la fois.** Le jeton a trois états, et non deux :

| État | Signification | Ce que fait la session qui le trouve |
|---|---|---|
| `libre` | personne ne travaille | le prendre, démarrage à froid |
| `occupé` + battement récent (< 2 h) | une session travaille | **ne pas reprendre** ; signaler à l'utilisateur |
| `occupé` + battement ancien (> 2 h) | session présumée interrompue | passer à `interrompu`, puis reprise à chaud (§7) |
| `interrompu` | interruption constatée | reprise à chaud (§7) |

Le seuil de deux heures est une convention, choisie parce que les limites d'usage se
réinitialisent à cette échelle. Plus court, deux sessions se marchent dessus ; plus long, on
attend pour rien.

**En commençant** : passer le jeton à `occupé`, mettre à jour le battement, déclarer le plan dans
`notes/EN-COURS.md` et le committer **avant toute autre modification**.
**En terminant** : exécuter le rituel de fin (§6), repasser le jeton à `libre`.

Le battement se met à jour à chaque commit d'étape. Il n'existe pas de processus d'arrière-plan :
une session ne peut signaler sa présence qu'en travaillant.

---

## 1. Ce qu'est ce projet

Conception du **système général de gestion de l'eau** d'un jeu vidéo de très grande échelle
(référence citée par l'équipe : Star Citizen, en plus grand). Le dépôt contient la conception, pas
le code : aucune ligne n'a encore été écrite.

Point de départ historique : deux documents d'intention, conservés non modifiés dans
`docs/sources/`. Tout le reste a été produit depuis.

> **À savoir avant de lire quoi que ce soit d'autre (S19).** **Il n'y a pas d'autres équipes.** Une
> seule personne travaille sur ce système ; les développeurs observent. Les onze « destinataires
> extérieurs » recensés dans le corpus — audio, IA, terrain, rendu, véhicules, personnage, réseau,
> assurance qualité… — **n'existent pas comme interlocuteurs**.
>
> Les contraintes qu'ils portaient restent vraies ; c'est leur destinataire qui manque. Une question
> classée « attend une autre équipe » est donc en réalité **une décision à trancher sans
> interlocuteur** — et le corpus en comptait quatorze. Voir
> [`ADR-028`](docs/adr/ADR-028-il-n-y-a-pas-d-autres-equipes.md) §2.
>
> **Ne pas reporter une question à une équipe.** Si elle relève de la conception, elle est à toi.

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
docs/01_INVARIANTS.md     ← 17 règles non négociables, à connaître avant toute proposition
docs/adr/                 ← 29 décisions d'architecture, numérotées, jamais réécrites
code/                     ← water-core et water-harness (Rust, sans dépendance) — étage H1
docs/specs/               ← SPEC-001 hydrodynamique · 002 phénomènes secondaires
                            004 interfaces (chemin tiré) · 005 outillage auteur
                            006 chemin poussé (ce que le système publie)
docs/validation/          ← SPEC-003 harnais · CAS-CANONIQUES · PLAN-BENCHMARK
docs/registres/           ← angles morts · questions ouvertes · revues croisées · trois audits
docs/sources/             ← documents d'intention d'origine, non modifiés
notes/                    ← METHODE · LECONS · JOURNAL
```

**Ordre de lecture pour reprendre** : ce document → `notes/JOURNAL.md` (dernière entrée) →
`docs/00_INDEX.md` → `docs/01_INVARIANTS.md` → `docs/adr/ADR-001`.

Une reprise complète demande une vingtaine de minutes de lecture. Ne pas la sauter : plusieurs
décisions ne se comprennent que par leur motif, et refaire un raisonnement déjà fait est le
gaspillage le plus fréquent d'un projet de ce type.

## 4. Où en est le projet

Vingt-et-une sessions, **29 ADR** *(dont un acté)*, six spécifications, huit registres — **et du
code qui tourne** : `code/`, étages **H1 et H3** du harnais, 18 tests et 12 assertions analytiques
au vert. Les 30 sections du document de
questions ouvertes d'origine sont traitées. Les vingt premiers ADR ont été confrontés les uns aux
autres en S05 (douze écarts, deux de gravité 1) et les cinq SPEC entre elles en S08 (dix écarts,
deux de gravité 1). Tous résolus — le dernier, le **chemin poussé**, par l'écriture de `SPEC-006`
en S09.

**Il n'y a plus de document bloquant, et les quatre interfaces inter-équipes ont enfin quelque chose
à soumettre.** Ce qui reste est du code, des mesures et des réunions.

**Une urgence de format, la seule.** `WaveEvent` (SPEC-006 §3.1) est une structure **répliquée** qui
porte trois champs demandés par l'équipe audio. Elle doit être arrêtée **avant** que le réseau ne
fige son format ; l'élargir après coûtera une migration de protocole. C'est le seul point où
attendre a un coût croissant.

Chemin critique : `ADR-020 acté → H1 → H3 → (C01, C02 → λ_cut → B2) et (H4 → B3) → B4`. Les deux
premiers maillons sont **faits** ; le suivant est C01, qui demande un `δ` si minuscule soit-il.

**Et S21 a montré pourquoi cet ordre n'était pas une précaution.** Le harnais déterministe H1 était
vert sur un champ **faux** : la vitesse orbitale était en quadrature au lieu d'être en phase avec
l'élévation, donc l'eau n'avançait pas sous une crête. Dix-neuf sessions de conception et six audits
ne l'avaient pas vu ; une identité fermée, `u = ω·η`, l'a fait tomber au premier passage. **Le
déterminisme et la justesse sont deux propriétés sans rapport** (leçon L67) : un solveur écrit sans
H3 aurait hérité du défaut, et le harnais l'aurait certifié stable.

Détail à jour : `docs/00_INDEX.md`, section « État d'avancement ».

## 5. Ce qui n'est pas à toi de décider

**Les cinq arbitrages de design ont été tranchés en S18** par
[`ADR-027`](docs/adr/ADR-027-les-cinq-arbitrages-tranches.md), sur délégation explicite. Deux des
cinq se sont **dissous** plutôt que choisis. Ne pas les rouvrir sans demande ; ADR-027 dit pour
chacun ce qu'il faudrait changer pour l'inverser.

**Quatorze demandes extérieures** attendent en revanche toujours une réponse, en fiches présentables
dans **`docs/DOSSIER-REUNIONS.md`** (S17), classées par ce que la réponse débloque. Et **trois choses
restent hors de portée d'une session** : nommer les personnes, constater l'état réel du projet, agir
sur l'infrastructure. Voir `CLAUDE.md`.

Les rappeler en fin de session tant qu'ils sont ouverts. Ne pas les trancher, ne pas les contourner
par une hypothèse implicite.

## 6. Rituel de fin de session — obligatoire

Il est **lui-même une étape du plan** déclaré dans `notes/EN-COURS.md`. Une session interrompue
laisse ainsi cette étape visiblement non cochée, ce qui dit à la suivante exactement ce qui manque.

Avant de rendre la main, dans cet ordre :

1. **Écrire l'entrée de journal** dans `notes/JOURNAL.md` : entrées, sorties, décision
   structurante, chiffres qui ont orienté la conception, ce qui n'a pas été fait, session suivante
   recommandée, arbitrages en attente.
2. **Enregistrer les angles morts trouvés** dans `docs/registres/ANGLES-MORTS.md`, avec sévérité.
3. **Enregistrer les leçons généralisables** dans `notes/LECONS.md` — une leçon qui ne sert que
   dans son cas d'origine n'y a pas sa place.
4. **Relever les actions décidées en séance** — « à ajouter au banc », « à porter à tel
   document ». Celles qui n'ont pas été faites deviennent une étape du plan, ou un point ouvert
   daté. Une annonce en prose est une intention, pas une tâche : S15 en a retrouvé trois, perdues
   depuis six sessions (L55).
5. **Mettre à jour `docs/00_INDEX.md`** : nouveaux documents, avancement, arbitrages. **Et vérifier
   les décomptes recopiés** — nombre d'ADR, d'invariants, de spécifications, d'angles morts, de cas
   canoniques — dans `README.md`, `REPRISE.md` et l'index. Un décompte recopié se périme en
   silence : le défaut a été trouvé en S07, puis de nouveau en S10.
5. **Corriger ce qui a été invalidé** : un ADR n'est jamais réécrit, mais une erreur factuelle
   reçoit une note corrective visible et datée, et une décision changée fait l'objet d'un nouvel
   ADR qui remplace explicitement l'ancien. **Et parcourir les listes « ce qui reste ouvert » qui
   citaient ce qu'on vient de décider** — une correction se propage vers la prose qui l'explique,
   jamais vers les points ouverts qui la réclamaient (S11 en a trouvé quatre instances ; c'est une
   recherche de texte, pas une relecture).
   **Enfin, relire les invariants que la décision cite** et se demander si l'un d'eux devient faux.
   Deux pages, trois minutes — S13 a trouvé deux invariants périmés sur dix-sept, dont un de
   gravité 1, et ils avaient survécu à deux revues croisées et à un audit des points ouverts (L49).
6. **Mettre à jour ce document** : jeton, numéro de session, état, session suivante.

Une session qui n'exécute pas ce rituel laisse le projet dans un état où la suivante devra
reconstituer ce qu'elle a fait — c'est-à-dire perdre l'essentiel de son apport.

## 7. Si la session précédente a été interrompue

Une session coupée par une limite d'usage n'a aucune occasion d'écrire qu'elle s'arrête. Le
dispositif ne repose donc **pas** sur une action au moment de l'arrêt, mais sur une déclaration
faite avant le travail.

Une session qui ouvre ce dossier est dirigée ici automatiquement par [`CLAUDE.md`](CLAUDE.md) : le
dispositif ne dépend ni de la mémoire d'un compte, ni de ce que l'utilisateur pense à écrire dans
son premier message.

→ **La procédure de reprise à chaud est dans [`notes/EN-COURS.md`](notes/EN-COURS.md)**, en tête du
fichier, avec l'état et le plan de la session interrompue.

En deux lignes : *ce qui est committé est fait ; ce qui est modifié non committé appartient à
l'étape marquée `[>]` et doit être soit complété soit annulé, jamais laissé en suspens.* Compter
cinq minutes — la lecture complète du dépôt (§3) ne sert qu'au démarrage à froid.

Prévenir aussi l'utilisateur : la session interrompue n'a probablement pas pu rendre compte de son
travail, et il ne l'a peut-être jamais vu.

## 8. Règles de tenue du dépôt

- **Un ADR n'est jamais réécrit.** L'historique du raisonnement a autant de valeur que la
  conclusion. Une décision qui change fait l'objet d'un nouvel ADR.
- **Aucun nombre sans provenance** (invariant I-14) : formule citée dans SPEC-001 ou SPEC-002, ou
  étiquette « à calibrer » avec le banc qui le fixera.
- **Les documents de `docs/sources/` ne sont pas modifiés.** Leur relecture critique vit dans
  `docs/registres/`.
- **Distinguer les statuts** : résolu · dissous · partiel · ouvert par décision. « Ouvert par
  décision » est un statut légitime et doit être dit.

## 9. Limites connues de ce dispositif

À signaler à l'humain plutôt qu'à contourner :

- **Le dépôt est sous git depuis S07** (commit de base `c6886a7`). Les sessions S01 à S06 n'ont pas
  d'historique : elles tiennent dans ce seul commit et dans `notes/JOURNAL.md`.
- **Il n'y a pas de dépôt distant.** Sans lui, la passation entre deux machines repose sur un
  dossier partagé, et deux sessions qui écriraient en parallèle n'auraient aucun moyen de
  fusionner. **À proposer à l'utilisateur, pas à faire sans accord.**
- **Le partage des fichiers entre comptes relève de l'infrastructure de l'utilisateur.** Ce
  document ne peut pas y suppléer : si un autre compte ne voit pas ces fichiers, il ne peut pas
  reprendre le projet, quelle que soit la qualité de la passation.
- **Les mémoires privées d'un compte ne voyagent pas.** Rien d'important ne doit vivre uniquement
  là. En cas de contradiction, ce document et le journal font foi.
- **Aucune session ne doit supposer que la précédente était la sienne.** Vérifier le journal et
  `git log` plutôt que se fier à un souvenir.
- **Le battement n'est pas une preuve de vie.** Une session peut être coupée juste après un commit
  et paraître active pendant deux heures. Le seuil protège du conflit, il ne le supprime pas.
