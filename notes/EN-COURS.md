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

Session : S597 — **en cours**. En autonomie (ADR-247), **7.3 — les microbulles visuelles** (absent).

**Ce que la session fait.** `microbulles.rs` : un **nuage** laissé par un déferlement — des classes de diamètre, `N` bulles par m² chacune,
réparties uniformément sur une profondeur `D` ; chaque classe remonte à sa vitesse terminale (`bulle::vitesse_terminale`, Tomiyama, S540) ;
la **fraction restante** d'une classe `max(0, 1 − v·t/D)` ; l'**épaisseur optique** `τ = Σ N·2·π·r²·fraction` (l'extinction géométrique,
`Q_ext` = 2) ; l'**opacité** `1 − e^(−τ)` — ce que le rendu blanchit. Ne fait pas : la dissolution des bulles, l'émission (combien et de
quelles tailles un déferlement en fait), la turbulence qui les retient, le rendu.

**Références, calculées avant** (ce script les écrit et vérifie la taille d'ensemble, ADR-250 D1). Pour situer (Stokes) : 50, 100, 200 µm
remontent à **1.36, 5.44, 21.77 mm/s** ; un mètre se vide en 735, 184, 46 s. La
référence de l'essai : la fraction analytique `max(0, 1 − v_t·t/D)` avec la vitesse terminale de Tomiyama, aux instants [10, 30, 60, 120] s.

**Quantum** (ADR-236, ADR-250) : une population de 10000 bulles par classe (profondeurs uniformes par une suite déterministe), intégrées
une à une par `Bulle::pas` (le transitoire et la traînée implicite) ; l'écart-type d'une fraction ≤ 0,5/√N = 0.005.
**Critères, écrits avant.** (1) la fraction submergée de chaque classe à 0.05 près de l'analytique, aux quatre instants ; (2) `τ(0)` égal
à `Σ N·2·π·r²` à 10⁻¹² relatif, l'opacité décroissante ; (3) refus : diamètre, nombre ou profondeur non positifs.

### Plan

- [x] **P1** — jeton ; plan.
- [ ] **P2** — `microbulles.rs` et ses essais ; (1)–(3).
- [ ] **P3** — preuve ; liste 7.3 ; rituel.

### Notes de reprise
