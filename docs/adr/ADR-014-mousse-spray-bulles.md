# ADR-014 — Mousse, écume, spray et bulles

- **Statut** : proposée
- **Session** : S02
- **Résout** : `zones_ouvertes §24`, `architecture_globale §13`
- **Dépend de** : ADR-001, ADR-004, ADR-005, ADR-012

---

## 1. Le piège à éviter

L'implémentation naturelle est un système de particules alimenté par le solveur. Elle échoue pour
trois raisons indépendantes, et l'échec n'apparaît qu'en fin de production.

1. **B et W n'ont pas de particules** et couvrent 99,9 % de la surface. Un système alimenté par δ
   ne produit d'écume que dans les quelques mètres carrés simulés — c'est-à-dire nulle part.
2. **L'écume persiste et dérive.** L'écume de déferlement vit des dizaines de secondes et se
   déplace avec la surface. Un système de particules qui la porte doit maintenir des centaines de
   milliers de particules à faible valeur unitaire.
3. **L'écume est le principal signal de lecture de la mer.** Sans elle, une houle correcte paraît
   plastique. C'est le poste où un rendu d'eau se juge, et le dernier qu'on budgète.

## 2. Décision : un champ, puis des particules

**Représentation primaire : un champ de moussage `F(x,t)` ∈ [0,1]**, texture 2D ancrée au monde,
advectée par la vitesse de surface, alimentée par les trois couches, décroissante.

**Représentation secondaire : des particules**, réservées au proche-caméra, éphémères, jamais
persistantes, jamais gameplay.

```
F ← advection(F, u_surface)                       // semi-lagrangien, une passe GPU
F ← F + Σ sources(B) + Σ sources(W) + Σ sources(δ)
F ← décroissance à deux canaux                     // cf. §2.2
```

### 2.1 Le détail qui fait la différence : advecter avec la vitesse orbitale

L'écume advectée par le seul courant moyen reste uniforme et lit comme une texture. Advectée par la
**vitesse de surface complète** — orbitale de B + W + courant — elle s'accumule dans les zones de
convergence et forme spontanément les traînées alignées au vent qu'on observe en mer.

Le réalisme est ici gratuit : la vitesse de surface est déjà calculée par `EvalWater` pour la
flottabilité. Il suffit de ne pas la jeter.

### 2.2 Deux canaux de décroissance

L'écume réelle a deux durées de vie très différentes, et les confondre donne soit une mer qui
« clignote », soit une mer barbouillée en permanence.

| Canal | Demi-vie | Origine | Rendu |
|---|---|---|---|
| **actif** | ≈3 s | bulles d'air fraîches, blanches, opaques | fort albédo, relief |
| **résiduel** | ≈30 s | film stabilisé par les tensioactifs | traînées ténues, transparentes |

`F` est donc un vecteur à deux composantes. Le canal actif alimente le résiduel en décroissant.
C'est ce qui produit les longues traînées persistantes derrière un déferlement, effet
immédiatement reconnaissable et impossible à obtenir avec une décroissance unique.

### 2.3 Ancrage et persistance

