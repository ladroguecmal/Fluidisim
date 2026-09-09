# S105 — Contexte et publication du candidat pression

2026-09-09. Suite de S104-1. Adaptateur `bound_pressure`, sans changement de modèle ni
nouvel ADR : règles de préparation empruntée S80/S93, contexte S83 et publication S101.

## Contrat construit

`Context::new(Settings, HalfSpectrum)` conserve la recette issue du demi-spectre opaque.
Le demi-spectre conserve maintenant cette recette en plus du hash source. Les réglages
contiennent le référentiel, la cellule, la gravité, la densité, les deux coins locaux et
la fenêtre temporelle. Gravité/densité finies positives, rectangle ordonné strictement
à l'intérieur de ±4096 m, durée positive au plus16 s : limites du candidat S95.
Les contrôles de recette viennent de la cuisson S97/S99, sans copie de ses constantes.

L'identité compare tous les paramètres, dont les bits des flottants et les deux
résolutions. Aucun hash seul ne décide de la compatibilité. Deux contextes reconstruits
avec les mêmes données sont compatibles ; un changement de signe du zéro flottant
peut entraîner un refus conservateur. L'identité de recette suppose la version V1
unique du binaire actuel ; aucun format persistant de contexte n'est défini ici.

`Prepared::build` vérifie la recette, la date demandée et la présence des segments dans
la fenêtre, puis délègue au calcul spectral existant. Contiguïté, durées, positions,
pressions et finitude continuent d'être vérifiées par celui-ci. Un segment futur
invalide reste refusé. Une trajectoire vide ou une date avant sa première naissance
reste refusée par le calcul spectral. Aucune phase n'est redémarrée pour entrer dans
la fenêtre : les dates d'origine sont conservées.

**Le champ préparé est un instantané.** `sample_batch` exige le contexte complet et
exactement l'instant préparé, y compris pour un lot vide. L'horizon borne les dates
auxquelles on peut reconstruire, pas celles auxquelles les coefficients déjà calculés
peuvent être relus. Énergie et puissance portent le même instant, exposé par `time()`.

## Publication et mémoire

L'hôte prépare dans un pool candidat distinct du pool de la vue active. Le résultat
`Ok(Prepared)` est le point où il peut remplacer cette vue. Au refus, la vue précédente
reste empruntée et inchangée ; le candidat peut contenir des écritures partielles.
Rust interdit la mutation du pool publié tant que sa vue l'emprunte. Le candidat peut
être réutilisé après abandon du résultat refusé. Pas de copie de secours cachée.

Le lot conserve le contrat S101 : scratch modifiable, sortie publiée seulement au
succès complet, préfixe de longueur des points, reste de sortie inchangé. Un refus de
contexte ou de temps arrive avant toute écriture, même pour un lot vide.

Aucune allocation ajoutée sur les chemins de construction/requête par inspection ;
pools de nœuds et slots inchangés, métadonnées de recette/contexte supplémentaires.
Pas de mesure de performance ni de compteur d'allocation dans cette session.
Il s'agit d'une vue empruntée et d'un protocole de publication hôte, pas encore d'un
contrôleur cyclique gérant seul les deux pools, une file de commandes ou leur sauvegarde.

## Réception

Quatre tests d'adaptateur sur petite recette8×8 (elle teste le contrat, pas la précision) :

- Changer séparément référentiel, cellule, gravité, densité, chacun des quatre bords,
  début, fin, sigma, coupure, résolution radiale et angulaire entraîne un refus ; témoin
  compatible accepté. Recette incompatible refusée aussi à la préparation.
- Onze réglages invalides, dates extrêmes, débordement de durée de source, pression
  future NaN et requête décalée de1 µs : refus explicites. Lot vide compatible accepté.
- Deux pools : pression1e30 Pa provoquant un refus numérique après préparation du
  candidat ; sortie et énergie actives conservées. Dernier point NaN ou scratch trop
  petit : sortie complète inchangée. Nouvelle préparation valide dans le pool refusé
  publiée à3 s, résultat distinct de celui à1 s ; les queues des sorties restent intactes.
- Enveloppe contre appel spectral direct : sept grandeurs, énergie et puissance à
  bits identiques. Même réception à époque0 et près de u64::MAX : identité conservée.

Suite debug complète :129 core +93 harnais =222 réussis, cinq ignorés ; quatre
avertissements préexistants. Les quatre tests de cet adaptateur passent aussi en release.
Ces tests n'étendent pas la réception spatiale S103 ou énergétique S104 à d'autres
configurations. Les calculs physiques existants ne sont pas modifiés.

## Ce qui reste à construire

Le référentiel et la cellule sont une association déclarée par l'hôte. L'adaptateur
ne prouve ni l'origine des coordonnées de la trajectoire ni l'uniformité du milieu.
Pas de profondeur finie, rotation de repère, bathymétrie ou fond B ajouté implicitement.
I-03/I-15 restent à recevoir entre plateformes avant autorité ; I-05 n'est pas certifié.
I-06/I-07/I-08 sont conservés dans ce périmètre, sans modification des invariants.

**S105-1, S106 :** raccorder la pression candidate à une requête monde commune avec B,
avec conversion unique des points, instant commun, contrôle de contexte et publication
atomique du lot composé. Recevoir toutes les composantes de vitesse et la normale,
et définir le contrôle de pente sans confondre mesure ponctuelle et borne du champ.
Le codec des causes de pression, LiveWater et la sauvegarde des sillages restent ouverts.
73 ADR,193 angles morts,17 invariants,6 spécifications,23 cas inchangés.
