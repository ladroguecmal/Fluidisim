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

Session : S245 — en cours
Agent : Claude Code, Opus 5 (fichiers, git, cargo, Python/numpy/sympy, GPU local et accès web)
Entrée : « Continue », master propre à `feb1dd9`, une seule copie, jeton libre, secteur, Maillons 1.

**Objectif.** Attaquer le **nombre d'itérations** de la pression de δ par une **multigrille
géométrique employée comme préconditionneur** du gradient conjugué, sur le chemin à **couvercle
fixe**. S244 l'a désignée : le coût d'une itération est stable en structure à toutes les tailles,
c'est leur nombre qui double à chaque raffinement — 30, 61, 114, 220, 425 —, et la multigrille est le
**seul levier dont le gain croît avec la taille**.
**Ce que la lecture corrige.** `jacobi = self.mobile` : le chemin à couvercle fixe — celui que S230
et S244 mesurent — n'a **aucun** préconditionneur ; `dir = res`. L'en-tête ADR-131 de
COUT-DELTA-S244 annonce « Jacobi diagonal » : c'est vrai du mode mobile, faux du mode mesuré. **Note
corrective datée à porter en P2.** Et Jacobi n'y servirait à rien : sur une grille uniforme la
diagonale est constante, donc un préconditionnement scalaire, sans effet sur le gradient conjugué.
**Ce qui rend ce lot sûr.** Un préconditionneur **ne peut pas rendre la réponse fausse** : il change
les directions de recherche, jamais le test d'acceptation. Le résidu premier (10⁻⁶) et la tolérance
physique (ADR-144) restent les mêmes portes. Le pire cas de ce lot est donc « ça ne gagne rien, et
c'est mesuré », jamais « ça donne un résultat faux ».
**Ce que la méthode exige en retour.** Le gradient conjugué **n'est valide qu'avec un
préconditionneur symétrique défini positif**. La recette retenue l'assure par construction : lissage
de **Jacobi amorti** (diagonal, donc symétrique), **restriction R = Pᵀ** avec prolongation bilinéaire,
et **autant de lissages avant qu'après**. La symétrie ne sera pas supposée : elle sera **testée**
numériquement, `⟨M⁻¹x, y⟩ = ⟨x, M⁻¹y⟩`.
**Décisions déclarées avant construction.** (1) Hiérarchie par division par deux, arrêtée dès qu'une
dimension est impaire ou tombe sous quatre. (2) Opérateur grossier **re-discrétisé** avec `dx·2^l` et
ouvertures moyennées — approximation assumée pour les mailles coupées, licite puisqu'il ne s'agit que
d'un préconditionneur. (3) Lissage **Jacobi amorti à ω = 2/3**, valeur qui **se dérive** (elle
minimise le facteur de lissage des modes hautes fréquences du stencil à cinq points), non un réglage.
(4) **Chemin mobile inchangé** : il garde son Jacobi diagonal. (5) Tous les tampons alloués dans
`configure`, avant `seal` (I-06).
**Critères de réception, déclarés avant construction.** (1) Symétrie et positivité du
préconditionneur, testées. (2) **Compte d'itérations** aux cinq tailles de S244, et la **loi de
croissance** qui en sort — c'est la revendication, pas un point. (3) **Coût par pas** mesuré : une
itération coûte plus cher, c'est le produit qui décide. (4) Acceptation d'ADR-144 inchangée :
`delta_precision` dix cas non dégradés, `delta_filters` ordres ≥ 1,8, réception mobile S237/S238
intacte. (5) **Les bits du chemin fixe changeront** : nouvelle empreinte `delta_filters` publiée avec
son explication, ancienne conservée. (6) Aucune allocation après `seal`.
**Arrêt.** Multigrille construite, symétrie prouvée, itérations et coût mesurés, acceptation
conservée. **Ou**, si le lot dépasse la session, la hiérarchie et le cycle validés seuls, le
branchement déclaré en file avec son déclencheur — dit dans le plan, pas laissé à deviner.

### Plan

