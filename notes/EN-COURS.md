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

Session : S528 — **terminée**. En autonomie, **3.1 — les anneaux d'impact en eau peu profonde** (K2-12) : `RadialImpact` refuse le
régime peu profond (`Error::Regime`, profondeur ≤ π/k_min) ; W porte la profondeur uniforme depuis S522.

**Ce que la session fait.** `RadialImpact::new_in_depth(événement, milieu, domaine)` : le nombre d'onde effectif `κ = k tanh(kh)` de
S522 dans la pulsation (`ω² = g κ`), le potentiel (`η_t / κ`) et la vitesse horizontale (`k/κ` fois celle de l'eau profonde) ; pas de refus
de régime ; la borne de pente resserrée (`RHO_DISPERSION`, mesurée en eau profonde) n'y resserre pas (`slope_max_at` = `slope_max`) — la
borne de couronne, indépendante de la dispersion, reste. `new` inchangé. **La référence** : le champ initial de W (t = 0) sur une grille,
propagé par FFT avec la dispersion exacte `cos(√(g k tanh kh) t)` (`outils/reference_anneaux.py`), indépendante de la somme de Bessel.

**Ordre de grandeur, calculé.** λ = 4 m (k₀ = 1,57, nœuds de 0,785 à 3,14 rad/m), 1 m de fond : `tanh(k₀h)` = 0,917, la pulsation 4,2 %
sous l'eau profonde ; à 10 s, le déphasage au pic vaut **1.66 rad** — l'eau profonde est visiblement fausse. Domaine 40 m, 10 s.

**Critères, écrits avant.** (1) `new` au bit (suite) ; `new_in_depth` avec `2 k_min h` > 32 rend les échantillons de `new` au bit. (2) W
par 1 m de fond contre la référence FFT (convergée sur trois grilles) : écart quadratique ≤ 2 % sur le disque de 40 m à 5 et 10 s ; l'eau
profonde (le même champ initial) à plus de 5 fois cet écart. (3) La pente réelle de W à 0–10 s sous `slope_max_at` et `slope_max_beyond`
(la sûreté des bornes en eau peu profonde).

### Plan

- [x] **P1** — jeton, plan seul.
- [x] **P2** — le cœur ; (1).
- [x] **P3** — la référence, W ; (2), (3).
- [x] **P4** — preuve ; liste 3.1 ; rituel.

### Notes de reprise
- **P2 fini** — `RadialImpact::new_in_depth` (`κ`, potentiel, vitesse horizontale ; pas de resserrement) ; essais `s528` : au bit de `new`
  à 30 m (N 128 : 64 modes refusés par la résolution à 40 m et 10 s), le régime peu profond accepté ; la pente réelle au plus 0,990 de
  `slope_max_at` par 1 m de fond (le critère 3, en avance). Suite verte.
- **P3 fini** — le champ initial limité à ± 80 m (la portée de la somme à 128 modes ; 280 m refusé par la résolution) ; W contre la FFT
  exacte par 1 m de fond : **0,44 %** à 5 et 10 s sur les trois grilles ; l'eau profonde 51 % et 94 % → (2) tenu.
- **P4** — preuve ANNEAUX-PROFONDEUR-S528 ; liste 3.1 ; index ; journal.

