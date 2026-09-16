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

## 2. Ce qui a été construit

- **Oracle** (`examples/support/standing_background.rs`) : `sample_bounded` et `bounded_fields`,
  règle d'ADR-154 en profondeur finie ; le fond analytique reste inchangé sous le plan moyen.
- **Fournisseur B** (`background_differential.rs`) : `differential_local_extended`,
  `differential_extended` et `differential_batch_extended`. Sous le plan moyen, ils appellent le
  chemin d'ADR-113 ; au-dessus, ils appliquent la règle par composante (`m = 1 + kz`, `E = 1`).
  `differential_local` refuse toujours `z > 0`.
- **Banc** : `delta_mobile couple_cas borne <a> <nx>`, mode `couple_borne`, qui ne diffère du mode
  `couple` de S253 que par le fond prolongé.

## 3. Essais

| essai | résultat |
|---|---|
| oracle : continuité en `z = 0` (U, U_t, P, ∇P) | à 4 ulp du fond analytique |
| oracle : divergence au-dessus du plan moyen, 5 et 10 cm, 4 instants, 4 abscisses, 4 hauteurs | à l'arrondi |
| oracle : gradients, `du_dt`, laplacien contre différences finies du champ publié | sous 10⁻⁶ et 10⁻⁵ relatifs |
| oracle : `S` publié (f32) contre sa recomposition f64 | sous 10⁻⁵ relatif |
| oracle : ordre deux, `z = s·a`, rapport `S(10 cm)/4·S(5 cm)` | 1,000 à 1,115 (dérive de `T + kz`) |
| B : `z ≤ 0`, `−0,0` et `0` compris, 4 instants, 3 abscisses, 6 hauteurs | **identique au bit** à `differential_local` |
| B : refus (hors domaine, `z = 4096`, NaN, densité), lot atomique | refus, sortie intacte |
| B : au-dessus, deux composantes non alignées : divergence | ≤ 16 ulp de l'échelle des gradients |
| B : gradients, `du_dt`, laplacien contre différences finies ; accord avec la règle f64 | tenus |
| **intégration** : B de production (`Hs` 0,3 m, `Tp` 4 s, une composante), 50 pas couplés mobiles depuis le repos, 32 colonnes | **reçus** ; 590 faces mouillées au-dessus du plan moyen consommées |

Le premier jet de l'essai B contrôlait `du_dt` par une différence **avant** à `t = 0`. Sa
troncature, 0,025, dépassait la tolérance de 5·10⁻³ : l'essai échouait par son instrument. Il
contrôle désormais par différence centrée à `t ≥ 1 ms`, tolérance inchangée.

L'intégration montre où vit la perturbation : `u'` atteint 6,6·10⁻² m/s près des murs, contre
6,1·10⁻³ à l'intérieur, pour une vitesse du fond de 0,29 m/s. Les murs imposent `v·n = −U·n`, ce
qu'une onde progressive ne vérifie pas. C'est la frontière du total, hors de ce lot ; cet essai ne
dit rien de la précision sous B réel.

## 4. Réception contre HOS

`delta_mobile couple_cas borne <a> <nx>`, six processus parallèles, une période, pas de 1 ms,
secteur 99 %. La précision ne dépend pas de la concurrence. Le coût n'a pas été mesuré proprement :
les temps imprimés par le banc sont pris sous charge et ne sont pas publiés comme mesure.

| a | nx | profil prolongé | `b₂` prolongé | profil S253 analytique | `b₂` S253 analytique |
|---:|---:|---:|---:|---:|---:|
| 5 cm | 32 | 0,964 % | 3,47 % | 1,214 % | 1,62 % |
| 5 cm | 64 | 0,380 % | 1,19 % | 0,276 % | 0,54 % |
| 5 cm | **128** | **0,168 %** | **0,58 %** | 0,162 % | 0,34 % |
| 10 cm | 32 | 1,007 % | 3,05 % | 0,908 % | 1,51 % |
| 10 cm | 64 | 0,485 % | 1,43 % | 0,379 % | 0,69 % |
| 10 cm | **128** | **0,244 %** | **0,66 %** | 0,213 % | 0,53 % |

- **Critères tenus** aux deux amplitudes : profil ≤ 2 % et `b₂` ≤ 20 % à 128 colonnes, les deux
  décroissants de 32 à 128.
- **Le coût de la règle** : l'écart d'harmonique est environ deux fois celui du fond analytique
  à 32 et 64 colonnes, et 1,2 à 1,7 fois à 128. Le profil ne change presque pas. Cet écart est
  cohérent avec le pli de `∂zU` en `z = 0`, que δ porte (ADR-154 §3), mais il n'a pas été attribué
  par une mesure séparée.
- Volume perturbatif : dérive ≤ 7,5·10⁻⁹ m. Pas au plancher, tous reçus : 119 (5 cm / 128),
  27 (10 cm / 128) ; plafond d'itérations atteint 1 104 sur 4 000.

## 5. Limites

- **Couches W** au-dessus du plan moyen non prolongées. Impacts et pression gardent le refus
  `z > 0`, et leurs dérivées secondes à `z = 0` restent à écrire.
- **Précision sous B réel non reçue** : l'intégration traverse les composants, mais dans un bassin
  à murs où `W(fond) ≠ 0`. Il faut d'abord les frontières du total et les bords ouverts.
- **Étalement large bande** non éprouvé contre un oracle non linéaire à plusieurs modes. La règle
  borne l'amplification, mais sa précision sous une mer large bande n'est pas établie.
- Coût non mesuré, 2D x-z seulement, et une seule cible machine.

## Reproduction

```powershell
cargo test --release --offline --manifest-path code/Cargo.toml -p water-core --lib _s254 -- --nocapture
cargo run --release --offline --manifest-path code/Cargo.toml -p water-core --example delta_mobile -- couple_cas borne 0.05 128
```

Suite du dépôt après le lot : **463 réussis, 18 ignorés, 0 échec** (459 à S253, plus les quatre
essais S254). Le pas δ n'est pas modifié ; empreinte `delta_precision` de S253 non rejouée.
