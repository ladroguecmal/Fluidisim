# ADR-004 — État minimal de l'eau simplifiée

- **Statut** : proposée
- **Session** : S01
- **Résout** : `zones_ouvertes §3` (ouvert — critique) et `§16` partiellement
- **Dépend de** : ADR-001, ADR-002, ADR-003

---

## 1. Reformulation du problème

Le document source demande : *quelles variables stocker par zone pour pouvoir initialiser une
simulation crédible sans rendre la fausse eau coûteuse ?* — et liste hauteur, phase, amplitude,
direction, vitesse, courant, énergie, turbulence.

La question contient son propre piège. Stocker *hauteur* et *phase* par zone, c'est stocker un
échantillonnage d'un champ qui est déjà une fonction fermée. C'est payer de la mémoire pour une
information dérivable, et créer un problème de cohérence entre zones voisines qui n'existait pas.

**Réponse retenue : la fausse eau ne stocke rien par zone.** Son état est un descripteur
régional interpolé, et le champ est reconstruit à la demande.

---

## 2. Ce qui est réellement stocké

### 2.1 Table de composantes globale (immuable, partagée par toute la planète)

```
struct WaveComponent {          // 20 octets, N = 128 par défaut
    k_dir      : vec2<f16>      // direction unitaire
    k_mag      : f32            // nombre d'onde
    omega      : f32            // = sqrt(g·k)  (eau profonde) — précalculé
    phase0     : f32            // phase initiale, PRNG(seed_global, i)
    band       : u8             // index de bande spectrale, pour la pondération régionale
    _pad       : u8[3]
}
```

