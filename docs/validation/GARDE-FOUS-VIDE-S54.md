# Les dix garde-fous sans objet mesurable — S54, 2026-09-07

Action S43-2. Inventaire d'origine : [AUDIT-GARDE-FOUS-S34](../registres/AUDIT-GARDE-FOUS-S34.md).
Base : 91e26df. Six tests regroupent les dix contrôles avec absence et témoin.

## Définir le vide avant de tester

Un domaine sec est un état physique valide. L'absence d'un réglage optionnel choisit
un défaut documenté. L'absence d'une mesure ne donne pas une mesure égale à zéro.
Ces trois situations ne demandent donc pas toutes le même refus.

| Contrôle | Absence exercée | Résultat | Témoin |
|---|---|---|---|
| G1 pas de temps | domaine entièrement sec | dt=1 s, volume toujours nul après évolution | domaine mouillé, dt fini positif différent |
| G2 bornage CFL | aucun réglage explicite ; demande de CFL nulle | défaut identique à 0,45 explicite ; zéro borné à 0,05 documenté | valeur nominale donnant un pas supérieur au minimum ; test S34 conserve 1,5 |
| G3 plancher | cinq erreurs exactement nulles | Plancher, aucune stabilité validée | suite d'erreurs divisées par deux : ordre un |
| G4 longueur | aucune grille | Indetermine, stabilité None | cinq grilles doublées, stabilité vraie |
| G5 amplitude | seiche exactement sans excitation, 200 cellules et 20 périodes | None | même montage à amplitude 0,02 m : mesure disponible |
| G6 réflexion | batteur arrêté dans la géométrie du témoin sans réflexion | None, pas de décroissance inventée | batteur nominal 0,05 m/s, mesure disponible |
| G7 front | domaine sec | None avant et après évolution | domaine mouillé, front présent |
| G8 référence | mesure NaN face à référence zéro puis un ; référence NaN | échec, pas d'écart zéro fabriqué | mesure zéro/référence zéro et mesure un/référence un passent |
| G9 u_max | aucune eau ni paroi mobile | vitesses absolue et gouvernante nulles | domaine mouillé : vitesse gouvernante positive ; présence de paroi couverte par S34 |
| G10 ordre grossier | aucune grille ou erreurs toutes nulles | brut None, valeur de filtre 1 conservée distinctement | suite d'ordre un : brut et borné égaux à un |

**Les six tests passent. Aucun faux succès supplémentaire sur ces entrées n'est trouvé.**
G1 testait déjà le domaine sec en S34 : l'affirmation de S43 selon laquelle aucun contrôle
n'avait d'entrée vide était trop générale. D'autres cas ont été couverts depuis par S43,
S46, S50 et S53 ; cette session complète une table cohérente des dix.

G6 illustre la portée d'un contrôle : une fenêtre géométriquement admissible ne prouve pas
qu'une onde existe. Le montage complet refuse grâce au profil d'amplitude vide. Le test
ne prétend pas que le garde-fou de réflexion détecte lui-même l'absence de source.

## Modification du montage

La fonction publique c33_decroissance_entretenue appelle désormais c33_avec_batteur avec
la valeur nominale 0,05. Cette fonction interne permet au test d'injecter zéro.
Il n'y a qu'un corps de montage, aucun calcul ou seuil du solveur modifié. La fenêtre,
le logarithme, la régression et la conversion sont identiques pour absence et témoin.
Il ne s'agit pas d'un nouvel outil de configuration destiné au rapport courant.

## Observation hors du test à vide

Le premier témoin choisi pour G5, 400 cellules et 60 s, est refusé malgré l'excitation
nominale de 0,02 m. Ce résultat ne suffit pas à démontrer un défaut : le détecteur impose
un nombre de passages à zéro et d'extrema exploitables. Le témoin final utilise le montage
effectivement publié, 200 cellules et 20 périodes, qui passe.

**Le refus initial n'est pas effacé des conclusions.** Action S54-1 : localiser sa cause
et distinguer une insuffisance de données d'un défaut du détecteur, avant toute correction.
Ce suivi ne remet pas en cause les mesures nominales conservées, et ne justifie aucun
assouplissement de seuil. S43-2 est close pour les dix garde-fous de l'inventaire S34.

## Limites

Le vide n'est pas une campagne de tous les paramètres invalides : NaN de CFL, dimensions
incohérentes, champs corrompus et toutes les combinaisons de parois restent hors de ce tableau.
Les saturations ont leur instrumentation propre depuis S38. Aucun résultat de validation 3D.

## Vérification finale

121 tests réussis (38 cœur + 83 harnais), deux ignorés. Six nouveaux tests à vide verts ;
compilation release réussie. Rapport physics comparé à S53 : seules trois lignes de durées
diffèrent, mesures et verdicts inchangés. Sortie 1 attendue pour C04 ordre un. check :
zéro échec, hashs 0x3e2c06a7b00e73e3 et 0x1a8b0629a9f51b6e inchangés.
