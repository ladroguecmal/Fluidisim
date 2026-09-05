# ADR-016 — Audio de l'eau

- **Statut** : proposée — à confirmer avec l'équipe audio
- **Session** : S02
- **Comble** : angle mort A12 (absent des deux documents sources)
- **Dépend de** : ADR-004, ADR-009, ADR-014

---

## 1. Pourquoi cet ADR existe

Aucun des deux documents sources ne mentionne l'audio. L'audio d'eau consomme pourtant exactement
les mêmes données que le rendu : hauteur, vitesse, cambrure, énergie dissipée, champ d'écume,
événements de déferlement, état d'immersion. S'il est câblé après coup, il le sera sur des sondes
parallèles, avec ses propres distances de coupure et son propre LOD — et l'on entendra une vague
déferler avant de la voir.

**Décision : l'audio est un consommateur de première classe de `EvalWater` et du bus d'événements
W, au même titre que le rendu et que la flottabilité.**

---

## 2. Trois lits et un bus

| Source | Pilotée par | Nature |
|---|---|---|
| **Lit d'état de mer** | `Hs`, `Tp`, `U10` de `HydroSample` | ambiance non localisée, fondu par *paramètre*, pas par position |
| **Lit d'écume** | intégrale du champ `F` (ADR-014) autour de l'auditeur | souffle du blanc, dosé par la quantité d'écume réellement présente |
| **Lit de rivage** | ligne de déferlement dérivée de W + bathymétrie | émetteur spatial le long d'une polyligne, intensité = énergie dissipée |
| **Bus d'événements** | `WaveEvent` (ADR-009) | impacts, déferlements, claques de coque, vannes et fuites de V |

Le lit d'écume est le gain principal : la question « où l'eau est-elle blanche » est déjà résolue
par un champ que le rendu calcule de toute façon. L'audio n'a pas à la reposer.

Le lit de rivage exige que le système d'eau **publie la polyligne de déferlement** — dérivable hors
ligne de la bathymétrie et de l'état de mer (ADR-005 §4.1). À produire dans le même passage
d'outillage que le précalcul côtier.

---

## 3. Délai de propagation — détail à coût nul, absence audible

Le son parcourt 343 m/s dans l'air. Un événement déclenché à 500 m doit être entendu **1,46 s plus
tard**.

| Distance | Retard |
|---|---|
| 100 m | 0,29 s |
| 500 m | 1,46 s |
| 1 km | 2,92 s |
| 3 km | 8,7 s |

Une explosion lointaine se voit puis s'entend. Le bus d'événements portant déjà un horodatage
`T_sim`, appliquer le retard est une soustraction. Ne pas l'appliquer produit un décalage
audiovisuel que les testeurs signalent sans savoir le nommer.

---

## 4. Sous l'eau : ce n'est pas un filtre passe-bas

Trois faits physiques dictent le mixage immergé, et aucun n'est une question de goût.

### 4.1 L'interface air-eau est un mur acoustique

Le rapport d'impédances est de 3 600 environ (`Z_eau ≈ 1,48·10⁶` contre `Z_air ≈ 415 rayl`). Le
coefficient de transmission en énergie vaut :

```
T = 4·Z₁·Z₂ / (Z₁+Z₂)² ≈ 1,1·10⁻³        soit  −29,5 dB
```

**Moins de 0,2 % de l'énergie sonore traverse la surface.** Un son aérien n'est pas assourdi sous
l'eau : il est quasiment absent. Le traitement correct n'est pas un passe-bas sur le mixage aérien,
c'est une coupure quasi totale et un **lit sous-marin distinct**, avec ses propres sources.

### 4.2 La localisation s'effondre

La célérité passe à 1 482 m/s, soit 4,3 fois celle de l'air. Les écarts interauraux de temps sont
divisés d'autant et tombent sous le seuil de discrimination humain : **on ne localise pas un son
sous l'eau**. Un mixage sous-marin fortement spatialisé est physiquement faux ; élargir et
dé-localiser est le rendu juste.

### 4.3 L'eau aérée est acoustiquement opaque

Un rideau de bulles diffuse et absorbe massivement — c'est le principe des rideaux de bulles
anti-bruit et la raison pour laquelle un sillage masque un sous-marin. Le champ `A` (ADR-014 §5)
doit donc alimenter un terme d'occlusion sous-marine.

Conséquence gameplay directe : **un sillage est une couverture acoustique**. Détection, discrétion
et sonar deviennent physiquement fondés sans système dédié.

---

## 5. Remplissage d'un contenant

La hauteur du son de remplissage monte parce que le volume d'air résiduel diminue :

```
f = (c / 2π) · √( A / (V_air · L_eff) )
```

`V_air` vient gratuitement de `shape_lut` (ADR-010 §2), qui donne déjà volume et hauteur. Un
bidon, un réservoir, une cale qui s'inonde produisent leur montée de hauteur sans échantillon
dédié — l'un des sons les plus reconnaissables, presque jamais généré proprement.

---

## 6. Ce que l'audio demande d'ajouter à `WaveEvent`

Trois champs, ≈4 octets, à intégrer avant de figer le format :

| Champ | Usage |
|---|---|
| `material_id` | coque métal, bois, roche, sable, chair |
| `displaced_ml` | volume déplacé — pilote la taille perçue plus fidèlement que l'énergie |
| `above_surface` | l'événement est-il né au-dessus ou au-dessous de la surface |

---

## 7. LOD

Même logique qu'ADR-014 : **champ d'abord, sources ensuite**. Au-delà d'une distance, les
émetteurs individuels sont remplacés par le lit d'écume et le lit de rivage. Le nombre d'émetteurs
d'eau simultanés est plafonné par profil, comme les domaines.

---

## 8. Ce qui reste ouvert

1. Validation de l'ensemble avec l'équipe audio — cet ADR est une proposition d'interface, pas une
   conception sonore.
   → **S11** : **ce point était inexécutable jusqu'en S09** : l'écart E04 de la revue S08 a montré qu'aucune
   de ces interfaces n'avait de signature écrite, donc rien à soumettre. `SPEC-006` §3 (bus
   d'événements, `WaveEvent` complet) et §4 (agrégats d'écume et d'aération) les portent désormais.
   Ce qu'on apporte à la réunion existe. **Urgence de format** : les trois champs qu'ADR-016 §6
   demande sont sur une structure **répliquée** — à arrêter avant que le réseau ne fige `WaveEvent`,
   sans quoi les ajouter coûtera une migration de protocole.
2. ~~Format de la polyligne de déferlement publiée.~~ **Clos.**
   → **S11** : **SPEC-006 §6** : `BreakerVertex` et `BreakerLineView`, avec le flux dissipé en **kW/m** — et
   non en W/m, qui saturerait un `half` dès `Hs = 4 m`.
3. Faut-il un lit distinct pour la pluie sur l'eau ? Probablement oui : la signature est très
   différente de la pluie sur le sol, et elle est pilotée par les mêmes données.
4. ~~Coût de l'occlusion par aération : intégrale le long du segment, ou tabulation grossière ?~~
   **Tranché.**
   → **S11** : **tabulation azimutale en 16 secteurs** (SPEC-006 §4.3). Motif que ce point ne pouvait pas
   connaître : l'intégrale par segment est un chemin *tiré*, dont le coût croît avec le nombre de
   sources. Reliquat — une ou deux bandes radiales — porté par SPEC-006 §9.2.
