# La cadence complète de l'hôte — S225, 2026-09-13

Travail nécessaire de J1 (ADR-131 D6), nommé depuis S213 et jamais mesuré. Aucun ADR : la session
mesure, et le verdict de coût qu'elle confirme est celui d'ADR-125.

## En-tête de mesure (ADR-131 D3)

- **Techniques présentes** : phases repliées de B au GPU (S211) ; table de Bessel de l'impact à λ/16
  (ADR-129) ; repli temporel du sillage (S213) ; publication `[A, B, kx, ky]` rebasée ; somme modale
  par sommet sur GPU dans l'emprise ; grille projetée à 2 px.
- **Techniques absentes** : espace (grille et transformée) ; LOD spatial, spectral, temporel ;
  visibilité ; mutualisation ; parallélisme CPU (un fil). Ce sont les quatre premières qui restent à
  J1-bis, et toutes sont du côté GPU — mais §4-bis montre que « le GPU domine » ne vaut qu'à la
  charge mesurée.
- **Domaine de validité** : scène J1 (mer S201, un impact, un sillage prescrit 64×128), fenêtre
  960 × 540, **caméra fixe**, aucune interface, aucune ombre, aucune autre géométrie ; AMD Ryzen
  AI 7 350, RTX 5070 Laptop, DX12, Windows, release, un fil. La **caméra balayée** est mesurée à
  part (§4-bis). **Ne dit rien** d'un jeu, ni d'un autre format.
- **Rang de passage** : quatre passages séparés ; écarts publiés au §2.

## 1. Ce qui manquait, et pourquoi une passe isolée ne pouvait pas le dire

`Gpu::benchmark` dessine **hors écran**, sur une texture créée pour l'occasion. Sa ligne de sortie
porte sa propre réserve depuis S211 — *« sky, upload, readback, presentation excluded »* — et la
paire d'horodatages n'entoure que la passe « water only » : le ciel est dessiné et non chronométré.

Trois choses manquaient donc, et ce sont celles qu'une cadence exige : la **chaîne d'échange**
(acquérir, présenter), le **coût de ce qui est exclu**, et la **cadence elle-même** — l'intervalle
réel entre deux images, dont rien ne disait qu'il valait la somme des passes.

**Le piège est la vsync.** En `AutoVsync`, l'intervalle mesure l'écran, pas le coût : on obtiendrait
60 ou 120 Hz quel que soit le travail. La mesure se fait donc en **`AutoNoVsync`**, et la ligne
publiée le dit.

**Et les deux mesures ne peuvent pas cohabiter.** Relire un horodatage appelle
`poll(wait_indefinitely)`, qui **sérialise** CPU et GPU : le faire à chaque image détruit le
recouvrement, donc la cadence. D'où deux phases — 590 images sans relecture pour l'intervalle,
200 avec relecture pour la décomposition, cette dernière publiée sous le nom
`DECOMPOSITION_serialisee` et qui **n'est pas** une cadence.

## 2. Ce que la cadence vaut

| grandeur | médiane | p95 | max |
|---|---:|---:|---:|
| **intervalle entre présentations** | **5,0450 ms — 198,2 Hz** | 5,8988 | 6,5087 |
| CPU de trame | 4,3160 | 5,1743 | 5,8655 |
| — dont **acquisition d'image** | **2,2212** | — | 3,9164 |
| — dont sillage (cœur) | 1,2453 | — | 2,2971 |
| — dont transfert | 0,2076 | — | 0,4883 |
| — reste (B, profil, soumission) | 0,6419 | — | — |
| présentation | 0,5989 | — | 1,8279 |
| **GPU passe d'eau** | **4,1585** | — | 4,8583 |
| **GPU trame entière** (ciel + eau) | **4,2455** | — | 4,9569 |

**Rang de passage.** Quatre passages : intervalle 4,9187 / 4,9560 / 4,9433 / 5,0450 ms — **écart
2,6 %** —, GPU d'eau 4,1312 / 4,1348 / 4,1485 / 4,1585 — **écart 0,7 %**. C'est bien plus
reproductible que les 20 à 40 % relevés en S213 sur une mesure de CPU, et la raison est la charge :
**une mesure prise sous charge soutenue ne varie pas comme une mesure prise en rafales courtes**.
À verser à L289.

## 3. Les exclusions, chiffrées

La réserve écrite depuis S211 était honnête. Elle est maintenant mesurée :

> **La passe d'eau vaut 97,95 à 98,06 % de la trame GPU.** Tout le reste — ciel, changements de
> cible, résolution des requêtes — fait **0,087 ms**.

Les mesures de coût de S211 à S224 ne cachaient donc **rien de matériel** du côté GPU. C'était la
moitié prévisible de la prédiction, et elle est confirmée plus fortement qu'annoncé.

## 4. Ce qui n'était pas prévu : le CPU d'une trame est surtout de l'attente

La prédiction annonçait 6 à 7 ms de GPU pour la trame complète et ~150 Hz. La trame coûte **4,25 ms**
et la cadence atteint **198 Hz** — contredite, dans le bon sens.

Le fait neuf est ailleurs. **L'acquisition d'image vaut 2,22 ms sur 4,32 de CPU, soit 51 %** — et ce
n'est pas du travail : c'est la contre-pression du GPU, le fil attendant qu'un tampon de la chaîne
d'échange se libère. Le travail réel du CPU fait **2,10 ms** : sillage 1,25, transfert 0,21, reste
0,64.

