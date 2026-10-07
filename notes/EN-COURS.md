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

Session : S603 — **en cours**. En autonomie (ADR-247) : **12.5 — la portée d'une modification bornée par partition** (SPEC-005 §8 ; absent).
La ligne qu'on sous-estime : **la bathymétrie**, dont la portée va jusqu'à l'isobathe où la plus longue houle cesse de sentir le fond —
`h = λ` depuis ADR-196 D3 (et non `λ/2`).

**Ce que la session fait.** Un module `portee.rs` : `isobathe_limite_m(T, g)` = `λ` à `h = λ` (`L₀·tanh 2π`) ; `celerite(h, T, g)` — la
célérité de phase de la houle et sa dérivée en `h` (dispersion complète, Newton) ; le tracé d'un faisceau de rayons de houle par
`refraction::tracer` (une profondeur équivalente `c²/g`) jusqu'à l'isobathe d'arrivée ; `portee_bathymetrie(scène, ancien, nouveau,
support)` — **les plages à recuire** : aucune si le support reste plus profond que l'isobathe limite ; sinon les plages des rayons qui
passent sur le support, avant et après. Ne fait pas : les quatre autres lignes de la table (contenant, nœud, tronçon, trait de côte), un
trait de côte quelconque (ici une côte droite et des plages en intervalles de `y`), le branchement à `cotier::Bibliotheque`.

**Scène, et références calculées avant** (ce script les calcule par un traceur indépendant, numpy vectorisé). Houle de 8 s, plateau à
1:200, 363 rayons partis à 24 km (121 départs tous les 200 m, trois directions : π, π ± 0,35), arrivée à l'isobathe 5 m, pas de 2 s, huit
plages de 2 km. L'isobathe limite : **99.923142536 m** (à 20 km du rivage ; `K_s − 1` y vaut −4,0·10⁻⁵, ADR-196). Trois bosses (cos², rayon
1,5 km) : **profonde** (centre (23 km, 1 km), 7 m ; le support, ancien et nouveau fond, ≥ **107.14 m**) — décalage max d'une arrivée **0.0568 m**, portée **∅** ; **entre λ/2 et λ** (centre (14 km, 1 km), 7 m ; ≥
**62.14 m**) — décalage **8.958 m**, portée [1, 2, 3, 4, 5, 6, 7] (la règle `λ/2` l'aurait manquée) ; **côtière** (centre (6 km, 1 km), 12 m ; ≥ **17.51 m**) — décalage **788.0 m**,
portée [2, 3, 4, 5, 6] : trois plages sur huit hors de cause.

**Quantum** : f64 ; la tolérance d'un rayon est un demi-texel de la bibliothèque côtière (S599) : **0.25 m** ; l'accord entre traceurs
**0.001 m** (rapport 250, asserté). **Critères, écrits avant.** (1) l'isobathe limite à 10⁻⁹ près ; `dc/dh` contre une
différence centrée à 10⁻⁶ relatif, à 5, 30 et 90 m ; (2) sans bosse, les 363 rayons arrivent ; neuf arrivées rejoignent le traceur du plan à 0.001 m — départs à y = −4, 0, 4 km, directions
π − 0,35, π, π + 0,35 : 4017.6787, -4000.0000, -12017.6787, 8017.6787, 0.0000, -8017.6787, 12017.6787, 4000.0000, -4017.6787 m ;
(3) la bosse profonde : portée vide, et aucune arrivée décalée de plus de 0.25 m (le fond n'est plus senti) ; (4) la bosse entre λ/2 et λ :
une arrivée décalée de plus de 10 × 0.25 m, portée non vide ; (5) la bosse côtière : la portée [2, 3, 4, 5, 6], et toute plage dont une arrivée
change de plus de 0.25 m y est (les deux plages de ce rayon, avant et après) ; (6) refus : période, gravité ou pas non positifs, bornes non croissantes.

### Plan

- [x] **P1** — jeton ; plan.
- [ ] **P2** — `portee.rs` et ses essais ; (1)–(6).
- [ ] **P3** — preuve ; liste 12.5 ; rituel.

### Notes de reprise
