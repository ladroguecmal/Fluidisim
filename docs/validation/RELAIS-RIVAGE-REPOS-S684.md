# Le raccord au rivage au repos : APIC 3D et Saint-Venant 2D côte à côte — S684 (liste 4.14)

*S684, 2026-10-08, en autonomie, vers la v2.* L'étape 1 du relais au rivage ([conception](../registres/RELAIS-RIVAGE-S679.md),
[ADR-271](../adr/ADR-271-le-film-du-rivage-a-saint-venant.md)), faite de ses trois briques (S680, S682, S683).

## Reproduire

- `cargo test --manifest-path code/Cargo.toml --release --offline -p water-core --lib s684 -- --nocapture` (≈ 4 min).

## Ce qui a été fait

Le module `relais_rivage.rs`. `RelaisRivage` tient APIC 3D et Saint-Venant 2D côte à côte, à la même maille et au même nombre de
rangées. À chaque pas :

1. le niveau et la vitesse du bord 3D nourrissent le bord gauche de Saint-Venant ;
2. Saint-Venant rend son flux ;
3. APIC le reçoit comme vitesse de son bord droit, fait entrer le reflux et fait son pas ;
4. ce qu'APIC a laissé sortir contre ce que Saint-Venant a pris devient une dette.

La masse se compte : Saint-Venant + particules × quantum + réservoir − dette.

## Mesuré

**Le montage** : les plages de S678, le raccord à trois mailles de fond au moins, Saint-Venant jusqu'à 0,3 m après la ligne d'eau.
L'eau est au repos pendant 2 s. La plage 1:10 à 0,18 m de S678 est remplacée par 1:10 à 0,30 m : elle n'a nulle part trois mailles de fond
à 5 cm.

| plage | maille | le raccord | la ligne d'eau dans la maille | APIC | Saint-Venant | la masse |
|---|---|---|---|---|---|---|
| 1:3, 0,40 m | 5 cm | 16,0 cm de fond | 0,10 | 2,1·10⁻⁶ m/s | 1,3·10⁻⁶ m/s | 3,0·10⁻¹⁶ |
| 1:3, 0,40 m | 2,5 cm | 8,1 cm | 0,20 | 4,1·10⁻⁶ | 5,4·10⁻⁷ | 1,5·10⁻¹⁶ |
| 1:10, 0,30 m | 5 cm | 15,3 cm | 0,10 | 2,5·10⁻⁶ | 7,2·10⁻⁷ | 5,4·10⁻¹⁶ |
| 1:10, 0,30 m | 2,5 cm | 7,7 cm | 0,20 | 3,7·10⁻⁶ | 2,6·10⁻⁶ | 2,7·10⁻¹⁶ |
| 1:3, 0,30 m | 5 cm | 16,0 cm | 0,10 | 2,2·10⁻⁶ | 5,2·10⁻⁷ | 1,2·10⁻¹⁶ |
| 1:3, 0,3125 m | 2,5 cm | 8,5 cm | 0,70 | 5,0·10⁻⁶ | 1,8·10⁻⁶ | 2,2·10⁻¹⁶ |

| critère (écrit avant) | mesuré |
|---|---|
| (1) sous 1 cm/s partout, des deux côtés | **tenu** : au plus 5,0·10⁻⁶ m/s. APIC seul, sur les plages de S678 : de 0,21 à 0,58 m/s |
| (2) la masse à 10⁻¹² près | **tenu** : au plus 5,4·10⁻¹⁶ |
| (3) les essais d'APIC et de Saint-Venant inchangés | tenu |
| ADR-272 D1, trois places de la ligne d'eau | 0,10 ; 0,20 ; 0,70 |

**Un montage corrigé en route.** Au premier essai, la plage 1:3 à 0,31 m manquait à 5 cm (7 cm/s d'un côté, 0,5 m/s de l'autre). Les
particules sont posées au quart de maille, si bien que l'eau 3D s'arrêtait à 0,30 m, alors que Saint-Venant partait de 0,31 m. Le
raccord faisait couler ce centimètre, comme le plan l'avait prévu (« des niveaux mal accordés font couler un flux »). Saint-Venant part
maintenant du niveau de la 3D, si bien que cette plage devient 0,30 m à 5 cm et 0,3125 m à 2,5 cm.

## Ce que cela dit

Le défaut de S640 et S678 disparaît : le film du rivage n'est plus à APIC. L'eau au repos s'y tient à quelques µm/s, cent mille fois
mieux qu'APIC seul. La masse se compte au bit à travers le raccord.

**Ne fait pas encore** :

- l'onde qui traverse le raccord (l'étape 2 : la remontée contre Synolakis et le tout-Saint-Venant) ;
- le reflux (l'étape 3) ;
- la vague qui plonge avec les deux raccords (l'étape 4) ;
- un état extérieur par rangée (la moyenne, exacte sur une côte uniforme) ;
- le remboursement de la dette.
