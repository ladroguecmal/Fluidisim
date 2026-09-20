# Le premier bilan de conservation d'un domaine δ — S310

2026-09-20. **Lot 1 d'[ADR-178](../adr/ADR-178-strategie-en-trois-systemes-physiques.md) D7**,
ouvert par l'angle mort **A302** : le couplage δ ↔ B/W existait depuis S250, tournait en 3D depuis
S302, et **rien n'avait jamais compté ce qui y entrait ni ce qui en sortait**. La revue R11 avait
déclaré le raccord d'un domaine « invisible » — un jugement de l'œil, là où la stratégie demande
« des échanges cohérents de masse, de quantité de mouvement et d'énergie ».

Machine de référence ([ADR-174](../adr/ADR-174-arbitrages-du-2026-09-19.md) D1). Aucune tolérance
n'est décrétée ici : §6 en **propose** trois, à trancher par l'utilisateur.

---

## 1. Pourquoi le bilan de masse est exact, et pas seulement précis

Le pas couplé ne déplace `η` qu'à deux endroits : `transport_coupled3`, qui applique la divergence
des flux de colonne, et `relax_coupled3`, l'éponge. Le transport s'écrit, pour la colonne `(i, j)` :

```
η[i,j] += −(dt/dx) · ( Fx[i+1,j] − Fx[i,j] + Bx[i+1,j] − Bx[i,j]
                     + Fy[i,j+1] − Fy[i,j] + By[i,j+1] − By[i,j] )
```

`F` est le flux de la **perturbation**, `B` celui de la **bande** de fond B/W. Sommée sur le
domaine, chaque ligne télescope — `Σᵢ (F[i+1] − F[i]) = F[nx] − F[0]` —, **et il ne reste que les
faces de bord**. Ce n'est pas une approximation du bilan : c'est le bilan. L'écart que la mesure
trouve **est** le plancher numérique du schéma, ce qui permet de le publier comme tel au lieu de
l'absorber dans une tolérance.

**Trois précisions ont changé le chiffre**, et deux d'entre elles ont été trouvées en mesurant,
pas en réfléchissant :

1. **La hauteur qui compte est `η − eta_roundoff`**, la hauteur compensée — celle que la pression
   lit déjà. Un volume sommé sur `η` seul manquerait exactement ce que la somme compensée de S233
   existe pour retenir.
2. **Le prélèvement de l'éponge ne vaut pas son `increment`.** Compter l'incrément sur-compte de
   `eta_roundoff` à chaque colonne et à chaque pas : **0,86 %** du volume retiré, soit **quatre
   ordres de grandeur au-dessus du plancher**. Le défaut a été trouvé par l'essai, pas par la
   relecture. Le transport, lui, était juste d'emblée — son incrément retranche déjà le reste que
   la hauteur compensée rajoute.
3. **Le volume se somme en `f64`** sur des champs `f32` : un instrument qui mesure un plancher ne
   doit pas en fabriquer un.

**Et une lecture du code que le bilan vérifie à chaque pas** : aux faces extérieures, le flux de
*perturbation* est **nul par construction** — la garde `a > 0 && a < n` du transport l'y laisse à
zéro, et seule la bande y transporte. C'est « δ ne ressort pas vers W » écrit une seconde fois,
dans une boucle. Le compteur le publie quand même : une valeur non nulle voudrait dire que cette
lecture est fausse, et le dire coûte une addition.

---

## 2. La cuve fermée : ce que le schéma perd quand rien n'entre ni ne sort

`--delta3d-cuve-longue`, cuve d'[ADR-175](../adr/ADR-175-architecture-d-execution-de-delta-en-3d.md)
§4.1 : 8 × 4 m, profondeur 4 m, mode oblique (1, 1), **murs sur les quatre côtés, aucun fond,
aucune éponge**. 32 × 16 × 18 mailles, 5 000 pas de 1 ms, 5 s simulées, 2,33 période.

| grandeur | mesure |
|---|---|
| volume passé par les **murs** | **0 exactement**, à chaque pas |
| dérive du volume de perturbation | **2,37·10⁻¹⁰ m³** sur 32 m² |
| en hauteur moyenne | **7,40·10⁻¹² m** |
| rapportée à l'amplitude du mode | **1,48·10⁻¹⁰** |
| énergie, début → fin | 100,55 J → **100,08 J** |
| **dissipation numérique** | **4,67·10⁻³** sur 5 s, soit **0,0935 % par seconde** |