Total : **2,5 Ko, chargés une fois, jamais streamés, jamais répliqués** (dérivables d'une graine).

Le point essentiel : les composantes — donc les directions, les nombres d'onde et les phases —
sont **les mêmes partout sur la planète**. Seules leurs *amplitudes* varient spatialement.

### 2.2 Grille de paramètres hydrographiques

Une grille 2D lâche sur la planète (pas ≈ 10 km), streamée avec le terrain :

```
struct HydroSample {                 // 40 octets par nœud
    Hs          : f16       // hauteur significative                    [m]
    Tp          : f16       // période de pic                           [s]
    theta_mean  : f16       // direction moyenne                        [rad]
    spread      : f16       // étalement directionnel
    gamma       : f16       // pic JONSWAP
    wind_uv     : f16[2]    // vent 10 m — pilote écume, embruns, capillaires
    depth       : f16       // profondeur bathymétrique (réfraction, levée)
    fetch       : f16       // fetch effectif — limite Hs près des côtes
    tide_amp    : f16       // amplitude locale de marée
    tide_phase  : f16       // déphasage local de marée
    current_uv  : f16[2]    // courant de surface — cf. ADR-011
    liquid_id   : u8        // eau, méthane, acide… cf. angle mort A17
    flags       : u8
    _pad        : u8[6]
}
```

Une région de 32 km avec un pas de 10 km tient dans **quelques kilo-octets**.

### 2.3 Champ reconstruit

```
B(x, t) = Σ_i  a_i(x) · G( k_i · x_tangent + phase_i(t) )
avec  a_i(x) = A_band[ band_i ]( Hs(x), Tp(x), γ(x) ) · D( θ_i − θ_mean(x), spread(x) )
```

`Hs(x)`, `Tp(x)`, `θ_mean(x)` sont obtenus par interpolation bilinéaire (ou bicubique) sur la
grille `HydroSample`.

---

## 3. Pourquoi cette construction est sans coutures — et pourquoi l'alternative évidente échoue

L'approche naturelle consiste à générer un champ de houle par région, puis à fondre les champs
voisins dans une bande de recouvrement. **Elle est mathématiquement fausse.**

Deux champs gaussiens indépendants de même hauteur significative Hs, mélangés à parts égales, ont
une variance résultante `σ² = (Hs²/16 + Hs²/16)/2·…` — concrètement, la hauteur significative du
mélange vaut `Hs/√2 ≈ 0,71·Hs`. **Il apparaît une bande de mer calme de 29 % le long de chaque
frontière de région.** Sur un océan pavé, on obtient une grille de calme visible depuis l'altitude.

La construction retenue supprime le problème à la racine : les composantes étant globales, il n'y
a **aucun raccord de champ**. On interpole des amplitudes ; le champ reste continu, dérivable, et
son énergie locale est exactement celle demandée par `Hs(x)`.

**Règle générale, à retenir au-delà de ce cas** :
> On n'interpole jamais deux réalisations d'un champ stochastique. On interpole les paramètres qui
> les engendrent.

---

## 4. Ce que B fournit aux consommateurs

Une seule fonction, appelée partout :

```
struct WaterSample {
    height       : f32       // élévation le long du radial local
    displacement : vec3<f32> // déplacement de Gerstner (crêtes)
    velocity     : vec3<f32> // vitesse orbitale de surface
    normal       : vec3<f32>
    d_height_dt  : f32       // pour la flottabilité amortie (ADR-008)
    steepness    : f32       // pour l'écume et le seuil de déferlement
}
WaterSample EvalWater(vec3 x_local, FrameRef f, u64 T_sim, LayerMask m);
```

`LayerMask` sélectionne B seul, B+W, ou B+W+δ. Un consommateur gameplay demande **B+W** ; le rendu
demande **B+W+δ** ; un test de collision lointain demande **B** seul.

Coût mesuré attendu : ≈128 sin/cos par échantillon en pire cas. Deux optimisations obligatoires :

- **LOD spectral** : on ne somme que les composantes dont la longueur d'onde dépasse un seuil lié
  à la distance de l'observateur ou à la taille de l'objet échantillonné. Une bouée n'a pas besoin
  des composantes de 500 m ; un porte-conteneurs n'a pas besoin de celles de 30 cm. Le nombre
  effectif de composantes tombe typiquement à 16–32.
- **Cache de tuile** : pour le rendu, B est évalué une fois par tuile dans une texture de
  déplacement, pas par pixel.

---

## 5. Traitement par type de masse d'eau

Le document source demandait un état minimal *variable selon le type de zone*. La grille
`HydroSample` absorbe le cas général ; les cas particuliers se déclarent par `flags` :

| Type | Ce qui change | Ce qui est ajouté |
|---|---|---|
| Haute mer | rien — cas nominal | — |
| Côte / plage | `depth` et `fetch` dominent ; réfraction et levée calculées dans W | zone de déferlement dérivée (ADR-005 §4) |
| Lac | `Hs` borné par le fetch, `Tp` court, marée nulle | niveau moyen = nœud d'un volume V (ADR-010) |
| Rivière / canal | B ≈ plan + ondulation faible ; le champ dominant est le **courant**, pas la houle | polyligne de débit (ADR-011 §4) |
| Volume fini | B non applicable | régime substitutif intégral (ADR-001 §3.3) |

Une rivière n'est donc *pas* un océan à faible Hs : c'est une entité de la couche courant, avec une
surface reconstruite à partir du débit et de la géométrie du lit. Cela évite d'essayer de faire
tenir un écoulement dirigé dans un modèle spectral, ce qui n'aurait pas fonctionné.

---

## 6. Ce qui reste ouvert

1. Nombre de composantes N et découpage en bandes → benchmark B1.
2. Gerstner sommé vs tuile FFT locale : la FFT est moins chère à grand N mais réintroduit un
   pavage et un raccord. Piste retenue : FFT pour le **détail haute fréquence non
   gameplay-pertinent** (capillaires), Gerstner sommé global pour les **composantes longues**
   qui portent la flottabilité. À valider.
3. Pas de la grille `HydroSample` près des côtes — 10 km est probablement trop lâche là où la
   bathymétrie varie ; prévoir un raffinement côtier.
4. Modèle de marée : global harmonique (quelques constituantes) vs table précalculée.
