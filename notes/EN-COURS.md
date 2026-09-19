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

Session : S295 — **porte B, lot 1 : la référence δ tridimensionnelle, surface linéarisée**
Agent : Claude Opus 5, application desktop Claude Code ; fichiers, git, cargo, outils locaux.
Entrée : suite désignée par S294 — porte en cours B (FEUILLE-DE-ROUTE §3 bis), ADR-175.
Objectif : recevoir la première partie du critère 1 de la porte B (ADR-175 §4.1) sur le mode le
plus simple : un domaine MAC x-y-z à surface **linéarisée** (le mode 2D de S233, ADR-141),
référence CPU dans le cœur, **nouveau module, la 2D intacte**. Critères posés avant le code :
1. `ny = 1` reproduit la trajectoire du solveur 2D de S233 sur le même bassin (écart publié ;
   l'identité au bit n'est pas exigée, elle sera dite si elle a lieu) ;
2. une hauteur initiale indépendante de `y` le reste, à l'arrondi près ;
3. l'onde stationnaire **oblique** d'une cuve rectangulaire — mode (1, 1), `Lx` 8 m, `Ly` 4 m,
   `h` 4 m, `A` 1 cm, vitesse nulle, 1 s — suit `η = A·cos(πx/Lx)·cos(πy/Ly)·cos(ωt)`,
   `ω² = g·k·tanh(k·h)`, `k = π·√(1/Lx² + 1/Ly²)` : erreur maximale normalisée par `A`
   publiée à plusieurs résolutions et pas, **décroissante en raffinant**, sous **1 %** au cas fin
   — la tolérance de banc de S233, reprise telle quelle ;
4. repos exact au bit, aucune allocation dans le pas, refus atomique sur non-convergence.
Hors lot : surface mobile (lot 2), couplage à B/W (lot 3), production GPU (lot 4), coût.

### Plan

- [x] **P1** — état réel, jeton, plan seuls.
- [x] **P2** — module `delta3d` : domaine, champs, configuration comptée auprès de l'hôte (I-06),
  opérateur de pression 3D ; essais : refus de configuration, symétrie et positivité.
- [x] **P3** — projection : gradient conjugué sans préconditionneur (le chemin 2D à couvercle
  fixe), critère premier `10⁻⁶`, tolérance d'ADR-144, certificat d'arrondi `γ₁₀` dérivé pour six
  faces (ADR-143) ; correction et divergence ; essais : divergence projetée, repos exact.
- [ ] **P4** — pas à surface linéarisée (flux de colonne, somme compensée, refus atomique) ;
  essais : `ny = 1` contre la 2D, invariance en `y`, aucune allocation.
- [ ] **P5** — réception de l'onde oblique : banc `delta3d_lineaire`, preuve
  `docs/validation/DELTA3D-LINEAIRE-S295.md`.
- [ ] **P6** — rituel §6.

### Notes de reprise

P2 (15:05-15:06) : `delta3d` créé, opérateur 3D symétrique et défini positif ; **à `ny = 1` il est identique au bit à l'opérateur 2D** (même ordre des faces, murs `y` sans contribution). Seul ajout côté 2D : `apply_for_tests`, sous `#[cfg(test)]`. 4 essais verts.
P3 (15:06-15:09) : projection 3D — gradient conjugué du chemin 2D à couvercle fixe, vrai résidu, porte d'ADR-144 avec cible resserrée, arrêts au plancher par `γ₁₀` et par empreinte (Brent). Champ aléatoire projeté sous `10⁻⁵` sur trois formes, repos exact au bit, murs nuls au bit. Une assertion de l'essai comparait deux ordres d'opérations flottantes au bit : remplacée par un écart relatif `10⁻⁶`, la valeur publiée étant bien celle du champ publié.
