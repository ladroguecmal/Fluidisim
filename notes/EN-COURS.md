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

Session : S240 — en cours
Agent : Claude Code, Opus 5 (fichiers, git, cargo, Python/numpy/sympy et GPU local disponibles)
Entrée : « Continue », master propre à `45d24a5`, une seule copie, jeton libre, secteur.
Hôte GPU vérifié sur cette machine : RTX 5070 Laptop, DX12, `--smoke` 120 images, code 0.

**Objectif.** **I-06 pour la pile graphique** : que les allocations par image de l'hôte soient
**mesurées** — et donc que l'invariant soit tenu, ou que sa portée soit requalifiée avec provenance
pour cette couche, comme ADR-139 l'a fait pour I-08 et V.
**Ce que la lecture fixe.** I-06 dit « aucune allocation à l'exécution » et le cœur la reçoit par
allocateur compteur (`Arena`, refus après `seal`). Pour l'hôte, elle n'a **jamais été mesurée** :
[CADENCE-HOTE-S225](docs/validation/CADENCE-HOTE-S225.md) §Suite l'écrit noir sur blanc (« les
allocations de la pile graphique, I-06, toujours non reçues ») et la feuille de route la garde dans
les travaux nécessaires de J1 (ADR-131 D6). Le budget de 2 ms (ADR-125) et **A265** — le recouvrement
CPU/GPU varie avec la charge sans que la mesure l'explique — donnent le consommateur : une allocation
par image est un suspect direct de gigue.
**Thèse.** Ce qui n'est pas compté n'est pas tenu. Un allocateur compteur global dans le binaire de
l'hôte, sans dépendance nouvelle, sépare **ce qui est à nous** de **ce que la pile impose**. Le
premier se supprime ou se justifie ; le second se chiffre, et décide si I-06 vaut pour cette couche.
**Critères, déclarés avant construction.** (1) Allocations par image en régime — nombre et octets —,
les dix premières images écartées comme le banc le fait déjà. (2) **Attribution** par phase :
préparation CPU du sillage, planification de la grille, encodage et soumission GPU, acquisition de
la surface, boucle d'événements. (3) Séparation à nous / à la pile, par site nommé. (4) **La mesure
ne déplace pas les chiffres publiés** : `VERIFY` à 3 mm et `BENCH` GPU rejoués avec et sans
l'instrument, écart publié. (5) Ce qui est à nous est supprimé, ou justifié par écrit ; re-mesure.
(6) Coût (ADR-131) : techniques présentes, absentes, domaine.
**Arrêt.** I-06 reçue pour l'hôte, ou **requalification datée** de sa portée pour la pile graphique,
avec le chiffre et la cause. Aucun seuil inventé, aucun banc modifié pour obtenir du vert.

### Plan

- [x] **P1** — amorce, jeton, plan seuls.
- [x] **P2** — lectures ciblées (boucle d'image de l'hôte, bancs S225/S235, I-06 et ADR-006) ;
  protocole écrit avant mesure, avec ce qui serait reçu et ce qui serait une requalification.
- [ ] **P3** — instrument : allocateur compteur global du binaire de l'hôte, marquage par phase,
  sans dépendance nouvelle ; contrôle qu'il ne déplace pas `VERIFY` ni `BENCH`.
- [ ] **P4** — mesure : allocations par image en régime, taille et attribution, scène S235 à
  960×540, dans le champ et hors champ.
- [ ] **P5** — ce qui est à nous : supprimé ou justifié, puis re-mesuré ; verdict.
- [ ] **P6** — rituel §6, file, feuille de route, jeton.

### Notes de reprise

P2 : protocole `docs/validation/ALLOCATIONS-HOTE-S240.md`. **Base avant instrument** (`--multi
--cadence`, 960x540, secteur, ce jour) : intervalle 4,7062 / 5,8529 / **16,4724** ms ; CPU
3,9828 / 4,8242 / **15,7814** ms ; sillage 3,1185 / **13,5879** ; transfert 0,2209 / 1,2159 ;
acquisition 0,0237 / 0,1035 ; presentation 0,5771 / 2,2853 ; GPU eau 0,4399 / 0,4552.
**Le maximum vaut quatre fois la mediane et le GPU ne bouge pas** : la gigue est CPU, surtout dans
la preparation du sillage. Attribution prevue **par phase**, pas par site d'appel (une pile
d'appels demanderait une dependance et deplacerait la mesure) — limite declaree.
A verifier en P3 : les tampons de millisecondes du banc grandissent pendant la mesure et faussent
donc leur propre banc ; les reserver.

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