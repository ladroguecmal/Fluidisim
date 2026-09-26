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

Session : S391 — **en cours**. **A321** : à 30 Hz, la scène de la porte B explose en 24 à 40 s, quel que soit le solveur de
pression ; à 60 Hz la minute tient ([angle mort](../docs/registres/ANGLES-MORTS.md), [preuve](../docs/validation/MULTIGRILLE-3D-S385.md)
§5). Demande de l'utilisateur (2026-09-26) : *« Corrige A321 d'abord »* — avant la pluie, pièce 5, et C3b. Agent : Claude
Opus 5.5, Claude Code (application de bureau) au poste — fichiers, git, cargo, Python, RTX 5070 Laptop, Godot 4.6.3.

**Hypothèse, écrite avant la mesure.** La prédiction advecte δ par Euler explicite et différences centrées — `advect` (`u'·∇u'`)
et les termes croisés d'`extra` (`U·∇u'`, `u'·∇U`) ; le cœur fait de même (`advect_mobile3`, `extra3`). Ce schéma (FTCS) est
**instable pour tout pas** : il porte une anti-diffusion `(dt/2)·U_a·U_b·∂_a∂_b`, de taux ≈ `|U|²·dt/(2·dx²)` au plus fort, sur
les modes de deux à quatre mailles. **Prédictions** : (1) le temps d'explosion varie à peu près comme `1/dt` ; (2) le mode qui
croît est de l'échelle de la maille, dans les vitesses ; (3) éteindre `U·∇u'` (et `u'·∇u'`) supprime l'explosion à 30 Hz. Second
suspect : le transport de la hauteur par des flux centrés (`fluxes`, la bande de B).

**Critères.** (1) **Instrument** : le banc `--delta3d-a321`, commutateurs éteints, rend la production au bit (empreintes de
`--delta3d-empreinte`). (2) **Attribution** : temps d'explosion à 16,7, 25 et 33,3 ms ; échelle du mode qui croît ; un témoin
par terme éteint (L136) — le terme dont l'absence retire l'explosion est nommé, sinon l'hypothèse est dite fausse. (3) **Le
correctif**, choisi sur (2) et déclaré ici **avant** son code, dans la référence et la production, même formule. Reçu si : la
scène tient **deux minutes** à 30 et à 60 Hz ; les trois cas de cuve restent à 3 mm de la référence ; les réceptions de
dispersion de la référence tiennent leurs tolérances ; empreintes nouvelles expliquées champ par champ. (4) Suites, zéro
avertissement.

### Plan

- [x] **P1** — jeton, plan seul.
- [x] **P2** — l'instrument : commutateurs de banc dans le pas (éteints : au bit), banc `--delta3d-a321` (croissance par
  seconde, échelle du mode, premier pas non fini) ; critère 1.
- [x] **P3** — l'attribution : pas de temps, témoins terme par terme ; critère 2 ; le correctif déclaré ici.
- [x] **P4** — le correctif dans la référence (cœur) : option `Volume3::enable_advection_correction`, éteinte par défaut
  (aucune empreinte du cœur ne bouge) ; essai de von Neumann : sans elle un mode de quatre mailles croît, avec elle non (vu
  échouer) ; [ADR-209](../docs/adr/ADR-209-l-advection-de-delta-au-second-ordre-en-temps.md).
- [x] **P5** — le même dans la production, **actif par défaut** ; critère 3 (deux minutes à 30 et 60 Hz, cuves avec la
  référence corrigée, empreintes nouvelles expliquées) ; coût du pas et porte C remesurés (deux parts ≤ 2 ms).
- [x] **P6** — critère 4 ; preuve ; A321, file, feuille de route.
- [ ] **P7** — rituel.

### Notes de reprise

**P2 — critère 1 tenu**, au second essai. Commutateurs dans `Step::switches` (l'ancien mot de remplissage) ; banc
`viewer/src/delta3d_a321.rs`, `--delta3d-a321`. **Vu en chemin** : des branches `if (switches…)` compilées dans les
pipelines de production changeaient les empreintes (60 pas `0xf1d768aa…` au lieu de `0x6e90a7ae…`), commutateurs à zéro — le
compilateur réarrange l'arithmétique voisine (L345). Remède : constante `override BENCH_SWITCHES`, vraie seulement dans une
seconde série de pipelines (`step_bench`), prise quand un commutateur est allumé ; empreintes d'avant retrouvées au bit.
Le bit 32 prend les pipelines de banc sans rien éteindre : témoin du bruit d'arrondi, la scène amplifiant tout écart.

**P3 — critère 2 tenu : l'hypothèse FTCS est confirmée.** Production à 30 Hz (`--delta3d-a321`) : la part de l'échelle
de la maille dans les vitesses horizontales monte de 0,4 % (1 s) à 2 % (10 s), 8 % (30 s), 31 % (39 s), puis la vitesse
saute de 1,4 à 11 m/s au point (52, 62, 11), juste sous la surface ; explose à 40 s. **Pas de temps** (prédiction 1) :
explose à **72 s** à 16,7 ms, **33 s** à 25 ms, 23 à 40 s à 33,3 ms — à peu près `1/dt`. **Témoins à 30 Hz** (60 s) :
pipelines de banc sans rien d'éteint 23 s (bruit : 23 contre 40 s) ; sans `u'·∇u'` tient 60 s, mais explose à **68 et 75 s**
sur deux minutes (deux réalisations), la part de la maille montant encore à 15 % — l'auto-advection accélère la fin, elle
n'est pas toute la cause ; sans `U·∇u'` 12 s ; sans `u'·∇U` 23 s ; sans le résidu 17 s ; sans la bande 27 s. **L'épreuve
directe** (commutateur 64) : le terme que l'Euler explicite omet, `+(dt²/2)·Σ V_a·V_b·∂_a∂_b u`, `V = U + u'` — la scène
**tient deux minutes à 30 Hz et à 60 Hz**, la part de la maille reste sous 1,2 % (u) et 3,2 % (w).

