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

Session : S664 — **terminée**. En autonomie vers la v2 ; 2.7, « les chemins de B » et la bathymétrie 2D qu'ADR-196 §3 laissait au lot 2D.
R41 reçu (« Je valide, continue »).

**Ce que la session fait.** `Cote2D` (`bathymetrie_cote2d.rs`) : la côte cuite **en 2D** par le modèle de pente douce à grand angle
(S660), lue par B comme la côte 1D de S364 — chaque composante garde sa pulsation et sa phase temporelle entière, et reçoit, par nœud
d'une grille (`s` le long de la normale vers la côte, `n` le long de la côte), une **correction de phase** entière (Q32), un **facteur
d'amplitude** `|A|`, son **vecteur d'onde local** (le gradient de la phase totale) et `coth(kh)`. Au large de la côte cuite (`s ≤ 0`),
l'évaluation est celle de B, **au bit** (ADR-196 D1). La marche suit la **normale à la côte** pour toutes les composantes (une rangée suit
une isobathe : `k̄` y est le `k` local) ; une composante oblique entre par `A(0, n) = e^(i·k₀·sin θ·n)` ; la correction vaut
`ψ(s) + arg A − k₀·(s·cos θ + n·sin θ)`. Linéaire (la dispersion d'amplitude de S662 ne se superpose pas entre composantes).

**Contrôles du plan** (ADR-266, ADR-267)

- **témoin** : sans objet au départ.
- **instrument** : la côte 1D de S364 (`Cote`, WKB exact sur isobathes droites), sur sa propre plage (1:50, de 80 à 2 m sur 3900 m),
  une houle de 10 s, 1 m, à 30°. Ce qui départagerait : une `Cote2D` juste rend le facteur à 2 % et la correction à quelques degrés de
  `Cote` ; une faute de convention de phase rend un décalage dès le bord du large (`s` petit, où la correction doit être nulle) ; une
  marge trop étroite rend des oscillations latérales (le facteur lu à `n` = ±largeur/2 différent de `n` = 0).
- **calcul** (ce script) : la marge `L·tan θ` = 2252 m ; la marche 1951 × 2353 (73 Mo, asserté sous 400) ;
  l'erreur de phase de Padé attendue sur le chemin, ≤ 7.1° (asserté sous 15°).
- **ADR** : ADR-196 (D1 au bit au large, D3 le bord du large à λ₀ — ici 80 m pour λ₀ = 156 m, soit λ₀/2 : la plage de S364 elle-même),
  ADR-264 (le calcul en grille locale), ADR-260 (le module rangé).
- **pièges** : les parois latérales de la marche (la marge) ; `k̄` sur une rangée oblique aux isobathes (la marche le long de la normale) ;
  la limite de Padé au-delà de 45° (une composante plus oblique est refusée) ; l'interpolation de la phase entière (moins d'un demi-tour
  entre nœuds, refusé sinon) ; l'eau profonde des composantes (refusée sinon, comme `Cote`).

**Critères, écrits avant.** (1) Au large (`s ≤ 0`), l'évaluation de B **au bit**. (2) Sur la plage de S364, au centre (`n` = 0), le long
du profil : le facteur à **2 %** de `Cote`, la correction de phase à **15°** ; et au bord de la largeur (`n` = ±100 m), le facteur à 1 % de
celui du centre (la marge tient). (3) Refus : une composante à plus de 45°, une profondeur non positive, une géométrie invalide.

### Plan

- [x] **P1** — jeton ; plan.
- [x] **P2** — `Cote2D` et ses essais ; (1)–(3).
- [x] **P3** — preuve ; liste 2.7 ; rituel.

### Notes de reprise
- **P2 fini** — (1) au bit ; (2) la phase 2,93° (tenu), le facteur 6,6 % et le bord 15 % (manqués) — démêlés : la normalisation au départ
  (A = 1 où la levée vaut 0,990), les parois (le témoin de la marge), `K_r` (deux corrections rejetées : l'une instable, l'autre empire
  l'eau mince ; pente_douce.rs remis à S662) ; (3) tenu.
