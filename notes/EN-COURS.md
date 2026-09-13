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

Session : S212 — en cours
Agent : Claude Code (Opus 5 ; fichiers, git et cargo disponibles)
Objectif : file J1, couche W — sillage issu du cœur dans l'hôte GPU, comparé au cœur, coût mesuré.

### État réel

master et trois copies propres à 0e204ad (S211 close 10:04, jeton libre). Maillons 0 ; la suite
S211 nomme une ligne de la file (J1/W), pas un reliquat. Aucune dépendance nouvelle prévue.

### Thèse et critères, déclarés avant toute mesure

Le sillage du cœur est une somme modale (`spectral_pressure::Field`) : ADR-107 chiffre 25,6 ms
de préparation CPU pour 8 192 nœuds. Il n'a pas la forme de B (32 composantes). **Hypothèse à
éprouver, pas à croire** : publié en coefficients rebasés `[A, B, kx, ky]`, il se rend par somme
par sommet ; le coût dira si ce chemin tient ou s'il faut un autre chemin d'image (grille/texture,
transformée), à la manière d'ADR-129 pour l'impact.

Publication : `η(q) = Σ A cos(k·q) − B sin(k·q)`, pente `−k (A sin + B cos)`, où `(A + iB)` est
la réponse pondérée tournée de la phase repliée `k·origine` (PhaseQ32, aucun atan2, aucun temps
absolu au GPU — I-08). Emprise du champ appliquée au GPU comme au cœur.

Fixture sillage J1 (Froude de S156, lois de domaine transportées par similitude — **à vérifier**) :
σ 2 m, coupure 3 rad/m (σk 6), recette 64×128 (4 096 nœuds du demi-spectre) ; huit tronçons de
2 s à [3, 0] m/s sous 19 620 N (≈ 2 t), départ (−24, 4) m à la naissance de l'impact ; contexte
de 40 s ; emprise [−64, −48]–[64, 56] m ; repère/cellule 0, milieu 9,81 / 1025.

Critères : hauteur GPU contre cœur (B `eval` + impact direct + pression `sample_batch`) ≤ 3 mm
aux sondes, âges sillage 0/4/8/16/24/39 s ; témoin de résolution 64×128 contre 128×256 publié
(pas de seuil inventé, écart max rapporté à l'amplitude max) ; couture au bord de l'emprise
publiée ; admission `bound_pressure::Prepared::sample_world_batch` à `BREAKING_SLOPE` rapportée,
refus publié et non contourné ; coûts séparés CPU (préparation + publication) et GPU (passe
d'eau) en 640×360 et 960×540, deux recettes. Toute incompatibilité avec 2 ms est publiée ; une
issue technique va en ADR, une incompatibilité sans issue technique va à l'utilisateur (ADR-127 D7).

### Plan

- [x] **P1** — jeton, thèse, critères et plan seuls.
- [>] **P2** — cœur : `bound_pressure::Prepared::render_components` rebasé, refus atomiques, test contre `sample_batch`.
- [ ] **P3** — hôte : fixture sillage, préparation par image, buffer GPU, somme modale bornée à l'emprise ; R/B/Home ; compilation.
- [ ] **P4** — `--verify` : GPU contre cœur, témoin de résolution, couture, admission, coûts deux recettes, captures, fenêtre.
- [ ] **P5** — réception HOTE-GPU-S212, décision chiffrée du chemin d'image du sillage, suite complète des tests.
- [ ] **P6** — rituel §6, file plurielle, passation, jeton libre, copies avancées.

### Notes de reprise

*(vide)*
