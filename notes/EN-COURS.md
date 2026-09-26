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
- **Ce fichier ne porte que la session en cours** ([ADR-187](../docs/adr/ADR-187-methode-refondue-s321.md)
  D3). À la clôture, ce qui doit survivre des notes va à la preuve ou au journal ; la session
  suivante remplace ensuite toute la section. Aucune section d'archive, 300 lignes au plus :
  `outils/etat_projet.py --check` le vérifie. Notes de S301 à S320 : `git show 78622a19:notes/EN-COURS.md`.

---

## Session en cours

Session : S387 — **en cours**. **C2b** de la campagne ([ADR-208](../docs/adr/ADR-208-la-colonne-graduee.md) D4, ADR-207 D5) :
**la colonne graduée au pas mobile**. Demande de l'utilisateur (2026-09-26) : *« Continue »*. Agent : Claude Opus 5.5, session
cloud Claude Code ; fichiers, git, cargo, Python ; ni carte graphique, ni Godot.

**Thèse.** Au pas mobile, les couches cubiques doivent contenir **toute la course de la surface** (ADR-208 D4). Or la scène de
la porte B n'a que **3,5 m d'eau sous 3,5 m d'air** (28 couches de 25 cm) sous une mer de `Hs` ≈ 2,5 m (`--houle` : vent 1,5 m
à 6 s, houle 2 m à 12 s) : **le balayage de S386, étiqueté « porte B », supposait 7 m d'eau sous une surface fixe** — une
erreur d'étiquette à corriger d'abord. Ensuite, **mesurer** la course réelle de la surface dans cette scène (B évalué sur le
domaine), et en tirer où la colonne graduée paie : la haute mer, ou l'eau calme des contenants. Puis la construire au pas
mobile, sur la grille fine, comme au pas linéaire.

**Critères, écrits avant.** (1) L'étiquette corrigée partout où « porte B » désigne le cas de 7 m (preuve, ADR-208, liste,
file, feuille de route). (2) La course de la surface mesurée sur la scène de la porte B — `B` seul sur dix minutes, puis
avec un repère qui suit la moyenne de `B` sur le domaine — et le nombre de couches cubiques qu'elle impose, **publiés avec
le gain qui en reste**. (3) Au pas mobile, colonne graduée : tous les nœuds redonnent le pas mobile fin à 10⁻⁶ m ; volume
conservé à l'arrondi ; une surface qui entre dans la partie graduée est **refusée**, état rendu au bit ; un ballottement suit
le pas fin à l'écart que la dispersion calculée prévoit. (4) Suite entière inchangée sans colonne graduée, zéro avertissement.

### Plan

- [x] **P1** — jeton, plan seul.
- [x] **P2** — l'étiquette corrigée (critère 1).
- [x] **P3** — la course de la surface sous la mer de la porte B : `examples/delta3d_course_surface.rs` (critère 2) ; les
  couches cubiques qu'elle impose, le gain qui reste (outil) ; le cas d'un bassin calme.
- [ ] **P4** — la colonne graduée au pas mobile (`project_mobile3`) : garde de la course, départ chaud par injection aux
  nœuds, divergence restreinte ; essais (critère 3).
- [ ] **P5** — le ballottement gradué contre le fin ; critère 4.
- [ ] **P6** — preuve (section datée de COLONNES-HAUTES-S386) ; ADR-208 (note), liste, file, feuille de route.
- [ ] **P7** — rituel.

### Notes de reprise

**P3 — la course de la surface** (`delta3d_course_surface`, 10 s ; mer `--houle` reconstruite, domaine de la porte B, 840
points, 0,1 s, 600 s) :

| lecture | min | max | course | 0,1 % / 99,9 % | couches cubiques, paquet compris |
|---|---:|---:|---:|---|---:|
| `B` | −2,487 m | +2,299 m | **4,79 m** | −1,89 / +1,83 | **26 sur 28** |
| `B` moins sa moyenne sur le domaine | −1,674 | +1,716 | 3,39 m | −1,14 / +1,14 | 20 sur 28 |

**La colonne graduée ne paie pas en haute mer** dans un repère fixe : 26 couches cubiques au moins, deux nœuds au mieux
dessous — rien à gagner ; un repère qui suivrait la moyenne de `B` en laisserait 20 (÷1,27 au plus). **Fait vu en passant** :
sur dix minutes, le creux de `B` (−2,49 m) plus le paquet (0,65 m) descend à 0,11 m du fond du domaine (3,5 m d'eau) — la
scène de la porte B n'a été éprouvée que sur des durées courtes ; à consigner, pas à régler ici.

**En eau calme** (bassin de 3 m, course supposée ±0,5 m — *hypothèse*, non mesurée) : la **dispersion** fixe les couches
cubiques, pas la course — 10 cm : 14 inconnues sur 30 (÷2,14 ; `r` 1,25, 6 cubiques) ; 5 cm : **24 sur 60 (÷2,50** ; `r` 1,2,
11 cubiques). La colonne graduée sert donc les **contenants** (ADR-200, ADR-202) — le pas mobile de la piscine —, pas la
haute mer.
