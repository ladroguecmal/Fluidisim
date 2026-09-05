# Revue croisée des cinq spécifications — S08

Confrontation systématique de **SPEC-001, SPEC-002, SPEC-003, SPEC-004 et SPEC-005 les unes contre
les autres**. La revue de S05 avait confronté les vingt ADR ; elle n'avait touché les SPEC que par
ricochet, et SPEC-005 n'existait pas encore. Dix écarts, dont **deux de gravité 1**.

Gravité : **1** = deux documents s'excluent, une décision doit trancher · **2** = incohérence
opérationnelle, corrigeable par précision · **3** = documentaire ou mineur.

---

## Tableau

| # | Écart | Documents | Grav. | Résolution |
|---|---|---|---|---|
| E01 | La colonne « cellules éparses » mêle trois taux d'occupation | 001 §2.4 → 005 §6 | 3 | note corrective SPEC-001 |
| E02 | « Ré-établi en 2 à 3 s » sans provenance, et probablement trois fois trop optimiste | 005 §6 ↔ 001 §1 | 2 | note corrective SPEC-005 |
| E03 | *(dérivation, pas un écart)* le fetch borne la glace en plaque | 002 §4 × 001 §4 | — | chiffre porté à l'arbitrage n°2 |
| E04 | **Le chemin poussé n'existe dans aucun document** | 004 ↔ ADR-014/016/018 | **1** | point ouvert n°6 de SPEC-004, objectif de S09 |
| E05 | Le seul canal spécifié livre la mauvaise vitesse au calcul de danger | 004 §2 ↔ 002 §5 | 2 | conséquence de E04, tranchée avec lui |
| E06 | L'écume accumulée n'est lisible par personne | 004 §2 ↔ 002 §1, ADR-016 §7 | 3 | symptôme de E04 |
| E07 | **Cuisson « bit à bit » exigée d'un solveur qui n'est jamais D1** | 005 §7.2 ↔ 003 §2 | **1** | note corrective SPEC-005 : autoritaire, pas reproductible |
| E08 | Le facteur d'économie 64 s'effondre dans le plus gros domaine | 004 §6.2 ↔ 001 §2.4 | 2 | note corrective SPEC-004 : contrainte, pas constante |
| E09 | 9 h de cuisson calculées en temps simulé, annoncées en temps de calcul | 005 §11.5 ↔ 005 §7.1 | 3 | note corrective SPEC-005 |
| E10 | La passe d'obsolescence est logée dans le mode qui exclut ce qu'elle lit | 005 §7.3 ↔ 003 §4 | 2 | note corrective SPEC-005 : manifeste, pas fichiers |

---

## E04 — Le chemin poussé n'existe dans aucun document *(gravité 1)*