**Le correctif, déclaré avant son code.** Ce terme, dans la prédiction, après l'advection de `u'` et avant les termes du fond
et l'éponge ; mêmes faces que la prédiction ; une direction omise dès qu'un voisin sort de la grille de l'axe ; `V` = vitesse
de B à la face plus `u'` interpolé (`collocated`) ; vitesses du début du pas. **Production : actif par défaut** (c'est elle
qui explose). **Référence : option** `Volume3::enable_advection_correction`, éteinte par défaut — les réceptions du cœur
restent au bit, le chemin corrigé s'y reçoit par un essai de von Neumann et par les cuves ; la migration du défaut du cœur
attend son déclencheur (ADR-209). Écarté : l'amont du premier ordre (viscosité `|U|·dx/2` ≈ 0,25 m²/s, 3 %/s sur le paquet
de 16 m), les Runge-Kutta (trois prédictions par pas), le semi-lagrangien (hors du schéma de référence).

**P4 — la référence.** `code/water-core/src/delta3d_advection.rs` : option `enable_advection_correction`, terme
`correct_advection3` appelé après `advect_mobile3` dans le pas mobile (`None`) et le pas couplé (échantillons de B) ;
`face_index3`, `velocity3`, `collocated3` passés `pub(super)`. **Essai** `second_order_advection_stops_the_ftcs_growth_s391`
: courant de 2 m/s, Courant 0,264, différence de deux passages (avec et sans le mode) pour écarter ce que les murs font au
courant — **FTCS ×7,547 en 60 pas (×7,4 prédits), corrigé ×0,1339 (×0,134 prédits)**. Premier essai sans la différence :
×18 et ×10, les murs dominaient. Suite du cœur **681 réussis**, 0 échec, 18 ignorés, zéro avertissement — l'option éteinte ne
change rien. [ADR-209](../docs/adr/ADR-209-l-advection-de-delta-au-second-ordre-en-temps.md) ; preuve ouverte,
[A321-S391](../docs/validation/A321-S391.md).

**P5 — critère 3 tenu.** `btd` actif par défaut dans `predict` (commutateur 64 : le retirer) ; les 14 `Volume3` des bancs de
`delta3d_step.rs` allument l'option. **Durée** : deux minutes tenues à 33,3, 25 et 16,7 ms et en multigrille 6 ; **cinq
minutes à 30 Hz** ; témoin sans le terme : 23 s. **Cuves** (Jacobi 64) : cas 3 2,6 / 1,9 / 2,4·10⁻⁸ m ; cas 2 7,1·10⁻⁷ ; cas 1
2,4·10⁻⁸ / 3,2·10⁻⁷ (5 cm), 1,8 / 2,5·10⁻⁵ (10 cm). **Dispersion** du cas 3 inchangée à la sixième décimale (+2·10⁻⁵ point à
48). **Empreintes** 60 pas `0xacd172ae…` / `0x6d20e53d…`, 600 pas `0x28f35d9d…` / `0x5c25dd80…` (au centre, 5·10⁻⁵ m à 1 s).
**Porte C** : 1,910 / 1,927 ms q99 (S348 : 1,848 / 1,918), secteur 96 %. Afficheur 37 réussis, 2 ignorés (l'essai de la
carte lancé : réussi), zéro avertissement.

**P6 — critère 4 tenu** : cœur 681 réussis, 18 ignorés ; afficheur 37 réussis, 2 ignorés ; zéro avertissement. Preuve
[A321-S391](../docs/validation/A321-S391.md) ; A321 **corrigée** (angles morts, note datée) ; file : la ligne A321 remplacée par
ADR-209 D3 (le défaut du cœur, avec son déclencheur) ; feuille de route (porte C) ; index (ADR-209, preuve) ; note datée dans
MULTIGRILLE-3D-S385 §5.

