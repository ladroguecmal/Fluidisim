# Queue spectrale de B en pentes par pixel — S256

Contrat : [ADR-155](../adr/ADR-155-queue-spectrale-en-pentes-par-pixel.md). Origine : verdict R1,
[REVUE-VISUELLE](REVUE-VISUELLE.md) §7.

## 1. Protocole, écrit avant construction

### 1.1 Cuisson de la queue (cœur)

Recette R1 (`Hs` 1,5 m, `Tp` 6 s, γ 3,3, bande `[0,5 ; 4]`, 32 composantes), queue `[4 ; 32]·fp`.

1. **Densité absolue** : `Σ a²/2` de la queue égale `(Hs²/16)·∫_4^{32} q / ∫_{0,5}^{4} q`, calculé en
   f64 par l'intégration indépendante de `rugosite_b`, à 1 % près.
2. **Rugosité** : `mss` bande plus queue égale la `mss` continue coupée à `32 fp` (0,01953), à 2 %
   près.
3. **Continuité** : le rapport `a²/Δln f` des deux cellules de part et d'autre de `4 fp` reste dans
   le rapport des densités continues aux centres de ces cellules, à 5 % près.
4. **Refus** : borne de queue non finie, `b_Q ≤ b_B`, nombre hors `[16, 256]`, fréquence non
   représentable. La bande cuite n'est pas modifiée.
5. **Reproductibilité** : empreinte identique sur deux cuissons, et différente de celle de la bande.

### 1.2 Hôte

6. **GPU contre CPU** : aux sondes de `--verify`, la pente de queue calculée au GPU égale sa
   référence CPU f64, prise sur les mêmes composantes rebasées à la caméra et le même poids, à
   2·10⁻⁴ près (pente sans unité).
7. **Filtre** : pour `h_px` ≥ `π/k_max`, la pente de queue est nulle ; pour `h_px` → 0, le poids vaut 1.

   **Correction du critère, datée du 2026-09-16 (P6), après sa première exécution.** Le poids
   d'ADR-148 s'annule pour `k·h ≥ π`. Une empreinte `π/k_max` n'éteint donc que la composante la plus
   courte ; toutes s'éteignent pour `h ≥ π/k_min`. Le critère écrit était faux, et l'exécution l'a
   montré : pentes de 0,02 à `1,001·π/k_max`. Critère corrigé : pentes nulles à `1,001·π/k_min`, et
   composante `k_max` de poids nul à `1,001·π/k_max`, vérifiée sur la référence CPU. Aucune
   tolérance déplacée. La première exécution avait aussi lu `k_max` avant la mise à jour des
   composantes (`k_max = 0`), donc sans rien contrôler ; c'est corrigé.
8. **Réceptions existantes intactes** : `--verify` et `--multi --spectral-verify` rendent les mêmes
   écarts, puisque la queue ne touche ni la hauteur ni les pentes géométriques.
9. **Coût** (ADR-131) : GPU eau à 960×540 et 1280×720, poses de référence et rasante, avec et sans
   queue, secteur relevé.
10. **Rendus R2** aux sept poses de R1, avec empreintes, envoyés à l'utilisateur. Seul son verdict
    dit si le défaut perçu recule.

Un critère manqué est publié tel quel.
