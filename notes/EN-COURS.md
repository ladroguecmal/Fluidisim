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

Session : S242 — en cours
Agent : Claude Code, Opus 5 (fichiers, git, cargo, Python/numpy/sympy, GPU local et accès web)
Entrée : « Continue », master propre à `39a2af8`, une seule copie, jeton libre, secteur.
**Maillons 1** : cette session doit faire avancer une capacité.

**Objectif.** Faire baisser la **préparation CPU du sillage**, poste dominant du budget de l'hôte
depuis le LOD de S234 : **3,17 ms de médiane et 13,2 ms de maximum** par image, contre **0,44 ms** de
GPU eau (S240), et 2 ms pour toute l'eau (ADR-125). Sans changer un seul bit publié.
**Ce que la lecture fixe.** `Timeline::render_components` fait, à chaque image et pour chacun des
**4 096 nœuds**, une boucle sur les **24 segments** du journal (trois sillages de huit tronçons),
testant `mode.birth() < time && time < mode.forcing_end()` — soit **98 304 tests par image**. Or
`fold` dit déjà, en commentaire et dans son code, que **naissance et durée sont les mêmes pour tous
les nœuds** : il ne lit que la rangée 0 pour les fins. L'ensemble des segments actifs ne dépend donc
**que du temps**, jamais du nœud. Les huit tronçons d'un sillage durent 2 s chacun : à tout instant
**au plus un segment par sillage est en forçage**, et après 16 s **aucun**.
**Thèse.** Un test dont le résultat est le même pour les 4 096 nœuds se calcule **une fois par
image**, pas 4 096 fois. Hisser cette sélection hors de la boucle des nœuds ne change **aucune
opération flottante** : les mêmes termes sont additionnés dans le même ordre. Le gain est donc
gratuit au bit, et il croît avec le nombre de tronçons — c'est-à-dire avec la scène.
**Critères, déclarés avant construction.** (1) **Décomposition mesurée avant toute modification** :
où vont les 3,17 ms — repli, boucle des nœuds, boucle interne des segments, boucle de sortie — et
combien de segments sont actifs. (2) **Au bit** : `--multi --verify` rend 0,368476 mm, `--multi
--retour` rend 0 image différente, et les coefficients publiés du sillage sont comparés au bit avant
et après. (3) **Coût publié** : CPU sillage médian, p95 et maximum, avant et après, même banc ;
allocations de `update` toujours à **zéro** (S240). (4) **Loi contre le nombre de tronçons** :
mesurée à un sillage (8 segments) et à trois (24), avant et après — la revendication porte sur la
croissance, pas sur un point. (5) Coût (ADR-131) : techniques présentes, absentes, domaine.
**Arrêt.** La sélection hissée, reçue au bit et chiffrée ; **ou** constat mesuré que la boucle interne
n'est pas où va le temps — et alors la mesure désigne le vrai poste, et la session le dit. Aucun seuil
modifié, aucune ambition touchée, aucun ADR attendu.

### Plan

- [x] **P1** — amorce, jeton, plan seuls.
- [ ] **P2** — protocole écrit ; instrument de décomposition de la préparation par image.
- [x] **P3** — mesure avant toute modification : où va le temps, et combien de segments sont actifs.
- [ ] **P4** — construction : sélection des segments actifs hissée hors de la boucle des nœuds ;
  tests d'identité au bit.
- [ ] **P5** — réception : `--verify`, `--retour`, cadence avant/après, loi contre les tronçons, coût.
- [ ] **P6** — rituel §6, file, feuille de route, jeton.

### Notes de reprise

P2 : protocole `docs/validation/PREPARATION-SILLAGE-S242.md` + banc `examples/sillage_troncons.rs`,
qui fait varier les troncons au lieu d'instrumenter l'interieur de la fonction.

