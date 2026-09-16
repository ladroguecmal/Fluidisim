# Revue visuelle — l'utilisateur superviseur des rendus

**Ouverte S254, 2026-09-16.** L'utilisateur reprend le projet comme **superviseur des rendus
visuels** : il envoie, à la demande, des références réelles et aide à comprendre la réalité et la
perception humaine. Le travail technique reste délégué (S71).

Jusqu'ici, aucune réception perceptive n'était possible : il manquait un observateur et des
références. COUPURE-S249 (« perception non reçue »), A282 et le volet perception de B4 le
disaient. Ce document fixe comment cette capacité s'emploie sans se confondre avec une preuve
physique.

## 1. Ce qu'une revue peut établir, et ce qu'elle ne peut pas

| une revue établit | une revue n'établit pas |
|---|---|
| un **écart perçu** entre un rendu daté et une référence réelle | une précision physique : elle reste l'affaire des oracles et des bancs |
| le **seuil de perception** d'un défaut connu (visible, gênant, invisible) | une tolérance numérique, sans formule ou banc qui la dérive |
| la **priorité ressentie** entre défauts visibles | une réduction de périmètre (ADR-127) |
| ce qu'un joueur **attend de voir** d'un phénomène (écume, gerbe, reflet) | l'appartenance d'un phénomène à une couche : ADR-001 §2 |

Un verdict **qualifie l'implémentation rendue à sa date**, comme un dépassement de coût
(ADR-131). Il ne retire jamais une fonctionnalité, et il ne se « corrige » pas en déformant B ou W
hors de leur physique pour qu'ils paraissent justes.

## 2. Ce que la session envoie

Pour chaque image :

- le **fichier PNG**, converti d'un PPM de banc local (ADR-124), jamais publié ;
- la **commande** qui la reproduit, et l'**empreinte FNV** du PPM ;
- la **pose** (œil en m, lacet, tangage, champ vertical), l'**âge de scène** et la définition ;
- les **couches présentes** (B, impacts W, sillages W ; δ et V absents à ce jour) et les
  techniques actives (filtre spectral, grille du sillage) ;
- ce qui est de l'**habillage de banc** et ne relève pas du système d'eau : ciel, soleil,
  couleur de l'eau, brouillard, absence d'écume, de réfraction et de sous-surface ;
- des **questions précises**, et la liste des **références demandées**, avec les conditions
  qui les rendent comparables.

## 3. Ce qui revient

- **Références** : photo ou vidéo réelle, avec ce qu'on en sait — source, vent ou état de mer,
  hauteur et distance d'observation, vitesse du bateau, lumière. Une référence sans conditions
  connues sert à la perception, pas à la mesure.
- **Verdicts** en langage libre.

Les références ne sont **pas versionnées** — binaires, et leurs droits appartiennent à leurs
auteurs. Elles se rangent localement dans `viewer/captures/references/`, que `.gitignore`
exclut ; le dépôt garde leur description, leur source et leurs conditions.

## 4. Comment un verdict se consigne

Chaque retour devient une ligne du registre ci-dessous. Le verdict est **résumé fidèlement**, puis
**attribué** à l'une de ces classes, car chacune mène à un travail différent :

| classe | exemple | suite |
|---|---|---|
| **physique fausse** | angle du sillage, vitesse de phase, forme des crêtes | angle mort, oracle ou banc qui le confirme |
| **physique juste, perçue fausse** | houle linéaire aux crêtes trop rondes, lumière | modèle manquant nommé (couche et ADR), pas un réglage |
| **habillage de banc** | ciel, couleur, brouillard | réglage libre de l'hôte, **étiqueté comme tel** |
| **hors capacité construite** | écume, gerbe, cavité, vue sous-marine | jalon de la feuille de route qui la porte |
| **artefact numérique** | alias à l'horizon, coutures, scintillement | angle mort du rendu, mesure de l'artefact |

Un défaut d'une classe physique n'est tenu pour établi qu'après confirmation par une mesure ; le
verdict en est le **déclencheur**. Un verdict qui contredit une réception existante se consigne
tel quel, et c'est la réception qu'on réexamine d'abord.

## 5. Registre des revues

| revue | date | images | références | verdict résumé | classe et suite |
|---|---|---|---|---|---|
| **R1** | 2026-09-16 | sept rendus J1, §6 | demandées, §6.3 ; aucune reçue au premier verdict | **« La mer est trop lisse, on dirait un lac »** (22:13) | provisoire : **physique juste mais incomplète**, spectre de B coupé à `4 fp` ; confirmation par la `mss` contre Cox–Munk (§7) |

## 6. R1 — la scène J1 telle qu'elle est, S254

