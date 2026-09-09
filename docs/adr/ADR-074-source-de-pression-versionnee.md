# ADR-074 — Source candidate de pression versionnée

- **Statut : ACTÉE**, S108, 2026-09-09, délégation technique.
- Complète ADR-055/069/071/072 ; ne modifie ni Impact V1, ni WJNL, ni WLIV.

## Décision

Construire `pressure_source::Source`, vue immuable sur une trajectoire hôte validée,
avec identifiant, époque et cause gameplay (entité, commande, ordinal), contexte local
et recette gaussienne V1. Construire son codec séparé **WPRS V1**, little-endian,
116 octets d'en-tête puis40 par segment. Le virage à deux segments occupe196 octets.
Les coefficients de champ ne sont pas enregistrés : le champ est reconstruit depuis
les paramètres, la trajectoire et les dates d'origine. I-08 et I-17 sont conservés.

WPRS représente les entrées d'une tranche candidate reçue, avec sa fenêtre et sa
résolution ; ce n'est pas encore un événement de pression admis par le journal général.
La fenêtre start/end borne la reconstruction, ne devient ni TTL physique ni règle de
purge. Une autre recette ou fenêtre change le descripteur ; aucune mise à jour implicite
d'une identité existante n'est autorisée par ce codec. Réconciliation à construire.

Aucun drapeau serveur/client n'est inventé : les octets ne prouvent pas leur autorité.
L'hôte devra authentifier la provenance et vérifier l'époque lors de l'admission.
Époque et cause sont transportées, sans contrôle contre un serveur courant dans le
codec. L'id ne vaut pas ordre serveur ; aucun numéro de séquence n'est fabriqué.

## Format V1

| Offset | Octets | Champ |
|---|---:|---|
|0|4|ASCII WPRS|
|4|2|version1|
|6|2|réservé, zéro exigé|
|8|4|taille totale u32|
|12|4|nombre de segments u32, non nul|
|16|8|époque|
|24|8|identifiant|
|32|8|cause : entité|
|40|8|cause : commande|
|48|4|cause : ordinal|
|52|4|référentiel|
|56|8|cellule,60 bits utiles comme Impact|
|64,68|4 chacun|gravité, densité f32|
|72,76,80,84|4 chacun|min x/y, max x/y locaux f32|
|88,96|8 chacun|début/fin de fenêtre, SimTime en µs|
|104,108|4 chacun|sigma, coupure f32|
|112,114|2 chacun|résolutions radiale et angulaire|

Chaque segment : naissance u64 à0, durée u64 à8, origine x/y f32 à16/20,
vitesse x/y f32 à24/28, pression signée f32 à32, quatre octets réservés zéro à36.
Pression positive vers le bas. Les bits f32, y compris -0, sont conservés ; aucune
normalisation silencieuse des paramètres n'est ajoutée. Les noms de magic/version
et tailles désignent le format ; le hash de recette reste un diagnostic extérieur.

## Validation et publication

Même validation des métadonnées au constructeur et au décodage : recette réutilisant
le validateur de cuisson existant, contexte réutilisant celui de S105, cellule bornée.
Trajectoire non vide, durées positives dans la fenêtre ≤16 s, temps sans débordement,
positions et extrémités strictement dans ±4096 m, vitesses et pressions finies,
contiguïté temporelle et spatiale exacte suivant scale_integer de S95. Les pressions
nulles ou négatives sont permises par le modèle, aucune calibration de coque ajoutée.

Le codec contrôle la taille exacte : troncature, suffixe, version ou réservés inconnus
refusés. Taille calculée sans débordement, limitée à u32 par le format ; capacité réelle
limitée par les pools hôte. Aucun maximum arbitraire de segments ajouté. Le transport
devra borner ses messages avant allocation ; ce module n'alloue pas de tampon réseau.

Décodage en deux passages : contrôler tous les segments avant d'écrire le premier.
Le pool reste intégralement inchangé au refus, queue préservée au succès. Encodage dans
un préfixe après contrôle de capacité ; sortie intacte au refus. Les emprunts Rust
protègent les segments d'une source publiée. Pas d'allocation dans ces opérations.

Une source représentable n'est pas une promesse de préparation finie : une pression
finie immense peut faire déborder l'énergie ensuite. Les refus numériques du candidat
restent obligatoires. Le validateur ne prouve ni la précision ni le milieu réel.

## Réception et point de rupture

Trois tests : aller-retour binaire exact avec reconstruction des sept grandeurs et
bilans à bits identiques, époque proche de u64::MAX ; toutes les troncatures du virage,
suffixe, réservés, version, compte extrême, capacités et NaN dont le dernier segment,
pool intact ; trajectoires discontinues, débordements, contexte invalide refusés.
Témoins pression signée/nulle et très grande finie acceptés selon leur contrat.
La bibliothèque est testée entièrement et les tests ciblés aussi en release.

La première réception emploie8×8 pour tester le codec, pas une nouvelle résolution
physique. Aucun dépôt durable, transport, journal multi-types, admission autoritaire
ni sauvegarde de service de pression n'est livré ici. **S108-1, S109** : admission
bornée et idempotente des sources, époque/cause/identifiant et conflits explicites,
conservation au refus, avant extension des sauvegardes. Les formats actuels continuent
de refuser les kinds inconnus ; ne jamais leur faire ignorer silencieusement WPRS.
