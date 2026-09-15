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

Session : S241 — terminée
Agent : Claude Code, Opus 5 (fichiers, git, cargo, Python/numpy/sympy, GPU local et **accès web**)
Entrée : « Continue, j'ai découvert un projet nommé Niagara Pyro, cela peut être intéressant à
étudier. » Master propre à `dc0a236`, une seule copie, jeton libre, secteur.

**Objectif.** Étudier **Niagara Fluids** d'Unreal Engine — dont *Pyro*, son solveur de gaz 3D — comme
**comparable externe**, et en tirer ce qui s'applique à ce projet : ce qu'il corrobore, ce qu'il
n'autorise pas, et quel lot il désigne. La demande de l'utilisateur prime sur la suite automatique
(REPRISE §6.7) : la préparation CPU du sillage, prévue en S241, repart en file avec son déclencheur.
**Ce que la lecture fixe, et la règle qui gouverne ce lot.** Une documentation d'éditeur est une
**affirmation de fournisseur**, pas une mesure de ce projet. REPRISE §2 interdit de transformer un
fait externe inconnu en hypothèse acquise, I-14 exige une provenance pour toute valeur, et ADR-028
rappelle qu'il n'y a pas d'autres équipes : Epic n'est pas un interlocuteur, c'est une publication.
**Aucun nombre lu chez un éditeur n'entre dans ce dépôt comme seuil.** Chaque affirmation portera son
URL et son statut : documenté, déduit, ou non trouvé.
**Thèse.** Un comparable ne se juge pas sur ce qu'il fait mieux, mais sur **les questions qu'il ne
pose pas**. Si un moteur qui expédie du fluide temps réel ne demande jamais la convergence de sa
pression, cela ne dit pas que notre critère est faux — cela situe la classe de fidélité que δ vise,
et cela dit ce que coûtera d'y arriver.
**Critères, déclarés avant l'étude.** (1) Toute affirmation externe porte sa source et son statut.
(2) Confrontation **à nos propres nombres déjà mesurés** — architecture ADR-001, pression de δ
(S238/S239), coût (S235/S240), volumes bornés de J2 — sans nouvelle campagne. (3) Ce qui est
corroboré, ce qui est contredit et ce qui reste indécidable sont **séparés**. (4) Sortie : un
document de comparables **durable** (pas un par session), des points de file datés avec déclencheur,
et l'index. (5) Aucun ADR, aucun seuil, aucune ambition modifiée : ADR-127 n'est pas rouvert.
**Arrêt.** Le document écrit et la file à jour ; ou constat que le comparable ne change aucune
décision, écrit tel quel.

### Plan

- [x] **P1** — amorce, jeton, plan seuls.
- [x] **P2** — étude sourcée : architecture de Niagara Fluids, solveur de pression de Pyro, 2D contre
  3D, cuisson en volumes épars ; chaque affirmation avec URL et statut.
- [x] **P3** — confrontation à nos nombres mesurés : ADR-001, pression de δ (A273/A275), coût de δ
  contre ADR-125 et ADR-131, volumes bornés de J2. Aucune campagne nouvelle.
- [x] **P4** — `docs/COMPARABLES-EXTERNES.md`, file active (déclencheurs, dont la préparation CPU du
  sillage remise en file), index.
- [x] **P5** — rituel §6, jeton.

### Notes de reprise

P2 : `docs/COMPARABLES-EXTERNES.md` — porteur **durable**, un comparable par section datee, jamais un
document par session. Trois trouvailles qui portent : (1) chez Epic, la pression se regle par un
**nombre d'iterations et un facteur de relaxation**, et **aucun critere de convergence n'est expose** ;
(2) les gabarits **2D sont pour les jeux, les 3D pour les cinematiques**, et le temps reel d'un gaz 3D
couteux passe par la **cuisson** en volume epars ; (3) l'eau peu profonde d'Epic est un **champ de
hauteur** bon marche pour grandes surfaces — meme partage que B/W contre delta.
Non trouve (donc inconnu, pas absent) : methode du solveur, valeur par defaut des iterations,
precision de la grille, cout par image chiffre, toute mesure d'erreur physique.

P3 : confrontation faite **sans campagne nouvelle** — tous les chiffres viennent de S230 (dt = 1/60 s :
0,0842 / 0,6822 / **5,5125 ms** a 128 / 512 / 2 048 mailles) et du releve `delta_precision` de S239
(dt = 2 ms : 0,107 / 0,602 / 4,234 / **35,51 ms** jusqu'a 8 192). Loi : quadrupler les mailles
multiplie le temps par ~8, coherent avec O(N) x O(racine N) du gradient conjugue et avec les
iterations mesurees (28/58/112/219/417).
**Trouvaille de priorite** : ADR-125 donne 2 ms a toute l'eau ; a 2 048 mailles delta seul vaut
**2,8 fois** ce budget, et a 8 192 une image en demande ~296 ms. Donc **a la taille ou A275 mord,
le cout est deja deux a trois ordres de grandeur au-dessus** : un lot qui ne corrigerait que la
precision a 32 768 mailles ne debloquerait rien. L'ordre de la file est a renverser.
Cout a 32 768 mailles : **non mesure** (417 iterations mesurees, c'est tout).

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