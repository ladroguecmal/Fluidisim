# Mesures dupliquées — S51, 2026-09-07

Action S42-3 ; base examinée : 8fc8a65. Audit des estimateurs et de leurs appelants
dans water-harness/src : physics, physics_shallow, physics_dispersif, oracle,
c22_shallow, rapport_convergence et main. Les solveurs du cœur restent indépendants.
Recherche des fonctions, logarithmes, sommes de régression, passages à zéro,
normes et références de Ritter ; lecture des blocs puis recherche de leurs usages.
Ce relevé manuel n'est pas une preuve d'absence de toute duplication sémantique.

## Inventaire et décisions

| Famille | Chemins exécutés | Constat et traitement |
|---|---|---|
| Demi-vie shallow | demi_vie_seiche et c03_seiche → demi_vie_depuis_enveloppe ; tests via oracle | une implémentation depuis S42 ; pas de copie inline restante |
| Ordre de Richardson | Convergence::ordre → rapport principal/C22 ; ordre_grossier_estime → filtre C22 delta, diagnostic Ritter shallow, tests G10 | deux formules dans physics.rs ; refus non finis ajouté seulement au premier en S46 ; correction S51 |
| Régression centrée pente/R² | mesurer_seiche_cfl → C03 et balayages ; c33_decroissance_entretenue → rapport spatial et test G9 | même algèbre copiée, ln temporel contre log2 spatial ; extraction possible en conservant fenêtres et unités ; action S51-1 |
| Régression shallow | demi_vie_depuis_enveloppe | sommes non centrées et pics par blocs de 32, période de référence ; distincte de la régression delta sur extrema et période mesurée ; ne pas changer silencieusement la mesure |
| Période | passages_a_zero/periode_moyenne → shallow C03 et dispersion exacte ; mesurer_seiche_cfl → delta C03 ; zéro_montant/temps → C02 analytique | montants échantillonnés, descendants en intégration, recherche spatiale/temporelle : supports et refus différents, pas une extraction oubliée |
| Ritter et L1 | ritter ponctuel delta ; ritter_h_moyenne → shallow C04/C08/front_exact ; erreur_l1_ritter_ponctuelle → seul test historique | comparaison aux centres et comparaison aux moyennes sont distinctes ; le témoin ponctuel explique une ancienne publication, il ne doit pas remplacer la référence actuelle |
| Projection oracle | c08_convergence_reguliere delta ; erreur dans c22_shallow | même moyenne conservative et norme relative sur les hauteurs positives ; dénominateur signé côté delta, absolu côté shallow, refus des entrées et contrôle de contamination différents ; montages conservés |
| Comparaison croisée | Ecart::entre → confronter_c01/c04, vitesse hors zone sèche et tests | fonction partagée déjà appelée ; normes de comparaison champ à champ différentes des assertions analytiques |
| Front et bilan | front_mouille → sorties shallow ; Cas::passe et Bilan::ajouter → main | refus traité en S50 ; classification C08 unifiée en S46, diagnostic Ritter séparé en S47 ; pas de seconde classification de validation retrouvée |
| Absorption | amplitudes_c05 shallow ; energies_dispersif/trace_jauge | amplitude à jauge contre énergie et fenêtres dispersives : mesures distinctes, pas de substitution entre elles |

Les formules de prédiction dans les tests restent des témoins théoriques, pas des
implémentations concurrentes d'une mesure publiée. Les montages indépendants des deux
véhicules restent distincts conformément à ADR-043.

## Défaut reproduit sur l'ordre

ordre_grossier_estime appliquait log2 sans vérifier les entrées ni le résultat.
Une entrée NaN ou un rapport débordant pouvait devenir Some(NaN) ou Some(inf),
alors que Convergence::ordre refusait depuis S46. Le refus existe dans le type,
mais le chemin numérique doit réellement l'utiliser. Le filtre signale déjà
l'absence ou un ordre hors bornes ; le diagnostic Ritter affiche explicitement
l'absence. Aucun succès nominal indu n'est établi par cet audit.

S51 partage le calcul de Richardson entre les deux chemins, en conservant leurs
contrats : plancher configuré et catégorie Plancher dans Convergence, plancher zéro
et valeur bornée de secours 1 pour le filtre. Les valeurs nominales doivent rester intactes.

**Précisions de portée.** Le test historique c08_l_ecart_au_p_publie_vient_du_changement_de_reference
recalcule Richardson pour comparer les anciennes et nouvelles références ; il ne fournit
aucune mesure au rapport. Le quotient direct e1/e2 de DiagnosticRitter est son contrôle de
cohérence, pas une copie de Richardson (qui utilise des différences de trois erreurs).
La formule de Richardson n'a plus qu'un corps utilisé par les chemins de rapport/filtre.

La projection conservative des deux C22 reste une duplication algébrique identifiée, mais
avec des contrats de refus et de contamination différents. S51-1 doit décider explicitement
si son partage améliore le suivi des refus avant de la modifier. Le relevé ne justifie pas
une fusion automatique de toutes les opérations ressemblantes.

## Vérification finale

Le test ajouté échouait avant correction avec (Some(NaN), NaN) ; il passe après correction
pour NaN, les deux infinis et un débordement du rapport à partir de nombres finis.
Les tests existants couvrent valeurs positives, négatives, séries constantes, plancher et
stabilité : leurs contrats restent inchangés. Suite complète : 108 tests réussis
(38 cœur + 70 harnais), deux ignorés. Compilation release réussie.

Rapport physics comparé au rapport final S50 : seules trois lignes de durées diffèrent.
Cinq familles C08 principales restent sans verdict ; diagnostic Ritter inchangé.
Sortie physics 1 attendue pour C04 ordre un. check : zéro échec, hashs
0x3e2c06a7b00e73e3 et 0x1a8b0629a9f51b6e inchangés. C22 à 102400 cellules non relancé :
son algèbre de rapport est couverte ici ; aucun montage numérique modifié.

S42-3 close comme inventaire exécuté avec correction du défaut reproduit ; S51-1 porte
les duplications restantes identifiées, sans prétendre les avoir supprimées.

**Suite exécutée S52.** Régression centrée partagée et projections examinées :
[MESURES-PARTAGEES-S52](MESURES-PARTAGEES-S52.md). S51-1 close ; admission des familles
C22 delta à tester avant extraction des projections (S52-1).
