# Surface mobile couplée B/W→δ — S253

Contrat : [ADR-152](../adr/ADR-152-surface-mobile-couplee.md). Suite de
[RACCORDEMENT-DELTA-S250](RACCORDEMENT-DELTA-S250.md) (volume, surface imposée) et de
[SURFACE-MOBILE-S237](SURFACE-MOBILE-S237.md) (surface mobile, sans fond).

## 1. Protocole, écrit avant construction

### 1.1 Oracle

Onde stationnaire de S237 : bassin à murs `L = 2 m`, profondeur `h = 2 m`, `k = π/L`,
`ω² = gk·tanh kh`, départ au repos avec `η = a·cos kx`. Référence **totale** : véhicule HOS
d'ordre `M = 3` de S193/S237 (Q = 16, K = 256, dt = 1 ms), inchangé.

Le **fond** est l'ordre un analytique de profondeur finie :
`ζ_fond = a·cos kx·cos ωt`, `P = ρga·C(z)·cos kx·cos ωt`, `U = (agk/ω)·C·sin kx·sin ωt`,
`W = −(agk/ω)·S·cos kx·sin ωt`, avec `C = cosh k(z+h)/cosh kh` et `S = sinh k(z+h)/cosh kh`,
dérivées exactes et laplacien nul. Il est prolongé **analytiquement** au-dessus du plan moyen,
ce qui le garde incompressible : le prolongement de Taylor d'ordre un ne l'est pas
(`div = z·U_xz`). Ce fond vérifie les murs et `W(−h) = 0`. La perturbation naît à zéro
(`η' = 0`, `v = 0`, `p' = 0`) et doit reconstruire la part non linéaire de HOS.

Contrôles du fournisseur, avant usage : sur une grille de points sous et au-dessus du plan moyen,
`div U = 0` et `U_t + ∇P/ρ = 0` à l'arrondi près, et `ζ_t = W(0)`. Le résidu contracté `S` se
réduit donc au terme `(U·∇)U`, comparé à son expression.

### 1.2 Critères, déclarés avant mesure

1. **Fond nul, identité au bit.** Avec des échantillons nuls et sans éponge, 50 pas de
   `step_perturbation_mobile` rendent u, w, p, `eta` et restes identiques au bit à
   `step_surface_mobile` depuis le même état, `a = 5 cm`, `nx = 32`.
2. **Refus et reprise.** Contexte, forme, non-planarité, `eta` incohérent dans une colonne et
   surface hors gardes : refus sans rien modifier. Expiration à plusieurs points : aucune
   avancée, état intact, mode couplé éteint, reprise identique au bit. Zéro allocation pendant
   le pas.
3. **Profil** (a = 5 et 10 cm) : `max|ζ − η_HOS|/a` sur tous les points et tous les pas d'une
   période ≤ **2 %** à `nx = 128`, et décroissant de 32 à 128. Tolérance de S237, critère 5.
4. **Harmonique `2k`** : écart maximal de `b₂` au véhicule ≤ **20 %** de `max|b₂,HOS|` à
   `nx = 128`, décroissant en raffinant. Tolérance de S237, critère 6.
5. **Témoin discriminant**, à `a = 5 cm` et `nx = 32` sur une période. Le pas couplé et le même
   pas **sans résidus de surface** (valeurs fantômes du fond et bande éteintes par un commutateur
   de test, géométrie totale conservée) sont comparés à l'ordre deux fermé de S237
   (`b₂ = a²·B₂(t)`). Le premier reste ≤ 20 % de `max|a²B₂|`, le second le dépasse.

**Correction du témoin, datée du 2026-09-16 (P6), après sa première exécution et avant toute
mesure des critères 3 et 4.** Le premier témoin mettait à zéro toute la correction du fond sur
les fantômes latéraux. Il retirait ainsi `−P_fond(Γ)` en entier, y compris sa partie d'**ordre
un** `ρg·ζ_fond`, et cassait la dynamique linéaire. Il divergeait (0,70 m/s dès le pas 100, refus
`Domain` au pas 1145), ce qui ne mesure pas les résidus de surface. Dans le témoin corrigé, la
pression du fond à l'interface est **linéarisée** en `ρg·ζ_fond(x_Γ)` (Taylor en x compris) et la
bande est éteinte. Seuls les termes d'élévation d'ordre deux disparaissent. Tolérance inchangée.
Le pas couplé, lui, rendait déjà `b₂` à 2,17 % de l'ordre deux sur cette première exécution.

