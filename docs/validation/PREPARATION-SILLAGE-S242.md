# La préparation CPU du sillage, par image — S242, 2026-09-15

Traite le poste devenu dominant du budget de l'hôte. Depuis le LOD spatial de
[S234](LOD-SILLAGE-S234.md), le GPU eau vaut **0,44 ms** par image et la seule préparation CPU du
sillage **3,17 ms de médiane pour 13,2 ms de maximum** ([S240](ALLOCATIONS-HOTE-S240.md) §2.2), quand
[ADR-125](../adr/ADR-125-budget-image-60hz-deux-ms.md) donne **2 ms à toute l'eau**. La conclusion de
[S225](CADENCE-HOTE-S225.md) — « les techniques restantes sont du côté GPU » — n'est plus vraie.

## 1. Protocole, écrit avant mesure et construction

### 1.1 Ce que la lecture du code établit

`Timeline::render_components` fait, **à chaque image et pour chacun des 4 096 nœuds**, une boucle sur
les **24 segments** du journal — trois sillages de huit tronçons de 2 s — en testant
`mode.birth() < time && time < mode.forcing_end()`. Soit **98 304 tests par image**.

Or `Timeline::fold`, juste au-dessus, écrit et utilise le fait que **naissance et durée sont les
mêmes pour tous les nœuds** : il ne lit que la **rangée 0** pour connaître les fins de tronçon.
L'ensemble des segments actifs à un instant ne dépend donc **que du temps**, jamais du nœud.

Les huit tronçons d'un sillage se suivent sans recouvrement : à tout instant, **au plus un segment
par sillage** est en forçage — trois sur vingt-quatre —, et après 16 s de forçage, **aucun**.

### 1.2 Thèse

Un test dont le résultat est identique pour les 4 096 nœuds se calcule **une fois par image**. Hisser
cette sélection hors de la boucle des nœuds **ne change aucune opération flottante** : les mêmes
termes sont additionnés dans le même ordre, seuls les tests qui ne retenaient rien disparaissent. Le
gain est donc **gratuit au bit**, et il croît avec le nombre de tronçons de la scène.

### 1.3 L'instrument, et pourquoi il ne regarde pas dans la fonction

Instrumenter l'intérieur de `render_components` demanderait d'y poser des horloges, donc d'en
déplacer le coût. **On fait varier le paramètre au lieu d'ouvrir la boîte** : le même banc mesure la
préparation à **un, deux et trois sillages** — 8, 16 et 24 segments — pour un nombre de nœuds
**inchangé** (4 096). La pente contre le nombre de segments *est* la part de la boucle interne, et
elle se mesure avant comme après. Banc : `examples/sillage_troncons.rs`, fixture de S212/S235
inchangée.

Deux fenêtres, parce qu'elles n'exercent pas le même chemin :

