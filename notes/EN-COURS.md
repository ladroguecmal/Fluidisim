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

Session : S633 — **terminée**. En autonomie (ADR-247 : la physique des partiels). **3.5** — achever les rayons de S632 : les sommets de SPEC-006
§6 (position, flux dissipé, direction de crête) le long d'un faisceau ordonné — la polyligne chaînée d'une côte quelconque.

**Ce que la session fait.** `deferlement::sommets_sur_rayons(rayons, b₀, …)` : pour chaque rayon du faisceau (son voisin : le suivant, ou le
précédent pour le dernier), le point de déferlement de S632, le flux `ρ·g·H²/8·c_g` en kW/m (`H` = 0,78·h au point), la direction de crête (θ
du rayon, interpolé) ; les sommets dans l'ordre du faisceau. Ne fait pas : les caustiques, un faisceau qui se replie, la publication.

**Références, calculées avant** (`s633_ref.py`, numpy ; le montage de S632, ses bornes assertées alors). Côte droite, l'analytique : flux
**16.973287676 kW/m**, direction (-0.994088221713 ; 0.108575353794). Le long des rayons, pas de ½ s : x = 112.600476235 m, flux
**16.973662175 kW/m** (à 2.2e-05 de l'analytique), direction (-0.994088284396 ; 0.108574779879) (à 5.7e-07).

**Quantum** : f64. **Critères, écrits avant.** (1) un faisceau de cinq rayons (y = 0, 10, 20, 30, 40 m) : cinq sommets, dans l'ordre ; le premier
égal à la référence à 10⁻⁹ (position, flux, direction) ; (2) le flux à 10⁻⁴ relatif de l'analytique, la direction à 10⁻⁵ ; les cinq positions
`x` à 10⁻⁹ m l'une de l'autre (la côte droite) ; (3) refus : moins de deux rayons.

### Plan

- [x] **P1** — jeton ; plan.
- [x] **P2** — `sommets_sur_rayons` et son essai ; (1)–(3).
- [x] **P3** — preuve ; liste 3.5 ; rituel.

### Notes de reprise
- **P2 fini** — (1)–(3) tenus du premier essai. Suite : 818 essais listés.
- **P3** — preuve SOMMETS-RAYONS-S633 ; ligne 3.5 ; index ; journal.
