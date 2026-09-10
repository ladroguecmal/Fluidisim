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

Session : S139 — en cours
Agent : Claude Code (Opus 5 ; fichiers, git et cargo disponibles)
Objectif : S138-1 — `max_slope` vaut 0,1 partout depuis S77 sans provenance (A205), et décide
de l'admissibilité de tout champ. Mesurer le rapport entre `slope_bound` — la borne L1
`Σ|a_k|·k·dk·k` calculée dans `RadialImpact::new` — et la **pente réelle** maximale du champ,
échantillonnée par `sample().slope`. Si ce rapport est stable, `max_slope` se dérive de la
cambrure de Stokes (SPEC-001 §4) au lieu de rester un nombre sans origine.

### Plan

- [x] **P1** — état réel, jeton, **plan déclaré et committé seul**.
- [x] **P2** — sonde `pente_reelle` : rapport `slope_bound / max|slope|` sur le disque et sur
      l'âge, pour un cas. Établir d'abord **où** le maximum se trouve — centre, premier anneau,
      instant initial — avant de balayer quoi que ce soit.
- [x] **P3** — balayer λ, énergie, N et rayon du domaine. Le rapport est-il une constante du
      modèle, ou dépend-il d'un paramètre ? C'est la question qui décide de tout le reste.
- [x] **P4** — si le rapport est stable : dériver `max_slope` de `πH/λ ≈ 0,449` divisé par lui,
      et **vérifier la conséquence** — quelles énergies deviennent admissibles ou refusées à
      0,1 contre la valeur dérivée. Sinon : dire de quoi il dépend, et ce que cela coûte.
- [x] **P5** — ADR : d'où vient `max_slope`. Livrable de mesure dans `docs/validation/`.
      Notes correctives datées là où 0,1 est cité comme une donnée du milieu.
- [ ] **P6** — rituel de fin (§6), jeton rendu, fusion `--ff-only`.

### Notes de reprise

Départ 041dfed = master ; worktree `886155`. Tests au départ : 269, cinq ignorés (176+93).

Ce qui est déjà connu et n'est pas à remesurer :
- `slope_bound` est une borne **L1**, construite avec `|J1| ≤ 1` (commentaire de
  `radial_impact.rs:170`, « borne conservative »). Le maximum réel de `J1` vaut 0,5819 en
  x ≈ 1,841 : le rapport ne peut donc pas descendre sous 1,72, et les phases temporelles
  `cos(ω_k t)` comme les zéros de `J1(k r)` l'écartent encore.
- la forme initiale est **exactement** homothétique en λ (S136, écart nul de 0,5 à 32 m) ;
  si le rapport dépend de λ, ce ne peut être que par la discrétisation ou la portée de Bessel,
  pas par la physique.
- `slope_bound ∝ √E` (S136) : le rapport devrait être **indépendant de l'énergie**. Le vérifier
  quand même — c'est justement ce genre de « devrait » que S136 a pris en défaut sur `α`.

Piège à éviter : conclure « la pente réelle vaut ρ fois moins » depuis un échantillonnage trop
grossier. Le maximum d'une somme de 64 Bessel oscille ; sous-échantillonner **sous-estime** le
maximum et **surestime** le rapport, donc rend `max_slope` trop permissif. Raffiner jusqu'à
stabilité, et le dire.
