# Travail en cours — journal d'intention

> **Pourquoi ce fichier existe.** Une session coupée par une limite d'usage n'a *aucune* occasion
> d'écrire « j'ai été interrompue ». Tout dispositif de passation qui suppose une action au moment
> de l'arrêt est donc inutile. Seule survit une déclaration faite **avant** le travail.
>
> Ce fichier déclare ce qui va être fait, avant de le faire. Git enregistre ce qui a effectivement
> été fait. L'écart entre les deux est exactement ce qui a été interrompu.

---

## Reprise à chaud — procédure

À suivre lorsque l'état ci-dessous n'est pas `terminée`. Cinq minutes ; **ne pas lire tout le
dépôt** — la lecture complète (`REPRISE.md`) ne sert qu'au démarrage à froid.

1. **Lire l'état et le plan** de la session en cours, plus bas.
2. `git log --oneline -15` — **ce qui est committé est fait**, définitivement. Ne pas le refaire.
3. `git status --short` — les fichiers modifiés non committés appartiennent à l'étape marquée
   `[>]`. C'est elle qui a été interrompue, et elle seule.
4. `git diff` — **lire avant de décider**. Deux issues, pas trois :
   - **compléter** l'étape, si le diff est cohérent et si la thèse déclarée dans le plan est
     claire ;
   - **annuler** l'étape (`git restore <fichiers>`), si le diff est incohérent ou
     incompréhensible.

   Ne jamais laisser un état intermédiaire non tranché, et écrire dans le journal lequel des deux
   a été choisi.
5. **Lire les notes de reprise** de la session interrompue. C'est là que vivent les chiffres déjà
   calculés, les décisions prises mais pas encore écrites et les impasses déjà explorées —
   l'information la plus coûteuse à reproduire, et la seule que git ne conserve pas.
6. Reprendre au premier `[ ]`, ou à `[>]` si l'étape a été complétée.
7. **Prévenir l'utilisateur** : la session précédente a probablement été coupée avant d'avoir pu
   rendre compte de son travail. Résumer ce qu'elle avait fait — il ne l'a peut-être jamais vu.

---

## Règles pour la session qui travaille

- **Déclarer le plan complet avant la première modification**, et le committer seul. C'est
  l'écriture anticipée : sans elle, une interruption ne laisse aucune trace d'intention.
- **Aucune étape ne dépasse une quinzaine de minutes de travail.** Si elle est plus grosse, la
  découper. C'est la seule prophylaxie réelle contre une coupure — pas un confort d'organisation.
- Marquer `[>]` **avant** de commencer une étape. Basculer `[x]` **en dernière action avant le
  commit de cette étape**, jamais après : le commit doit contenir à la fois le travail et la case
  cochée, sinon l'historique ment dans un sens ou dans l'autre. Un `[x]` sans commit est un
  mensonge que la session suivante paiera ; un commit sans `[x]` fera refaire du travail déjà fait.
- **Un commit par étape**, message `S<n> P<k> — <description>`. Le plan et le journal git disent
  alors la même chose de deux façons indépendantes ; si l'un est faux, l'autre le révèle.
- Déposer dans **Notes de reprise** tout ce qui n'est pas encore dans un fichier : un chiffre
  calculé, une décision prise, une impasse explorée. **Une impasse est aussi précieuse qu'un
  résultat** — sans elle, la session suivante la réexplore intégralement.
- **Le rituel de fin (`REPRISE.md` §6) est lui-même une étape du plan.** Une session interrompue
  laisse ainsi cette étape visiblement non cochée, ce qui dit à la suivante exactement ce qui
  manque.

---

## Session en cours

```
Session          : S17
État             : terminée
Battement        : 2026-09-05
Objectif         : le dossier de réunion des onze destinataires extérieurs
```

### Plan

Tout ce qu'on demande à l'extérieur existe — dispersé sur 26 ADR et 7 spécifications. Rien n'est
présentable. C'est le dernier travail disponible qui ne demande ni mesure ni décision humaine
préalable.

- [ ] **P1** — déclarer le plan, prendre le jeton, mettre à jour le battement.
- [x] **P2** — **classer par ce que la réponse débloque**, et non par l'importance du sujet.
  *Thèse : l'ordre de `00_INDEX.md` classe par gravité de conséquence. Le bon critère est
  l'irréversibilité — ce qui bloque la première ligne de code passe devant ce qui bloque le format
  d'une autre équipe, qui passe devant ce qui bloque un banc. Je m'attends à ce que l'ordre change,
  et notamment que « qui possède le harnais » remonte très haut : il conditionne H1, qui conditionne
  la première ligne du solveur.*
- [x] **P3** — dossier §1–2 : comment le lire, et le tableau de synthèse ordonné.
- [x] **P4** — dossier §3 : les fiches de rang 1 et 2 — ce qui bloque du code, ce qui bloque un
  format extérieur.
- [x] **P5** — dossier §4 : les fiches de rang 3 et 4 — données à obtenir, cadrages.
- [x] **P6** — dossier §5–6 : **ce que nous ne demandons pas** — la section qui évite les
  malentendus coûteux — et ce qui reste ouvert.
- [x] **P7** — index, angles morts, décomptes.
- [x] **P8** — rituel de fin (`REPRISE.md` §6) : journal S17, leçons, index, jeton libéré.

### Notes de reprise

- **Forme** : `docs/DOSSIER-REUNIONS.md`, à la racine de `docs/` et non dans un sous-dossier — c'est
  le seul document du corpus destiné à être **sorti du dépôt** et lu par quelqu'un qui n'y reviendra
  pas. Une fiche par destinataire, tenant seule, sans renvoi obligatoire.
