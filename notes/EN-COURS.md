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

Session : S637 — **en cours**. En autonomie (ADR-247 : la physique des partiels). **5.10 — l'articulation V↔δ** : un manque nommé, « V qui
déclenche δ » ; et la mécanique d'ADR-025 §3 — l'amorçage au niveau du nœud, la relaxation de la masse de δ vers celle du nœud.

**Ce que la session fait.** Un module `articulation.rs` : `amorcer(grille, niveau)` — δ (`SaintVenant2D`) né au niveau de V ; `forcer(δ, volume du
nœud, dt_V, τ)` — la correction `(M_nœud − M_δ)·dt_V/τ`, une couche uniforme sur les mailles mouillées (le relief de la surface intact) ;
`declenche(nœud, publié, formes, g, seuil)` — le seuil de S564 en hauteur de surface. δ n'écrit jamais dans V (C21 : aucune fonction ne le
permet). Ne fait pas : un bassin quelconque (ici à fond plat : niveau = volume/aire), la dérive d'un solveur réel, la destruction de δ quand V
se calme.

**Références, calculées avant** (ce script). La piscine de S564 (50 m², 1 m), un robinet de l'hôte de 2 L par pas de V (10 Hz) : la surface
monte de **40.0 µm** par pas ; au seuil de 500 µm, V déclenche δ au pas **13**, au niveau **1.000520000 m**. Après, 300 pas :
le retard de masse `V − M_δ`, de 0.001800000 m³ au premier pas, s'établit à **0.018000000 m³** — `Q·(τ − dt)`, exact en pas discrets ;
au niveau, 360.0 µm (ADR-025 §3 : sous le perceptible). **Bornes du montage** (ADR-257 D1, assertées) : 300 pas établissent le
régime (`(1 − dt/τ)³⁰⁰` = 2·10⁻¹⁴) ; la piscine ne déborde pas.

**Quantum** : le ml (V) ; f64 (δ). **Critères, écrits avant.** (1) le déclenchement au pas 13, pas avant ; (2) δ amorcé au niveau du nœud
à 10⁻¹² m ; (3) à chaque pas, le retard de masse égal à la référence à 10⁻⁹ m³ (la masse de δ ne bouge que par le forçage : murs, aucun
flux) ; établi à `Q·(τ − dt)` ; (4) la surface de δ reste plate : vitesse sous 10⁻¹² m/s ; (5) V ne reçoit que le robinet : son volume
exact au ml (assemblage, C21) ; (6) refus : `τ` ou `dt_V` non positifs.

### Plan

- [x] **P1** — jeton ; plan.
- [ ] **P2** — `articulation.rs` et son essai ; (1)–(6).
- [ ] **P3** — preuve ; liste 5.10 ; rituel.

### Notes de reprise
