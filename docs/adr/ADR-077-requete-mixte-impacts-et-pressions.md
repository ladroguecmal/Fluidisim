# ADR-077 — Requête commune aux impacts et pressions

- **Statut : actée**, autonomie technique S71, construction S115.
- **Prolonge :** ADR-062/063, ADR-071/073 et les enveloppes S105/S106.
- **Résout :** S114-1, assemblage du champ échantillonné ; aucun format remplacé.

## Problème

Deux chemins produisent chacun B+W : impacts radiaux et pression mobile. Additionner
leurs `WaterSample` compterait B deux fois ; additionner les normales serait faux.
Une requête doit porter un seul point, un seul instant et un seul résultat publié.

## Décision

`prepared_water::mixed::sample_world_batch` reçoit un fond lié au repère/cellule,
une vue préparée du journal d'impacts et une éventuelle vue préparée de pression.
Un journal d'impacts vide signifie absence d'impacts ; `None` signifie absence
**déclarée** de pression. Un échec de préparation ou une attente ne devient pas `None`.
L'hôte doit conserver le refus ; les vues existantes continuent de protéger les
journaux et pools empruntés contre la mutation pendant la requête.

Avant de traiter les points, la requête contrôle frame, cellule, gravité, densité,
instant, expiration des impacts, perte connue, pente maximale et capacités.
Gravité et densité doivent coïncider en bits entre impacts et pression. Le fond
actuel impose toujours g=9,81f32. La densité est désormais conservée dans la vue
d'impacts, y compris celles construites par les contrôleurs existants.
Les contrôles globaux valent également pour un lot vide.

Pour chaque point :

1. Conversion monde/local une fois, puis évaluation de B à cet instant.
2. Extraction de la pente de B depuis sa normale ; initialisation de son enveloppe.
3. Accumulation des impacts confirmés en ordre du journal, puis du champ de pression
   déjà superposé. Élévation, dérivée temporelle, vitesses et pentes sont additives.
4. Addition des enveloppes de pente, contrôle de la limite totale, puis **une seule
   normalisation** de `(-slope_x,-slope_y,1)`. Aération de B conservée.
5. Contrôle de finitude et écriture dans le brouillon. Copie vers la sortie uniquement
   après réussite du lot complet ; les suffixes des deux tableaux ne sont pas touchés.

Les domaines de validité restent ceux des composantes : leur intersection détermine
les points recevables. Une composante hors domaine ou expirée refuse la requête ;
elle ne contribue pas silencieusement zéro. Le champ de pression est lié à son
instant exact. La requête ne prépare, ne renouvelle et ne sérialise aucun champ.

## Portée physique et numérique

Cette construction est la superposition linéaire déjà retenue pour B+W. Elle ne
modifie pas les modèles : impacts à dispersion de profondeur finie, pression en
eau profonde. La compatibilité frame/g/rho ne certifie pas leur approximation
physique commune dans un milieu donné. La réception physique d'un montage mixte
et son coût restent à mesurer après cette construction.

L'enveloppe totale est une majoration algébrique calculée en f32, pas une borne
formelle d'arrondi ni un certificat de résolution continue. Le potentiel de pression
reste accessible sur son champ ; `WaterSample` n'acquiert pas ce champ supplémentaire.
**Aucune énergie ni puissance globale mixte n'est publiée** : additionner les bilans
séparés omettrait les interférences entre impacts et pression (L203).

Les chemins antérieurs restent disponibles. Sur les témoins S115, les réductions
sans pression et sans impacts leur sont identiques en bits. Aucun nouveau format,
coefficient de calibration ou budget de temps n'est décidé ici.

## Réception et suite

Voir [MIXTE-S115](../validation/MIXTE-S115.md) : contributions séparées puis normale
de référence en f64, réductions aux chemins existants, refus tardifs et enveloppe
totale, suite de tests complète. Cette référence vérifie l'assemblage, pas le modèle
de chaque source. Suite S115-1 : campagne mixte aux recettes reçues S113 et coût complet.
