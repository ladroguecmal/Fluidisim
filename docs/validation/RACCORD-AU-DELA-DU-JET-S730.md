# Le raccord du rivage au-delà du jet — S730 (liste 4.14 ; ADR-284, R43)

*S730, 2026-10-09.* R43 est reçu (ADR-284), réaliste et cohérent, sauf au raccord du rivage : le jet de la 3D y retombait contre un mur
d'eau de Saint-Venant, avec une poche d'air (la capture de l'utilisateur, à ≈ 3,0 s). ADR-284 D2 : une frontière entre deux modèles ne se
place pas là où agit une physique que l'un des deux n'a pas.

## Ce qui est fait

- `deux_raccords_porteur_xf` : le même montage, avec le raccord du rivage à `x_f`. 10,775 m reste le défaut, au bit.
- **L'instrument du mur** (ADR-284 D4), `J(t)` : le niveau de Saint-Venant à sa première maille, moins celui de la 3D à sa dernière colonne
  (le fond + l'épaisseur lue par φ, ADR-280 D1). Son plancher de bruit est le plus grand `|J|` avant l'arrivée de l'onde.
- `outils/rendu_bout_en_bout.py` : des titres en option, et la planche du jet de près (`<sortie>_jet.png`).

## Reproduire

- `python outils/essai.py the_shore_relay_beyond_the_jet_s730 --ignore` (≈ 29 min). Le filtre prend aussi E2,
  `the_all_3d_reference_with_the_shore_relay_beyond_the_jet_s730`, dont le nom contient celui de E1.
- `python outils/rendu_bout_en_bout.py calculs/s720_bout.bin calculs/s730_bout_12.bin captures/s730 "Avant (R43) : …" "Après (S730) : …"`.

## Mesuré

**E1 — de bout en bout** (SGN, la bande 3D depuis 5,0 m, la mort à 3,2 s, Saint-Venant) :

| | raccord à 10,775 m (R43) | **raccord à 12,0 m** | critère |
|---|---|---|---|
| le mur, le plus grand `J` sur [2,4 ; 3,2 s] | **281,5 mm** (à 2,934 s) | **0,0 mm** | < 1 cm et < ¼ de R43 |
| le plancher de bruit de `J` | 0,69 mm | 0,00 mm | sous le critère |
| le retournement | 2,569 s, 9,863 m | 2,588 s, 9,913 m | 0,1 s, 0,15 m |
| l'air enfermé | 2,747 s, 10,294 m | 2,762 s, 10,288 m | — |
| la masse | 1,7·10⁻¹⁵ | 1,6·10⁻¹⁵ | 10⁻¹² |
| la remontée maximale | 0,3714 m à 3,859 s | 0,3364 m à 3,820 s | rapportée |
| le coût | 291 s | **308 s (1,06×)** | rapporté ; la suite si > 2× |

À 12,0 m, `J` ne dépasse jamais zéro : Saint-Venant n'est jamais plus haut que la 3D au raccord. Le jet retombe, puis rebondit, dans la 3D.

**E2 — le tout-3D, raccord à 12,0 m**, le nouveau témoin :
- le retournement à 2,620 s et 9,938 m (S717 : 2,637 s et 9,988 m) ;
- l'air à 2,804 s, 10,375 m ;
- la masse 1,2·10⁻¹⁶ ;
- le mur 0 ;
- la remontée 0,3547 m à 3,942 s (S717 : 0,3387 m) ;
- 1 152 s (S717 : 1 191 s).

**Critères : tenus.**

## Ce que cela dit

- **Le mur de R43 était le raccord, et non la vague.** Repoussé au-delà du rivage, il disparaît pour 6 % de calcul en plus. La 3D porte
  alors la lame mince sur le sable jusqu'à sa mort, et le pas ne tombe qu'à 2,5 ms (non sous 0,3 ms comme sur Synolakis, S712).
- **Le juge tout-3D de S717 portait ce mur.** Corrigé, sa remontée monte de 1,6 cm. De bout en bout contre lui, la remontée passe de
  +3,3 cm (S718) à **−1,8 cm**. La conclusion de S718–S719 (« le large SGN en cause ») est donc à relire : une part de l'écart venait du
  raccord.
- Le retournement bouge de 0,02 à 0,05 s avec la place du raccord : la réflexion du mur remontait jusqu'au déferlement.

## La suite (ADR-284 D3)

1. **La règle tirée de la prédiction** : la place du raccord choisie par le point de déferlement prévu, et la distance de chute du jet.
2. **Les frontières qui se déplacent** pendant le calcul : le geste de B4b (S728).
3. Puis la bande étroite (ADR-275 D2, étape 4) : au repos, quatre fois moins de particules (S730, calculé sur ce montage).
