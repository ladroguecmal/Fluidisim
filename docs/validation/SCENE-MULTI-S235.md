# Scène multi-sources, visibilité et retour dans le champ — S235, 2026-09-14

Suite de [LOD-SILLAGE-S234](LOD-SILLAGE-S234.md), priorité 4 du
[bilan S227](../registres/BILAN-GLOBAL-S227.md) : **scène représentative** et **retour visible**.
Aucun ADR. Code : `viewer/src/scene.rs` (`WAKE_OFFSETS`, `IMPACTS`, `ImpactSlot`),
`viewer/src/lod.rs` (`footprint`, `touches_rect`, `touches_disc`), `viewer/src/main.rs`
(`--scene-admission`, `--multi --admission-reelle`, `--multi --retour`, `--multi --verify`).

## 1. La scène, déclarée avant toute mesure

- **Trois sillages** dans un même journal de pression, même recette (64×128, cutoff 3), même
  emprise [−64, −48]–[64, 56] m, trajectoires y = 4 / −26 / 34 m — l'espacement « éloignés » de
  S222 (30 m, au-delà de la largeur de Kelvin à 3 m/s). Le premier est le sillage S212.
- **Huit impacts** de l'entrée S203, nés toutes les 4 s de 0 à 28 s, dans le champ de la caméra
  S201, disques de 52 m qui se recouvrent. Variante **dense** à 1 s d'écart, publiée pour ce qu'elle
  refuse.

Un hôte n'inscrit un impact au journal qu'à sa naissance. Le premier essai de prédiction l'ignorait :
avec les huit impacts inscrits dès 0 s, le plancher valait 2,52 π/7, parce que `slope_max_at` rend
le maximum de naissance **avant** la naissance (ADR-133 : on ne resserre pas ce qu'on n'a pas
mesuré). C'est un choix du cœur, pas un défaut ; le journal de la prédiction est reconstruit à chaque
instant avec les seuls impacts nés.

## 2. Admission par le cœur : refusée, et seulement par ses majorants

`mixed::slope_floor` (ADR-128, ADR-133, ADR-138), tous les 0,25 s sur 40 s :

| variante | instants refusés / 161 | premier refus | pire plancher |
|---|---:|---:|---:|
| scène (4 s) | **49** | 8 s (3e naissance) | 1,251 π/7 à 28,25 s |
| dense (1 s) | 40 | 3 s | 1,420 π/7 à 7 s |

À chaque naissance, le plancher additionne la pression des trois sillages (0,175–0,202, soit 39 à
45 % de π/7), l'impact neuf (0,213, 47 %) et les majorants des impacts plus anciens ; entre les
naissances il redescend (0,78–1,07 π/7) et, après 32 s, reste à 0,73–0,86. S222 écrivait que « huit
impacts passent » : c'était vrai pour des impacts **âgés**, pas pour des naissances renouvelées.

**Pente réelle aux instants refusés.** Perturbations seules (B à amplitude nulle), sillage en somme
directe au GPU (écart au cœur ≤ 2·10⁻⁴ en pente, S234), balayage à 0,25 m de la boîte
[−86, 86]×[−60, 102] m qui contient l'emprise et les huit disques (447 161 points), raffinement à
0,02 m sur ±0,24 m autour des seize meilleurs points.

| grandeur | valeur |
|---|---|
| refus de **majorant seul** | **49 / 49** |
| refus de pente réelle | 0 |
| pente réelle maximale aux instants refusés | **0,2154** — 48,0 % de π/7 (24 s) |
| marge minimale π/7 / pente réelle | **×2,058** |
| plancher / pente réelle | 2,13 à 8,16 |
| témoins admis (10 / 18,5 / 32 / 39 s) | réelle 0,056–0,106 ; plancher/réelle 3,7–6,8 |

Aux naissances, le maximum réel (0,2127–0,2154) se trouve au centre de l'impact neuf et vaut son
maximum de naissance, exact à cet instant (0,2126, S139) : **les autres sources n'y ajoutent rien de
mesurable**, alors que le plancher leur réserve 0,24 à 0,35. Le maximum réel oscille de 0,07 à 0,21
d'un quart de seconde à l'autre, avec la phase de l'anneau neuf.

**Ce que cela décide.** Le budget de pente refuse une scène représentative dont la pente réelle
reste à moitié du seuil. C'est le déclencheur écrit d'**A255/A261** (« un usage échoue à
l'admission actuelle ») : resserrer les bornes — enveloppe de pression à sources séparées (A261,
pessimisme 1,64–2,35 en S222) et majorants d'impacts anciens loin du maximum — sans mesure
substituée à une borne (I-18). *Limite* : maximum échantillonné, donc inférieur ou égal au vrai ;
la marge ×2,06 couvre une sous-estimation de quelques pour cent au pas de 2 cm, pas davantage.

## 3. Rendu de la scène contre le cœur

