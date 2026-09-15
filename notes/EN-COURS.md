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

Session : S237 — en cours
Agent : Claude Code, Opus 5 (fichiers, git, cargo, Python/numpy/sympy et GPU local disponibles)
Entrée : « Continue », master propre à f5f78bf, trois copies au même commit, jeton libre,
secteur. Cœur : 419 réussis, 5 ignorés.

**Objectif.** Première surface **géométriquement mobile** dans le candidat δ MAC x-z : la frontière
à pression imposée se trouve à la hauteur réelle `η(x)` et non plus au couvercle fixe `z₀` ; les
mailles entrent et sortent du fluide ; l'advection quadratique est présente. Reçue contre une
référence **non linéaire indépendante**.
**Ce que la lecture fixe avant le plan.** Mode S233 : Dirichlet `p_dyn = ρg(η−z₀)` au couvercle
`z₀ = nz·dx`, facteur de demi-maille 2 dans l'opérateur, η transporté par flux de colonnes jusqu'à
`z₀`, advection retirée. Le dépôt possède un véhicule potentiel **non linéaire d'ordre 3** reçu
contre Stokes (S193, `examples/support/nl_surface.rs`, ADR-122) : périodique de longueur `2L`,
condition initiale paire, il donne l'onde stationnaire d'amplitude finie d'un bassin à murs.
**Thèse.** Fonction hauteur sur la grille MAC ; condition de Dirichlet par **fluide fantôme**
(Gibou) à la distance `θ·dx` de la surface, verticale et horizontale — opérateur symétrique, CG
conservé ; flux de colonne intégré jusqu'à la hauteur mouillée de chaque face ; vitesses
extrapolées dans la bande d'air ; niveau de référence au repos distinct du sommet du domaine, pour
un repos exact. À petite amplitude, le mode mobile doit rejoindre S233 ; à amplitude finie, il doit
produire l'harmonique `2k` que le mode linéaire ne peut pas produire.
**Critères, déclarés avant construction.** Repos exact au bit à un niveau intérieur ; opérateur
symétrique au bit ; volume conservé à l'arrondi ; petite amplitude à ≤1 % du mode linéaire S233 ;
amplitude finie : profil contre HOS M=3 sur une période, erreur décroissante en raffinant et
harmonique `2k` captée, le mode linéaire servant de témoin (harmonique nulle) ; tolérances de banc
écrites en P2 avec leur motif, jamais ajustées après mesure. Refus atomiques : surface à moins
d'une maille du fond ou du sommet, garde de pas, non-convergence ; zéro allocation.
**Arrêt.** Mode mobile construit et reçu, ou constat chiffré de ce qui manque. Aucune ambition
réduite ; déferlement, mouillage du fond, air, cavité et 3D restent hors lot et nommés.

### Plan

- [x] **P1** — amorce, jeton, plan seuls.
- [ ] **P2** — lectures ciblées (ADR-122, SURFACE-LIBRE-NL-S193, API `nl_surface`, ADR-141, tests
  δ surface) ; protocole écrit : modèle discret, gardes, domaine d'amplitude, tolérances et motifs.
- [ ] **P3** — référence : onde stationnaire HOS M=3 (bassin 8×4 m, ka 0,01/0,1/0,2), séries
  `η(x,t)` aux centres de colonnes ; recoupement sympy de l'ordre deux à petite amplitude.
- [ ] **P4a** — cœur : géométrie mobile (ensemble fluide, `θ`, opérateur, second membre et
  correction à Dirichlet mobile) ; tests repos exact et symétrie.
- [ ] **P4b** — cœur : extrapolation, flux mouillé, advection, `step_surface_mobile` atomique ;
  tests petite amplitude contre S233, volume, changement de topologie, refus, allocation.
- [ ] **P5** — réception : banc MAC contre HOS, trois résolutions, deux amplitudes, harmonique,
  témoin linéaire, coût ; document de validation.
- [ ] **P6** — rituel §6, file, feuille de route, jeton ; copies à synchroniser.

### Notes de reprise

Base : S236 — cœur 419/5 ; S233 mode linéaire (erreurs fines 0,066 % Terre / 0,017 % Lune).