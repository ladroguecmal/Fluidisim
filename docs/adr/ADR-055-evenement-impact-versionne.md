# ADR-055 — Le premier événement W est un impact versionné

- **Statut : ACTÉE**, S72, 2026-09-08, délégation technique de S71.
- **Applique** ADR-054 W1, I-03, I-06, I-08 et I-11.
- **Remplace pour la tranche Impact** la représentation proposée par SPEC-006 §3.1.
- **Implémentation** : `code/water-core/src/wave_event.rs`.

## Décision et domaine

Construire Impact avant les autres kinds. Énergie : énergie positive transférée aux ondes,
en joules, et non énergie totale de la cause. Le générateur physique devra établir ce transfert.
Longueur d’onde positive en mètres ; volume déplacé non négatif en litres, descriptif audio,
sans débit ni transfert de masse V. La direction est un angle en tours dans [0,1), accompagné
d’une anisotropie dans [0,1] ; zéro signifie isotrope et rend la direction canonique nulle.
Cette paramétrisation décrit une source, sans fixer encore sa fonction d’expansion en paquets.

Les coordonnées locales restent f32, chacune strictement entre -4096 et 4096 m, dans la cellule
et le référentiel désignés. L’hôte résoudra et vérifiera leur existence et leur relation ; le
codec ne peut pas le faire. Un half autour de 4096 m espace ses valeurs de plusieurs mètres,
alors que I-08 vise la précision f32 locale. Énergie et volume half plafonnent à 65504 dans
leur unité : les borner ainsi avant de connaître les grandes causes ferait saturer le contenu.
On conserve donc les f32 finis, sans écrêtage. Ces bornes sont de représentation, **pas une
preuve que la propagation acceptera tout ce domaine**. Le modèle devra refuser explicitement
ce qui sort de sa validité physique ou de ses ressources.

## Encodage V1 : 76 octets, little endian, sans padding

| Offset | Taille | Champ |
|---|---|---|
| 0 | 2 | version = 1 |
| 2 | 2 | longueur = 76 |
| 4 | 8 | id, entier u64 |
| 12 | 4 | frame_id |
| 16 | 8 | cell, seuls les 60 bits bas sont admis |
| 24 | 8 | naissance, microsecondes u64 |
| 32 | 8 | durée de présence demandée, microsecondes u64 positives |
| 40 | 12 | position locale, trois f32 |
| 52 | 4 | énergie J |
| 56 | 4 | longueur d’onde m |
| 60 | 4 | direction, tours |
| 64 | 4 | anisotropie |
| 68 | 4 | volume déplacé L |
| 72 | 2 | matériau |
| 74 | 1 | kind = 0, Impact uniquement |
| 75 | 1 | bit 0 : above_surface ; bits 1–2 : serveur=0, anticipation=1, local=2 |

Les autres bits, kinds, origines et versions sont refusés. Un retrait sera une commande du
journal W2 : le bit 3 historique est refusé, et ne doit pas être interprété comme un impact.
La durée est entière, sans passage du temps en flottant. Naissance + durée doit tenir en u64.
Elle n’autorise pas à supprimer un effet encore nécessaire au rejeu : W2 devra concilier
expiration de la source, durée de ses effets et rétention. Le codec ne prend aucune décision
sur une arrivée tardive ou sur la priorité d’un événement.

Les zéros signés sont normalisés. L’encodage canonique sert à comparer les doublons ; les octets
entrants peuvent être normalisés et doivent être réencodés avant comparaison canonique.
Tous les flottants non finis sont refusés. Pas d’allocation dans construction/encodage/décodage,
aucun cast mémoire de struct. Un vecteur d’octets indépendant fixe endian, offsets et taille.
Une nouvelle version sera nécessaire pour étendre ce contrat ; aucun format réseau préexistant
n’a été constaté, et aucune migration d’un tel format n’est prétendue effectuée.

## Autorité et identité

Le codec accepte une provenance locale pour le bus interne ; `decode_server` la refuse.
**Un bit ne prouve pas une origine.** L’hôte doit authentifier le canal serveur avant cet appel.
Aucun paquet client marqué serveur ne doit atteindre ce chemin. La création locale est possible
pour l’anticipation et le cosmétique, sans permission de réplication vers le monde.

Les id restent opaques ici. W2 doit gérer leur espace de noms : une prédiction cliente ne connaît
pas le prochain server_seq. Il faut une corrélation explicite avec la cause autoritaire, pas
une égalité supposée entre compteurs. Retraits, conflits de contenu et politique de rétention
restent des travaux ouverts nommés ; le codec ne les résout pas.

## Réception et limites

Le module constitue la tranche Impact de W1. W1 complet reste ouvert : les huit autres kinds,
la réconciliation et leur contrat ne sont pas implémentés. Aucun champ W n’est encore propagé.
Les tests exercent octets de référence, aller-retour, troncature, données surnuméraires, versions,
flags, non-finis, bornes, débordement du temps et refus de provenance locale sur le canal serveur.
Ils ne certifient ni l’authentification réseau, ni les valeurs physiques d’une cause, ni une
exécution interplateforme. Le coût brut à 20 événements/s serait 1520 octets/s, hors enveloppe
transport, contre les 1000 estimés avec les 50 octets précédents ; compression à mesurer ensuite.

> **Actualisation S84 — 2026-09-08.** ADR-066 remplace la borne numérique au TTL :
> l'horizon se mesure depuis la naissance, indépendamment de la durée source. Renouvellement
> à résolution inchangée testé jusqu'à 16 s ; aucune purge TTL, rétention générale encore ouverte.