**Ce que cela tranche sur A298.** Cet angle mort mesure une dérive de hauteur **carte / référence**
d'environ 1,2·10⁻⁷ m par seconde sur cette même cuve ([CUVE-GPU-S305](CUVE-GPU-S305.md) §6). La
**référence** conserve son volume à 7,4·10⁻¹² m sur cinq secondes — quatre à cinq ordres de
grandeur en dessous. **La dérive d'A298 n'est donc pas une fuite de volume du schéma** : elle est
propre au chemin de la carte, ce qu'A298 soupçonnait sans pouvoir l'isoler.

**Et un nombre que le dépôt n'avait pas** : la dissipation numérique borne la durée de vie utile
d'un domaine. À 0,0935 % par seconde, un domaine perd environ **un cinquième de pour cent d'énergie
par période** sur ce cas — c'est ce qui décide si une onde piégée dans un domaine long s'éteint
avant d'en sortir.

---

## 3. La scène couplée : ce que l'éponge fait réellement

`delta3d_preview --spectral --resolu` : 32 × 24 × 36 à 25 cm, **fond spectral réel** à 64
composantes, impulsion de 18 cm, **éponge de 1 m à 2 s⁻¹** sur les quatre bords, 6 s. C'est la
scène la plus proche de celle de [S302](SCENE-DELTA3D-S302.md) qui tourne dans la **référence
CPU** ; celle de S302 vit sur la carte, et le compteur n'y est pas encore.

| grandeur | mesure sur 6 s |
|---|---|
| **résidu du bilan** | **1,28·10⁻¹⁰ m³** pour une échelle de 0,25 m³ — **5·10⁻¹⁰ relatif** |
| échange de l'éponge, en valeur absolue | **0,1546 m³**, soit **0,0258 m³/s** |
| rapporté au contenu perturbatif du domaine | **10,2 % par seconde** |
| éponge, en net signé | **−7,80·10⁻³ m³** |
| bande B/W, en net | **−0,1096 m³** |

*L'échelle n'est pas le volume signé : l'impulsion est un chapeau mexicain, d'intégrale nulle en
continu. C'est `Σ|h − repos|·dx²`, la quantité de perturbation **présente**, qui dit si un
prélèvement est grand ou petit.*

**Ce que ces nombres disent, et c'est A302 transformée.** L'éponge ne « laisse pas sortir » la
perturbation : elle l'**efface**, à raison d'un dixième du contenu du domaine par seconde, et rien
de cela n'entre dans W. Le raccord que R11 a jugé invisible l'est parce que l'éponge est **douce**,
pas parce qu'elle **conserve**. Distinguer les deux demandait un compteur ; il existe.

---

## 4. Ce qui ne se ferme pas, et pourquoi

Le bilan de masse est exact parce que le transport ne produit que des flux de colonne. **L'énergie
et la quantité de mouvement n'ont pas cette chance** : leur bilan demande le **travail de la
pression aux faces de bord** et le **flux advectif de quantité de mouvement**, que le pas ne
calcule nulle part sous une forme récupérable. Les fabriquer après coup donnerait un nombre qui
ressemble à un bilan sans en être un — l'erreur exacte qu'A302 reproche au raccord « invisible ».

Le dépôt publie donc, pour ces deux-là, des **états** — `perturbation_energy`,
`perturbation_momentum` —, à lire par différence, et le dit dans sa documentation. Sur un domaine
**fermé** la différence a un sens unique (§2) ; sur un domaine couplé, elle n'en a pas encore.

**Ce qu'il faudrait pour les fermer**, nommé et non improvisé :

