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

Session : S296 — **porte B, lot 2 : la surface mobile dans la référence δ 3D** — reprise à chaud par Codex à P2
Agent : Codex GPT-6, application desktop ; fichiers, git, cargo, outils locaux.
Entrée : « continue » de l'utilisateur, 2026-09-19 ; suite désignée par S295, porte en cours B.
Objectif : la référence `delta3d` gagne le mode **à surface géométriquement mobile** de S237 —
fonction hauteur `η(x, y)`, fluide fantôme aux faces verticales et latérales, advection centrée,
extrapolation, transport par débits mouillés — étendu aux deux dimensions horizontales, la 2D
intacte. Critères posés avant le code :
1. à `ny = 1`, trajectoire **identique à la 2D** mobile au chemin de Jacobi (celui de S237) sur
   le cas S237 à `nx` = 32 — l'identité au bit est visée, puisque le lot 1 l'a obtenue ; à défaut
   l'écart est publié ;
2. à `ny = 1`, le protocole S237 contre le véhicule HOS d'ordre 3 (L = h = 2 m, une période,
   1 ms) : profil sous **2 %** de `a` à 128 colonnes et décroissant, harmonique `2k` sous **20 %**,
   aux tolérances publiées de S237 ;
3. en 3D : une onde le long de `y` dans une cuve transposée rend la même trajectoire que l'onde le
   long de `x` (symétrie x↔y du schéma), à l'arrondi près ; petite amplitude oblique contre le
   mode linéaire du lot 1 sous **1 %** de `a` (critère 4 de S237) ;
4. repos exact au bit à un niveau non aligné, aucune allocation, refus atomiques (gardes de
   géométrie, convergence).
Préconditionneur de Jacobi : l'invariance en `y` n'est alors tenue qu'à l'arrondi — c'est ce
qu'ADR-175 §4.1 demande. Hors lot : couplage B/W, production GPU, scène, coût.

### Plan

- [x] **P1** — état réel, jeton, plan seuls.
- [x] **P2** — géométrie mobile et opérateur : mailles mouillées, fantômes vertical et latéraux
  (x et y), opérateur, second membre et diagonale ; erreur inverse du mode mobile, `γ` selon le
  nombre de faces ; essais : symétrie avec fantômes, identité au bit avec la 2D à `ny = 1`.
- [x] **P3** — projection mobile (Jacobi, départ depuis la pression publiée d'ADR-169),
  correction fantôme, extrapolation, divergence des lignes franches ; essai : identité avec la 2D.
- [x] **P4** — advection 3D, transport mouillé en x et y, `step_surface_mobile`, gardes et refus
  atomiques ; essais : repos exact, trajectoire `ny = 1` contre la 2D, symétrie x↔y, allocations.
- [ ] **P5** — réception : banc `delta3d_mobile` (S237 à `ny = 1` contre HOS, petite amplitude
  oblique contre le linéaire), preuve `DELTA3D-MOBILE-S296.md`.
- [ ] **P6** — rituel §6.

### Notes de reprise
Passation (2026-09-19, demande de l'utilisateur : « commit tout, je vais le faire avec Codex »).
Rien n'est écrit pour P2 : reprendre à P2, code intact depuis S295.

Ce que la lecture du 2D a établi, pour ne pas le refaire :
- Le mode mobile 2D vit dans `delta_mobile.rs` : `wet` (centre sous `η` de la colonne),
  `ghost_up` (`θ = (η − z_c)/dx` borné par `SURFACE_THETA_MIN`, valeur `ρg((η − rest) − reste)`),
  `ghost_side` (`θ = (h_i − z_c)/(h_i − h_j)`, valeur `ρg(z_c − rest)`), `apply_mobile`,
  `rhs_mobile` (second membre et diagonale de Jacobi), `correct_mobile`, `extrapolate_mobile`
  (constante verticale au-dessus de la dernière face corrigée), `transport_mobile` (débit intégré
  jusqu'à `½(η_i + η_{i+1})` avec fraction mouillée `clamp((η_f − k·dx)/dx, 0, 1)`, tous lisent
  `ηⁿ`), `surface_in_bounds` (≥ 2 mailles au-dessus du fond, ≤ sommet − 1 maille).
- Ordre du pas 2D : garde, sauvegarde, `advect(dt)` (centrée, `delta_projection.rs` l.1090 :
  voisins manquants remplacés par la valeur centrale ; face `w` du sommet non advectée), projection
  mobile **à départ chaud** (ADR-169 : `p` publiée, nulle hors mailles mouillées, résidu vrai),
  extrapolation, transport, validation de onze champs, garde, publication.
- Préconditionneur : la 2D prend la **multigrille mobile** (ADR-167) dès que la grille se divise ;
  pour l'identité à `ny = 1`, forcer son chemin de Jacobi dans les essais par le
  `thread_local` `crate::delta_projection::MOBILE_MULTIGRID_OFF` (cfg(test)). Chemin Jacobi :
  `dir = prec·res` (`β = 0`), `rz = Σ r·prec·r`, puis `β = zn/rz`, `dir = prec·res + β·dir`.
- Certificat d'arrondi : utiliser `γ₈` quand `ny = 1` (quatre faces par ligne) et `γ₁₀` sinon ;
  le lot 1 emploie `γ₁₀` partout — à aligner, sans quoi l'identité à `ny = 1` peut casser au
  plancher.
- Advection 3D : écrire `uc·ux + wc·uz + vc·uy` (terme `y` en dernier) pour garder les bits 2D à
  `ny = 1`, même règle que la divergence et le transport du lot 1.
- Oracle HOS et protocole : `examples/delta_mobile.rs` (fonctions `hos`, `hos_eta`, `hos_b2`,
  L = h = 2 m, `nz = 2,25/dx`, repos à 2 m, une période, 1 ms, plafond 4 000) et
  `examples/support/nl_surface.rs` ; à recopier dans un banc `delta3d_mobile.rs`.

