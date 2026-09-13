# S227 — Audit global et remise en ordre du travail

2026-09-13. Demande de l'utilisateur : intentions initiales, dérives, gestion, documentation,
zones d'ombre et correctifs. Base examinée : `dfd1507` (fin S226), copie principale propre,
trois copies propres au même commit, lignée B archivée, aucun distant. Audit transversal avec
inspection ciblée du code ; ce n'est pas une certification exhaustive de la physique.

## 1. Diagnostic

**L'ambition est conservée ; la livraison est déséquilibrée.** Les quatre couches répondent aux
intentions initiales, mais le dépôt construit surtout B/W et leurs instruments. La source place
la crédibilité perçue, les interactions et l'adaptation au budget au centre (§1, §7, §12, §17).
Le prochain progrès doit donc se constater dans un usage, une intégration ou une propriété
nécessaire de cet usage, et pas seulement dans un nouveau résultat de banc.

| intention initiale | preuve dans le dépôt à S226 | écart restant |
|---|---|---|
| Eau crédible en temps réel, physique utile (§1) | B/W visibles dans `viewer/`, comparaison au cœur ; cadence S225 | Un sillage prescrit, pas de scène représentative avec interactions libres ; budget eau non reçu |
| Grandes masses simplifiées, détail local (§2–5, §19) | B analytique, W propagatif ; décomposition ADR-001 cohérente avec cette intention | δ MAC x-z expérimental, couvercle imposé ; pas de δ général intégré |
| Domaines adaptatifs, prédiction, hors caméra (§6–9, §12) | ADR-006/012/013, contrats et pools | L'orchestrateur de domaines mobiles/fusionnés et la dégradation utile ne sont pas construits |
| Volumes finis, transferts et inondations (§2.2) | V ouvert S224, gravité dirigée S226, C12 reçu | A266, état restaurable, pompes/pluie/liquides et articulation V↔δ manquants |
| Gameplay commun, détails locaux libres (§15, §18) | I-04/10/11/15/17, événements et codecs B/W | I-03 reste à recevoir sur une seconde cible ; le GPU de l'afficheur est cosmétique |
| Courants, plages, bathymétrie (§10–11) | Spécifications et jalon J5 | Modèles exécutés à fond uniforme ; faibles profondeurs non linéaires sans oracle reçu (A234) |
| LOD distincts et coût maîtrisé (§16–17) | J1-bis énumère les techniques, levier temporel présent | Espace, LOD, visibilité et mutualisation restent à construire et à éprouver ensemble |

Le passage des trois régimes sources aux quatre couches est une décision motivée, pas une dérive
en soi. Le durcissement de l'autorité et de la persistance est traçable aux ADR et invariants.
La réduction de périmètre introduite en S201 a **déjà été corrigée par ADR-127** : ne pas la
réintroduire sous couvert d'efficacité. Ni la cible de 2 ms, ni le seuil B4 de 2 %, ni les cinq
arbitrages d'ADR-027 ne sont rouverts ici.

## 2. Les freins mesurés

### Documentation active devenue historique

Mesure des fichiers suivis à l'entrée, sans `target/`, cache ni copies de travail :

| fichier | lignes | octets UTF-8 |
|---|---:|---:|
| REPRISE | 3 273 | 265 261 avant prise du jeton |
| README | 956 | 80 686 |
| Index | 1 932 | 162 805 |
| Journal | 11 716 | 794 330 |
| Leçons | 5 152 | 346 466 |
| Questions ouvertes | 1 918 | 151 789 |

Les trois points d'entrée totalisent **6 161 lignes**. La consigne de lecture intégrale de
REPRISE oblige à absorber l'histoire avant de travailler. La taille totale n'est pas le défaut :
un journal peut être long. C'est l'histoire **dans le chemin obligatoire**, et son état répété,
qui coûtent. Les références scientifiques, ADR et preuves ne sont pas à supprimer.

Contradictions actuelles vérifiées : REPRISE annonce encore un hôte à autoriser après son
autorisation S210/S211 ; sa table S208 dit V absent ; la table de la file active dit aussi
« aucun module » après les ajouts S224/S226. La feuille de route dit A254 close puis la donne
encore à faire ; elle conditionne encore la mutualisation à A255 alors que S222 a levé ce
préalable. Une date historique ne suffit pas si la phrase garde la fonction de consigne.

