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

Session : S522 — **terminée**. En autonomie ; deux maillons : un lot qui fait avancer une capacité. **W en profondeur finie** : la pression
de W (sillages, impacts de pression) suppose l'eau profonde (`ω² = g k`, aucune `tanh`) ; C07 peu profond, les anneaux en eau peu profonde
(K2-12) et 2.7 l'attendent.

**Ce que la session fait.** (a) **Le cœur** : `ModalPressure::new_in_depth(k, g, ρ, profondeur, …)` — une profondeur uniforme `h`
(`Option`, `None` : le chemin profond, inchangé au bit) ; le nombre d'onde effectif `κ = |k|·tanh(|k|h)` remplace `|k|` dans la
pulsation (`ω² = g κ`), le forçage (`−κ p/ρ` : `η_t = κ φ` en surface) et, dans les échantillons, la conversion de la vitesse verticale
en potentiel et en vitesse horizontale, et l'énergie (`|w|²/κ`). `tanh` sans libm, par l'exponentielle déterministe du cœur (`decay`) ;
`tanh` = 1 exactement au-delà de `2|k|h` = 32. `spectral_pressure::prepare_in_depth` ; `prepare` y passe sans profondeur. (b) **La
référence** (`outils/reference_sillage.py`) en profondeur finie : `ω = √(g k tanh kh)`, forçage `k tanh kh`. (c) **W contre elle**.

**Ordre de grandeur, calculé.** Par 5 m de fond, `√(gh)` = 7,00 m/s ; à `Fr_h` = 0,9 (U = 6,3 m/s), l'onde transverse fait **36,6 m**
contre 25,4 m en eau profonde (`kh` = 0,86) — l'écart que la mesure doit voir ; à `Fr_h` = 0,5, 7,9 m des deux côtés. Zone établie à
`T` = 40 s : `U·T/2` = 126 m (3,4 λ). Recette 512 × 256 à coupure 3 (σ = 2 m) : rayon honnête 179 m ; en eau peu profonde, `c_g` ≤ 7 m/s,
la récurrence `2π/dk` = 1 072 m n'est pas atteinte.

**Critères, écrits avant.** (1) **Au bit** : sans profondeur, la suite du cœur et les bancs inchangés ; une profondeur où `2|k|h` > 32
pour tous les nœuds rend le chemin profond au bit. (2) **Un mode** : la pulsation libre après le forçage, mesurée sur le signal du mode,
à 10⁻⁴ relatif de `√(g k tanh kh)` pour `kh` ∈ {0,1 ; 0,5 ; 1 ; 3} ; la hauteur statique sous une pression tenue, `−p/ρg`, inchangée par
la profondeur. (3) **Le sillage** à `Fr_h` = 0,9 (h = 5 m, U = 6,3 m/s, σ = 2 m, 40 s) : W contre la référence en profondeur finie,
écart quadratique ≤ 10 % sur la zone établie (de 1 λ à `U·T/2` − 1 λ, coin de 60°) ; la référence profonde, elle, à plus de deux fois cet
écart (la profondeur se voit). La référence convergée sur trois grilles (ADR-230).

### Plan

- [x] **P1** — jeton, plan seul.
- [x] **P2** — le cœur en profondeur finie ; (1), (2).
- [x] **P3** — la référence et le sillage ; (3).
- [x] **P4** — preuve ; listes 2.7, 3.2 ; rituel (lot dû à S522 : fait en S521, `--lot` non requis).

### Notes de reprise
- **P2 fini** — `effective_wavenumber`, `ModalPressure::new_in_depth`, `prepare_in_depth` ; `add_segments` reçoit la profondeur (le nœud
  reste à 64 octets : l'essai de taille l'a rappelé). Essais `s522` : pulsation libre à **2·10⁻⁷** de `√(g k tanh kh)` (kh 0,1 à 3), creux
  `−2p/ρg` à 10⁻⁷, le chemin profond au bit (modes et champ), 5 m de fond diffère. Suite du cœur : **684**, verte.
- **P3 fini** — `c07_profondeur` (W, 40 119 points, 22 s) ; référence par 5 m de fond (`reference_sillage.py profondeur`) : λ transverse
  36,48 m ; écart quadratique **2,0 %** sur les trois grilles ; la référence profonde **121 %** → (3) tenu.
- **P4** — preuve W-PROFONDEUR-S522 ; listes 2.7, 3.2 ; dépendances ; index ; journal.

