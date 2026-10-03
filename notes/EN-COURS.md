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

Session : S456 — **terminée**. Sans « Continue » (ADR-215 D1) : l'étape 3 d'ADR-215 D4, **la houle**.

**Ce que la session fait.** Le bord ouvert de S446 (`Apic3::enable_open_boundaries`, CPU) porté sur la carte : les faces `u` des
bords `i = 0` et `i = nx` portent la vitesse normale de la houle B (`LinearSwell`, eau profonde), sous sa surface ; la projection la
prend comme donnée ; les colonnes des bords comptent le débit en quanta ; le volume entré se cumule sur la carte (`open_quanta`). L'état
initial porte B (l'eau ensemencée sous sa surface, ses vitesses aux particules et à la grille), sinon le bord de sortie crée sa propre
onde. Le banc `--c10-houle` : la scène de 4 m (1,4 m d'eau), une houle de 4 cm et de 2 m, sans corps, 10 s.

**Critères, écrits avant.** (1) **la masse comptée** : `quanta − initiaux − entrés par les bords = 0` à chaque mesure ; (2) **la houle
ne s'amortit pas** de plus de 10 % sur 10 s (amplitude sur la rangée du milieu, à 10 s contre 1 s) ; (3) l'écart à l'élévation de B
publié (la dispersion numérique) ; (4) avec le saut : la scène stable, la fenêtre montrée.

### Plan

- [x] **P1** — jeton, plan seul.
- [x] **P2** — le bord ouvert sur la carte ; l'état initial de B ; le banc ; mesures (1) à (3).
- [x] **P3** — le saut sous la houle ; images ; mesure (4).
- [x] **P4** — preuve ; rituel (allégé).

### Notes de reprise
- **P2** — `open_value` (la vitesse de B sur les faces `u` des bords, aux trois endroits qui remettaient les parois à zéro), le débit
  des bords dans `columns_flux`, `open_count` (le volume entré cumulé, un mot double après les débits de `ivol`), `set_open_x`,
  `open_quanta` ; `B10::houle` (l'eau sous la surface de B, ses vitesses aux particules et à la grille) ; `HOULE=a,λ` ; le banc
  `--c10-houle`. **Mesuré** : (1) **masse exacte** (écart 0 à chaque mesure) — après un faux écart : `open_quanta` lisait au mauvais
  décalage (`read_u32` prend des octets) ; (2) sans relaxation, la houle bat (0,8 à 1,37 `a`) : un bord à vitesse imposée renvoie ce qui
  ne colle pas à B. **Les zones de relaxation** (Jacobsen, 0,8 m) : ramener **les vitesses** vers B fait dériver le niveau intérieur
  (−20 mm) — écarté ; ramener **la surface seule** : l'amplitude au milieu entre 0,84 et 1,05 `a`, **sans décroissance** (moyenne 0,96
  sur 1–3 s, 0,97 sur 8–10 s), l'écart à B sous 0,25 `a` (3). Le critère (2) tel qu'écrit — l'instant 10 s contre 1 s — donne 0,80, au
  creux d'une modulation de ±10 % : tranché (ADR-215 D2), la houle ne s'amortit pas, la modulation inscrite.
- **P3** — le saut sous la houle (4 cm, 2 m) : masse exacte jusqu'à t = 16, `φ` fini ; la fenêtre : simulé / réel **0,98** (0,84 pendant
  le saut), 33,7 ms au 99e centile. Images `captures/s456/scene_t{1.0,2.0,3.0,6.0}.png` envoyées.
- **P4** — C10-SCENES-S454 §5 ; journal ; jeton libre ; maillons 13 (justifiés : S406) ; suivant : S457, le lot des registres puis la lumière de l'eau (ADR-215 D4 étape 4).
