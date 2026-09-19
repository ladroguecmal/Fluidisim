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

Session : S299 — en cours : premier étage du pas GPU 3D résident (ADR-175 §4.2).
Agent : Claude Opus 5, application desktop ; fichiers, git, cargo, outils locaux, carte réelle.
Entrée : « Continue », 2026-09-19, après S298 close et jeton libre.
Carte constatée : NVIDIA GeForce RTX 5070 Laptop GPU, backend Dx12 — machine de référence
d'ADR-174 D1. Afficheur construit hors ligne, dépendances verrouillées S210/S211.

Capacité visée : le domaine 3D de δ vit **sur la carte** — tampons réservés à la création
(I-06), opérateur de pression appliqué sans matrice sur le GPU, projection à travail **borné**
(ADR-175 D2) — et son écart à la référence CPU de S297/S298 est mesuré, pas supposé.
Consommateur : la scène de mer étalée de la porte B, dont S298 a mesuré qu'elle ne peut pas
tenir sur le banc CPU, puis la revue utilisateur (ADR-175 §4.3).

Critères avant code, posés ici :
- Aucune allocation ni travail en `O(N)` sur CPU pendant le pas ; **aucune lecture synchrone**
  (SPEC-004 §8.4). Les diagnostics se relisent en différé, avec leur âge.
- L'action de l'opérateur assemblé sur la carte se compare à `apply_mobile3` du cœur sur le
  même état et le même champ : écart publié, **aucune identité au bit exigée** (ADR-175 D4).
- La projection fait un nombre de cycles **fixé par le profil**, jamais une boucle jusqu'à
  convergence dans la boucle d'image.
- Au-dessus de la tolérance d'ADR-144 (`10⁻⁵`, aucun nombre nouveau), le pas est **déclaré
  dégradé** ; il n'est ni refusé ni refait.
- L'écart à la référence se publie en hauteur et en pente, contre le repère de 3 mm de S201.
- Aucun état δ sérialisé (I-17), aucune grandeur de jeu issue de δ (I-04, I-15).
- La référence CPU ne bouge pas : elle est l'instrument, pas le sujet.

### Plan

- [x] **P1** — amorce, lecture ciblée du lot, carte constatée ; plan seul.
- [x] **P2** — `viewer/src/delta3d.rs` et son WGSL : domaine 3D résident, tampons réservés à la
  création, géométrie téléversée une fois ; noyau de l'opérateur sans matrice.
- [>] **P3** — recevoir l'action de l'opérateur contre `apply_mobile3` : même état, même champ
  d'entrée, écarts publiés ; refus et réserve testés.
- [ ] **P4** — projection bornée résidente : cycles fixés, aucun retour CPU entre itérations,
  aucune lecture synchrone ; pression comparée à la référence.
- [ ] **P5** — diagnostics par réduction sur la carte : divergence projetée des lignes franches
  et dérive de masse, relus en différé avec leur âge ; dégradation déclarée.
- [ ] **P6** — coût du pas borné sur le poste de référence, et écart à la référence en hauteur
  et pente contre les 3 mm de S201.
- [ ] **P7** — preuve et rituel REPRISE §6 : file, feuille de route, index, journal, jeton libre.

### Notes de reprise

Le cœur n'exporte **aucune** ligne d'opérateur en 3D : `apply_mobile3` est sans matrice, six
voisins par maille via `mobile_row(i,j,k)` qui rend `(voisin, a, valeur)` — `a` le coefficient
fantôme, `valeur` le fantôme lui-même. La production doit donc porter cette règle, pas lire des
lignes. ADR-172 (export de lignes) ne vaut plus que pour les essais 2D (ADR-175 §3).

`viewer/src/pressure_solver.rs` porte un CG résident **2D** (S289) qui prend des lignes du cœur :
il sert de modèle d'ordonnancement GPU, pas de code à réutiliser tel quel.
Aucune 3D n'existe côté afficheur : `grep Domain3 viewer/src` est vide.

Banc 2D disponible pour comparer les ordres de grandeur : `--pression-gpu` sur 31×19 donne
gpu_mediane 2,2 µs à 0 lissage, 67 µs à 32 lissages, premier passage 4,94 ms.

P2 : l'opérateur ne dépend que de la **géométrie** — `a = 1/θ` dans les deux branches de
`ghost_up3`/`ghost_side3`, `homogeneous_ghost` ne change que la *valeur*, pas le coefficient.
Le noyau n'a donc besoin que des hauteurs de colonne, de `dx` et de `SURFACE_THETA_MIN` (1e-3).
Ordre d'accumulation x−, x+, y−, y+, z−, z+ respecté : l'addition f32 n'est pas associative.
z− n'est jamais testé mouillé (une maille sous une mouillée l'est), le fond est un mur.
wgpu 30 : `PollType::wait_indefinitely()` et `get_mapped_range()` rend un `Result`.
Banc P2 : 1920 mailles, sèches exactement nulles, max|A·p| 1,817143e2, tout fini.
