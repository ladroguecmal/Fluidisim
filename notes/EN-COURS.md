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

Session : S569 — **en cours**. En autonomie : **le lot** (dû ; feuille de route S567–S568), puis **5.8** — deux manques : la vitesse
commandée d'une pompe, et un raccord qui se dénoie (aujourd'hui refusé : `Domain`, et l'hôte ne peut plus avancer).

**Ce que la session fait.** (a) `Organe::Pompe` reçoit sa vitesse `n` (0 à 1, la commande d'ADR-199 D1) : lois de similitude, `H ∝ n²`,
`Q ∝ n`, soit `H(Q) = H₀·n² − H₀·Q²/Q_max²`. (b) **Un raccord hors de l'eau est un exutoire à l'air libre** : sa charge est la cote du
raccord (la pression atmosphérique), et il ne fait que **recevoir** — un débit qui sortirait du nœud (le réseau aspirerait de l'air) est
coupé comme par un clapet. Plus de refus.

**Références, calculées avant** (ce script les écrit). (1) La pompe de S568 à `n` = 0,9 (à 0,8 elle ne monterait plus à 20 m : 19,2 m de barrage) — bissection : **`h_j` = 20.758823529 m, `Q` =
0.015904125 m³/s**. (2) Exutoire : A (1 m², 1,5 m) se vide par le fond vers B dont l'arrivée est à 1,0 m, au-dessus de son eau (0,2 m) ;
`√(h_A − 1) = √0,5 − t/(2√R)`, `R` = 4·10⁴ : **`h_A` = 1.208947 m à 100 s, 1.042893 m à 200 s**, A s'arrête à 1,0 m vers
**283 s**, B à 0,7 m ; l'essai dure 600 s. (3) Un raccord qui se dénoie : la sortie de A en paroi à 1,0 m, B rempli par le fond
(0,2 m) ; A se vide jusqu'à ce que sa surface passe sous sa sortie, puis plus rien ; le dernier pas dépasse d'au plus `√(0,3/R)·dt` =
**0.274 mm**.

**Quantum** (ADR-236 D1) : 10⁻⁹ pour (1) ; 1 µm en V ; l'Euler du pas pour (2) — l'écart attendu comme en S567 (~2·10⁻⁴ m). **Critères,
écrits avant.** (1) à 10⁻⁸ ; (2) `h_A` à 10⁻³ m de la loi fermée à 100 et 200 s, l'état final à 2·10⁻⁴ m de 1,0 et 0,7 m, la masse à
l'entier ; (3) A final entre 1,0 m − 0.274 mm et 1,0 m, puis immobile ; aucun refus ; (4) S565–S568 inchangés.

### Plan

- [x] **P1** — jeton ; le lot ; plan.
- [ ] **P2** — la vitesse, l'exutoire ; (1)–(4).
- [ ] **P3** — preuve ; liste 5.8 ; rituel (`--lot`).

### Notes de reprise
