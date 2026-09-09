# S115 — Construction de la requête B+impacts+pressions

2026-09-09. [ADR-077](../adr/ADR-077-requete-mixte-impacts-et-pressions.md), S114-1 réalisée.

## Réception de l'assemblage

Fond réel16 composantes, Hs0,1 m, Tp6 s, direction0,125 tour, graine42,
ancre(10⁹,-10⁹,0) m. Repère7/cellule9, g9,81f32, densité1025.
Un impact radial de0,01 J, longueur d'onde4 m, naissance0, TTL4 s,
profondeur20 m, rayon16 m et horizon4 s,64 modes. Deux pressions du montage
S112/S113, recette16×24, instant1,5 s, rectangle[-8,12]².
La recette de pression est un témoin rapide de contrat, **non reçu spatialement**.

Trois tests couvrent :

- Trois points(1,0),(2,1),(-3,2) où impact et pression sont effectivement non nuls.
  Élévation et vitesses comparées à la somme en f64 des contributions individuelles,
  normale comparée à celle reconstruite depuis la pente totale en f64 : seuil1e-7
  de fixture, cohérent avec les comparaisons d'assemblage S106. Aération conservée,
  dérivée temporelle égale à la vitesse verticale, enveloppe totale vérifiée.
  Le suffixe de sortie reste intact.
- Réduction sans pression identique en bits au chemin B+impacts ; réduction sans
  impacts identique en bits au chemin B+pression, sur deux points. Tous les dix
  champs de sortie sont comparés, pas seulement l'élévation ou une empreinte.
- Dernier point hors rectangle de pression mais dans le rayon d'impact ; dernier
  point dans le rectangle mais hors rayon ; point monde non représentable.
  Chaque refus porte l'indice1 et laisse la sortie intacte. Mauvais repère,
  densité différente, mauvais instant même à lot vide, impact expiré même à lot vide,
  brouillon trop court et enveloppe totale excessive sont refusés. Le témoin nominal
  reste accepté après les refus.

Le test de pente utilise une limite qui accepte chacune des deux contributions
séparées, mais refuse leur somme. Il couvre donc le contrôle **total**, et pas
seulement la présence d'un garde-fou déjà exercé dans chaque chemin.

L'API est `prepared_water::mixed::sample_world_batch`. Les pools de vues préparées
restent protégés par emprunt ; le brouillon peut être modifié lors d'un refus tardif,
la sortie est transactionnelle. Aucun nouveau tableau alloué par ce chemin.

## Vérification et limites

Suite complète :150 core +93 harnais =243 tests réussis, cinq ignorés ; quatre avertissements préexistants. Les trois tests ciblés passent aussi en release.
Les chemins historiques ne sont pas réécrits. La seule donnée supplémentaire dans
la vue d'impacts est la densité ; elle est transmise par ses trois constructeurs.
La réception ci-dessus vérifie la composition et ses refus, pas la précision physique
du montage mixte, sa conformité entre plateformes ou son coût de production.
Elle ne crée pas de bilan énergétique mixte. Enveloppe f32 non formelle.

**S115-1, S116 :** recevoir un scénario B+impacts+pressions aux recettes224×128 et256×128,
sur les instants et domaines communs ; comparer la composition à une référence
indépendante des opérations d'assemblage et mesurer préparation/requête complète.
Conserver les refus d'expiration et de domaines, sans tronquer les composantes.
Renouvellement pression, durabilité et bilan énergétique mixte restent ouverts.
77 ADR,193 angles,17 invariants,6 spécifications,23 cas.

**Correction S116, 2026-09-09 :** les impacts radiaux sont profonds, comme la pression,
et non à dispersion de profondeur finie (description erronée dans ADR-077 et le
journal S115). La profondeur20 m sert de garde sur la bande de l'impact. Voir la
note corrective ADR-077 et [RECEPTION-MIXTE-S116](RECEPTION-MIXTE-S116.md).
