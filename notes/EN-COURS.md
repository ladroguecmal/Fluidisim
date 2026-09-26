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

Session : S396 — **terminée**. **C8a** de la campagne ([ADR-207](../docs/adr/ADR-207-la-campagne-du-solveur-volumique-3d.md) D5),
en référence : **la fusion et la séparation de domaines**, par ensembles de blocs ([ADR-006](../docs/adr/ADR-006-cellules-domaines-solveurs.md)
§3–4). Demande de l'utilisateur (2026-09-26) : *« continue »* ; S395 a désigné C8 (S294 : pas de troisième session de suite
sur le raccord). Agent : Claude Opus 5.5, session cloud Claude Code ; fichiers, git, cargo, Python ; ni carte graphique, ni
Godot. Sert **4.9** (absent), 1.5, 1.6.

**Thèse.** ADR-006 §3 : un domaine est un **ensemble de blocs** d'un réseau commun ; **fusion = union, séparation =
partition**, sans remaillage ni interpolation, si `dx` et le repère coïncident. §4 : fusion quand les ensembles dilatés du
rayon de couplage `r_c = λ_cut` (4 m, ADR-005 : deux blocs de 8 mailles à 25 cm) se touchent ; séparation quand la partition
en composantes connexes des blocs dilatés en compte plusieurs **pendant plus de 1,0 s** ; durée de vie minimale 0,75 s.
**Simplification déclarée** : les domaines de δ couvrent toute la profondeur (ADR-175) — un bloc est une colonne de 8 × 8
mailles. Dans la référence CPU, l'état d'un domaine vit dans la boîte qui l'enveloppe ; **l'état de l'union est celui des
parties, recopié au bit, le reste au repos** ; une séparation pose des murs sur la coupure — ce qu'elle perd est le débit à
travers la coupure, mesurable. Aucune rupture si la fusion survient avant que les ondes n'atteignent les murs des parties.

**Critères, écrits avant.** (1) **Ensembles** (essais) : union ; deux blocs connexes si leurs dilatations se touchent
(Chebyshev ≤ `2r + 1`) — la même relation pour fusionner et pour séparer, sans quoi une fusion se déferait aussitôt ; une
séparation seulement après 1,0 s continue à deux composantes, jamais avant 0,75 s de vie ; une composante qui clignote à
moins d'une seconde ne sépare jamais. (2) **L'état** : aller-retour `C → (A, B) → C'` **au bit** partout sauf les faces de
la coupure ; leur débit perdu publié. (3) **Sans rupture** : deux bosses, deux domaines ; la fusion à l'instant où leurs
ensembles actifs (colonnes à plus de 1 mm) dilatés se touchent ; puis 5 s ; écart de surface au domaine unique tenu depuis le
départ **≤ 1 % de l'amplitude**. Publié : fusions plus tardives, et une séparation. (4) Suite, zéro avertissement.

### Plan

- [x] **P1** — jeton, plan seul.
- [x] **P2** — `domain_blocks.rs` : ensembles de blocs, dilatation, composantes, critères de fusion et de séparation avec leurs
  délais ; essais ; critère 1.
- [x] **P3** — `Volume3::transplant` : l'état d'un domaine recopié dans un autre sur le réseau commun ; l'aller-retour ;
  critère 2.
- [x] **P4** — l'exemple `delta3d_fusion` : deux bosses, la fusion au critère, contre le domaine unique ; critère 3 ; publiés.
- [x] **P5** — critère 4 ; preuve `FUSION-S396` ; liste, file, feuille de route, index.
- [x] **P6** — rituel.

### Notes de reprise
**P2 — `domain_blocks.rs`** : `Block`, `BlockSet` (trié, capacité réservée, `insert`, `union_with`, `bounds`, `touches`,
`components` par union-find sans allocation), `SplitClock`. **Critère 1 tenu** : quatre essais — tri et capacité ; la fusion
est l'union et la même relation sépare (écarts 4 à 7 blocs au rayon 2 : liés jusqu'à 5) ; composantes numérotées ; une
seconde continue, un clignotement de 0,9 s / 0,1 s pendant 10 s ne sépare jamais. **Vu échouer** : liaison à `2r` au lieu
de `2r + 1`, deux essais tombent. **Précision du plan** : la durée de vie minimale de 0,75 s (ADR-006 §4) règle
l'**extinction**, qui est à l'ordonnanceur ; ici, une fusion fait naître un domaine dont l'horloge repart de zéro — il vit au
moins 1,0 s avant de se séparer.

**P3 — `delta3d_regions.rs`** : `Volume3::clear_to_rest`, `Volume3::transplant(src, offset)` — surface, reste compensé, trois
vitesses, pression de départ ; refus `Domain` hors du réseau commun (dx, nz, repos, densité, gravité) ou avec découpe ; les
murs du receveur restent nuls (leçon de S350). **Critère 2 tenu** : 32 × 16 à 25 cm après 20 pas, `C → (A, B) → C'` **au bit**
hors de la coupure ; la coupure (x = 4 m) perd au plus **7,8 mm/s**, 0,058 m³/s — l'onde de la bosse l'a déjà atteinte en
0,4 s. La coupure rendue, `C'` refait vingt pas **au bit** avec `C`, mêmes itérations : la recopie porte tout ce que le pas
lit. **Vu échouer** sans la pression de départ (deux essais).

**P4 — `delta3d_fusion`** (43 s par cas). **Fusion** (32 m, bosses de 5 cm à 5 et 27 m ; A sur [0, 14 m), B sur [18, 32 m)) :
le critère fusionne à **0,94 s** ; écart au domaine unique avant la fusion 0,16 % de l'amplitude, à la fusion 0,19 %, sur les 5 s
suivantes au plus **0,26 %** — **critère 3 tenu**. Témoins : fusion forcée à 0 s, **au bit** ; à 2 s, les ondes ayant frappé les
murs de A et B, **3,4 %** ; à 3 s, **8,5 %** — l'instant du critère compte. **Séparation** (40 m, bosses à 5 et 31 m) : deux
composantes, due à **1,00 s**, coupure à 18 m ; écart par seconde 0,00 / 0,03 / 0,32 / 1,45 / 2,61 % — rien ne saute à la
coupure, puis les ondes se réfléchissent sur les nouveaux murs (la limite des murs, publiée). **Trouvé en chemin** : au premier
passage, bosses à 5 et 35 m, l'écart était nul sur 5 s — symétriques autour de la coupure à 20 m, qui était un plan de
symétrie où un mur ne change rien ; témoin sans valeur, remplacé par le cas asymétrique.

**P5 — critère 4 tenu** : 691 réussis, 18 ignorés, zéro avertissement. Preuve [FUSION-S396](../docs/validation/FUSION-S396.md) ;
**4.9 passe à partiel** (décompte 3 / 70 / 47) ; 4.2 ; ADR-006 (note datée : trois lectures de §3–4) ; file, feuille de route
(S394–S396), index.
