# Le vent comme paramètre de scène — S263

Contrat : [ADR-160](../adr/ADR-160-vent-parametre-de-scene.md). Origine : verdict R5,
[REVUE-VISUELLE](REVUE-VISUELLE.md) §12.

## 1. Protocole, écrit avant construction

1. **Recette** : `Hs` et `Tp` de la mer de vent égaux aux formules de Pierson–Moskowitz, à l'arrondi f32.
2. **Rugosité** : `mss` totale (mer de vent + houle + queue gardée) égale à Cox–Munk au même vent, à
   une composante de queue près (écart publié), ou limitée par la coupure capillaire, ce qui se dit.
3. **Pointe et replis**, par vent (3, 5, 8,4 m/s), instrument statistique, 10⁶ points : `c40`, `c22`,
   `c04` publiés en regard de Cox–Munk ; aucun repli CWM.
4. **Scènes existantes au bit** : R2 à R4 rejoués, et `--vagues --modulation` sans `--vent` identique
   à S262.
5. **Accord CPU/GPU** : `--cwm-verify` et `--cwm-query-verify` sous `--vent=5`, tolérances de S260 et S262.
6. **Rendus de calibration R6** : trois vents × trois poses, ciel clair, envoyés à l'utilisateur, qui
   choisit.

## 2. Ce qui a été construit

- **Cœur** (`background_spectrum`) : `fully_developed_wind_sea`, `cox_munk_mss`, `capillary_ratio` et
  `tail_count_for_mss`. Une seule source pour l'hôte et pour l'instrument.
- **Hôte** : `Scene::build(houle, vagues, wind)`, option `--vent=U` avec `--vagues --modulation`.
  Chaque lancement imprime `VENT` : `U`, `Hs`, `Tp`, coupure, lignes de queue, `mss` et Cox–Munk.
  Rendus de calibration sous `--revue=r6_vN` (`captures/s263`).
- **Instrument** : `examples/vent_rugosite.rs`.

## 3. Résultats

| critère | 3 m/s | 5 m/s | 8,37 m/s |
|---|---:|---:|---:|
| 1. `Hs` / `Tp` (Pierson–Moskowitz) | 0,193 m / 2,19 s | 0,535 m / 3,65 s | 1,500 m / 6,11 s |
| coupure de la queue | 21,0 fp (capillaire) | 32 fp (cuisson) | 32 fp (cuisson) |
| lignes de queue gardées | 43 | 50 | 64 (toutes) |
| 2. `mss` du spectre / Cox–Munk | 0,0182 / 0,0184 | 0,0290 / 0,0286 | 0,0446 / 0,0459 |
| 3. `mss` rendue (CWM, modulation) | 0,0184 | 0,0295 | 0,0457 |
| `c40` / `c22` / `c04` | 0,124 / 0,046 / 0,139 | 0,234 / 0,081 / 0,234 | 0,359 / 0,122 / 0,364 |
| replis | 0 | 0 | 0 |

- **Critère 2 tenu**, à une composante près. À 8,37 m/s, la queue entière (32 fp) donne 0,0446 : la
  limite de cuisson, et non la limite capillaire, arrête la coupure ; l'écart vaut 2,8 %.
- **Critère 3** : aucun repli. **La pointe décroît avec le vent** : `c40` vaut 0,12 à 3 m/s, sous la
  borne basse de Cox–Munk (0,40 − 0,23 = 0,17). `M` = 2 a été ajusté à 8 m/s, et Cox–Munk ne publie
  pas de dépendance de la pointe au vent. Constat publié, non corrigé.
- **Critère 4 tenu** : sans `--vent`, R2 et la scène complète `--vagues --modulation --ciel-clair` sont
  identiques au bit au code de fin de S262 (empreintes `captures/s262/revue-s262-final.txt`). La
  recette S201 n'est pas touchée.
- **Critère 5 tenu** : `--cwm-verify --vent=5`, déplacement 1,5·10⁻⁶ m, pente 2,05·10⁻⁴, 0 repli ;
  `--vent=3`, pente 1,26·10⁻⁴ ; `--cwm-query-verify --vent=5`, hauteur à 0,29 mm de l'image sur
  5 592 sondes, contre 0,210 m pour la requête linéaire.

**Rendus de calibration R6** (critère 6), ciel clair, une exécution par vent. La reproductibilité
du rendu est établie par les séries antérieures, avec le même code et sans `--vent` :

| image | 3 m/s | 5 m/s | 8,37 m/s |
|---|---|---|---|
| référence | `0x62bc4ad4f5af9a8f` | `0x0f178a2e5a4fcb7d` | `0x049094cca0ce54f5` |
| rasante | `0x935f4f499e0b452a` | `0x5562726c968b06a4` | `0x49f83f67766f6c7c` |
| haute | `0x21d0cfd21f9a0cc0` | `0x842b123277bb9999` | `0x92cf015680bd83a2` |

Les sept poses de chaque vent sont dans `viewer/captures/s263/revue.log`. Neuf images ont été envoyées à
l'utilisateur, qui choisit.
