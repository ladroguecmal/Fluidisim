# Prolongement du fond au-dessus du plan moyen — S254

Contrat : [ADR-154](../adr/ADR-154-prolongement-borne-du-fond.md). Répond à A286. Suite de
[SURFACE-COUPLEE-S253](SURFACE-COUPLEE-S253.md), dont l'oracle et les tolérances sont repris sans
changement.

## 1. Protocole, écrit avant construction

### 1.1 Oracle prolongé

Onde stationnaire de S253 (`L = h = 2 m`, `k = π/L`, fond d'ordre un de profondeur finie).
Sous le plan moyen, le fond est inchangé au bit. Au-dessus, avec `T = tanh kh`, `q = agk/ω` et les
facteurs de temps de S253, la règle d'ADR-154 donne :

```text
U = q sin kx sin ωt                      W = −q (T + kz) cos kx sin ωt
P = ρga (1 + kTz) cos kx cos ωt
```

Dérivées de ce même champ ; laplacien `−k²·(U, W)`.

**Contrôles avant usage**, sur une grille de points au-dessus du plan moyen et plusieurs instants :

1. **continuité** : `z → 0⁺` rend `U`, `W`, `P` et `du_dt` à l'arrondi f32 près du fond en `z = 0` ;
2. **incompressibilité** : `∂xU + ∂zW` nul à l'arrondi ;
3. **dérivées** : les gradients, `du_dt` et le laplacien publiés s'accordent aux différences finies
   centrées du champ publié en x, z et t, sous la borne de troncature du pas choisi ;
4. **cinématique linéaire** : `ζ_t = W(0)` inchangé ;
5. **résidu** : `S` publié égal à `du_dt + (U·∇)U + ∇P/ρ` recalculé en f64, et d'ordre deux
   (`S/a²` borné quand `a` passe de 5 à 10 cm).

### 1.2 Réception contre HOS, critères de S253 inchangés

Banc `delta_mobile`, pas couplé mobile, fond **prolongé** à la place du fond analytique.
Une période, pas de 1 ms, `a` = 5 et 10 cm, `nx` = 32, 64 et 128 :

- **profil** `max|ζ − η_HOS|/a` ≤ **2 %** à 128 colonnes, décroissant de 32 à 128 ;
- **harmonique `2k`** : écart de `b₂` ≤ **20 %** de `max|b₂,HOS|` à 128 colonnes, décroissant.

Les chiffres de S253 (fond analytique) sont publiés en regard. Ce ne sont pas des critères, mais
l'écart entre les deux fonds dit ce que la règle coûte. Un critère manqué est publié tel quel,
sans tolérance déplacée. Si le banc refuse, on s'arrête au diagnostic.

### 1.3 Fournisseur B de production

`Background::differential_local_extended` et ses variantes :

1. `z ≤ 0` : **identique au bit** à `differential_local`, tous champs, sur une grille de points
   et d'instants ;
2. `z > 0` : contrôles 1 à 3 du §1.1 sur un fond à plusieurs composantes et directions non
   alignées ; `div = Ak cosθ(|d|² − 1)` à l'arrondi ;
3. **accord avec l'oracle** : un mode de B, construit pour l'onde progressive de même `k` et
   `ω`, rend au-dessus du plan moyen les champs de la règle en profondeur infinie (`T = 1`),
   calculés en f64, à l'arrondi f32 ;
4. **refus** : point non admis, paramètres ou résultats non finis ; lot atomique, sortie
   intacte sur refus ; aucune allocation.

**Consommation** : un essai remplit `BackgroundFaces` par le fournisseur prolongé, faces au-dessus
du plan moyen comprises, et enchaîne des pas couplés mobiles depuis le repos, sans refus. C'est un
essai d'**intégration**, pas une réception de précision sous B réel. Celle-ci attend les frontières
du total et les bords ouverts.

### 1.4 Arrêt

Oracle prolongé contrôlé, banc 32/64/128 × 5/10 cm mesuré et publié, fournisseur B construit et
éprouvé, essai d'intégration vert. Couches W au-dessus du plan moyen hors lot.
