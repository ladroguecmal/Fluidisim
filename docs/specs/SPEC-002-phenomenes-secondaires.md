# SPEC-002 — Phénomènes secondaires et interfaces : fiche de référence chiffrée

- **Statut** : référence — formules fermes, valeurs dérivées à confirmer par mesure
- **Session** : S02
- **Complète** : `SPEC-001` (hydrodynamique de base). Même règle : citer une ligne d'ici plutôt que
  réinventer un chiffre.

---

## 1. Écume, embruns, fragmentation

**Couverture de moutons** (Monahan) : `W ≈ 3,84·10⁻⁶ · U10^3,41`

| U10 | 5 | 10 | 15 | 20 | 25 m/s |
|---|---|---|---|---|---|
| couverture | 0,09 % | 1,0 % | 3,9 % | 10,4 % | 22 % |

**Seuils de vent** : embruns arrachés aux crêtes à partir de ≈8 m/s ; écrêtage massif à ≈11 m/s.

**Déferlement** : cambrure limite de Stokes `H/λ = 1/7 = 0,143` ; accélération descendante de crête
`0,45 g` (glissant) → `g` (plongeant) ; en eau peu profonde `H/h = 0,78` (SPEC-001 §3).

**Fragmentation** — nombre de Weber `We = ρ·v²·d/σ`, `σ_eau = 0,072 N/m`, rupture au-delà de
`We ≈ 12` :

| d ↓ · v → | 2 m/s | 5 m/s | 10 m/s |
|---|---|---|---|
| 0,5 mm | 28 | 174 | 694 |
| 2 mm | 111 | 694 | 2 780 |
| 10 mm | 556 | 3 472 | 13 900 |

→ toute nappe projetée au-delà de ≈1 m/s se fragmente.

**Demi-vies de l'écume** : canal actif ≈3 s, canal résiduel ≈30 s.

---

## 2. Bulles et aération

**Vitesse de remontée terminale**

| Diamètre | Vitesse | Remontée de 3 m |
|---|---|---|
| 0,1 mm | 5,5 mm/s (Stokes) | 9 min |
| 1 mm | 0,12 – 0,25 m/s | 12 – 25 s |
| 5 mm | ≈0,25 m/s | 12 s |

Stokes : `v = (ρ_eau − ρ_air)·g·d² / (18·μ)`, `μ_eau = 1,0·10⁻³ Pa·s`.

**Densité effective d'une eau aérée** : `ρ_eff = (1 − α)·ρ`. 10 % d'aération = 10 % de portance
en moins.

---

## 3. Air : compression et vide

**Pression ambiante** : `P = P_atm + ρ·|g_eff|·h`. **Boyle** (`P·V = cte`) :

| Profondeur | Pression | Volume d'une poche |
|---|---|---|
| 0 m | 101 kPa | 100 % |
| 10 m | 202 kPa | 50 % |
| 20 m | 303 kPa | 33 % |
| 30 m | 405 kPa | 25 % |

Compression rapide (impact) : adiabatique, `P·V^1,4 = cte`.

**Eau exposée au vide** — point triple à **611 Pa / 0,01 °C**. Depuis 20 °C :

```
chaleur à évacuer par kg gelé = 4,18·20 + 334 ≈ 418 kJ/kg
chaleur absorbée par kg évaporé ≈ 2 500 kJ/kg
fraction évaporée ≈ 14 %   ·   fraction gelée ≈ 86 %
```

**Entrée dans l'eau** : Froude d'entrée `Fr = v/√(g·D)` ; cavité franche au-delà de `Fr ≈ 5` ;
pincement vers 2 à 4 diamètres de profondeur (à calibrer, banc B10).

---

## 4. Glace

**Croissance** (Stefan) : `h = √(2·k·ΔT·t / (ρ·L))`, `k = 2,2 W/m/K`, `L = 334 kJ/kg`,
`ρ = 917 kg/m³`. Forme pratique : **`h ≈ 0,035·√FDD`** (m, FDD en K·jour).

| FDD | 10 | 50 | 100 | 200 |
|---|---|---|---|---|
| h | 11 cm | 25 cm | 35 cm | 50 cm |

**Portance** — résistance en flexion, `P ∝ h²` (Gold) :

| h | 5 cm | 10 cm | 20 cm | 30 cm | 50 cm |
|---|---|---|---|---|---|
| charge | rien | une personne | un groupe, motoneige | voiture légère | camion léger |

**Flottaison** : `ρ_glace = 917` → 8,3 % émergé. **Formation en plaque** : exige `Hs < 0,15 m`.

---

## 5. Danger et traversabilité

**Produit d'emportement** : `HR = d·(v + 0,5)` (d en m, v en m/s)

| HR | < 0,75 | 0,75 – 1,25 | 1,25 – 2,50 | > 2,50 |
|---|---|---|---|---|
| | faible | dangereux pour certains | pour la plupart | pour tous |

→ **0,5 m à 2 m/s emporte déjà un adulte.** Une voiture flotte vers 0,30 m, est emportée vers
0,60 m.

**Seuils de profondeur humanoïde** : 0,15 / 0,50 / 1,00 / 1,30 m (éclaboussures / ralenti /
équilibre précaire / nage).

---

## 6. Acoustique

| Grandeur | Air | Eau |
|---|---|---|
| Célérité | 343 m/s | 1 482 m/s (×4,32) |
| Impédance | 415 rayl | 1,48·10⁶ rayl |

**Transmission à l'interface** : `T = 4Z₁Z₂/(Z₁+Z₂)² ≈ 1,1·10⁻³`, soit **−29,5 dB**.

**Délai de propagation aérien** : 0,29 s à 100 m · 1,46 s à 500 m · 2,92 s à 1 km · 8,7 s à 3 km.

**Résonance de remplissage** (Helmholtz) : `f = (c/2π)·√(A / (V_air·L_eff))`.

---

## 7. Optique sous-marine

**Indice** `n = 1,333`. **Angle critique** `48,6°` → fenêtre de Snell de **97,2°** ; au-delà,
réflexion totale interne. Vu depuis l'air, un objet immergé apparaît à **3/4** de sa profondeur.

**Atténuation de l'eau pure**

| λ | coefficient | 1 % de transmission |
|---|---|---|
| 650 nm (rouge) | ≈0,34 m⁻¹ | ≈13 m |
| 550 nm (vert) | ≈0,06 m⁻¹ | ≈75 m |
| 450 nm (bleu) | ≈0,015 m⁻¹ | ≈300 m |

**Visibilité** : `Secchi ≈ 1,7 / c`. Océan clair 30–50 m · côtier tempéré 5–15 m · estuaire en
crue 0,2–1 m.

---

## 8. Sources

Monahan & O'Muircheartaigh pour les moutons ; Stokes pour la cambrure limite et la remontée des
bulles ; critère de Weber classique pour la fragmentation ; Boyle et loi adiabatique pour l'air ;
équation de Stefan et formule de Gold pour la glace ; produit de danger issu des standards
d'hydrologie de crue ; Snell et coefficients d'absorption de l'eau pure pour l'optique ;
impédances acoustiques standard. Aucune n'est spécifique au projet.
