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

Session : S559 — **en cours**. En autonomie, **5.7 — plusieurs liquides** (absent ; A17) : **ADR-241** (non miscibles, en couches ; l'état
entier, une composition parallèle ; la pression par couches ; le débit à la session suivante), puis sa première pièce — **la pression en un
point d'un nœud stratifié** (`hydro_liquids.rs`, sous-module de V).

**Ce que la session fait.** `Liquid { density_kg_m3 }` ; `pressure_at(nœud, composition, liquides, formes, g_eff, point)` : les couches
rangées par densité (stable), chaque interface le plan de la géométrie pour le volume cumulé, la somme des `ρᵢ·|g|·épaisseurᵢ` au-dessus du
point. Refus : composition qui ne somme pas au volume, densité non positive, plus de 8 liquides.

**Références, calculées avant** (ADR-239 D1 : des formes fermées indépendantes de la géométrie du code). (1) Cuve droite 4 × 1 × 2 m,
eau 4 m³ (1 m) sous huile 2 m³ (0,5 m, ρ = 850) : au fond `9,81·(1000·1 + 850·0,5)` = **13 979,25 Pa** ; à 1,2 m, `9,81·850·0,3` =
2 501,55 Pa. (2) La même sous `g_eff = (1 ; 0 ; −9,759)` : chaque interface passe par la colonne centrale à la hauteur `V/A` (le plan ne
touche ni le fond ni le couvercle : pente 0,1025, demi-largeur 2 m → 0,205 m de dénivelé, sous les 0,5 m d'huile et au-dessus du fond),
d'où la pression au coin bas `x = −2 m` par les distances le long de la verticale. (3) La carène en V des essais de géométrie (section
`|x| ≤ z`, 1 m de long : `V(h) = h²`) : eau 1 m³ (`h` = 1 m) sous huile 0,69 m³ (`h` = 1,3 m) → à la quille `9,81·(1000 + 850·0,3)` =
**12 311,55 Pa**. (4) Un seul liquide : `ρ·|g|·(surface − z)`, la surface du pas présent.

**Quantum** (ADR-236 D1) : l'inversion géométrique tient le demi-millilitre ; sur 4 m² de section, 0,125 µm de hauteur, soit
≈ 1,2·10⁻³ Pa sur 1,4·10⁴ — 10⁻⁷ relatif. **Critères, écrits avant** : (1)–(4) à 10⁻⁵ relatif (rapport 100) ; (5) les refus ; (6) l'ordre
des couches ne dépend pas de l'ordre de la table (l'huile déclarée avant l'eau).

### Plan

- [x] **P1** — jeton ; ADR-241 ; plan.
- [ ] **P2** — `hydro_liquids.rs` et ses essais ; (1)–(6).
- [ ] **P3** — preuve ; liste 5.7 ; A17 ; rituel.

### Notes de reprise