- **Énergie** : le travail `∫ p u·n dA` sur les quatre faces de bord, évalué avec la même
  quadrature que la projection, plus la dissipation explicite de l'éponge sur les vitesses
  (`predict_coupled3` l'applique aussi à `u`, pas seulement à `η`).
- **Quantité de mouvement** : le flux advectif `ρ u (u·n)` aux mêmes faces, plus la force de
  pression de bord, plus la part que les bandes B/W injectent.

Les deux sont des lots à part entière. Ils ne bloquent pas le lot 2 : **la masse suffit à juger un
retour δ → W**, puisque c'est un volume qui manque aujourd'hui.

---

## 5. Ce que les essais garantissent

Quatre essais neufs dans `tests_delta3d.rs`, plus un sur les formules d'énergie — **544 essais du
cœur et du harnais passent, 0 échec**, et les réceptions au bit sont inchangées.

| essai | ce qu'il éprouve | mesure |
|---|---|---|
| `bilan_de_masse_se_ferme_au_plancher_s310` | le bilan se ferme sous un fond traversant | résidu **9,85·10⁻⁸** relatif, ≈ 1,6 ulp de f32 par pas |
| `cuve_fermee_garde_son_volume_s310` | bande et éponge exactement nulles, volume constant | **1,13·10⁻¹¹ m** sur 200 pas |
| `l_eponge_est_un_puits_et_il_se_compte_s310` | **tout** ce qui manque est passé par l'éponge | **9,8·10⁻⁷** relatif |
| `un_refus_ne_publie_pas_de_bilan_s310` | un pas refusé ne laisse pas le compte rendu d'un pas qui n'a pas eu lieu | égalité au bit |
| `energie_et_quantite_de_mouvement_…_s310` | nul au repos, potentielle quadratique, quantité de mouvement au facteur des murs | exact à 10⁻⁶ |

Le bilan est **atomique** : il s'écrit après le dernier contrôle du pas, pas avant.

---

## 6. Trois tolérances **proposées**, à trancher par l'utilisateur

Le dépôt n'a aucune tolérance de conservation ; ADR-178 §3 dit qu'elle se propose avec le premier
bilan publié. Voici la proposition, chacune adossée à une mesure de cette session.

| | critère proposé | mesuré aujourd'hui | marge |
|---|---|---|---|
| **T1 — l'instrument** | résidu du bilan ≤ **10⁻⁶** de l'échelle du pas | 9,85·10⁻⁸ (essai), 5·10⁻¹⁰ (scène) | ×10 à ×2 000 |
| **T2 — domaine fermé** | dérive de volume ≤ **10⁻⁶** de l'amplitude de la perturbation sur 10 s | 1,48·10⁻¹⁰ sur 5 s | ×10⁴ |
| **T3 — couplage** *(critère du lot 2)* | ce que δ absorbe **reparaît dans W à 5 % près**, et la réflexion artificielle au bord reste **sous 1 % en énergie** | absorbé : 100 % effacé. Réflexion : 0,14–0,16 % mesurés en 2D ([S269](REFLEXION-PAQUET-S269.md)) | T3 n'est pas tenu : c'est le lot 2 |

**Pourquoi ces valeurs.** T1 et T2 sont des seuils d'**instrument** : ils disent que le compteur
ferme et que le schéma ne fuit pas, et ils sont largement tenus — les fixer plus bas ne mesurerait
plus que l'arrondi de la machine, ce qu'A98 interdit de revendiquer sur une seule cible. **T3 est
le seul critère de fond**, et il est délibérément modeste : 5 % sur le retour, parce qu'un premier
retour δ → W ne sera pas exact et qu'un seuil inatteignable ne sert à rien ; 1 % sur la réflexion,
parce que S269 a déjà mesuré 0,14–0,16 % en 2D et que le passage à trois dimensions ne devrait pas
coûter un ordre de grandeur.

**Ce que je ne propose pas** : une tolérance sur l'énergie ou la quantité de mouvement. Tant que
leur bilan n'est pas fermé (§4), un seuil porterait sur un nombre qui n'est pas un échange.

---

## 7. Limites

Les mesures portent sur la **référence CPU**, pas sur la production GPU : le compteur n'existe pas
encore sur la carte, et ADR-175 fait de la référence le juge, pas l'inverse. La scène du §3 est une
scène **voisine** de celle de S302, pas la même — mêmes maille, éponge et nature de fond, domaine
plus petit. Une seule machine ; aucune seconde cible (A98). Aucun banc nouveau n'est déclaré reçu :
ce document publie un instrument et ses premières lectures.