- [x] **P1** — amorce, jeton, plan seuls.
- [x] **P2** — protocole écrit ; note corrective datée sur S244 ; relevé de référence des itérations.
- [x] **P3** — hiérarchie allouée dans `configure` et opérateurs de transfert, avec leurs essais.
- [ ] **P4** — cycle en V et lissage ; **symétrie testée** avant tout branchement.
- [ ] **P5** — branchement comme préconditionneur ; itérations et coût aux cinq tailles.
- [ ] **P6** — réception ADR-144, empreintes, suite ; rituel §6, ADR, file, jeton.

### Notes de reprise

P2 : protocole `docs/validation/MULTIGRILLE-S245.md` ; note corrective datee portee dans
COUT-DELTA-S244 §6 (« Jacobi diagonal present » etait faux du chemin mesure).
Reference avant modification, chemin fixe, gradient conjugue **nu** : iterations
**30 / 61 / 114 / 220 / 425**, pas 0,089 / 0,699 / 5,256 / ~37-42 / **286,2** ms.

P3 : `code/water-core/src/delta_multigrid.rs` — `Level`, `level_count`, `hierarchy_floats`
(comptage I-06 **avant** allocation), `apply_level`, `diagonal`, `coarsen_into`, `restrict`,
`prolong_add`, `smooth`. Hierarchie construite dans `configure` **apres `cut()`**, la geometrie y
etant figee une fois pour toutes. Regle d'arret : paire et au moins 8 -> 64x32 donne **3** niveaux
(32x16, 16x8, 8x4), 16x8 en donne 1, une dimension impaire en donne 0.
Trois essais verts : (1) `apply_level` rend **les memes bits** que `Volume::apply` sur la grille
fine, fond plat et fond coupe — c'est ce qui interdit aux deux ecritures de diverger (L137) ;
(2) restriction et prolongation **adjointes a un facteur quatre pres**, ce qui rendra le cycle
symetrique ; (3) hierarchie, dimensions, `inv` double au carre, diagonale positive sur mouille et
nulle sur sec.

---

Notes de S244, conservées pour référence immédiate :

Decomposition : ecritures disjointes **67-73 %** du pas a toutes les tailles (dont `apply` 47 %),
reductions 12-13 %, reste hors boucle. Iterations **30 / 61 / 114 / 220 / 425** a 128 / 512 / 2 048 /
8 192 / 32 768 mailles. Pas mesure : 0,089 / 0,699 / 5,256 / ~37-42 / **286,2 ms**.
Prix d'un appel parallele : **~125 us par fil**, presque independant du travail -> parallelisme
**ferme** pour cette boucle (passes de 21,7 / 5,5 / 3,8 us).
Echange des boucles d'`apply` : exact au bit, **aucun gain a aucune taille**, annule.
Les pas aux grandes tailles sont bruites ; ce sont les **passes** qui font foi.

---

Notes de S243, conservées pour référence immédiate :

`parallel_fill_f32(out: &mut [f32], grain, fill: &(dyn Fn(usize, &mut [f32]) + Sync))`, defaut du
trait = reference sequentielle ; `ScopedJobs` dans le harnais (thread::scope, portion contigue par
fil, aucun tampon). Garantie **inconditionnelle** : ni grain, ni fils, ni ordre (ADR-146).
x2,63 a huit fils sur la preparation du sillage ; **16 fils sont plus lents que 8**.
Un fil coute ~67 us a creer **et alloue** : le chemin d'image reste a un fil (A278).
Le grain encode le travail par element et seul l'appelant le connait.

---

Notes de S242, conservées pour référence immédiate :

Poste dominant : `ModalPressure::sample`, 0,87 ms par troncon actif sur 4 096 noeuds, **87 %** de la
preparation ; cout fixe par noeud 76 ns. Six empreintes du banc `sillage_troncons` :
`0x95a8239f6f9cc139`, `0x0af49e8f4c2aa21c`, `0xe14cee491a007edc`, `0x24b5f8e1f47300d0`,
`0x393d0b9d526daf84`, `0xdf150d01e0afe270`. Temoin de bruit : une variante neutre mesure +2,6 a
+7,7 % en fenetre de forcage. Impasse : `phase(-p)` n'est pas le conjugue **au bit** (A277).

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