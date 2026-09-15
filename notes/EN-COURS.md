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

Session : S243 — en cours
Agent : Claude Code, Opus 5 (fichiers, git, cargo, Python/numpy/sympy, GPU local et accès web)
Entrée : « continue », master propre à `bbc57bd`, une seule copie, jeton libre, secteur, Maillons 0.

**Objectif.** Donner au système un **parallélisme CPU déterministe**, et le faire consommer par le
poste dominant du budget de l'hôte. Prérequis **partagé** : il sert la préparation du sillage
(87 % dans `ModalPressure::sample`, S242) et le coût de δ (A276).
**Ce que la lecture établit — et ce n'est pas un changement d'interface.** SPEC-004 §8.2 **spécifie
déjà** `parallel_for`, « réservé aux écritures disjointes », à côté de `parallel_reduce_ordered`.
Seul le `trait JobSystem` de Rust ne l'a jamais porté, et `SequentialJobs` écrit noir sur blanc :
« le jour où une version parallèle existera, l'assertion *changer `worker_count` change la vitesse,
jamais le résultat* se vérifiera contre celle-ci ». Cette session rend cette phrase vraie.
**Contraintes constatées dans le code.** `water-core` est en `#![forbid(unsafe_code)]` : le cœur ne
peut pas découper une tranche mutable entre fils par pointeur. `state.current` n'est **qu'un
temporaire** de `render_components` (vérifié : aucun autre usage) — la boucle des nœuds est donc une
**application pure** de données en lecture seule vers `out[n]`. `rustc 1.97` offre
`as_flattened_mut`, qui donne `&mut [f32]` depuis `&mut [[f32; 4]]` **sans `unsafe`**.
**Thèse, et la dissymétrie qui la fonde.** Pour une **réduction**, ADR-029 §3 a dû inscrire `grain`
dans le contrat : l'addition flottante n'est pas associative, donc le découpage change la somme.
Pour une **écriture disjointe**, il n'y a aucune accumulation d'une tâche à l'autre : chaque élément
de sortie est écrit par exactement une tâche, depuis des entrées en lecture seule. Le résultat est
donc indépendant du grain, du nombre de fils **et** de l'ordre — une garantie **plus forte** que
celle de la réduction, et c'est elle qu'il faut écrire, pas la recopier.
**Forme retenue, déclarée avant d'être écrite.** `fn parallel_fill_f32(&self, out: &mut [f32],
grain: usize, fill: &(dyn Fn(usize, &mut [f32]) + Sync))` : objet-sûr (aucun générique), sans
allocation dans le cœur, sur le **seul type de tampon flottant que le système publie**. L'hôte
découpe par `chunks_mut` et exécute ; le cœur ne crée aucun fil.
**Erreurs.** `sample` peut échouer. Le chemin parallèle est un **chemin rapide** : à la moindre
sortie non finie, la boucle **séquentielle est rejouée** et c'est elle qui rend le verdict. Le
variant d'erreur et son rang sont donc ceux d'avant, exactement.
**Critères, déclarés avant construction.** (1) `SequentialJobs` reste la **référence** ; la sortie
parallèle lui est identique **au bit** à 1, 2, 4 et 8 fils, et à plusieurs grains. (2) Assertion du
harnais, exécutable : à `n` et `grain` égaux, le résultat ne dépend pas de `worker_count` — pour la
nouvelle primitive **et** pour la réduction de S20, qui garde son cas. (3) Consommateur : les six
empreintes du banc S242 et `--multi --verify` / `--multi --retour` de l'hôte, inchangés. (4) Coût
publié contre le nombre de fils, et **le coût du chemin à un fil ne doit pas monter**. (5) Aucune
allocation ajoutée au chemin d'image (ADR-145). (6) ADR : la primitive et son argument de
déterminisme sont une décision.
**Arrêt.** Primitive, pool, assertion et premier consommateur reçus ; **ou**, si le lot dépasse la
session, la primitive et son assertion reçues seules, le consommateur déclaré en file avec son
déclencheur — et le plan le dit plutôt que de le laisser deviner.

### Plan

- [x] **P1** — amorce, jeton, plan seuls.
- [x] **P2** — protocole écrit : ce que SPEC-004 spécifie déjà, la dissymétrie réduction/écriture
  disjointe, la forme retenue et les formes écartées.
- [ ] **P3** — la primitive dans `JobSystem` + `SequentialJobs` (référence) + pool réel dans
  `host_impl` (`std::thread::scope`, sans dépendance) ; assertion du harnais.
- [ ] **P4** — premier consommateur : `render_components`, chemin rapide parallèle et verdict
  séquentiel ; identité au bit.
- [ ] **P5** — réception : empreintes à 1/2/4/8 fils, banc S242, hôte, coût.
- [ ] **P6** — rituel §6, ADR, file, feuille de route, jeton.

### Notes de reprise

P2 : protocole `docs/validation/PARALLELISME-S243.md`. Forme retenue
`parallel_fill_f32(&self, out: &mut [f32], grain, fill: &(dyn Fn(usize, &mut [f32]) + Sync))` :
objet-sure, sans generique, sans allocation dans le coeur, sans unsafe dans le coeur — l'**hote**
decoupe par chunks_mut. Quatre formes ecartees et pourquoi (generique non objet-sur ; tableau de
taches = allocation par image ; cellules atomiques = change le type publie ; fils dans le coeur =
contredit ADR-020). Erreurs : chemin rapide parallele, **verdict sequentiel rejoue**.

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