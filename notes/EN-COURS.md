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

Session : S630 — **en cours**. En autonomie (ADR-247 : la physique des partiels). **3.5 — le déferlement sur une côte quelconque** : S588
ne savait qu'une côte droite (un sommet par ligne) ; son manque nommé : « les marching squares et le chaînage des segments ».

**Ce que la session fait.** `deferlement::contours(…)` : l'écart `H − 0,78·h` aux nœuds (la convention de S588 : la terre à +∞) ; sur chaque
maille, les arêtes où l'écart change de signe, le point de passage interpolé depuis le nœud qui déferle (`t = f_p/(f_p − f_q)`, 0 si `f_p`
est infini, comme S588) ; les segments d'une maille, le cas selle tranché par la moyenne du centre (non éprouvé) ; le chaînage par les
arêtes partagées — les polylignes ouvertes d'abord, puis les fermées (premier point répété). Donnée cuite (SPEC-006 §6) : la fonction alloue
sa sortie, hors exécution. Ne fait pas : la hauteur réfractée par une côte courbe (la houle de `transformer` suppose des isobathes
parallèles : l'île est prise en incidence normale), le flux dissipé et la direction de crête sur les sommets du contour.

**Références, calculées avant** (`s630_ref.py`, Python indépendant : la levée par bissection sur `k`). Une île conique (pente 0,02, rivage à
100 m), houle de 8 s et 1,5 m, maille de 5 m sur 1 km : la profondeur de déferlement **2.289178940 m**, le cercle **r_b = 214.458946980 m** ;
**340 points** de passage, à au plus **0.012357134 m** du cercle. Deux îles disjointes (centres à −200 et 250 m) :
**680** points. La côte droite : la grille de pente 0,02, houle oblique de 0,2 rad.

**Quantum** : f64. **Critères, écrits avant.** (1) la côte droite : une polyligne ouverte, ses sommets ceux de `polyligne` (S588) à 10⁻¹² m ;
(2) l'île : une polyligne fermée de 340 sommets distincts, l'écart radial maximal égal à la référence à 10⁻⁹ m ; (3) les deux îles :
deux polylignes fermées, 680 sommets ; (4) refus : grille de moins de 2 × 2, pas non positif, tampon trop court.

### Plan

- [x] **P1** — jeton ; plan.
- [ ] **P2** — `contours` et ses essais ; (1)–(4).
- [ ] **P3** — preuve ; liste 3.5 ; rituel.

### Notes de reprise
