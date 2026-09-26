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

Session : S386 — **terminée**. **C2** de la campagne ([ADR-207](../docs/adr/ADR-207-la-campagne-du-solveur-volumique-3d.md) D2,
D5) : **les colonnes hautes dans la référence**. Demande de l'utilisateur (2026-09-26) : *« Réalise la suite »* ; et sa
décision, à consigner : *« Je suis d'accord avec toi pour le branchement à la fin »* — δ dans Godot en C11. Agent : Claude
Opus 5.5, session cloud Claude Code ; fichiers, git, cargo, Python ; ni carte graphique, ni Godot.

**Thèse.** Une colonne haute est un **sous-espace** de la grille fine : sous les `n − m` couches cubiques du haut, les `m`
couches du bas n'ont plus qu'une pression **linéaire** en `z` (deux inconnues par colonne au lieu de `m`). Écrite comme une
**restriction de Galerkin** du schéma reçu (`Aᵣ = Pᵀ A P`), elle reste symétrique définie positive, conserve la masse de la
colonne, et sa dispersion **se calcule** colonne par colonne comme `scheme_frequency` (S295) le fait pour la grille fine :
on sait donc, **avant** de l'écrire en 3D, combien de couches cubiques il faut garder. Mode **linéaire** d'abord, là où vit
la réception de dispersion ; le pas mobile et le stockage compact viennent ensuite (C2b).

**Critères, écrits avant.** (1) Le calcul de dispersion redonne `scheme_frequency` à `m ≤ 1` (Ω/ω − 1 = −1,435·10⁻² et
−3,683·10⁻³ pour les cas de S295). (2) **Précision rapportée à l'usage** : sur la configuration de la porte B (profondeur 7 m,
`dx` 25 cm) et pour toute longueur d'onde de `4·dx` à `2·profondeur`, l'erreur de fréquence **ajoutée** par les colonnes
hautes `|Ω_haut/Ω_fin − 1|` ne dépasse pas **max(|Ω_fin/ω − 1|, 10⁻⁴)** — pas plus que la maille n'en fait déjà, ou une phase
de 10⁻⁴, soit ≈ 3 mm sur une vague de 0,5 m après une minute (METHODE : 3 mm) ; le plus petit nombre de couches cubiques qui
le tient est **consigné**. (3) En 3D, mode linéaire : opérateur réduit symétrique (10⁻⁵) et positif ; repos exact ; volume
conservé à l'arrondi. (4) L'onde oblique de S295 avec colonnes hautes suit **la fréquence calculée de son propre schéma** à
10⁻³ près, comme S295 le fait pour la grille fine. (5) Sans colonnes hautes, suite entière inchangée, zéro avertissement.

### Plan

- [x] **P1** — jeton, plan seul.
- [x] **P2** — décision de l'utilisateur consignée (δ dans Godot en C11) : file, ADR-207 (note datée), conception §6.
- [x] **P3** — la dérivation, et `outils/colonnes_hautes.py` : la dispersion d'une colonne à colonne haute ; critère 1 ; essai.
- [x] **P4** — balayage : Ω/ω selon les couches cubiques gardées, porte B et S295 ; critère 2 ; `k` minimal consigné.
- [x] **P4b** — *ajoutée après P4* : [ADR-208], la colonne graduée (variante N) à la place de la colonne haute unique
  d'ADR-207 D2 ; ADR-207 (note datée), conception §3.3.
- [x] **P5a** — `Volume3` : la **colonne graduée** du mode linéaire (variante N : restriction et prolongation linéaires par
  morceaux, gradient conjugué réduit) ; critère 3.
- [x] **P5b** — l'onde oblique à colonne graduée contre sa fréquence calculée ; critère 4 ; inconnues comptées.
- [x] **P6** — critère 5 ; preuve `COLONNES-HAUTES-S386` ; liste, file, feuille de route, index.
- [x] **P7** — rituel.

### Notes de reprise

