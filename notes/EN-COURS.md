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

Session : S560 — **en cours**. En autonomie, **5.7 — le débit par couches** (ADR-241 D4), après la pression d'un nœud stratifié (S559).

**Ce que la session fait.** `step_liquids(…, composition, liquides, pluie)` : (a) un orifice ou une vanne débite sur la différence de
**pression** au seuil, `Q = C_d·A·√(2Δp/ρ)`, `ρ` le liquide de la couche amont au seuil (la condition `h_amont > h_aval` des surfaces ne
vaut plus : un côté chargé d'huile a sa surface plus haute à l'équilibre) ; (b) après le pas, la composition suit les transferts entiers :
chaque arête prend dans la couche à son seuil (le volume sous le plan du seuil, comparé aux volumes cumulés), puis au-dessus, puis
au-dessous ; les débordements, en dernier, la couche du dessus ; la pluie apporte le liquide `pluie`. Formes volumiques seulement (les
anciennes tables +Z n'ont pas de volume sous un plan) ; sans air scellé dans cette version. Le pas sans composition reste celui d'avant.

**Références, calculées avant.** *Manomètre en U* : deux cuves de 1 × 1 × 2 m reliées au fond par deux orifices (un par sens, 1 000 mm²,
`C_d` = 0,62) ; à gauche 1,5 m³ d'eau, à droite 0,5 m³ d'eau sous 0,4 m³ d'huile (ρ = 850). Équilibre `1000·h_g = 1000·h_d + 850·0,4`,
`h_g + h_d = 2` → **`h_g` = 1,17 m, `h_d` = 0,83 m**, la surface de droite à 1,23 m — plus haute que celle de gauche. La constante de temps
(ADR-240 D2) : `d√Δ/dt = −C_d·a·√(2g)/A`, Δ₀ = 0,66 m → **295 s** ; l'essai dure 600 s. Le dépassement du pas explicite près de
l'équilibre, `(2·C_d·a·√(2g)·dt/A)²` = 3·10⁻⁷ m, sous le quantum. *Vidange stratifiée* : une cuve de 1 m², 0,5 m³ d'eau sous 0,5 m³
d'huile, un orifice au fond vers dehors : l'eau sort seule, sous la charge `h_e + 0,425` ; `t_e = 2·(√0,925 − √0,425)/(C_d·a·√(2g))` =
**225,7 s**, puis l'huile.

**Quantum** (ADR-236 D1) : 1 ml sur 1 m², 1 µm de hauteur ; la durée, le pas de 0,1 s sur 225 s (4,4·10⁻⁴).

**Critères, écrits avant.** (1) Manomètre : `h_g` à 10⁻⁴ m de 1,17 m (rapport 100) ; l'huile reste entière à droite (400 000 ml, exact) ;
chaque liquide conservé à l'entier. (2) Vidange : l'huile intacte, au millilitre, tant que l'eau n'est pas épuisée ; l'eau épuisée à 0,5 %
de 225,7 s (rapport 11). (3) Un seul liquide : les volumes de `step_liquids` à 2 ml de ceux de `step` à chaque pas de la vidange d'eau
seule. (4) Refus : une table +Z, une composition qui ne somme pas.

### Plan

- [x] **P1** — jeton ; plan.
- [ ] **P2** — `step_liquids` et ses essais ; (1)–(4).
- [ ] **P3** — preuve ; liste 5.7 ; rituel.

### Notes de reprise