### Un indicateur d'activité pris pour un indicateur d'avancement

`outils/velocite.sh` compte les ajouts dans le harnais comme « bibliothèque / part système »,
ignore les sous-répertoires de `src`, ignore l'afficheur dans cette part, arrête ses ères à S199
et cherche chaque lien de suite dans **tout le journal**, pas dans l'entrée correspondante.
Une modification de commentaire suffit aussi à changer sa « dernière avancée ».
Il peut mesurer l'activité Git ; il ne peut pas décider si une couche devient utilisable.

Recalcul par `git log --first-parent --numstat` sur master, ajouts bruts (tests et commentaires
inclus dans chaque chemin, **ni temps de travail ni productivité**) :

| sessions | cœur src | exemples | afficheur src | Markdown |
|---|---:|---:|---:|---:|
| S199–S208 | 1 630 | 2 571 | 0 | 4 215 |
| S209–S218 | 1 295 | 1 670 | 2 057 | 5 057 |
| S219–S226 | 2 242 | 1 057 | 149 | 4 495 |

La construction a repris : le diagnostic « zéro système » de S198 ne décrit plus le présent.
Mais la remise à zéro des maillons par toute modification de `src` autorise encore une longue
série de recherches locales. S218–S221 construisent des bornes dont S222 mesure un coût de
25 s ; leur existence ne signifie pas leur intégration à l'admission ou au rendu.

### Une méthode qui incite à ouvrir des sujets

METHODE disait « si aucune impasse n'apparaît [...] je n'ai pas assez cherché » et « un ADR sans
questions résiduelles est suspect ». Ces prescriptions rendent un résultat sans nouvelle dette
suspect par principe. Elles doivent céder la place à une question décisionnelle, un critère
d'arrêt et l'autorisation de conclure **aucune anomalie trouvée**.

La rigueur utile reste : critère avant mesure, référence indépendante, domaine et unités,
contre-épreuve quand elle discrimine réellement. Une mesure nécessaire à un lot garde sa place ;
une campagne sans décision aval nommée attend. Un hash déplacé impose d'expliquer le déplacement,
pas d'interdire une correction : A213 et les restrictions d'ADR-090 montrent ce risque.

## 3. Code : revue et cibles de correction

La suite release de base passe : **383 tests réussis, 5 ignorés**, en mode hors réseau.
Cela vérifie les assertions présentes, pas les propriétés absentes de ces assertions.

- **Confluence V** : le limiteur d'arrivée lit la place libre initiale par arête. Plusieurs
  arêtes peuvent chacune la consommer ; la normalisation amont ne protège pas le receveur.
  Cible : majorer la **somme entrante** par la place libre, conserver exactement la masse et les
  proportions à l'arrondi entier près. Aucun seuil physique nouveau.
- **Coordonnées V** : `sub` soustrait deux `i64` avant conversion. Aux valeurs extrêmes de l'API,
  le pas peut paniquer ; le profil release active lui aussi les contrôles de débordement.
  Cible : différence entière élargie, même résultat sur le domaine ordinaire.
- **A266** : la pente correcte n'établit pas le volume correct. Le diagnostic géométrique S226
  impose une cote verticale centrale ; le module lit une distance normale au plan. Il faut
  éprouver aussi cette différence avant d'affirmer que les prismes inclinés sont exacts.
- **δ** : le budget mesuré n'est toujours pas un budget respecté (I-05), la pression est f64
  expérimentale, les faces coupées échouent au filtre spatial. Ce sont des travaux de construction
  de J2, pas des optimisations cosmétiques. Le module annonce honnêtement ses limites.
- **Rendu** : le produit sommets × modes domine le sillage. Priorité à une stratégie spatiale/LOD
  confrontée au chemin complet, avec frontières et retour en visibilité ; pas à une micro-mesure
  supplémentaire de préparation. A265 ne bloque que les conclusions dépendant du recouvrement.

## 4. Changements et vérification

À compléter après application P3–P5. Les anciens points d'entrée restent consultables dans Git
au commit `dfd1507`, et les preuves dans les ADR, documents de validation et le journal.
Le nettoyage des pages actives ne réécrit ni un ADR ni une source initiale.

## 5. Suite priorisée

À fixer après reproduction des défauts. A266, V restaurable et les contrats de J2 restent des
lots de construction ; les raffinements de bornes non intégrées ne prennent pas leur place.