Aucun banc hors écran ne pouvait le voir, puisqu'il dessine sur une texture et n'acquiert jamais
rien. La ligne `CPU_prepare_upload_submit_ms` de `benchmark` — 2,43 ms au même format — mesurait donc
le **travail**, et une trame réelle en passe la moitié à attendre.

**Première conclusion, et elle a été corrigée dans la même session — voir §4-bis.** À caméra fixe,
les 2,10 ms de travail CPU sont recouverts par les 4,16 ms de GPU, ce qui donnait « la trame est
bornée par le GPU, une seconde gagnée sur le CPU serait invisible ». La mesure à caméra balayée
montre que ce n'est vrai qu'à cette charge-là.

## 4-bis. La caméra balayée, qui corrige la conclusion précédente

La réserve « caméra fixe » du §En-tête méritait d'être mesurée plutôt qu'écrite. La grille est
projetée depuis la caméra : ce qui tombe dans l'emprise du sillage dépend de l'orientation, et le
coût GPU est proportionnel aux sommets qui s'y trouvent. Même protocole, caméra tournant en lacet et
en tangage :

| | caméra fixe | caméra balayée |
|---|---:|---:|
| GPU eau, médiane | **4,1585 ms** | **2,8479 ms** |
| GPU eau, maximum | 4,8583 | 4,2347 |
| GPU trame | 4,2455 | 2,9314 |
| intervalle | 5,0450 ms — 198,2 Hz | **4,8773 ms — 205,0 Hz** |
| travail CPU | 2,10 | 2,08 |

**Deux choses, et aucune n'était prévue.**

1. **La pose fixe de la fixture est proche du pire cas, pas du cas courant.** En balayage, la
   médiane du GPU d'eau tombe à **2,85 ms** et son maximum vaut 4,23 — c'est-à-dire que la valeur
   publiée depuis S212 est à peu près le maximum d'un balayage, non sa médiane. Le facteur au budget
   de 2 ms passe de **2,08 ×** à **1,42 ×** en médiane, et reste 2,12 × au pire.
2. **La cadence, elle, ne bouge presque pas** — 205 Hz contre 198 — alors que le GPU a perdu un
   tiers. La trame n'est donc **pas** purement bornée par le GPU à cette charge : en balayage,
   `travail CPU + GPU` vaut 2,08 + 2,93 = 5,01 ms pour un intervalle de 4,88, c'est-à-dire **presque
   aucun recouvrement**, tandis qu'à caméra fixe 2,10 + 4,25 = 6,35 ms donnent 5,05 — donc un
   recouvrement partiel. Le degré de recouvrement change avec la charge, et rien dans la mesure ne
   l'explique. Consigné en **A265**.

La conclusion du §4 doit donc se lire ainsi : **à caméra fixe et à cette charge, le GPU domine** ;
il ne suit pas qu'une optimisation CPU serait toujours invisible, et le dire aurait été aller plus
loin que la mesure.

## 5. Confrontation à ADR-125

| grandeur | mesurée | part d'une trame de 60 Hz (16,67 ms) | budget |
|---|---:|---:|---|
| GPU eau, caméra fixe | **4,1585 ms** | **24,9 %** | 2 ms — **dépassé 2,08 ×** |
| GPU eau, caméra balayée (médiane) | 2,8479 | 17,1 % | **dépassé 1,42 ×** |
| GPU trame (eau + ciel) | 4,2455 | 25,5 % | — |
| trame complète | 5,0450 | **30,3 %** | — |
| travail réel du CPU | 2,10 | 12,6 % | — |

**« L'hôte tient-il 60 Hz » a une réponse, et ce n'est pas la bonne question.** Il les tient trois
fois. Mais ADR-125 ne demande pas que l'eau tourne seule à 60 Hz : elle lui accorde **2 ms d'une
trame de 16,67** qui doit aussi porter un jeu. L'eau en prend 4,16 et la scène entière 5,05 : il
resterait **11,6 ms** pour tout le reste au lieu des 14,67 prévus.

Le verdict de coût ne bouge pas — il est **confirmé par un autre chemin**, et le rapport 2,08 recoupe
les 4,18 ms de S213 à 0,5 % près. Conformément à ADR-131 D1, il qualifie **l'implémentation
mesurée**, dont l'en-tête ci-dessus dit ce qui lui manque.

## 6. Contrôles

`VERIFY` inchangé (7,2271e-5 m à 16 s, tolérance de 3 mm tenue) ; `BENCH` inchangé (GPU eau
4,2037 ms à 960 × 540) ; `--smoke` 120 images, code 0. Les quatre horodatages n'ont déplacé aucun
chemin existant. Suite complète `code/` : **379 réussis, 5 ignorés** — la bibliothèque n'a pas été
modifiée cette session.

## Suite

**Ce que cette mesure ne couvre pas** : un autre format que 960 × 540, l'interaction manuelle, les
angles rasants soutenus, et les allocations de la pile graphique (I-06, toujours non reçues). La
caméra en mouvement, elle, **a** été mesurée (§4-bis) et elle a corrigé une conclusion.
**A265** reste ouverte : le recouvrement CPU/GPU varie avec la charge sans que la mesure l'explique.

**Ce qu'elle oriente** : les quatre techniques restantes de J1-bis sont du côté GPU, et la mesure le
confirme. C'est là que le facteur 2,08 se joue.

Restent, inchangés : A264, A261, A258, A263, J2/δ général et les deux briques suivantes de V —
direction de `g_eff`, état répliqué (ADR-022 §5.1).
