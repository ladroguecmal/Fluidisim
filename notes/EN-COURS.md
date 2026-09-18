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

Session : S277 — en cours
Agent : Claude Opus 5, Claude Code desktop ; fichiers, git, cargo, Python, GPU local.
Entrée : l'utilisateur signale que `cargo run … -- --delta --delta-direct` depuis la racine
« n'ouvre rien ». Constat (terminal) : cache des rejeux cherché relativement au dossier courant,
donc manqué depuis la racine ; ~3 min de précalcul muet, interrompu. master cafcea6.
Objectif : lancement immédiat depuis n'importe quel dossier, direct sans précalcul, progression
annoncée quand un calcul est nécessaire.

### Plan

- [x] **P1** — amorce, jeton et plan seuls.
- [x] **P2** — cache sous `viewer/` quel que soit le dossier courant ; `--delta-direct` sans
  rejeux ; message et progression du précalcul ; essai du lancement depuis la racine.
- [x] **P3** — verdict R10 : les deux retours de l'utilisateur, leurs causes mesurées, la suite
  en file. *(Découpage déclaré en cours de session : le verdict est arrivé pendant S277.)*
- [x] **P4** — onde injectée : `Live` accepte une surface initiale, `--onde` la pose (bosse
  gaussienne au centre du domaine) ; elle naît, se propage, renaît sur **Début**. Relevé de
  l'amplitude et de la position du maximum sur 30 s.
- [x] **P5** — l'interaction, mesurée : la même onde sur la houle et sur une mer plate ; la
  différence est l'effet de B sur l'onde, et c'est exactement ce que l'utilisateur veut voir.
- [x] **P6** — balayage des régimes : où `u_orbital/c` devient assez grand pour que la déformation
  se voie. **Un seul levier praticable** — la houle cambrée : l'onde plus courte est fermée par la
  résolution du domaine (`DX` = 2 m, il faut `λ ≥ 8·DX` = 16 m, et `σ` = 8 m y est déjà). Cas
  `Hs`/`Tp` : 2/8 (référence, 15 %), 4/8 (31 %), 4/6 (41 %), 6/6 (62 %) ; durée réduite à 10 s,
  onde sur mer plate calculée une seule fois.
- [x] **P6b** — ce qu'il regarde : pose qui montre l'onde traverser la houle, capture, commande.
- [ ] **P7** — rituel §6.

**P4 relevé** (`--delta --onde-mesure`, bosse 0,6 m, σ 8 m) : la bosse se sépare en deux fronts
qui avancent de 13 m à 2 s jusqu'à ≈ 105 m à 24 s, soit **4,2 m/s** de vitesse apparente ;
l'amplitude tombe de 0,59 m à ≈ 0,20 m en 2 s (séparation) puis décroît lentement par dispersion,
0,12–0,16 m après 20 s. **Les deux fronts ne sont pas symétriques** alors que l'onde initiale
l'est : à 8 s le maximum gauche est à −41 m et le droit à +23 m. La houle se propage vers +x et
c'est la seule chose qui brise la symétrie — mais le relevé suit le maximum de |η'|, qui saute
d'une oscillation à l'autre : **P5 doit le prouver par différence, pas par ce relevé**. Au-delà de
24 s le maximum tombe dans l'éponge (|x| > 96 m) et ne désigne plus le front : restreindre P5 au
domaine utile.

**P5 : l'interaction est faible — 4 à 8 % de l'onde en régime** ([mesure](../docs/validation/ONDE-INJECTEE-S277.md)).
Symétrie gauche/droite brisée de 1 à 4 % seulement ; contrôle interne : sur mer plate l'énergie des
deux côtés reste égale à 1,000 exactement. Cohérent avec `u_orbital/c ≈ 15 %` (0,785 m/s contre
≈ 5,1 m/s de vitesse de phase, λ ≈ 16,6 m). **Donc invisible à l'œil dans cette houle** : 5 % de
10 cm font 5 mm. Ce que l'utilisateur veut voir demande un régime à `u_orbital/c` grand — houle plus
cambrée ou onde plus courte — **non mesuré**, c'est la suite. Une exécution coûte 2 min 16 (trois
domaines × 30 s) : raccourcir la durée pour un balayage de régimes.

**P6 : la cambrure commande.** Écart/onde 8,8 % → 28,2 % → 73,3 % quand `ak` va de 0,063 à 0,224
(`Hs`/`Tp` 2/8, 4/8, 4/6). **4 m / 8 s est le compromis lisible** : 28 % de déformation, correction
couplée (75 mm) du même ordre que l'onde (87 mm). À 4/6 la correction couplée écrase l'onde
(223 mm). **6/6 refuse : `pas δ en direct : Domain`** — garde de géométrie non tenue, cause non
diagnostiquée, peut-être la hauteur libre (REST 96 m, sommet 102 m). Ne pas en conclure une limite
physique sans l'avoir cherchée.

*Découpage déclaré le 2026-09-18 à 21:41 : l'utilisateur a demandé l'onde injectée maintenant.
Le rituel, déclaré P3 puis P4, devient P7 — il reste la dernière étape.*

### Notes de reprise

Rejeu partiel écrit à la racine par le lancement interrompu : **retiré** en P2.

P2 mesuré depuis la racine, binaire déjà bâti : fenêtre en **1,1 s** avec `--delta`, **1,4 s** avec
`--delta --delta-direct` (contre ~3 min muettes avant). `--delta-mesure` relit les deux caches en
1,2 s et rend les chiffres de S275 au chiffre près. Cache du 16 ms effacé puis recalculé :
**identique au bit** à l'ancien, 38,6 s annoncées 34 s après vingt pas. Essais 21/21,
`--delta-direct-verify` identique au bit au rejeu (coût médian 23,1 ms).

Tous les chemins `captures/` du viewer sont ancrés au crate par `captures!`, pas seulement le
cache : une revue lancée depuis la racine écrivait ses images à la racine.

**Verdict R10 reçu le 2026-09-18, en deux points** : (1) le motif de surface est trop répétitif,
pas réaliste ; (2) les vagues et vaguelettes doivent interagir avec l'onde.

Causes **mesurées** en P3 par `code/water-core/examples/bandes_s277.rs` — houle δ contre mer
S201 : λ 40,1–198,7 m (rapport 5,0) contre 3,7–210,7 m (56,2) ; étalement 0,00° contre 87,19° ;
écart-type de η le long des crêtes **0,0000 m** contre 0,2872 m. Le constat de l'utilisateur est
exact. Lecture du code qui l'explique :
- la houle de `--delta` est la plus pauvre du dépôt **par choix de mesure** : `spread_turns: 0.`
  (étalement nul), bande `0,7–1,6 fp`, 32 composantes colinéaires. La scène S201 a `spread_turns:
  0.25` et une bande `0,5–4 fp` ; `--houle` y assemble en plus une houle longue et 64 composantes
  de queue ;
- le domaine δ est une **tranche 2D** `Domain { nx, nz, dx }` — aucune dimension y. Le profil de
  128 colonnes est répété tel quel sur 200 m de large, fondu compris. L'étalement nul n'est pas un
  choix esthétique : une houle étalée ne serait pas constante le long de y, et la tranche ne
  saurait pas la porter ;
- la queue spectrale S256 (les vaguelettes) est un **habillage de pentes non couplé à δ**, écrit
  tel quel dans DELTA-VISIBLE-S275. Elle ne peut donc pas interagir avec l'onde aujourd'hui.

Les deux retours désignent donc la même limite structurelle, pas un défaut de rendu : **δ général**
au sens d'ADR-127.