**P3 — l'outil** `outils/colonnes_hautes.py` (+ `test_colonnes_hautes.py`, six essais) : la dispersion d'une colonne sous
une restriction de Galerkin `Aᵣ = (PᵀDQ)M̂⁻¹(PᵀDQ)ᵀ`. **Critère 1 tenu** : le schéma fin redonne S295 (−1,4355·10⁻², −3,6827·10⁻³).
Propriétés vérifiées : `m = 1` et « un nœud par maille » redonnent le schéma fin ; Galerkin ne baisse jamais `s` (Ritz) ;
opérateurs réduits symétriques. Variantes : **G** (colonne haute linéaire, vitesses libres), **Q** (vitesse verticale
interne liée), **E** (grille étirée en volumes finis), **N** (Galerkin linéaire par morceaux, nœuds étirés).

**P4 — le balayage** (`python outils/colonnes_hautes.py`, 42 s ; porte B : `h` = 7 m, `dx` = 25 cm, 28 couches, `dt` = 1/30 s ;
λ de 1 à 14 m) — le moins d'inconnues par colonne qui tient le critère 2 :

| variante | réglage | inconnues | division | rapport au critère | pire λ |
|---|---|---:|---:|---:|---:|
| G, colonne haute linéaire | 17 cubiques + 1 | 19 | ÷1,47 | 0,834 | 14 m |
| Q, vitesse verticale liée | 16 + 1 | 18 | ÷1,56 | 0,629 | 14 m |
| E, grille étirée (VF) | r = 1,2, 5 cubiques | 14 | ÷2,00 | 0,966 | 14 m |
| **N, linéaire par morceaux** | **r = 1,25, 3 cubiques** | **11** | **÷2,55** | 0,923 | 14 m |

Bassin de 3 m à 10 cm (30 couches) : N 14 inconnues (÷2,14 ; r = 1,25, 6 cubiques), G 22 (÷1,36). **Critère 2** : tenu par
chaque variante à son réglage ; **l'estimation de S384 (÷3,1, huit couches cubiques et une colonne haute) est fausse** —
avec huit couches cubiques, G ajoute jusqu'à 1,4·10⁻² d'erreur de fréquence à λ = 14 m (≈ 16 fois le permis). Le pire cas
est toujours la plus longue vague, où le schéma fin est le plus juste (8,6·10⁻⁴). Conséquence : **ADR-207 D2 change** —
P4b ajoutée ; P5 construit N.

**P5a — la colonne graduée en 3D, mode linéaire** (`delta3d_graded.rs`) : `enable_graded(host, nœuds)`, prolongation `P`
et transposée exacte `Pᵀ` (mêmes coefficients), gradient conjugué réduit sur `Pᵀ·A·P` par la grille fine, arrêt 10⁻¹² puis
tolérance d'ADR-144 sur `Pᵀ·div u` rapportée au poids de chaque nœud ; refus `Domain` sur fond coupé et au pas mobile.
**Critère 3 tenu** (cinq essais `s386`) : comptage exact, nœuds invalides refusés ; opérateur réduit symétrique et positif,
`Pᵀ` transposée de `P` ; repos exact au bit ; volume gardé à 10⁻⁹ m³ sur 50 pas ; tous les nœuds = schéma fin à 10⁻⁶ m ;
refus du pas mobile, surface rendue au bit. **Vu échouer** : la transposée faussée (nœud du haut à moitié) casse l'essai de
symétrie.

**P5b — critère 4 tenu** (`graded_oblique_wave_follows_its_own_dispersion_s386`) : le mode (1, 1) de S295 sur colonne
graduée suit la fréquence calculée de son schéma à **0,0046 %** (n = 16, 5 nœuds sur 8 couches) et **0,0048 %** (n = 32,
7 sur 16) — comme la grille fine de S295 (0,0046 / 0,0047 %). Ω_gradué/Ω_fin − 1 = 1,808·10⁻³ et 2,020·10⁻³, identiques à
l'outil Python ; Ω_gradué/ω − 1 = −1,257·10⁻² et −1,670·10⁻³ (fin : −1,435·10⁻², −3,683·10⁻³). Inconnues par colonne :
`pressure_unknowns_per_column` ; nœuds de la porte B : [0, 6, 11, 15, 18, 20, 22, 24, 25, 26, 27], 11 sur 28.