```
cd viewer
cargo run --release --offline --locked -- --multi --revue
python ../outils/apercu_ppm.py captures/s254/<image>.ppm
```

Deux exécutions rendent les mêmes empreintes. GPU NVIDIA RTX 5070 Laptop, DX12, 1280×720, champ
vertical 50°, grille projetée à deux pixels, filtre spectral et grille du sillage actifs, visibilité
active.

### 6.1 Ce qui est rendu

- **B** : mer JONSWAP V1 de S201 — `Hs` 1,5 m, `Tp` 6 s, **32 composantes**, bande 0,5 à 4 `fp`,
  donc **aucune onde plus courte que 3,5 m**. Linéaire : crêtes et creux symétriques.
- **W** : trois sillages d'un journal commun (source de pression 19 620 N, σ 2 m, 3 m/s pendant
  16 s depuis x = −24 m) et huit impacts nés toutes les 4 s (S235).
- **Absents** : δ, V, écume, gerbes, cavités, ondes capillaires, réfraction, sous-surface.
- **Habillage de banc** : ciel uniforme, soleil, couleur de l'eau, brouillard d'horizon.

### 6.2 Images

| image | œil (m) | lacet / tangage (rad) | âge | contenu | empreinte |
|---|---|---|---:|---|---|
| `r1_reference_12s` | 0, −18, 7 | 0 / −0,131 | 12 s | B + W, 4 impacts, sillages | `0x7829238e42056201` |
| `r1_reference_fond_seul_12s` | 0, −18, 7 | 0 / −0,131 | 12 s | B seul | `0x275081ad983db7c9` |
| `r1_haute_12s` | 0, −40, 30 | 0 / −0,55 | 12 s | B + W | `0xe48879f40a422831` |
| `r1_plongeante_12s` | 0, 0, 90 | 0 / −1,2 | 12 s | B + W | `0x47815f87fba2073f` |
| `r1_rasante_12s` | 0, −18, 2 | 0 / −0,05 | 12 s | B + W | `0xdc2ef3e2cf2bb9de` |
| `r1_impact_proche_5s` | 0, −8, 3 | 0 / −0,3 | 5 s | B + W, 2 impacts | `0x1aa56f756745eff8` |
| `r1_large_horizon_29s` | 0, −18, 25 | 0,6 / −0,12 | 29 s | B + W, hors domaine honnête du sillage | `0xa329a85c503a2db3` |

La carte `captures/s248/mer_perturbation.png` (S248, `0x348c9e100ab10c7f`) accompagne R1 : ce
n'est pas un rendu, c'est la hauteur de W seule, vue de dessus, pour montrer où sont sillages et
impacts.

### 6.3 Questions posées et références demandées

Questions : échelle perçue (mer, lac, piscine ?) ; premier défaut qui saute aux yeux ; sillages et
impacts visibles ou non, et le seraient-ils en vrai à cette distance ; ce qui manque à l'horizon.

Références, avec leurs conditions si connues :

1. mer ouverte, **force 4 à 5** (vent ≈ 8–10 m/s, creux ≈ 1,5 m), vue depuis un pont à **2–7 m**,
   vers l'horizon ;
2. la même mer vue de **25–30 m** (passerelle, falaise) ;
3. **drone à la verticale**, 50–100 m, d'un petit bateau lent (**≈ 3 m/s, 6 nœuds**) et de son sillage ;
4. **objet tombant** dans l'eau libre, vu de 3–8 m : les anneaux, puis la gerbe ;
5. **vidéos** de 10–20 s des cas 1 et 3, pour le mouvement.

## 7. R1 — premier verdict : « la mer est trop lisse, on dirait un lac »

**Reçu le 2026-09-16 à 22:13**, sans référence jointe ; il porte sur l'ensemble des rendus.

**Hypothèse, écrite avant mesure.** B s'arrête à `4 fp` (`λ` ≥ 3,5 m) : aucune rugosité plus
courte ne module la lumière, et la surface paraît vitreuse comme un plan d'eau abrité. Classe
provisoire : physique juste mais incomplète. L'habillage (ciel uniforme, soleil fixe) y contribue
peut-être aussi, mais il ne se corrige pas avant la mesure.

**Mesure et critère** (SPEC-001 §1 sexies). Calculer la `mss` de la recette R1 (32 composantes
cuites) et celle du même spectre coupé à `4, 8, 16, 24, 32 fp`. La comparer à Cox–Munk au vent
minimal soutenant `Hs = 1,5 m`. **Défaut confirmé** si la `mss` de B est inférieure à la moitié de
la borne basse de Cox–Munk à ce vent. Sinon, le verdict se reporte sur l'habillage, et on le dit.