P3 : **la these du §1.2 est refutee dans sa grandeur.** Apres forcage (aucun segment actif) :
0,3114 / 0,3367 / 0,3709 ms a 8 / 16 / 24 segments — la boucle interne inutile vaut **3,7 us par
segment**, soit 0,0595 ms entre 8 et 24, environ **2 %** des 3,17 ms de l'hote.
**Le poste est `ModalPressure::sample`** : cout fixe par noeud 76 ns ; cout d'un segment actif
**0,87 ms** sur 4 096 noeuds, soit 211 ns par noeud et par segment ; 2,60 ms sur 2,97 = **87 %**.
**Impasse payante a connaitre** : `Complex::phase(negative(p))` n'est PAS le conjugue au bit —
`sin_cos` reconstruit l'angle par quadrant depuis l'entier `2^30 - W`, pas par soustraction
flottante, et a q = 0 le zero signe differe. Meme chose pour « la rotation libre est l'identite
pendant le forcage ». Reduire les sin_cos **change des bits publies** : lot separe, avec relevé.
**Ce qui reste exact et gratuit** : hisser la selection supprime un terme qui croit avec les
troncons **acheves** — 3,7 us par troncon mort et par image, 0,74 ms a 200 troncons. Terme de
croissance supprime, pas constante gagnee.

---

Notes de S241, conservées pour référence immédiate :

`docs/COMPARABLES-EXTERNES.md`. Chez Epic : pression reglee par un nombre d'iterations et un facteur
de relaxation, **aucun critere de convergence expose** ; 2D pour les jeux, 3D pour les cinematiques ;
cuisson en volume epars pour le temps reel. Confrontation : a 2 048 mailles delta seul vaut 2,8 fois
les 2 ms d'ADR-125 ; **le cout passe devant la precision** (A276 avant A275). L320.

---

Notes de S240, conservées pour référence immédiate :

134 allocations et 30 541 octets par image avant correction, mediane = p95 = maximum ; une seule a
nous (12 032 o, le `Vec` de `lod::footprint`). Apres correction : update 0, image 133 / 18 509 o.
L'allocation n'est pas la cause de la gigue d'A265. ADR-145, amendement date sous I-06.

---

Notes de S239, conservées pour référence immédiate :

P2 : protocole `docs/validation/TOLERANCE-PRESSION-S239.md`. `D = rho.theta.Lambda`.
P3 : `Lambda` double a chaque raffinement, `theta` decroit plus lentement que N^-1/2.
P4 : acceptation sur les lignes **franches** ; les lignes a fantome plafonnent a un ulp de leur
propre magnitude. P5 : 348 contre 347 iterations a 8 192 mailles ; degrade a 32 768 ;
empreinte delta_filters 0xfb12b2092df4ee6d ; suite 431/11.

---

Notes de S238, conservées pour référence immédiate :

Base : S237 — refus au pas 397, résidu 1,0492·10⁻⁶ figé 4 000/16 000/64 000 itérations,
divergence 1,45·10⁻⁷, 16 384 mailles ; `Report` construit littéralement en un seul endroit.

P2 : protocole `docs/validation/PRESSION-PLANCHER-S238.md` §1. Pourquoi l'amendement : un seuil
sur `ω` exigeait une constante `n` d'opérations par ligne et un facteur de marge, deux nombres à
fixer — alors que S199 a déclaré avant construction la tolérance physique de la projection, et que
`div u = r/scale` (car `scale·k1 = −1`) en fait une norme maximale du résidu. Détection de stagnation
par **non-décroissance stricte** du vrai résidu d'une relance à l'autre : aucun facteur. Le seuil
10⁻⁶ reste premier pour garder les bits. Estimation d'avant mesure : un résidu au critère S199
porté par le mode le plus lent donne `δu/u ≲ 10⁻⁵·(L/dx)/π² ≈ 1,3·10⁻⁴` à 128 colonnes.