Chiffres de S237 pour le **solveur total** (profil / `b₂`) : 10 cm 1,714/1,71 % (32),
0,592/0,98 (64), 0,230/0,43 (128) ; 5 cm 0,850/2,44 (32), 0,550/1,41 (64), 0,252/0,71 (128,
S238). Publiés en regard, ils ne servent pas de critères.

Un critère manqué est publié tel quel ; aucune tolérance n'est modifiée après mesure.

### 1.3 Arrêt

Pas couplé construit, critères 1 à 5 mesurés et publiés, coût médian à 128 colonnes relevé avec
techniques présentes et absentes. Si l'oracle refuse, s'arrêter au diagnostic et le publier.

## 2. Ce qui a été construit

API : `Volume::step_perturbation_mobile(time, durée_us, plafond, budget_us, &BackgroundFaces,
Sponge, jobs, horloge)`. `eta` porte `repos + η'` ; `set_free_surface(&[repos; nx], repos)` fait
naître un domaine à zéro.

- `delta_mobile.rs` : `height(i)` rend `η` hors du pas couplé et `ζ = η' + ζ_fond` pendant le
  pas. Mouillage, `θ` et gardes la lisent. Les fantômes vertical et latéral ajoutent leur terme
  du fond seulement en mode couplé ; le chemin S237 est identique au bit (`delta_mobile essai` :
  0,850 % / 2,44 %, comme S237).
- `delta_coupling.rs` : `prepare_surface_background` construit la géométrie totale, vérifie que
  `eta` est identique au bit sur chaque colonne w et calcule les valeurs du fond aux fantômes.
  `transport_coupled` calcule le débit S237 sur `ζ`, avec la bande en somme séparée.
  `step_perturbation_mobile` enchaîne gardes, sauvegarde, prédiction couplée (ADR-149),
  projection mobile, extrapolation, transport, validation, garde et publication.
- Mémoire : `nu + 2·nx` flottants de plus par volume (fantôme latéral par face u, surface totale
  et fantôme vertical par colonne), comptés avant allocation ; l'essai de comptabilité les
  recompte indépendamment.
- Fond de l'oracle : `examples/support/standing_background.rs`, partagé par le banc et les
  essais, avec l'ordre deux fermé de S237 (déplacé, arithmétique identique).

## 3. Essais

| essai | résultat |
|---|---|
| fond incompressible, linéaire, `ζ_t = W(0)`, `S = (U·∇)U`, sous et au-dessus du plan moyen | tenu à l'arrondi f32 |
| fantômes du fond contre `P` analytique à l'interface, 32 colonnes, 9 latéraux | sous la borne de Taylor `½(dx/2)²k²ρgaC` (1,79 Pa) |
| critère 1 : fond nul, 50 pas | **identique au bit** à `step_surface_mobile`, rapport compris |
| critère 2 : contexte, forme, non-planarité, `eta` incohérent, garde | refus, état intact, mode éteint |
| critère 2 : expiration à cinq points, reprise | aucune avancée, état intact, reprise au bit |
| critère 2 : allocations (pas complet et expiration) | **zéro** |
| critère 5 : `b₂` contre l'ordre deux fermé, 32 colonnes, 5 cm, une période | couplé **2,17 %** ; témoin **99,56 %** |

Le témoin sans résidus de surface ne produit pratiquement aucune harmonique `2k`. C'est cohérent
avec la physique du cas. La source volumique `(U·∇)U = ∇(|U|²/2)` d'un fond potentiel est un
gradient : la pression l'absorbe, et elle n'agit que par la valeur de `|U|²/2` à la surface. Or,
pour l'onde stationnaire à `kh = π`, cette valeur est presque uniforme en x, à
`1 − tanh²kh ≈ 0,75 %` près. L'harmonique vient donc des termes d'élévation de la surface,
précisément ceux que le témoin éteint. Le premier témoin, fautif, est décrit au §1.2.
