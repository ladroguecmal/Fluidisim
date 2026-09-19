# Le pas de δ, décomposé puis allégé — S291

2026-09-19. Point A293 de la file. Suite de [ENCODAGE-CYCLE-S290](ENCODAGE-CYCLE-S290.md), qui
laissait « ≈ 2,5 ms que personne n'a cartographiés ». Matériel : NVIDIA GeForce RTX 5070 Laptop,
DX12, secteur, release, réductions séquentielles.
Bancs : `viewer --pas-decomposition` (la carte), `--pas-cycle` (la longueur de cycle),
`--pas-couple` (le chemin de l'afficheur), `--delta-cadence` (les empreintes de S287).

## 1. Ce qui est reçu

Le pas de δ est **mesuré poste par poste** — dix-huit étapes, avec une horloge réelle — puis
allégé de tout ce que la mesure a montré inutile. **Rien de ce que la simulation produit n'a
changé** : les six empreintes de trajectoire publiées par S287 sont rendues à l'identique,
sommes d'itérations comprises.

Sur le chemin que la bande δ de l'afficheur emprunte réellement, à sa grille de production
(128 × 52, dx = 2 m, 200 pas) : **médiane 17,93 → 8,27 ms (×2,17)**, maximum 28,22 → 16,11
(×1,75), 200/200 propositions de pression retenues, aucun refus, aucun pas dégradé.

**Non reçu** : le budget eau de 2 ms (×4,1 sur ce chemin) ; le choix automatique de la longueur
de cycle ; le multiplateforme ; et surtout **la tenue du pire pas quand la carte a refroidi**,
qui est un défaut mesuré et non corrigé (§7).

## 2. L'instrument, et ce qu'il coûte

`Control` referme chaque segment de temps et l'attribue à **deux** décompositions : les huit
`Phase` publiques et dix-huit `Stage` internes. Les deux sommes valent `elapsed()` par
construction, et le banc **refuse** si elles diffèrent — c'est la paire qui doit rendre le même
nombre (L339). `Phase` n'a pas gagné de variante : son test d'exhaustivité de S230 exige que ses
huit valeurs soient atteignables comme points d'expiration de `step_budgeted`, et un découpage
de mesure n'a pas à peser sur un vocabulaire d'arrêt coopératif.

Coût de l'instrument : deux additions par `check` déjà existant, plus une quinzaine de `mark` par
pas. À comparer au coût de l'horloge elle-même, mesuré en §6.

## 3. La carte, avant tout allègement

6 656 mailles, candidat GPU de S289/S290 actif, horloge réelle, 5,7937 ms internes par pas.

| étape | ms | part |
|---|---|---|
| candidat (appel GPU, temps de l'hôte) | 2,4226 | 41,8 % |
| **préconditionneur** (un cycle multigrille) | **0,8291** | **14,3 %** |
| validation (finitude de onze tableaux) | 0,4376 | 7,6 % |
| erreur inverse (diagnostic du rapport) | 0,3405 | 5,9 % |
| itérations du gradient conjugué | 0,2948 | 5,1 % |
| second membre | 0,2409 | 4,2 % |
| divergence projetée | 0,2184 | 3,8 % |
| portes d'acceptation | 0,1591 | 2,7 % |
| export de l'opérateur | 0,1477 | 2,5 % |
| résidu initial | 0,1414 | 2,4 % |
| correction des faces | 0,1312 | 2,3 % |
| advection | 0,1311 | 2,3 % |
| six postes restants | 0,3283 | 5,7 % |

**Le poste le plus gros hors GPU était du travail jeté.** Le préconditionneur était appliqué
*avant* la boucle du gradient conjugué ; or quand le candidat a déjà convergé — ce qui est le cas
à presque tous les pas depuis S289 — cette boucle ne tourne pas, et le cycle multigrille partait
à la poubelle. Aucune relecture ne l'avait vu en trois sessions ; une mesure par étape l'a vu au
premier passage.

## 4. Quatre suppressions, et la preuve qu'elles ne changent rien

1. **Amorçage différé.** La direction préconditionnée est calculée à la première itération qui
   l'emploie, et à chaque relance — jamais avant. Quand une itération tourne, elle reçoit
   exactement la même direction et le même `⟨r, M⁻¹r⟩`, parce que `res` ne bouge pas entre les
   deux points. Mesuré : **0,8291 → 0,0121 ms** sur le chemin avec candidat.
2. **`‖b‖²` réduit une seule fois.** Il l'était deux fois sur le même tableau : une fois pour
   décider du départ chaud, une fois comme `b2`.
3. **Correction et divergence non refaites** quand la porte d'ADR-144 vient de les produire pour
   le même `p`. Les deux **phases** restent traversées, pour ne pas retirer un point d'expiration
   au pas. Gain nul sur le chemin avec candidat, 0,33 ms sans — la redondance n'existait que
   lorsque le gradient conjugué itérait jusqu'à la convergence, ce qui est dit ici plutôt que
   compté deux fois.
4. **Double sondage par maille** retiré de la boucle `w` de l'advection.

**Preuve d'identité.** Le banc d'empreintes laissé par S287 rend les six valeurs publiées dans
[PASSES-PRESSION-S287](PASSES-PRESSION-S287.md) §Identité — `92d65e868ec29942`,
`aee9db45c45129a9`, `c9c79e0075ee0ffc`, `1e5c6d5f5fed86bd`, `b0414fc315f5f3c6`,
`6c037edac4f55f05` — et les six sommes d'itérations (2648, 1562, 1116, 2895, 1551, 1105).
Ce banc emprunte le chemin **couplé**, celui de l'afficheur.

## 5. Deux gestes de natures différentes

**La validation garde exactement sa garantie et coûte quatre fois moins.** Les onze champs sont
toujours parcourus, dans le même ordre, avec le même grain de sondage et la même erreur ; seule
la forme change — onze tranches plates au lieu d'une chaîne de onze itérateurs, ce qui laisse le
test se vectoriser. **0,4189 → 0,1031 ms.**

L'autre idée, **retirer du contrôle les six tampons non publiés**, a été **écartée**. Établir
qu'aucun ne peut être relu avant d'être réécrit demandait un raisonnement par tampon — et `dir`
n'est justement pas réécrit sur les mailles solides. Une erreur y transformerait un `NaN` en
rendu silencieusement faux, et le gain était déjà pris sans ce risque.

**L'erreur inverse du rapport devient un choix de l'hôte.** S238 la déclare « diagnostic, jamais
seuil », et elle coûte une passe de stencil complète : **0,3379 ms**.
`set_report_backward_error(false)` la coupe, et `Report.backward_error` vaut alors `NaN` — « non
mesurée » plutôt qu'un chiffre faux. Défaut inchangé à `true`. **Le certificat d'arrêt d'ADR-143
n'est pas concerné** : il est calculé dans la boucle parce qu'il décide, et il le reste.

## 6. Ce que le sondage coûte, et ce que S289–S290 cachaient

Le contrôle coopératif lit l'horloge une fois par 64 mailles. **Toutes** les mesures de S289 et
S290 ont été prises avec une horloge **figée** (`now_ns → 0`), donc quasi gratuite.

| cas | horloge réelle | horloge figée | écart |
|---|---|---|---|
| 6 656 mailles, candidat | 3,9306 | 3,6392 | +8,0 % |
| 32 768 mailles, candidat | 24,4207 | 22,4219 | +8,9 % |
| 6 656 mailles, sans candidat | 10,2011 | 9,1869 | +11,0 % |
| 32 768 mailles, sans candidat | 45,5672 | 41,9096 | +8,7 % |

**Les chiffres de S289 et S290 sous-estiment donc la production d'environ 8 %.** Le grain n'a pas
été élargi : chaque sondage est un point d'expiration, et en retirer relève d'un arbitrage sur la
garantie d'arrêt coopératif d'ADR-007, pas d'une optimisation.

### Où en est le pas, à 6 656 mailles, candidat actif, horloge réelle

| état | médiane murale | contre le départ |
|---|---|---|
| départ de S291 (= S290 remesuré à l'horloge réelle) | 5,3443 ms | — |
| après les quatre suppressions | 4,3022 | ×1,24 |
| après la validation aplatie | 3,9306 | ×1,36 |
| avec le diagnostic coupé par l'hôte | **3,5371** | **×1,51** |

Sur la base « horloge figée » que S290 publiait (4,6315 ms) : **3,6392 ms**, ×1,27.
Ce qui reste, 3,8259 ms internes : candidat 2,0188 (53 %), puis onze postes sous 0,31 ms.
**Plus aucun poste dominant côté cœur.**

## 7. Le chemin de l'afficheur, et le défaut qui compte plus que le gain

### Une imprécision de S289, corrigée

S289 écrivait que le chemin consommateur était « le pas mobile réel, le même que pilote la bande
δ de l'afficheur ». La bande passe en réalité par `step_perturbation_mobile` — le pas **couplé**
de S253 — et non par `step_surface_mobile`. C'était la même fonction du cœur, pas le même chemin
de l'hôte, et le crochet n'était donc pas atteignable par le rendu. `step_perturbation_mobile_with`
existe désormais : le candidat entre dans la projection principale seulement, le repli multigrille
et l'affinage partant de zéro (ADR-153).

### La grille vivante, 128 × 52, dx = 2 m, 200 pas, horloge réelle

| cycle | médiane | maximum |
|---|---|---|
| 0 (témoin, diagnostic actif) | 17,9281 | 28,2240 |
| 0 (diagnostic coupé) | 17,5027 | 26,9360 |
| 128 | 14,7182 | 26,1217 |
| **192** | **8,2729** | 16,8865 |
| 256 | 8,5538 | **16,1106** |
| 384 | 10,1015 | 22,2520 |

### Le pic et la médiane ne désignent pas le même réglage

Sur le pas mobile à 6 656 mailles, le balayage est net : cycle 128 donne la meilleure médiane
(3,7983 ms) et un maximum de 25,7902 ; cycle 256 donne 4,8067 de médiane et **8,0980** de
maximum — **trois fois moins**. À 384, le cœur ne fait plus **aucune** itération. La variance
vient entièrement du nombre d'itérations qui restent au processeur ; les supprimer supprime la
queue. Pour un rendu, c'est le pire pas qui décide, et le réglage qui minimise la médiane n'est
pas celui-là. Rien n'arbitre encore entre les deux, et rien ne règle cette longueur
automatiquement.

### Un gel de 467 ms, reproduit, non corrigé

Un pas à **467 ms**, puis 132 ms à la reproduction, est apparu deux fois — toujours au premier
régime employant le GPU **après** des régimes qui ne l'employaient pas, soit après sept secondes
d'inactivité de la carte. Un envoi de préchauffage jeté avant la mesure **ne le supprime pas** ;
ce qui le supprime est d'exécuter les régimes GPU en premier, c'est-à-dire de ne pas laisser la
carte refroidir : le maximum retombe alors à 26,12 ms.

**C'est un chemin GPU refroidi, pas un chemin neuf.** Pour un rendu, 130 à 470 ms est un gel
visible, et il suffit que la bande δ ait été éteinte un instant. Aucune correction n'est
construite ici : garder la carte tiède quand la bande est éteinte est un arbitrage
d'ordonnancement — qui paie, quand, et contre quel budget — et non un réglage. C'est le point le
plus important que cette session laisse ouvert.

## 8. Ce que ce document ne prouve pas

Le budget de 2 ms : ×4,1 sur le chemin de l'afficheur au meilleur réglage. Une seule carte, un
seul backend, une version de wgpu. La tenue du pire pas. Le comportement sous famine de budget,
sous rétrécissement (A290) ou sous changement violent de topologie. L'ordonnanceur ne sait
toujours pas choisir la longueur de cycle, ni décider de garder la carte tiède. Et ni la 3D ni
les solides, qui restent obligatoires (ADR-127) : leur comparaison de priorité se rejoue
désormais sur un pas dont **tous** les postes sont connus, ce qui était l'objet d'A293.