Une seule table radiale sert pour les huit impacts (même entrée) ; chacun lit le profil de la table à
son âge. Tampon GPU `impacts` (centre relatif, actif), profils concaténés. Le chemin mono-impact
reste **identique au bit** : toutes les lignes `VERIFY` et `LOD_INTERIEUR` de S234 sont retrouvées.

`--multi --verify`, 23 âges déclarés (chaque naissance et la seconde suivante, pire plancher, fins de
forçage, de contexte et d'horizon), deux chemins du sillage :

| contrôle | grille | direct |
|---|---:|---:|
| max η GPU − cœur, 6 988 sondes | **0,368 mm** (12 s, 4 impacts) | 0,091 mm |
| intérieurs : grille − direct | 0,212–0,392 mm | — |
| borne de reconstruction | 2,45–2,98 mm (rapports 0,07–0,16) | — |
| pas / nœuds | 1,125–1,1875 m / ≤ 10 810 | — |
| saut à travers les arêtes | ≤ 7 µm | — |

Tolérance de 3 mm tenue à tous les âges. Le pas se resserre un peu (1,125 m contre 1,1875 en mono) :
les amplitudes modales des trois sillages s'additionnent dans les 4 096 modes du journal commun.

## 4. Visibilité et retour dans le champ

**Emprise.** Les sommets ne bougent qu'en hauteur : une source ne contribue à l'image que si un
sommet tombe dans son domaine horizontal. `lod::footprint` rend le contour de l'emprise de la grille
sur le plan d'eau, échantillonné à chaque sommet de bord avec les opérations de `ocean_vertex`.
Rangées et colonnes se projettent en segments droits (projection centrale d'une droite) ; seule la
borne de 1 500 m y crée un coude entre deux échantillons, que majore la longueur de l'arête. D'où une
**marge par arête** : quelques centimètres au premier plan, quelques centaines de mètres à
l'horizon. Une première version à marge globale — la plus longue arête — a été écartée avant usage :
elle aurait gonflé tout le premier plan.

**Ce qui est retiré hors champ** : repli du levier temporel, plan de grille et cuisson du sillage ;
profil et évaluation d'un impact. Les vérifications gardent `cull = false` : leurs sondes sont hors de
l'emprise de la caméra.

**Retour** (`--multi --retour`, 960×540, 10 → 14 s à 60 Hz). Passage continu sans visibilité, puis
même passage avec visibilité et caméra détournée (0, −300, 12), dos à la scène, sur [11 ; 13,5[ s —
intervalle qui contient une fin de tronçon (12 s), donc un repli après saut.

| grandeur | valeur |
|---|---:|
| images cachées | 150 |
| sillage retiré | 150 fois |
| impacts nés retirés | jusqu'à 4 |
| images comparées après le retour | 31 |
| **images différentes au bit** | **0** |

Comparés au bit : coefficients publiés du sillage, plan de grille, profils, activités et les 6 988
valeurs GPU aux sondes. Le repli de `pressure_timeline` ne dépend que de l'instant — il n'est
incrémental que lorsque la suite d'additions est celle d'un repli complet — et l'essai le vérifie au
lieu de le supposer.

## 5. Coût

### En-tête de mesure (ADR-131 D3)

- **Techniques présentes** : phases repliées de B (S211) ; table de Bessel partagée par les huit
  impacts (ADR-129) ; repli temporel du sillage (S213) ; **mutualisation des nœuds** des trois
  sillages d'un même journal et d'une même recette (4 096 modes au total, cœur, S222) ; grille
  locale du sillage et reconstruction bicubique (S234) ; **visibilité par emprise de la grille**
  (S235, CPU et GPU) ; grille projetée à 2 px.
- **Techniques absentes** : transformée ; LOD de maillage ; LOD spectral ; **LOD temporel** (le
  levier est replié et publié à chaque image) ; **parallélisme CPU** (un fil) ; occlusion ;
  visibilité des composantes de B (toujours évaluées).
- **Domaine** : scène §1, 960×540 (et 640×360 au banc), AMD Ryzen AI 7 350, RTX 5070 Laptop,
  DX12, Windows, release. **Alimentation : secteur** au début et à la fin de chacun des douze
  passages (A270). Binaire témoin S233 (`98430a1`) au premier et au dernier passage.

### 5.1 Banc hors écran — passe d'eau GPU, cuisson comprise

| format | scène | âge | grille | dont cuisson | direct | CPU sillage |
|---|---|---:|---:|---:|---:|---:|
| 960×540 | mono | 3 s | 0,4371 | 0,3792 | 4,2933 | 1,32 |
| 960×540 | mono | 16 s | 0,4406 | 0,3821 | 4,2931 | 0,39 |
| 960×540 | **multi** | 3 s | **0,4478** | 0,3803 | 4,4028 | **3,10** |
| 960×540 | **multi** | 29 s | **0,4731** | 0,3965 | 4,3373 | 0,48 |
| 640×360 | multi | 3 s / 29 s | 0,3990 / 0,4110 | 0,3716 / 0,3786 | 1,9956 / 1,9693 | — |

Médianes en ms. Trois sillages et huit impacts coûtent **+0,01 à +0,03 ms de GPU** par rapport à une
source de chaque : les nœuds des sillages sont mutualisés dans le journal, et un impact coûte une
lecture de table par sommet de son disque. Le témoin direct passe de 4,29 à 4,40 ms.

### 5.2 Cadence de la fenêtre

| passage | Hz | intervalle | GPU eau | CPU de trame | dont sillage | acquisition |
|---|---:|---:|---:|---:|---:|---:|
| témoin S233 (début) | 191,1 | 5,2338 | 4,1551 | 4,4324 | 1,2591 | 2,2942 |
| mono, visibilité | 341,3 | 2,9303 | 0,4650 | 2,1270 | 1,3219 | 0,0224 |
| multi 3 s, visibilité | **204,2** | 4,8973 | 0,4459 | **4,0699** | **3,1219** | 0,0289 |
| multi 29 s, visibilité | **415,4** | 2,4072 | 0,4665 | 1,6230 | 0,4832 | 0,0254 |
| multi 29 s, sans | 428,5 | 2,3336 | 0,4642 | 1,5721 | 0,4782 | 0,0257 |
| multi 29 s balayée, visibilité | 425,2 | 2,3520 | 0,4500 | 1,5879 | 0,4810 | 0,0232 |
| multi 29 s balayée, sans | 436,9 | 2,2887 | 0,4507 | 1,5239 | 0,4833 | 0,0241 |
| **multi 29 s hors champ, visibilité** | **726,7** | 1,3761 | **0,0616** | **0,6688** | 0 | 0,0179 |
| multi 29 s hors champ, sans | 423,4 | 2,3618 | 0,4480 | 1,5906 | 0,5137 | 0,0257 |
| témoin S233 (fin) | 194,8 | 5,1335 | 4,1628 | 4,4035 | 1,2637 | 2,3432 |

Quatre constats.

1. **La scène multi-sources tient la passe d'eau GPU sous 0,47 ms** dans toutes les poses.
2. **La cadence est bornée par le CPU**, et pendant le forçage des trois sillages ce CPU double :
   préparation du sillage 3,12 ms (repli et rotation par nœud **et par source en cours**), contre
   0,48 ms une fois le forçage achevé. La fenêtre passe de 415 à 204 Hz.
3. **Hors champ, la visibilité rend ce qu'elle promet** : GPU d'eau 0,448 → **0,062 ms** (seul B
   reste), CPU 1,59 → 0,67 ms, 423 → 727 Hz. Retour au bit vérifié (§4).
4. **Dans le champ, elle coûte un peu** : +0,05 ms de CPU (contour de ≈1 500 points et tests), −3 %
   de cadence (415 contre 428 Hz fixe, 425 contre 437 balayée). En balayage elle retire jusqu'à
   quatre impacts sans gain GPU mesurable (0,450 contre 0,451) : un impact est bon marché, la
   cuisson du sillage visible domine.

Le mono à 341 Hz reste en deçà des 384 Hz de S234 (CPU 2,13 contre 1,96 ms, présentation 0,61
contre 0,51) ; la part de la visibilité (≈0,05 ms) n'explique pas tout, le reste n'est pas
attribué.

### 5.3 Confrontation à ADR-125 — la passe GPU et le total

ADR-125 fait couvrir aux 2 ms « B/W/δ et le rendu associé sur le chemin critique » et ne fixe pas
la répartition CPU/GPU. Deux lectures, publiées ensemble :

| scène, fenêtre 960×540 | GPU eau | travail CPU de trame (hors attente) | somme |
|---|---:|---:|---:|
| témoin S233 | 4,16 | 2,14 | 6,30 |
| mono S235 | 0,47 | 2,10 | 2,57 |
| multi, forçage des trois sillages (3 s) | 0,45 | **4,04** | **4,49** |
| multi, forçage achevé (29 s) | 0,47 | 1,60 | 2,07 |
| multi, hors champ | 0,06 | 0,65 | 0,71 |

Le travail CPU est celui de l'hôte entier (boucle, caméra, B, profils, transferts, soumission) : il
majore l'eau seule. **Verdict selon ADR-131 D1** : sur ce domaine, l'implémentation S235 tient la
**passe GPU** de la scène multi-sources à moins d'un quart des 2 ms ; son **CPU mono-fil, sans LOD
temporel ni parallélisme**, porte la somme à 2,1–2,6 ms hors forçage et à 4,5 ms pendant le forçage
de trois sillages. Ce dépassement qualifie cette implémentation CPU, pas la scène ; les techniques
absentes qui le visent sont nommées dans l'en-tête.