- **pendant le forçage** (à partir de 3 s d'âge) : un segment actif par sillage ;
- **après le forçage** (à partir de 24 s) : **aucun** segment actif, tout est replié — c'est là que
  la boucle interne ne rapporte rien du tout, et que le coût inutile est le plus visible.

### 1.4 Critères de réception, déclarés avant construction

1. **Décomposition mesurée avant la construction** : coût par image contre le nombre de segments,
   dans les deux fenêtres ; la part imputable à la boucle interne en découle par la pente.
2. **Au bit** : `--multi --verify` rend **0,368476 mm** à 12 s avec 4 impacts ; `--multi --retour`
   rend **0 image différente** sur 31 comparées ; et le banc compare les **4 096 coefficients
   publiés** avant et après, au bit, sur une suite d'images.
3. **Coût publié** : banc avant et après ; puis `--multi --cadence` de l'hôte, CPU sillage médian,
   p95 et maximum, avec les allocations de `update` **toujours à zéro** (ADR-145).
4. **La revendication porte sur la croissance**, pas sur un point : la pente contre le nombre de
   segments doit tomber, et le dire.
5. **Coût** (ADR-131) : techniques présentes, absentes, domaine.
6. Aucun seuil modifié, aucune tolérance touchée, aucune ambition rouverte.

### 1.5 Arrêt

La sélection hissée, reçue au bit et chiffrée. **Ou** constat mesuré que la pente contre le nombre de
segments est négligeable — auquel cas la boucle interne n'est pas le poste, la mesure désigne le vrai,
et la session l'écrit sans rien construire.

## 2. La mesure, avant construction — et ce qu'elle réfute

`examples/sillage_troncons.rs`, release, un fil, 4 096 nœuds, 120 images par ligne.

| sillages | segments | fenêtre | actifs | médiane | p95 | maximum |
|---:|---:|---|---:|---:|---:|---:|
| 1 | 8 | forçage | 1 | 1,1990 ms | 1,4236 | 1,8061 |
| 1 | 8 | après forçage | **0** | **0,3114 ms** | 0,3766 | 0,9246 |
| 2 | 16 | forçage | 2 | 2,1004 ms | 2,5288 | 4,7211 |
| 2 | 16 | après forçage | **0** | **0,3367 ms** | 0,4026 | 1,0135 |
| 3 | 24 | forçage | 3 | 2,9679 ms | 3,3752 | 6,8566 |
| 3 | 24 | après forçage | **0** | **0,3709 ms** | 0,4653 | 1,3856 |

La médiane de trois sillages en forçage — **2,9679 ms** — rejoint les 3,17 ms que l'hôte mesure
(S240), le reste étant la planification du LOD, hors de ce banc.

### 2.1 La thèse du §1.2 est réfutée dans sa grandeur

Après le forçage, **aucun** segment n'est actif : tout ce que la boucle interne fait alors est
perdu. Son coût s'y lit directement, et il vaut **0,0595 ms entre 8 et 24 segments**, soit
**3,7 µs par segment inutile** — environ **2 % des 3,17 ms** de l'hôte. Hisser la sélection ne rend
pas deux millisecondes ; le protocole §1.5 avait prévu ce cas, et c'est celui-là.

### 2.2 Ce que la même mesure désigne, elle

Par différence entre les deux fenêtres, à segments égaux :

| grandeur | valeur mesurée |
|---|---:|
| coût fixe par nœud (aucun segment actif, 8 segments) | 0,3114 ms / 4 096 = **76 ns** |
| coût d'**un segment actif**, tous nœuds | (1,1990 − 0,3114) = **0,8876 ms** |
| idem, à trois segments actifs | (2,9679 − 0,3709) / 3 = **0,8657 ms** |
| par nœud et par segment actif | ≈ **211 ns** |

**Le poste est `ModalPressure::sample`** : 2,60 ms sur 2,97, soit **87 %** de la préparation pendant
le forçage. Chaque appel enchaîne cinq à sept `sin_cos` déterministes — deux dans `sample`, une à
deux dans chacune des deux `integral`, une pour la rotation libre —, et il y en a 4 096 × 3 par image.

### 2.3 Une impasse, à ne pas réexplorer sans la payer

`Complex::phase(negative(p))` **semble** être le conjugué de `Complex::phase(p)`, donc gratuit. Il ne
l'est pas **au bit** : `sin_cos` reconstruit l'angle par quadrant, et pour `−q` l'argument du
polynôme est recalculé depuis l'entier `2³⁰ − W`, pas obtenu par soustraction flottante de celui de
`q` ; les deux diffèrent d'une unité dans le dernier rang. À `q = 0` s'ajoute le **zéro signé** :
le conjugué rend `−0,0` là où `sin_cos(0)` rend `+0,0`. Même remarque pour le raccourci « pendant le
forçage, la rotation libre est l'identité » : elle l'est en valeur, pas en bits, pour la même raison
de zéro signé. **Réduire le nombre de `sin_cos` est possible, mais cela change des bits publiés** :
c'est un lot avec son propre relevé avant/après, pas une simplification gratuite.

### 2.4 Ce qui reste exact et gratuit, et pourquoi il vaut quand même d'être fait

La sélection hissée ne rend pas 2 ms, mais elle change la **forme** du coût : aujourd'hui, la
préparation par image croît avec le nombre de tronçons **achevés**, c'est-à-dire avec l'histoire du
journal. Un jeu qui accumule des sillages paie **3,7 µs par tronçon mort et par image**, pour
toujours. À 200 tronçons — une partie ordinaire —, c'est **0,74 ms par image**, plus du tiers du
budget de toute l'eau, pour des termes qui ne contribuent à rien. Ce n'est pas une constante gagnée,
c'est un terme de croissance supprimé.

## 3. La construction

La sélection des tronçons est **hissée hors de la boucle des nœuds** : la part repliée est posée
d'abord pour tous les nœuds, puis chaque tronçon **actif** — et lui seul — parcourt les nœuds. Les
mêmes termes sont ajoutés au même accumulateur dans le même ordre, `j` croissant ; aucune opération
flottante ne change.

Deux points de contrat, écrits parce qu'ils se voient :

- **Le verdict de finitude est celui d'avant**, seul l'instant où il est rendu change. L'erreur ne
  porte aucun indice de nœud, et `out` n'est écrit qu'après : il reste intact à tout refus, comme
  l'exige `refusals_leave_output_and_state_usable_s213`.
- Une variante a été construite puis **écartée par la mesure** : trier sur la rangée 0 en gardant
  l'imbrication d'origine. Elle ne gagne rien — la ligne `modes[n·segments + j]` est déjà contiguë,
  et ce n'est pas la lecture qui coûte, c'est la boucle. Elle a servi de **témoin** (§4.2).

## 4. Réception

### 4.1 Au bit

Les **six empreintes** des 4 096 coefficients publiés, sur les deux fenêtres et les trois tailles de
journal, sont identiques avant et après, et sur les trois exécutions :
`0x95a8239f6f9cc139`, `0x0af49e8f4c2aa21c`, `0xe14cee491a007edc`, `0x24b5f8e1f47300d0`,
`0x393d0b9d526daf84`, `0xdf150d01e0afe270`.

| contrôle | résultat |
|---|---|
| `--multi --verify`, 23 âges | **0,368476 mm** à 12 s avec 4 impacts — valeur de S235 |
| `--multi --retour` | 150 images cachées, 31 comparées, **0 différente au bit** |
| suite `code/` complète | **431 réussis, 0 échec, 12 ignorés** |
| chemin indépendant | `timeline_matches_prepared_across_instants_and_jumps_s213` passe — c'est lui qui aurait vu une sélection fausse |
| allocations de l'hôte | `update` **0**, image 133 / 18 509 o — inchangé (ADR-145) |

### 4.2 Coût, et ce que le témoin permet d'affirmer

**Après le forçage — aucun tronçon actif, tout le travail de tri était perdu :**

| segments | avant | après (trois exécutions) |
|---:|---:|---|
| 8 | 0,3114 ms | 0,3070 / 0,3022 / 0,2931 |
| 16 | 0,3367 ms | 0,2960 / 0,2972 / 0,3077 |
| 24 | **0,3709 ms** | **0,2979 / 0,3000 / 0,2986** |

Avant, la médiane **croissait de 19 % entre 8 et 24 segments**, régulièrement. Après, elle tient
entre 0,293 et 0,308 ms **sans aucune tendance** : le terme de croissance a disparu, et à 24 segments
le coût tombe de **19 %**.

**Pendant le forçage**, médianes après : 1,21 / 2,11 / 3,04 ms contre 1,199 / 2,100 / 2,968 avant,
soit **+0,5 à +2,4 %**. Ce n'est pas un écart mesurable : la variante écartée du §3, **sémantiquement
neutre**, a rendu 1,2579 / 2,2630 / 3,0462 sur le même banc, soit **+2,6 à +7,7 %** sur la même base.
Le bruit d'exécution de cette fenêtre dépasse donc l'écart qu'on aurait voulu attribuer au
changement, et ce document ne le lui attribue pas.

**Sur l'hôte** (`--multi --cadence`, scène S235 en forçage, donc la fenêtre neutre) : CPU médian
**3,9877 ms** contre 4,0367 en S240, sillage **3,1110** contre 3,1707 — dans le bruit, comme attendu.

### 4.3 Ce qui est réellement gagné

Pas une constante : **un terme de croissance**. La préparation ne dépend plus du nombre de tronçons
**achevés**, c'est-à-dire de l'histoire du journal. Au tarif mesuré avant — **3,7 µs par tronçon mort
et par image** —, une partie ordinaire qui aurait accumulé 200 tronçons payait **0,74 ms par image**,
plus du tiers du budget de toute l'eau (ADR-125), pour des termes qui ne contribuent à rien. Elle
paie désormais **zéro**.

### 4.4 Coût de la mesure (ADR-131)

- **Techniques présentes** : repli des tronçons achevés (S213), modes préconstruits, publication
  rebasée, **sélection hissée** (S242).
- **Techniques absentes** : parallélisme CPU — ce projet n'a **aucun** fil d'exécution, `JobSystem`
  n'expose qu'une réduction ordonnée —, SIMD explicite, LOD temporel, mutualisation entre journaux,
  réduction du nombre de `sin_cos` (§2.3 : elle change des bits).
- **Domaine** : fixture S212/S235, recette 64×128 (4 096 nœuds), 60 images/s simulées, release, un
  fil, une machine ; ni GPU, ni transfert, ni LOD spatial.

### 4.5 Ce qui n'est pas reçu

- **Le poste dominant reste entier.** `ModalPressure::sample` vaut toujours 0,87 ms par tronçon actif
  et par image, 87 % de la préparation en forçage. Rien ici ne l'entame.
- Un seul nombre de nœuds (4 096), une seule recette, une seule machine, un seul `dt`.
- La fenêtre de forçage est trop bruitée sur ce banc pour y décider d'un écart de quelques pour cent :
  un banc qui voudrait trancher là devra d'abord réduire son propre bruit.
