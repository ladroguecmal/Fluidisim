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

Session : S206 — terminée
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
- [x] **P3** — leviers : (L1) parallélisme `std::thread::scope` 1/2/4/8/16 fils, temps mur ;
 (L2) table radiale à matrice de Bessel précalculée — noyau N×M mesuré sur tableaux de la
 bonne taille, précalcul et mémoire par impact ; (L3) densité `c`. Nombre de composantes de B :
 déduit de B1 (48 ns), pas remesuré.
- [x] **P4** — campagne release et publication `COUT-IMAGE-S206` : tableau par configuration
 contre 2 ms, compatible / incompatible, `paquets_W_max` confronté.
- [x] **P5** — arbitrage explicite : options mesurées, dégradations, décideur ; ce qui est
 technique se tranche (ADR si décision), ce qui touche l'ambition, les dépendances ou le sens
 du budget (temps mur ou CPU) remonte à l'utilisateur. Feuille de route et file active.
- [x] **P6** — rituel §6 complet.

### Notes de reprise

P2 (`frame_cost`, release, un fil, 3 chauffes/11 images) — la scène vient de `render_impact.rs`
inclus comme module, pas recopiée ; 15 essais. Grille projetée :
| pas | sommets | dans R | B seul méd/max ms | image méd/max ms | B ns/pt | B+W ns/pt | image/budget |
| 8 px | 2 240 | 1 918 | 2,942 / 3,039 | 17,195 / 18,715 | 1 314 | 8 744 | 8,6 |
| 4 px | 9 044 | 7 652 | 12,012 / 17,666 | 74,013 / 84,157 | 1 328 | 9 431 | 37,0 |
| 2 px | 36 160 | 30 614 | 42,798 / 46,337 | 279,755 / 306,646 | 1 184 | 8 924 | 139,9 |
**86 % des sommets sont dans l'emprise** (grille projetée dense au premier plan, R 52 m autour
d'un point à 28 m). **B seul dépasse déjà 2 ms à 8 px.** 16 fils matériels disponibles.
P3 (`frame_cost levers`, release). **L1 parallélisme**, fils lancés et rejoints dans l'image,
temps mur médian (accélération / image÷budget) ; bits identiques à 1 fil dans les 15 cas :
8 px : 1 f 18,19 · 2 f 12,20 · 4 f 7,39 · 8 f 5,01 · **16 f 3,63 ms** (×5,01 ; 1,8) — lancement seul 1,57 ms.
4 px : 71,57 · 45,09 · 26,91 · 16,22 · **16 f 10,05 ms** (×7,12 ; 5,0) — lancement 1,39 ms.
2 px : 289,07 · 178,92 · 105,66 · 61,34 · **16 f 36,19 ms** (×7,99 ; 18,1) — lancement 1,24 ms.
**L2 table à matrice de Bessel** (N256 × M, valeurs synthétiques, coût indépendant des valeurs) :
pas λ/16 = 0,209 m, M 249 : **0,0266 ms/image par impact**, 502 Ko/impact, erreur table 0,0061 mm,
Hermite 8,1 ns/pt, 75 impacts dans 2 ms, **4096 impacts = 109 ms et 2,0 Go** ;
pas λ/8 = 0,419 m, M 125 : 0,0164 ms, 254 Ko, erreur 0,0901 mm, 8,8 ns/pt, 122 impacts, 67 ms.
Lecture : avec L2, W passe de ~9 µs/sommet à 27 µs/impact + 8 ns/sommet — **B (1,3 µs/sommet,
N32) devient le goulot**. Estimation 4 px un fil avec L2 : 12,0 + 0,03 + 0,06 ≈ 12,1 ms.
P4 : COUT-IMAGE-S206 publié (scène, densité exigée — c ≤ 5,8 px pour B, ≤ 2,75 px pour les
anneaux à 28 m —, un fil, L1, L2, `paquets_W_max`, B sur CPU, verdict, non mesuré). Seconde
exécution un fil archivée : 8 px 18,45 ms, 4 px 72,42 ms, 2 px 293,15 ms (écart ≤ 7 %).
Verdict : **incompatible sur CPU à toute densité qui montre l'impact** ; W n'est plus le goulot
avec L2, B l'est (1,2–1,3 µs/sommet).
P5 : **ADR-129 actée** (chemin d'image de W par table de Bessel précalculée, pas ≤ λ/8,
cosmétique jamais autoritaire, `paquets_W_max` retiré du profil par I-16, construction S207 avec
réception écrite). Note datée ADR-012 (qui prévoyait `gpu_sim_ms = 2,5`, jamais exercé).
**Arbitrage « chemin de rendu et hôte de J1 »** fusionné avec A247 et posé : (A) GPU par hôte
avec dépendances — recommandé ; (B) CPU seul, 2 ms en temps mur multi-cœurs ; (C) profil changé ;
(D) 8 px, perd l'impact. Feuille de route §4 et J1, file active mises à jour. À poser à
l'utilisateur en fin de session, après le rituel committé.
P6 : journal, A250, L283 ; index, README, REPRISE (§3, §4, file active), feuille de route et
file active faites en P5. Décomptes 129/250/283/18/6/23 vérifiés. Jeton libre. Arbitrage de rendu
à poser à l'utilisateur après ce commit.
