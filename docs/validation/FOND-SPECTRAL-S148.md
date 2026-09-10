# S148 — Premier fond spectral évaluable

2026-09-10, ADR-101. Aucun scénario historique migré.

## Réception

`cargo test --manifest-path code/Cargo.toml -p water-core s148 -- --nocapture`
et même commande avec `--release` : **cinq essais réussis** dans chaque profil.

Douze recettes : gamma1/3,3/7, N32/64/128/256, bande [0,5fp;4fp], Hs0,2 m, Tp6 s,
g9,81, graine42 et éventail30°. Les moments m1/m2/m4 des coefficients cuits sont comparés
à l'intégration indépendante S147. Écart maximal **0,001852739 relatif (0,185274 %)**,
pour un seuil de réception 0,2 % motivé par les mesures S147 (<0,186 %) plus marge
arithmétique. Hs reçu à2e-6 relatif ; diagnostics m0/m2 à2e-5 absolu ; densité maximale
dans une cellule logarithmique autour de fp. Ces tolérances reçoivent le calcul, pas la mer.

Hash de cuisson nominal : `26695af7314e21db` ; hash de champ à123456789 µs,
grille12×12/pas2 m : `2f32c548a0ff89d2`. Identiques debug/release, conservés en assertions.
Recuisson et reconstruction identiques ; changer la graine change le champ. Mer Hs0 reçue.

Dérivée temporelle reçue par différence centrée ±1 ms à trois dates, erreur absolue<2e-5 m/s ;
vitesse verticale et dérivée identiques en bits. Pente locale sous le majorant publié.
Composition avec un impact de0,01 J/λ4 m au point(1,2), âge1 s : sortie différente de B seul
et égale en bits à B+impact calculés séparément. Gravités9,81 et3 reçues ; gravité incompatible
refusée par le lot, sortie antérieure conservée.

Refus : entrées non finies, Hs négatif, gravité invalide, gamma hors profil, bande sans pic,
N nul, direction hors convention, fréquence non représentable et allocation après seal.
Cuisson sur tableaux fixes ; aucune allocation de bake constatée par inspection.

## Limites

Rejeu reçu **depuis la recette en mémoire**, pas depuis un nouveau codec ou un fichier.
Pas de réception du spectre directionnel, de l'ensemble des valeurs intermédiaires du profil,
des statistiques sur fenêtres finies, des coûts, de la composition pression/mixte avec ce
nouveau fond ou des plateformes distantes. S147-1 et A212 restent partielles, suite S148-1.
La gravité des trois compositions a été raccordée ; le témoin à gravité différente exerce
le chemin impacts. Les chemins pression/mixte gardent leurs essais historiques.

Suite complète workspace : **282 réussis/cinq ignorés**, zéro échec ; quatre avertissements préexistants du harnais. Aucun hash historique renouvelé.