- **Règle d'écriture propre à ce document** : chaque fiche porte **un chiffre**. Une demande sans
  chiffre se discute ; une demande avec un chiffre se traite. C'est L14 appliquée à une réunion.

#### P2 — le classement change, et il change beaucoup

`00_INDEX.md` classe en deux groupes — quatre interfaces, sept autres destinataires — et désigne le
terrain comme « le plus urgent des quatre ». C'est un classement par **gravité de conséquence**.

Le critère utile est autre : **qu'est-ce que la réponse débloque, et qu'est-ce qui devient
irréversible si elle tarde ?** Appliqué, il donne quatre rangs, et l'ordre n'est pas celui de
l'index.

**Rang 1 — bloque la première ligne de code.** Trois demandes, dont deux que l'index ne présentait
pas comme urgentes :

- **`int64` ou `f64` pour les positions monde** (ADR-002 §7.1). Décision partagée avec le réseau et
  la physique solide. Elle précède tout code qui manipule une position, c'est-à-dire tout code.
- **Qui possède le harnais de validation** (SPEC-003 §11.4). SPEC-003 §1 pose que « la qualité des
  décisions qui suivent est plafonnée par la sienne », et §10 que **H1 doit exister avant la première
  ligne du solveur**. Une question de propriété rangée jusqu'en S11 parmi des choix de format de
  fichier conditionne donc le premier livrable du chemin critique.
- **ADR-020 acté** — la bibliothèque sans dépendance moteur. Déjà signalé comme « bloquant, à acter
  avant la première ligne de code », mais absent de la liste des destinataires : personne n'est nommé
  pour l'acter.

**Rang 2 — bloque le format d'une autre équipe, et le retard se paie en migration.**

- **Le géoïde dans l'outil de terrain** — avant qu'un mètre carré de côte ne soit sculpté.
- **Les trois champs de `WaveEvent`** — avant que le réseau ne fige le format. Et l'audio doit donc
  répondre **avant** le réseau : deux destinataires, une seule échéance, ce que l'index ne dit pas.
- **Le signal de traversabilité** — avant que l'IA ne fige son format de maillage de navigation.
- **Le modèle du nageur** — avant que l'équipe personnage ne fige sa machine à états.

**Rang 3 — bloque un banc.** La table `a_max` par archétype (véhicules) conditionne B8 ; le modèle de
diffusion sous-marine (rendu) conditionne B11.

**Rang 4 — cadrages, sans échéance dure.** Air respirable, brèche vers le vide, équilibrage de `K`,
`V_min` et les TTL.

**Et les cinq arbitrages ne sont pas au même rang.** L'index les présente comme une liste homogène ;
ils ne le sont pas :

| Arbitrage | Ce que « oui » invalide | Rang |
|---|---|---|
| **Temps mis à l'échelle par joueur** | la cohérence multijoueur de B, donc ADR-009 en entier pour l'océan concerné, l'écume cohérente entre joueurs (ADR-014 §2.3) et l'autorité du signal de traversabilité (SPEC-006 §2.6) | **1** — cascade sur quatre documents |
| Propriété du harnais | rien, mais bloque H1 | **1** |
| Trait de côte mobile | le nombre d'états de la bibliothèque côtière et de la polyligne | 2 |
| Glace | deux champs de `TraversabilitySample` ; **coût désormais borné** — fetch max 3,4 km à 5 m/s | 3 |
| Durée de vie d'un nœud V | le volume de stockage du monde | 3 |

**L'arbitrage n°1 est le plus lourd du projet et il est présenté comme le premier d'une liste de
cinq.** Si la réponse est « oui », ce n'est pas un paramètre qui change : c'est le modèle de
réplication qui tombe pour l'océan concerné. Le dossier doit le dire ainsi.

#### P3 à P6 — dossier écrit d un tenant

`docs/DOSSIER-REUNIONS.md`, seize fiches et sept sections. Comme en S16, le découpage du plan s est
révélé artificiel : les fiches se tiennent par leur classement, et couper au milieu aurait produit
des commits illisibles seuls. Fait en un, déclaré ici.

**Trois choses que le dossier fait apparaître et que l index ne portait pas :**
- **quatorze demandes, pas onze.** Acter ADR-020 et figer `WaveEvent` après l audio sont deux
  demandes distinctes, adressées à deux destinataires que la liste ne nommait pas ;
- **les fiches 1 et 2 n ont pas de destinataire nommé** — « direction technique » et « assurance
  qualité technique » sont des rôles, pas des personnes. Personne n est identifié pour acter ADR-020
  ni pour arbitrer la propriété du harnais. C est la condition préalable à la tenue des réunions
  elles-mêmes, et cela n était écrit nulle part ;
- **une section « ce que nous ne demandons pas »**, sept lignes. Elle évite qu une équipe se croie
  sollicitée ou nous attribue une intention — en particulier « attendre que l eau soit finie pour
  commencer », que seules les quatre premières fiches justifieraient.

*Correctif P8.* Le contrôle des décomptes du rituel a attrapé `CLAUDE.md`, qui annonçait encore
« quatre interfaces et sept autres destinataires » et renvoyait au registre S11. C est le fichier
d amorce : une session neuve le lit en premier et aurait rappelé une liste périmée sans connaître le
dossier de réunion. Deuxième fois que ce contrôle attrape ce fichier précis — après S11.
