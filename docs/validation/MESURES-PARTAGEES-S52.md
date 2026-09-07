# Régression partagée et contrats C22 — S52, 2026-09-07

Suite de [l'inventaire S51](AUDIT-MESURES-S51.md), action S51-1.
Base : 50e952d. Aucun solveur, seuil physique ou scénario modifié.

## Régression centrée

`regression::centree` reçoit les couples déjà transformés et rend `Option<(pente, R²)>`.
Les deux anciennes boucles centrées sont remplacées par un appel à ce même calcul.
L'ordre des sommes reste celui du parcours initial pour conserver les résultats nominaux.

| Appelant | Points conservés chez l'appelant | Conversion conservée |
|---|---|---|
| mesurer_seiche_cfl | extrema temporels, seuil relatif 10⁻³ du premier, au moins quatre points ; au moins trois passages à zéro et six extrema en amont | ln(amplitude), demi-vie ln(2)/−pente puis division par la période mesurée |
| c33_decroissance_entretenue | fenêtre sans réflexion, loin du batteur et du front, au moins dix amplitudes positives | log₂(amplitude), longueur de demi-décroissance −1/pente |

La régression non centrée shallow reste distincte : sélection de pics par blocs de 32,
période de référence, pas de R². La remplacer ici déplacerait un autre montage.

**Refus du calcul partagé :** moins de deux points, entrée non finie, sommes ou produits
non finis, variance nulle en x ou en y, dénominateur non positif, résultat non fini.
Aucun point non fini n'est retiré pour sauver la régression. Une série horizontale a une
pente définie mais pas de R² : le couple demandé est donc refusé. Les anciennes boucles
produisaient alors R²=0, sans établir la qualité de l'ajustement. À distinguer d'une
covariance nulle avec deux variances positives : pente=0 et R²=0 restent retournés.

Une pente positive finie reste admise et conserve la demi-vie infinie chez les appelants.
L'absence de régression remonte par leur Option existante. Les messages de refus C03 et
du rapport spatial citent désormais aussi le montage et la régression : ils ne peuvent
plus attribuer tous les refus à une onde éteinte ou à une fenêtre trop courte.
Les balayages qui omettaient déjà les mesures absentes gardent ce comportement.

**Limite :** ce contrôle porte sur les points reçus. La sélection d'extrema, d'enveloppe
et de fenêtre précède la régression ; le helper ne prouve pas la validité du champ entier.
Il ne rétablit pas les données qu'un calcul amont aurait déjà écartées.

## Projections C22 : décision de ne pas les extraire maintenant

| Contrat | C22 delta, physics.rs | C22 shallow, c22_shallow.rs |
|---|---|---|
| Projection | moyenne arithmétique de cellules emboîtées | même moyenne |
| Norme | somme des écarts absolus / somme signée des moyennes | somme des écarts absolus / somme des valeurs absolues des moyennes |
| Entrées | tailles incompatibles écartées ; nx=0 non protégé avant modulo | vide, non-emboîté et non-fini refusés |
| Évolution | champ après avancer_equilibre | temps final atteint et absence de saturation vérifiés |
| Grille absente | continue après allocation refusée | erreur de campagne |
| Contamination | extrapolation de l'erreur d'oracle avec p borné, retain | deux oracles, préfixe sain coupé au premier rejet |
| Arrondi | plancher 10⁻⁷, calcul du solveur f32 | n_fin·epsilon_f64, solveur f64 |

Sur les hauteurs positives du montage courant, les normes coïncident ; elles ne désignent
pas encore la même opération avec le même contrat sur toutes les entrées. Partager la
seule somme masquerait surtout les différences d'admission et de conservation des grilles.
**Décision S52 : conserver les deux projections et traiter d'abord l'admission côté delta.**
Aucune nouvelle campagne à 102400 cellules n'est nécessaire pour comparer ces contrats.

La lecture trouve un point que la correction S46 ne couvre pas : elle conservait les
triplets refusés dans Convergence, mais le montage delta peut retirer une grille avant
de lui transmettre la famille. Convergence calcule les ordres par position dans le vecteur,
sans vérifier que les tailles doublent. Cela mérite un essai de grille absente/non emboîtée
avant tout partage (S52-1). Les paramètres fixes du rapport courant ne démontrent aucun
verdict nominal faux ; ce relevé ne prétend pas avoir reproduit une telle erreur.

S51-1 est exécutée : régression partagée et projections examinées, décision motivée ci-dessus.

## Vérification

111 tests réussis (38 cœur + 73 harnais), deux ignorés. Les trois tests du calcul partagé
vérifient une droite de pente −2, un résidu donnant R²=0,25, une covariance nulle valide,
les séries vides/dégénérées, les non-finis et débordements, ainsi que les deux conversions
logarithmiques sur des décroissances construites. Compilation release réussie.

Rapport physics comparé à S51 : seules quatre lignes de durées diffèrent. Les périodes,
demi-vies, longueurs de décroissance, R² et verdicts imprimés restent inchangés. Sortie 1
attendue pour C04 ordre un. check : zéro échec, hashs 0x3e2c06a7b00e73e3 et
0x1a8b0629a9f51b6e inchangés. Les durées ont été mesurées sous charge concurrente de tests,
ce ne sont pas des benchmarks. Aucun nouvel essai de production ou de validation 3D.

**Suite exécutée S53 :** [GRILLES-C22-S53](GRILLES-C22-S53.md). Les défauts d'admission
et de conservation des grilles sont reproduits et corrigés ; S52-1 close.