**Constat.** SPEC-004 spécifie deux chemins et deux seulement : le chemin **tiré**
(`EvalWaterBatch`, `sample_batch` — l'appelant demande, point par lot) et le chemin de
**branchement de solveur** (`IFluidSolver`, `IWaveSolver`, les six services d'hôte). Trois ADR
exigent un troisième chemin, où le système d'eau **publie** par tick, à basse fréquence, un champ
ou un signal que personne ne vient chercher :

| Document | Ce qui doit être publié | Motif explicite dans l'ADR |
|---|---|---|
| ADR-014 §2 | champ de moussage `F(x,t)`, deux canaux, texture 2D ancrée au monde | l'écume persiste et dérive ; elle n'est pas une fonction de `(x,t)` |
| ADR-018 §1 | `TraversabilitySample` par cellule `HydroGrid` | « jamais en interrogation continue, sans quoi la navigation devient un consommateur majeur de `EvalWater` » |
| ADR-016 §2, §6 | bus d'événements audio ; trois champs à ajouter à `WaveEvent` | le son suit les événements, pas l'échantillonnage |

Aucun des trois n'a de signature dans SPEC-004. Aggravant : **`WaveEvent` n'est défini nulle part
dans SPEC-004**. Il vit dans ADR-009 §2, il traverse trois frontières — réseau, transduction δ→W,
audio — et ADR-016 §6 demande de l'élargir de trois champs « avant de figer le format ». Le
document dont le titre est « signatures des interfaces » ne contient pas la structure la plus
partagée du système.

**Ce que la contradiction révèle** *(L22 — deux parades identiques inventées séparément signalent
un concept manquant ; ici il y en a trois)*. Ce ne sont pas trois oublis. C'est un **mode de
communication** absent du modèle : SPEC-004 a été écrite depuis le point de vue du consommateur qui
interroge, et tout ce qui se publie sans être demandé est passé au travers. Les trois ADR ont
chacun décrit sa propre publication dans son propre vocabulaire — trois implémentations divergentes
garanties, exactement le mécanisme de R07 en S05.

**Conséquence immédiate, et c'est elle qui donne la gravité 1.** Trois des quatre interfaces
inter-équipes listées dans `00_INDEX.md` comme attendant une réponse humaine — audio, IA/navigation,
et la part écume du rendu — **n'ont pas de document à présenter**. On demande à ces équipes de
confirmer une interface qui n'est écrite nulle part sous forme de signature. Le risque annoncé dans
l'index (« recâblage complet », « un générateur de maillage qui ne sait qu'enlever des zones ») est
donc sous-estimé : il ne s'agit pas d'obtenir un accord, il s'agit d'abord d'avoir quelque chose à
soumettre.

**Résolution.** SPEC-004 §10 reçoit un point ouvert n°6 nommant le chemin manquant, ses trois
consommateurs et les contraintes déjà connues. L'écriture des signatures est un travail de session
entière, pas une note corrective : **c'est l'objectif recommandé pour S09.** Contraintes que ce
chemin devra respecter, toutes déjà établies ailleurs :

- publication **par tick, à basse fréquence**, découplée du tick de simulation (ADR-018 §6 : 5 Hz) ;
- **instantané immuable à N lecteurs**, même discipline que `publish_snapshot` (SPEC-004 §1.2) —
  l'audio et l'IA lisent depuis leurs propres fils ;
- aucune allocation (I-06), donc anneau de tampons dimensionné par profil (I-16) ;
- le champ d'écume est produit sur GPU (ADR-014 §2) : sa lecture par l'audio retombe sur
  `poll_readback` et son âge (SPEC-004 §8.4), jamais sur une lecture bloquante ;
- `WaveEvent` migre dans SPEC-004 §2 avec ses trois champs audio, **avant que le format ne soit
  figé par le réseau**.

## E07 — Cuisson « bit à bit » exigée d'un solveur qui n'est jamais D1 *(gravité 1)*

**Constat.** SPEC-005 §7.2 : « Mêmes entrées et même version d'outil doivent donner le même octet »,
justifié par « le serveur et les clients peuvent charger des données divergentes ». Or la
bibliothèque côtière est produite en instanciant `WaterSystem` et en **faisant tourner δ**
(SPEC-005 §7.1, ADR-020 §5) — c'est même l'argument central du paragraphe : l'état cuit est
*exactement* ce que le jeu produirait.

Et SPEC-003 §2 pose : « Un solveur δ ne sera jamais D1 » (exact, inter-plateforme). Au mieux **D2** :
même binaire, même machine, même graine. Deux artistes sur deux machines ne produiront jamais le
même octet. L'exigence de §7.2 n'est pas difficile à tenir, elle est **inatteignable par
construction** — et le motif qui la fondait reste entier.

**Ce que la contradiction révèle** *(L24 — chercher l'hypothèse commune plutôt que départager)*.
L'hypothèse partagée par les deux branches est que la cuisson devrait être *reproductible*. Elle
n'a aucune raison de l'être. Ce que le motif exige réellement, c'est que **tous les participants
chargent le même octet** — ce qui s'obtient en ayant **un seul producteur**, pas en rendant le
calcul reproductible partout.

**Résolution.** Une cuisson livrable est **autoritaire, pas reproductible** :

1. un producteur désigné — machine de construction — cuit les artefacts livrés ;
2. l'artefact est identifié par l'empreinte de son contenu ; le `bake_manifest` de §7.3 porte déjà
   exactement ce qu'il faut, sans aucun ajout ;
3. les postes d'artistes cuisent en local pour itérer, jamais pour livrer ; une cuisson locale ne
   remplace un artefact livré que par une promotion explicite ;
4. **ce qui reste exigible et l'était déjà : le régime D2 de l'outil de cuisson.** Sans lui une
   cuisson n'est pas déboguable, et la discipline correspondante (PRNG entier, pas d'horloge
   murale, pas de parcours de disque dans l'ordre du disque, réductions ordonnées) est celle que
   §7.2 énumérait déjà — elle était juste, c'est sa conclusion qui était trop forte.

La résolution **retire** une exigence et n'ajoute aucun mécanisme.

---

## E01 — La colonne « cellules éparses » mêle trois taux d'occupation *(gravité 3)*

SPEC-001 §2.3 annonce que l'allocation éparse « divise le comptage par ≈4 **sur ce cas** ». §2.4
enchaîne trois domaines sous un en-tête « cellules éparses » sans redire le taux :

| Domaine | plein | annoncé « épars » | rapport réel |
|---|---|---|---|
| Bateau, 24×12×6 m, `dx = 0,10` | 1,73 M | 432 k | ÷ 4 |
| Impact, 6×6×4 m, `dx = 0,05` | 1,152 M | 1,15 M | **÷ 1** |
| Déferlement, 120×20×5 m, `dx = 0,25` | 768 k | 384 k | ÷ 2 |

Les trois valeurs sont défendables — une cavité d'impact est presque pleine, un rouleau occupe
environ la moitié de sa boîte — mais aucune n'est justifiée, et un lecteur qui applique le « ÷4 »
de §2.3 se trompe d'un facteur 4 sur le domaine d'impact. Deux consommateurs le lisent : le
contrôle de cohérence du budget 384 Mo au même paragraphe, et SPEC-005 §6 (197 Mo par plage). Les
deux conclusions tiennent ; c'est la provenance qui manque — **I-14**.

*Précision par rapport à S05.* La revue croisée S05 avait classé « cohérence mémoire de
SPEC-001 §2.4 avec ADR-012 §3 » parmi les contrôles passés. Elle l'est — le rapprochement des deux
budgets est juste. Le défaut est en amont, dans la provenance des comptages eux-mêmes, et un
contrôle de cohérence entre deux nombres ne peut pas le voir. C'est la limite structurelle de tout
audit par paires, déjà rencontrée en S05 (L21).

## E02 — « Ré-établi en 2 à 3 secondes » sans provenance *(gravité 2)*

SPEC-005 §6 fait reposer sur ce chiffre la faisabilité entière du précalcul côtier : « c'est le
facteur qui rend l'activation d'une plage possible dans une fenêtre de prédiction réaliste ».
Aucune formule ne l'accompagne — **I-14**.

Ce que les autres documents permettent d'en dire :

- la structure verticale d'un train de houle s'établit en ≈1 période, soit **4,4 s** (λ = 30 m) à
  **8,0 s** (λ = 100 m) — SPEC-001 §1 ;
- la fenêtre de préparation utile vaut `t ≤ √(2·R_domaine/a_max)`, soit **7,8 s** pour R = 60 m
  (demi-longueur d'une zone de 120 m) et `a_max = 2 m/s²` — ADR-013 §2 ;
- ADR-013 §2 classe « avancer la simulation dans le temps » comme la préparation la plus chère,
  exigeant `p ≈ 0,8`.

À 2–3 s la marge est confortable ; à 8 s elle est nulle, et l'activation d'une plage redevient un
pari à `p ≈ 0,8` sur un acteur manœuvrant. **La conclusion de §6 ne change pas — 8 s valent
toujours mieux que 40 — mais son confort, si.** Le chiffre doit être étiqueté « à calibrer » avec
le banc qui le fixera, et la comparaison honnête est *2–3 s revendiqués contre 4,4–8 s dérivés*.

## E05 — Le seul canal spécifié livre la mauvaise vitesse *(gravité 2)*

SPEC-002 §5 définit le produit d'emportement `HR = d·(v + 0,5)` où `v` est un **courant**.
`WaterSample.u` de SPEC-004 §2 est documenté « vitesse de surface (**orbitale** + courant) », et
aucun champ ne les sépare.

Un consommateur qui prend `u` obtient un danger qui **oscille à la période de la houle**. L'ordre de
grandeur interdit de traiter cela comme du bruit : la vitesse orbitale de crête vaut `πHs/T`, soit
**0,63 m/s à Hs = 1 m et T = 5 s** — du même ordre que le courant qu'on prétend mesurer, et le
double du seuil qui sépare « faible » de « dangereux » dans la table de §5.

ADR-018 §1 fait le bon choix (`flow_speed` = norme du **courant** de surface), mais sa structure
n'est justement pas dans SPEC-004 : E05 est la première conséquence chiffrée de E04, et se résout
avec lui. Famille de L12 — la grandeur qui gouverne n'est pas celle qui donne son nom au phénomène.

## E06 — L'écume accumulée n'est lisible par personne *(gravité 3)*

SPEC-002 §1 pose une écume résiduelle de demi-vie 30 s ; ADR-014 §2.2 en fait un canal de champ
persistant ; ADR-016 §7 remplace les émetteurs audio lointains par le « lit d'écume ».
`WaterSample` porte `steepness` — le *déclencheur* instantané — et `aeration`, jamais la couverture
accumulée. L'audio ne peut donc pas lire ce dont ADR-016 dit qu'il dépend. Symptôme de E04, sans
résolution propre.

## E08 — Le facteur d'économie 64 s'effondre dans le plus gros domaine *(gravité 2)*

SPEC-004 §6.2 rend le terme source perturbatif abordable par un facteur **64** : le champ de fond
est échantillonné « une cellule sur quatre par axe » puis interpolé trilinéairement, au motif qu'il
est lisse à l'échelle de `dx` puisque ses longueurs d'onde valent au moins `λ_cut`.

Ce qui décide est le rapport `λ_cut/dx`, et il varie d'un facteur 2,5 entre deux régimes déjà
écrits — avec `λ_cut = 4 m`, valeur de départ proposée par ADR-005 :

| Régime | `dx` | `λ_cut/dx` | points par `λ_cut` après décimation ×4 |
|---|---|---|---|
| Scénario nominal, SPEC-003 §3 | 0,10 m | 40 | 10 — confortable |
| Zone de déferlement, SPEC-001 §2.4 | 0,25 m | 16 | **4 — deux fois Nyquist** |

À quatre points par longueur d'onde, l'interpolation trilinéaire perd une fraction notable de
l'amplitude de `S`. Et le symptôme est **exactement celui que SPEC-004 §6.1 décrit comme
indiagnostiquable** : le domaine dérive lentement par rapport au fond, la frontière redevient
visible, et l'on conclut à tort que la décomposition en couches ne fonctionne pas. L'économie
disparaît donc dans le type de domaine le plus gros — 384 k cellules — celui-là même qu'elle devait
rendre abordable.

**Résolution.** Le taux de décimation n'est pas une constante mais une **contrainte** :
`dx ≤ λ_cut/N`, `N` à fixer au banc B4 dont c'est déjà un paramètre direct (SPEC-004 §10.3).
`is_smooth_at(dx)` doit renvoyer faux quand elle n'est pas satisfaite, et le solveur retomber sur
un échantillonnage plein — plus cher, mais juste. Note corrective dans SPEC-004 §6.2.

## E09 — Neuf heures de cuisson comptées en temps simulé *(gravité 3)*

SPEC-005 §11.5 : « 16 états × 40 s d'établissement × N plages […] ≈9 heures de calcul mono-fil ».
Les 40 s sont du temps **simulé** (ADR-013 §4). SPEC-005 §7.1 affirme par ailleurs que l'outil
tourne « plus vite que le temps réel ». Une zone de déferlement de 384 k cellules à `dx = 0,25`
tourne plus probablement plus **lentement** que le temps réel sur un fil. Les deux phrases ne
peuvent pas être vraies ensemble : **9 h est un plancher**, à présenter comme tel — la conclusion
« acceptable en nocturne, pas en interactif » n'en est que renforcée.

## E10 — L'obsolescence logée dans le mode qui exclut ce qu'elle lit *(gravité 2)*

SPEC-005 §7.3 place la passe de vérification d'obsolescence « dans le harnais de SPEC-003, **mode
`check`**, avec le reste de la batterie déterministe — pas dans un second système de validation ».
L'intention est juste : un second système de validation dériverait.

Mais SPEC-003 §4 définit `check` comme « ni GPU ni rendu ni **assets lourds** », moins de 60 s pour
tout le lot, à chaque commit — et ajoute que c'est **le seul mode dont la vitesse est un objectif de
conception**. Recalculer le sha256 des entrées d'une cuisson, c'est lire la bathymétrie et les
maillages : les assets lourds nommément exclus.

**Résolution.** Séparer la comparaison du recalcul.

- **Mode `check`, à chaque commit** : comparer les empreintes **déjà inscrites** dans le
  `bake_manifest` à celles inscrites dans le scénario et l'index d'assets. SPEC-003 §3 référence
  déjà ses données « par empreinte de contenu, jamais par chemin » — le canal existe, et la
  comparaison coûte une lecture de manifeste. Elle attrape le cas fréquent : un artefact cuit
  depuis une version d'entrée qui n'est plus celle que le dépôt déclare.
- **Cadence nocturne** : recalculer les empreintes depuis les fichiers eux-mêmes. Elle attrape le
  cas rare et grave : une entrée modifiée sans que son empreinte déclarée ait suivi.

L'intention de §7.3 est préservée — un seul système de validation, deux cadences.

---

## Ce qui a été vérifié et tient

Un audit qui ne rapporte que des défauts n'est pas vérifiable.

**Arithmétique des deux fiches chiffrées.** Une quarantaine de valeurs de SPEC-001 et SPEC-002 ont
été recalculées depuis leurs formules citées : dispersion en eau profonde et peu profonde, CFL et
comptage des sous-pas à 30 Hz, loi `dx⁻⁴` (`2,5⁴ = 39`, `5⁴ = 625`, `12,5⁴ = 24 400`), comptages de
cellules pleins, mémoire par domaine, `E = ρgHs²/16` et `P = E·c_g`, fetch SMB, demi-angle de
Kelvin, `λ = 2πv²/g`, ulp `f32` du temps (**62,5 ms à 10⁶ s** — juste : l'exposant binaire vaut 19
et non 20), Monahan, nombres de Weber, remontée de Stokes, Boyle, bilan d'exposition au vide
(14 % / 86 %), Stefan `h ≈ 0,035·√FDD`, produit d'emportement, transmission acoustique (−29,5 dB)
et délais aériens, angle critique et fenêtre de Snell, atténuation optique. **Aucune erreur.** Les
deux fiches sont numériquement saines ; les dix écarts ci-dessus portent sur la provenance et sur
les croisements.

**Chaîne harnais ↔ interfaces — huit contrôles, aucun écart.**

| Ce que SPEC-003 exige | Ce que SPEC-004 fournit |
|---|---|
| `t_sim_debut` explicite, scénario rejouable (§3) | `begin_tick(SimTime)` poussé ; **aucune horloge** dans `HostServices` (§3, §8) |
| `allocations = 0` (§6) | `seal()` fait *échouer* l'allocation (§8.1) — mécanique, pas surveillée |
| `gpu_p50/p99` par horodatage GPU, jamais CPU (§6, piège 2) | `begin_timer` / `poll_timer` ; aucun timer CPU exposé (§8.4) |
| Iso-qualité : deux candidats au même instant (§5.2) | « `step` avance exactement `dt_target`, ou dit qu'il ne l'a pas fait » (§4.1) |
| Batterie `starve` : constater la dégradation (§9.1) | `StepResult.Degrade` et `work_remaining` (§4) |
| Rejeu : journal d'événements à 40 o pièce (§8) | `WaveEvent` 40 o (ADR-009 §2), `push_events` (§3) |
| Régime D2 : réductions ordonnées (§2) | `parallel_reduce_ordered`, seule primitive d'accumulation (§8.2) |
| `latence_echantillon` = âge **à l'usage** (§6, piège 6) | `sample_batch` renvoie l'âge en µs ; `poll_readback` le renvoie toujours (§4, §8.4) |

**Autres croisements sans écart.** `SampleHints` / LOD spectral ↔ ADR-004 §4. Séparation
`WaterSample` (≈56 o) / `BackgroundSample` (≈80 o) ↔ coût par cellule de SPEC-001 §2.4 (24–32 o) :
les ordres de grandeur ne se contredisent pas. `steepness` ↔ cambrure limite `H/λ ≈ 1/7`
(SPEC-001 §3, SPEC-002 §1) : le déclencheur d'écume est atteignable sans aucune simulation, comme
annoncé. `aeration` ↔ `ρ_eff = (1−α)·ρ` (SPEC-002 §2). Correction R12 de S05 (`lambda_cut` hors de
`IBackgroundField`) : effectivement appliquée, et cohérente avec E08 qui la suppose. Table des
données de SPEC-005 §2 ↔ ses sept ADR consommateurs : aucune donnée à double source de vérité.
Portées d'invalidation de SPEC-005 §8 ↔ `h = λ/2` de SPEC-001 §1 : l'isobathe 50 m pour une houle
de 100 m est juste, et les 10 km au large sur une pente 1:200 aussi.

---

## Une dérivation, qui n'est pas un écart

**E03 — le fetch borne la glace en plaque.** SPEC-002 §4 exige `Hs < 0,15 m` pour une formation en
plaque ; SPEC-001 §4 donne `Hs ≈ 0,0016·U10·√(F/g)`. Les deux réunies donnent un fetch maximal :

```
F_max = g · ( 0,15 / (0,0016 · U10) )²
```

| U10 | 3 m/s | 5 m/s | 10 m/s |
|---|---|---|---|
| fetch maximal | 9,6 km | **3,4 km** | 0,86 km |

**La glace en plaque est un phénomène de lac et de baie abritée, jamais de haute mer** — et cela se
dérive de deux fiches déjà écrites, sans mesure ni décision nouvelle. À porter à l'arbitrage n°2
(ADR-017, « le projet veut-il de la glace ? ») : la réponse « oui » coûte moins cher que ce que
l'ADR laisse craindre, parce que la surface concernée est bornée par la géométrie des plans d'eau
et non par la météo. C'est un chiffre qui éclaire une décision humaine, pas un défaut.

---

## Suite

| Action | Où | Statut |
|---|---|---|
| Point ouvert n°6 — le chemin poussé | SPEC-004 §10 | appliqué |
| Écriture des signatures du chemin poussé | SPEC-006, ou §11 de SPEC-004 | **objectif recommandé pour S09** |
| Note corrective — cuisson autoritaire, non reproductible | SPEC-005 §7.2 | appliquée |
| Note corrective — obsolescence à deux cadences | SPEC-005 §7.3 | appliquée |
| Note corrective — `dx ≤ λ_cut/N` | SPEC-004 §6.2 | appliquée |
| Note corrective — taux d'occupation épars | SPEC-001 §2.4 | appliquée |
| Note corrective — 2 à 3 s à calibrer | SPEC-005 §6 | appliquée |
| Note corrective — 9 h est un plancher | SPEC-005 §11.5 | appliquée |
| Fetch maximal de la glace en plaque | arbitrage n°2, `00_INDEX.md` | porté |
