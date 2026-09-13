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

Session : S206 — en cours
Agent : Claude Code (Opus 5 ; fichiers, git et cargo disponibles)
Objectif : **A247**, bloquant J1 — confronter le coût de l'eau **par image** au profil ADR-125
(60 images/s, eau 2 ms) sur une **scène représentative** déclarée, mesurer les leviers, puis
poser l'**arbitrage explicite** (ADR-127 D7) : options, coûts mesurés, ce que chacune dégrade,
qui tranche. Aucune fonctionnalité retirée ; aucun seuil changé pour faire passer une mesure.

### État réel à l'amorce

master = copies = ef447fc (S205 P7), propres ; jeton libre depuis 02:06. Arbitrage « hôte
interactif » toujours sans réponse. Acquis à ne pas remesurer : B 48 ns par composante par
point (BANC-B1-S146) ; S203 : B 1,6 µs/pt, B+W N256 14 µs/pt, table radiale 2,7 ms/impact,
erreur Hermite ≤ 0,006 mm. `paquets_W_max = 4096` déclaré par ADR-012 §3, jamais confronté.

### Scène représentative déclarée avant mesure

Observateur S201 (640×360, 50°), mer S201 Hs 1,5 m (N32), un impact S203 (N256, R 52 m,
A 56 s) à +3 s, composé par le chemin hôte (`Prepared::sample_world_batch` dans R, B hors R).
**Charge = sommets d'une grille projetée** : un sommet par `c` pixels, intersection du rayon
avec z = 0 jusqu'à 600 m, `c` ∈ {8, 4, 2}. C'est la charge d'un maillage de surface évalué sur
CPU, pas celle du lancer de rayons hors ligne. Critère : temps mur médian et maximum sur 11
images après 3 de chauffe, comparé à 2 ms. Machine : celle de S202 (Ryzen AI 7 350).

### Plan

- [x] **P1** — état réel, scène déclarée, plan seul.
- [x] **P2** — banc `frame_cost.rs` : grille projetée, décompte des sommets (total, dans R),
 coût par image un fil : B seul partout ; B+W chemin hôte. Essais du banc.
- [ ] **P3** — leviers : (L1) parallélisme `std::thread::scope` 1/2/4/8/16 fils, temps mur ;
 (L2) table radiale à matrice de Bessel précalculée — noyau N×M mesuré sur tableaux de la
 bonne taille, précalcul et mémoire par impact ; (L3) densité `c`. Nombre de composantes de B :
 déduit de B1 (48 ns), pas remesuré.
- [ ] **P4** — campagne release et publication `COUT-IMAGE-S206` : tableau par configuration
 contre 2 ms, compatible / incompatible, `paquets_W_max` confronté.
- [ ] **P5** — arbitrage explicite : options mesurées, dégradations, décideur ; ce qui est
 technique se tranche (ADR si décision), ce qui touche l'ambition, les dépendances ou le sens
 du budget (temps mur ou CPU) remonte à l'utilisateur. Feuille de route et file active.
- [ ] **P6** — rituel §6 complet.

### Notes de reprise

P2 (`frame_cost`, release, un fil, 3 chauffes/11 images) — la scène vient de `render_impact.rs`
inclus comme module, pas recopiée ; 15 essais. Grille projetée :
| pas | sommets | dans R | B seul méd/max ms | image méd/max ms | B ns/pt | B+W ns/pt | image/budget |
| 8 px | 2 240 | 1 918 | 2,942 / 3,039 | 17,195 / 18,715 | 1 314 | 8 744 | 8,6 |
| 4 px | 9 044 | 7 652 | 12,012 / 17,666 | 74,013 / 84,157 | 1 328 | 9 431 | 37,0 |
| 2 px | 36 160 | 30 614 | 42,798 / 46,337 | 279,755 / 306,646 | 1 184 | 8 924 | 139,9 |
**86 % des sommets sont dans l'emprise** (grille projetée dense au premier plan, R 52 m autour
d'un point à 28 m). **B seul dépasse déjà 2 ms à 8 px.** 16 fils matériels disponibles.
