# Queue d'équilibre en f⁻⁴ et vagues pointues de Lagrange — S260

Contrat : [ADR-157](../adr/ADR-157-queue-d-equilibre-et-vagues-pointues.md). Origine : verdict R3,
[REVUE-VISUELLE](REVUE-VISUELLE.md) §10, où le modèle choisi a été calculé avant construction.

## 1. Protocole, écrit avant construction

### 1.1 Cœur

1. **Densité** : la variance de la queue d'équilibre égale `(Hs²/16)·q(4)·4⁴·(4⁻³ − 32⁻³)/3 /
   ∫_{0,5}^{4} q`, calculée en f64, à 1 % près.
2. **Rugosité** : `mss` bande + queue égale la `mss` continue de la même loi, à 2 % près. La
   valeur de l'instrument S260 (queue `a·√(x/4)`, 0,0483 avec la houle) est citée en regard.
3. **Continuité à 4 fp** : la densité par `ln f` des deux cellules voisines suit le rapport continu,
   à 5 % près.
4. **Refus et empreinte** : mêmes refus que `bake_tail_directional`, et empreinte distincte. Les
   essais d'ADR-155 et d'ADR-156 restent inchangés.

### 1.2 Hôte

5. **Déplacement** : aux sondes, `D_B` du GPU égale sa référence CPU f64, à 3 mm près.
6. **Normale** : aux sondes et pour plusieurs empreintes, la pente eulérienne du GPU
   `J⁻ᵀ·∇η` (bande + queue) égale sa référence CPU f64 à 5·10⁻⁴ près. Aucun repli.
7. **Scènes existantes au bit** : R2 (défaut) et R3 (`--houle`) rejoués avec leurs empreintes ;
   `--b-verify` et `--tail-verify` identiques.
8. **Écart au jeu** : écart vertical maximal, aux sondes, entre la surface rendue et la requête
   eulérienne linéaire, publié. Écart `D·∇W` des couches W, publié.
9. **Coût** : GPU eau, `--houle` contre `--houle --vagues`, 1280×720, deux poses, secteur.
10. **Rendus R4** aux poses de R1–R3, avec leurs empreintes, envoyés à l'utilisateur.

Un critère manqué est publié tel quel.
