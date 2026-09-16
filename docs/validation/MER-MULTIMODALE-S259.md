# Mer multimodale et étalement directionnel — S259

Contrat : [ADR-156](../adr/ADR-156-mer-multimodale-et-etalement.md). Origine : verdict R2,
[REVUE-VISUELLE](REVUE-VISUELLE.md) §8 ; A287.

## 1. Protocole, écrit avant construction

### 1.1 Cœur

1. **Spectre inchangé** : pour chaque système, amplitudes, `k` et fréquences Q32 **identiques au
   bit** à `bake` ; pour la queue, amplitudes identiques au bit à `bake_tail`.
2. **Loi inverse** : pour `s` ∈ {0,3 ; 1 ; 10 ; 75}, la moyenne de `cos(F_s⁻¹(u))` sur 4 096 `u`
   équirépartis vaut `s/(s + 1)` à 1 % près. Les directions restent dans `]−π, π]`.
3. **Décorrélation** : sur les 32 composantes de la mer de vent, `|ρ|` de Spearman entre le rang
   de fréquence et l'écart angulaire est au plus `3/√32 = 0,53`. La fixture V1 donne `ρ = 1`.
4. **Largeur selon la fréquence** : l'écart angulaire moyen des composantes proches du pic est
   plus faible que celui des composantes au-delà de `2 fp`.
5. **Assemblage** : `m0` total égal à `Σ Hs²/16` à 10⁻⁵ près (relatif) ; nombre de composantes
   égal à la somme ; phases différentes entre systèmes au même indice ; refus si la gravité
   diffère, si le total dépasse 256 ou si la liste est vide.
6. **Non-régression** : l'empreinte figée de `bake` (`0x26695af7314e21db`) et les essais du
   spectre sont inchangés.

### 1.2 Hôte

7. **Scène par défaut au bit** : R2 rejoué avec les sept mêmes empreintes, et `--tail-verify` rend
   la même ligne.
8. **Scène `--houle`** : les hauteurs GPU s'accordent au cœur à moins de 3 mm aux sondes de
   `verify`, à plusieurs âges, avec la tolérance historique.
9. **Coût** : GPU eau en 1280×720, poses de référence et rasante, défaut contre `--houle`, avec
   l'état du secteur.
10. **Rendus R3** aux poses de R1/R2 sur la scène `--houle`, avec leurs empreintes, envoyés à
    l'utilisateur.

Un critère manqué est publié tel quel.
