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

Session : S567 — **en cours**. En autonomie, **5.8 — le réseau en charge couplé au pas de V** : S565 résout un réseau entre des charges
données ; ici, les charges sont les surfaces des nœuds de V, et les débits les vident et les remplissent.

**Ce que la session fait.** `charge::pas_reseau(nœuds, formes, g_eff, dt, raccords, demandes, conduites, …)` : chaque raccord (un nœud de V
en un point) donne une charge fixe — la cote de la surface du nœud le long de la verticale locale ; le réseau est résolu (S565, départ chaud
sur les charges du pas précédent) ; le débit net de chaque raccord, intégré sur le pas, devient des millilitres entiers avec un reste par
raccord (comme les arêtes de V). Le réseau ne stocke rien : ce qu'il soutire aux jonctions (les demandes) sort, et la masse se compte
nœuds + sortie, à l'entier. Refus atomiques : un raccord hors de l'eau (le réseau aspirerait de l'air ; `Domain`), un nœud qui donnerait
plus qu'il n'a ou recevrait plus que sa place (`Capacity`). Les résistances sont celles de la gravité locale ; l'air des poches n'entre
pas (version suivante).

**Références, calculées avant** (ce script les écrit). Deux cuves de 1 m², l'eau à 1,5 et 0,5 m, reliées par trois conduites en série
(A–J0–J1–B, `R` = 10⁴ + 2·10⁴ + 10⁴ = 40000 s²/m⁵) : `d√Δh/dt = −(1/A + 1/B)/(2√R)` = −0.0050 s⁻¹, égalisées à **200 s**
(constante de temps, ADR-240 D2 ; l'essai dure 300 s). `Δh` à 50, 100, 150 s : **0.5625, 0.2500, 0.0625 m**. L'erreur
d'Euler au pas de 0.1 s, bornée par `dt·T·max|Δh''|/2` = **3.75e-04 m** à 150 s. Le robinet : 1 L/s soutiré à J0 pendant 100 s →
**100000 ml** sortis.

**Quantum** (ADR-236 D1) : 1 ml sur 1 m² = 1 µm. **Critères, écrits avant.** (1) `Δh` à 10⁻³ m de la loi fermée aux trois instants
(rapport 2.7 à la borne d'Euler), et à 10 µm d'un Euler f64 indépendant au même pas ; (2) la masse : nœuds + sortie
constants à l'entier à chaque pas, la sortie sans demande bornée par le nombre de raccords (2 ml) ; (3) le robinet : 100000 ml
sortis à 2 ml près ; (4) les refus, rien d'écrit.

### Plan

- [x] **P1** — jeton ; plan.
- [ ] **P2** — `pas_reseau` et ses essais ; (1)–(4).
- [ ] **P3** — preuve ; liste 5.8 ; rituel.

### Notes de reprise
