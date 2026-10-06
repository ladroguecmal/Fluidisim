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

Session : S525 — **terminée**. En autonomie, **la résonance de C07** (CAS-CANONIQUES, note S30) : « la pente de `log A` contre
`log|1 − Fr_h²|` vaut −½ ± 0,15 sur `Fr_h` ∈ {0,3 ; 0,5 ; 0,7 ; 0,9} ». Dernière branche de C07.

**Ce que dit la théorie, calculé avant.** En ondes longues (source large devant le fond), l'équation permanente
`(1 − Fr²) η_xx + η_yy = −∇²p/ρg` donne sous une source isotrope **exactement** `η(0) = −(p₀/ρg)/√(1 − Fr²)` (la moyenne angulaire de
`1/((1 − Fr²)cos²θ + sin²θ)` vaut `1/√(1 − Fr²)` : le facteur de Prandtl–Glauert) — d'où la pente −½. **Mais la référence exacte (finie,
dispersive, en temps fini ; `reference_sillage.py`) ne le donne pas sur les quatre points** : la dépression maximale près de la source,
rapportée à la statique, vaut 1,052 / 1,167 / 1,441 / **3,21** (σ 20 m, 5 m de fond, 64 s) contre 1,048 / 1,155 / 1,400 / **2,29** ; pente
**−0,72** (−0,60 à 32 s ; σ 10 m : −0,80 / −0,85). À `Fr_h` = 0,9 le régime n'est pas permanent (le temps d'établissement croît comme
`σ/((1 − Fr)c)`) et la dispersion y compte ; sur 0,3–0,7, la théorie suit Prandtl–Glauert à 3 % près. (La note de S523, « une source
fine », était fausse : la loi demande une source **large**.)

**Ce que la session fait.** `c07_profondeur` reçoit σ et une grille devant la source ; W (σ 20 m, recette 512 × 512 à coupure 0,4 : rayon
honnête 2 681 m) aux quatre `Fr_h`, 64 s ; l'instrument (le maximum de |η| à moins de 3σ de la source) sur W et sur la référence aux mêmes
points.

**Critères, écrits avant.** (1) **La théorie** : sur la référence, la pente sur {0,3 ; 0,5 ; 0,7} à −½ ± 0,05 ; sur les quatre points, sa
valeur publiée (−0,72) — l'assertion de C07 telle qu'écrite n'est pas celle de la théorie à durée finie. (2) **W** : les quatre amplitudes à
2 % de la référence, la pente sur quatre points à 0,05 de la sienne, sur trois points à −½ ± 0,15 (l'assertion, restreinte au régime
permanent). (3) La note de C07 corrige l'assertion (ADR-222 : une cible contredite par la mesure est remplacée).

### Plan

- [x] **P1** — jeton, plan seul.
- [x] **P2** — W et la référence ; (1), (2).
- [x] **P3** — preuve ; C07 ; listes 3.2, 13.2 ; rituel.

### Notes de reprise
- **P2 fini** — W aux quatre `Fr_h` (4 s chacun) : 1,0529 / 1,1672 / 1,4418 / 3,2071 contre 1,0531 / 1,1674 / 1,4422 / 3,2077 (0,03 %) ;
  pentes −0,721 et −0,544 des deux côtés → (1), (2) tenus.
- **P3** — preuve C07-RESONANCE-S525 ; note C07 ; listes 3.2, 13.2 ; index ; journal.