- **Écume transitoire** : cascades de textures ancrées au monde autour de chaque observateur
  (comme des cascades d'ombres). Ce qui sort de la cascade est perdu — sans conséquence, c'est
  hors de vue.
- **Écume permanente** (ligne de déferlement d'une plage, remous d'un rocher) : **re-dérivée**
  du modèle de déferlement de W, jamais stockée. Même mécanisme qu'ADR-001 : ce qui est
  déterministe n'a pas à être mémorisé.

Conséquence réseau : l'écume produite par W est identique chez tous les clients sans être
répliquée. Un sillage utilisé pour pister un navire est donc **cohérent entre joueurs** ; seule
l'écume issue de δ diffère, et elle est locale et minuscule.

---

## 3. Sources — les métriques demandées par `§24`

### 3.1 Depuis B — moutons de haute mer, coût nul

Deux critères, cumulés :

- **Cambrure locale** : `H/λ → 1/7` (limite de Stokes). En pratique on déclenche à partir de
  `0,6 × 1/7` avec une montée lisse.
- **Accélération verticale de la crête** : la crête déferle quand son accélération descendante
  approche `g`. Seuil de départ **0,45 g** pour un déferlement glissant, `→ g` pour un
  déferlement plongeant. Peu coûteux : l'accélération est la dérivée seconde analytique de B.

**Taux de couverture attendu** — relation de Monahan, utilisable pour caler le rendu sans réglage
d'auteur :

```
couverture_moutons ≈ 3,84·10⁻⁶ · U10^3,41
```

| U10 | 5 m/s | 10 m/s | 15 m/s | 20 m/s | 25 m/s |
|---|---|---|---|---|---|
| couverture | 0,09 % | 1,0 % | 3,9 % | 10,4 % | 22 % |

Une mer force 7 est blanche à 4 %, pas à 40 %. C'est le genre de valeur qu'un artiste pousse
naturellement dix fois trop haut ; la donner d'entrée évite une passe d'ajustement tardive.

### 3.2 Depuis W — déferlement et sillage

Dépôt proportionnel au **flux d'énergie dissipé** par le modèle de déferlement :
`dF/dt ∝ P_dissipée / (ρ g)`. La zone de surf est ainsi blanche là où l'énergie se perd, et
seulement là. Pour le sillage, le dépôt suit la crête des vagues divergentes de Kelvin et la
zone de brisure d'étrave.

### 3.3 Depuis δ — proche-champ

Injection là où la divergence de la vitesse de surface et la courbure dépassent un seuil, plus
une injection directe aux cellules d'impact. Faible surface, forte intensité.

### 3.4 Depuis le vent — embruns

- Apparition des embruns arrachés aux crêtes : `U10 ≳ 8 m/s`.
- Écrêtage massif (*spume*) : `U10 ≳ 11 m/s`.
- Direction : vent, avec une composante balistique.

---

## 4. Quand une nappe devient des gouttes — le nombre de Weber

`architecture_globale §13.3` demande de distinguer ce qui reste visuel de ce qui devient physique.
Le critère est physique, pas artistique :

```
We = ρ · v² · d / σ            σ_eau = 0,072 N/m
```

Une nappe ou une goutte se fragmente au-delà de `We ≈ 12`.

| Épaisseur de nappe | v = 2 m/s | v = 5 m/s | v = 10 m/s |
|---|---|---|---|
| 0,5 mm | We = 28 | 174 | 694 |
| 2 mm | 111 | 694 | 2 780 |
| 10 mm | 556 | 3 472 | 13 900 |

Conclusion pratique : **toute nappe d'eau projetée plus vite que ≈1 m/s se fragmente**. Une gerbe
n'est jamais une feuille d'eau qui retombe intacte ; elle se pulvérise. Modéliser la fragmentation
comme un état normal, pas comme un cas particulier.

**Retour vers la physique** : une masse fragmentée ne réémet une perturbation mesurable que si
elle dépasse un seuil de volume. Proposition : `V > 5 L` déclenche un événement W de faible
énergie ; en dessous, dépôt de `F` et rien d'autre. À calibrer.

---

## 5. Bulles

### 5.1 Microbulles — champ d'aération `A(x,t)`

Champ 3D grossier (texel de 0,25–1 m), fraction volumique d'air, advecté par la vitesse et remontant
à la vitesse terminale.

| Diamètre | Vitesse de remontée | Temps pour remonter de 3 m |
|---|---|---|
| 0,1 mm | 5,5 mm/s (Stokes) | 9 min |
| 1 mm | 0,12–0,25 m/s | 12–25 s |
| 5 mm | ≈0,25 m/s | 12 s |

La traînée d'aération derrière un navire persiste donc une vingtaine de secondes — durée qui
détermine la longueur du sillage blanc, et qui n'a pas à être réglée à la main.

### 5.2 L'aération change la flottabilité — effet à exposer

Une fraction volumique d'air `α` réduit la masse volumique effective à `(1−α)·ρ`. Conséquences
directes et gratuites :

- un corps flottant dans une eau aérée à 10 % s'enfonce de 10 % de plus ;
- un nageur dans l'eau blanche d'une chute ou d'un rouleau **ne flotte plus** — mécanique de
  noyade physiquement fondée plutôt que scriptée ;
- un navire pris dans un rideau de bulles perd de la portance.

`A` est donc exposé au calcul de flottabilité comme multiplicateur de densité.

> **Correction (S05, écart R02).** Ce paragraphe qualifiait le procédé d'« exception contrôlée à
> l'invariant I-04 ». **Ce n'en est pas une**, et l'étiquette était dangereuse : une exception
> admise se cite, puis se généralise.
>
> Le champ se scinde à la source : `A = A_rep(B, W_rep, vent) + A_local(δ, W_local)`. `A_rep` est
> reproductible à partir de données répliquées, donc autoritaire au titre d'I-15 — sans déroger à
> quoi que ce soit. `A_local` n'agit que sur la pose de rendu, plafonné comme toute contribution de
> δ. Voir ADR-021 §5.

### 5.3 Grosses bulles

Objets discrets, comptés et plafonnés, avec volume, remontée, déformation et coalescence. Elles
relèvent du modèle de poche d'air (ADR-015 §3).

---

## 6. Budget et LOD

| Composante | Coût | Critère de LOD |
|---|---|---|
| Champ `F` | 1 à 2 passes GPU par cascade | toujours actif — c'est le socle |
| Champ `A` | 1 passe volumique grossière | actif si un observateur est immergé ou proche d'une source |
| Particules de spray | budget dédié, plafonné | surface écran + distance |
| Grosses bulles | quelques dizaines d'objets | présence d'un observateur |

Ordre de dégradation, cohérent avec ADR-012 §4 : les particules tombent avant le champ. Un rendu
sans particules mais avec un champ d'écume correct reste crédible ; l'inverse ne l'est pas.

---

## 7. Ce qui reste ouvert

1. Résolution et nombre de cascades de `F` → benchmark B9.
2. Demi-vies exactes des deux canaux, par type d'eau (mer / rivière / eau douce de lac —
   la mer mousse davantage, ses tensioactifs stabilisent l'écume).
3. Seuil de volume pour le retour goutte → événement W.
4. Représentation de `A` : texture volumique grossière vs colonnes 2,5 D. La seconde suffit
   probablement, sauf pour une cavité d'impact.
5. ~~Couplage `A` ↔ audio (une eau aérée est acoustiquement opaque) → ADR-016 §4.~~ **Clos.**
   → **S11** : traité par ADR-016 §4.3 et **spécifié** par SPEC-006 §4.3 : `A` est publié par auditeur en
   16 secteurs azimutaux — ceux d'ADR-005 §3 — et alimente le terme d'occlusion sous-marine.