P3 : trace `PRESSURE_TRACE` (cfg(test)) + `backward_error()` (Oettli–Prager, modes linéaire et
mobile). Famille S231 : converge jusqu'à **256×128** (rel. 9,97·10⁻⁷), ω 6,6·10⁻⁷–1,3·10⁻⁶, mais
divergence S199 **1,58·10⁻⁵ à 256** (dépasse 10⁻⁵ alors que le résidu passe). Mobile 5 cm quart de
période : 32/64 colonnes convergés, ω **8,2·10⁻⁵ / 4,7·10⁻⁴** ; 128 : **3 481 relances d'une itération,
résidu alternant exactement 1,0610/1,0630·10⁻⁶, ω = 5,5–6,2·10⁻⁸ = 1 u**. Décisions : (1) ω ne peut
pas être seuil (4,7·10⁻⁴ sur des pas convergés) ; (2) non-décroissance stricte écartée (S233 : hausse
isolée précède convergence) ; (3) **cycle certifié par identité au bit de `p` à une relance
antérieure** (empreinte 64 bits, deux relances), puis acceptation ssi divergence S199 ≤ 10⁻⁵.
Inquiétude à tenir en P5 : à 256×128 famille S231, divergence 1,58·10⁻⁵ > 10⁻⁵ sur un pas **convergé** —
ne change rien à la règle (critère S199 appliqué au cycle seulement) mais montre que le critère S199
n'est pas tenu partout par le chemin existant ; à publier, pas à corriger ici.

P4 : `Report.cycle`, `Report.backward_error`, `PROJECTION_DIVERGENCE_TOLERANCE = 1e-5` (S199 §5),
empreinte FNV-1a 64 bits de `p` aux deux dernières relances ; arrêt sur retour au bit ; `degraded =
résidu > tol && !(cycle && divergence ≤ 1e-5)`. Suite **428 réussis, 7 ignorés** (2 mesures S238
ignorées), empreinte S232 inchangée **avant** les tests nouveaux. Tests : cycle réel (montage S237
P4b) certifié en **9 relances / 51 itérations**, résidu 1,128·10⁻⁶, divergence 5,2·10⁻⁸, ω 2,6·10⁻⁸,
reçu, déterministe au bit ; **Neumann incohérent** (couvercle fermé, face de mur ouverte à 0,5 m/s) :
cycle à 280 itérations, résidu 1,0, divergence 1,0 → **dégradé**, et **ω = 9,5·10⁻¹⁷** (dérive du
noyau qui gonfle `|A||p|`) — deuxième preuve qu'ω ne peut pas être seuil ; identité `div u =
r/scale` : écart 0,18 pour un second membre de 2,48·10⁶, rapport **7,2·10⁻⁸** ≤ 64 ε.

P5 : **deux constructions écartées par la mesure**. (a) Mémoire de deux relances : le pas 397 cycle
sur **420 relances** (482 états, retour de 62) → Brent. (b) Brent seul : dans le banc, 7 386 itérations au
pas 397, pas 404 refusé sous 16 000 ; ω = 0,9–1,3 u à chaque pas. **Règle finale** : `ω ≤ γ₈ = 8u/(1−8u)`
(ligne à quatre faces γ₇ + représentation u) **ou** retour au bit (Brent), acceptation ssi divergence ≤ 1e-5.
Réception (secteur) : 32/64/128 profil 0,850/0,550/**0,252 %**, b₂ 2,44/1,41/**0,71 %** ; au plancher 12/39/38
pas, ω 2,2–7,8 u, résidu relatif jusqu'à **3,0e-6**, divergence ≤ 1,5e-7 ; pire pas 526 itérations ; 256 ms
médian à 128. **Bits de la trajectoire S237 à 32/64 changés** (b₂ au 5e chiffre, volume 3,7e-9 → 1,5e-8) :
promesse §1.2.1 non tenue, publiée ; tests et empreinte S232 `delta_filters` au bit. f64 : 521 contre 789
itérations, p 3,0e-5, **u 5,5e-8** (estimation 1,3e-4). Incohérent : dégradé (42 it, ω 2,2e-8). Test du
repli Brent avec commutateur de test : 9 relances/51 it reçu, incohérent 280 it dégradé. Suite 429/9.
A273 à ouvrir : divergence 1,58e-5 sur un pas convergé à 256×128 (famille S231), **et 1,02e-5 à
128×64 bosse dans `delta_precision`** (domaine S231, critère S199 appliqué là-bas à 32×16 seulement) ;
`delta_precision` : dix cas convergés, aucun au plancher, résidus 4,96–8,70e-7 comme S231. L317 écrite.