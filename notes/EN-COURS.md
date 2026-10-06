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

Session : S568 — **terminée**. En autonomie, **5.8 — les pompes et les clapets du réseau en charge**.

**Ce que la session fait.** Une conduite porte un organe : aucun, un **clapet** (le débit de `a` vers `b` seulement ; fermé, une fuite
linéaire de 10⁻¹² m²/s garde la jacobienne inversible — 0,1 ml par jour sous 1 m), ou une **pompe** centrifuge avec son clapet (la loi de
V, ADR-199 D3 : `H(Q) = H₀·(1 − (Q/Q_max)²)`, soit `h_a − h_b + H₀ = (R + H₀/Q_max²)·Q²`, `Q ≥ 0`).

**Références, calculées avant par bissection ou forme fermée** (ce script les écrit). (1) Une pompe (`H₀` = 30 m, `Q_max` = 0,05 m³/s,
`R` = 2 000) d'un réservoir à 0 m vers une jonction reliée (`R` = 3 000) à un réservoir à 20 m : **`h_j` = 21.764705882 m, `Q` = 0.024253563
m³/s**. (2) Les trois réservoirs de S565, la branche de 80 m munie d'un clapet qui ne laisse passer que vers le réservoir : il se ferme,
**`h_j` = 71.428571429 m** (`1500·(100 − h) = 2000·(h − 50)`). (3) Couplé : deux cuves de 1 m² (1,5 et 0,2 m), une pompe (`H₀` = 1 m,
`Q_max` = 0,01 m³/s, `R` = 10⁴) de la basse vers la haute : l'équilibre au refoulement nul, **0,35 et 1,35 m**, atteint à 10⁻⁴ m vers
**213 s** (un Euler fin, ADR-240 D2) ; l'essai dure 426 s ; ensuite rien ne revient (le clapet).

**Quantum** (ADR-236 D1) : les références à 10⁻⁹ ; 1 µm en V. **Critères, écrits avant.** (1) `h_j` et `Q` à 10⁻⁸ ; (2) `h_j` à 10⁻⁸ m
(la fuite du clapet la déplace de ~4·10⁻⁹ m, calculé : `10⁻¹²·8,6/2,1·10⁻³`) ; (3) les cuves à 2·10⁻⁴ m de 0,35 et 1,35 m, la masse à
l'entier, aucun retour une fois l'équilibre atteint (la haute ne baisse plus) ; (4) sans organe, S565 et S567 inchangés (la suite).

### Plan

- [x] **P1** — jeton ; plan.
- [x] **P2** — les organes et leurs essais ; (1)–(4).
- [x] **P3** — preuve ; liste 5.8 ; rituel.

### Notes de reprise
- **P2 fini** — pompe 21,764705882 m ; clapet 71,428571430 m ; couplé 0,350000 / 1,350001 m sans retour ; S565, S567 inchangés. En route :
  l'arrêt au plancher flottant (une conduite de résistance 10⁻⁶ rendait la tolérance inatteignable : `NonFinite`). Le plan avait un
  nombre fait à la main (la fuite, un majorant) — contre ADR-243 D1, noté pour la revue de S571. Suite 738.
- **P3** — preuve RESEAU-ORGANES-S568 ; liste 5.8 ; index ; journal.

