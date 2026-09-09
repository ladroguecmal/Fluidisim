# S114 — Reprise multisource jusqu'à B+pression

2026-09-09. S113-1 réalisée sur le montage S112/S113, sans modification de bibliothèque.

## Cycle reçu

`restart_multisource` utilise les deux sources de [S113](RECEPTION-MULTISOURCE-S113.md) :
10 Pa, départ(0,0), vitesse(2,0), active0–2 s ; 7 Pa, départ(1,1), vitesse(0,2),
active0,5–2,5 s. Même sigma1 m, coupure6 rad/m, gravité9,81f32, densité1025,
frame7/cell9, rectangle[-8,12]² et horizon0–8 s. Recettes224×128 et256×128.

B possède16 composantes, Hs0,1 m, Tp6 s, direction0,125 tour, graine42 et ancre
monde(10⁹,-10⁹,0) m. Le lot64 est le préfixe de la grille11² de pas2 m de S113.
Le seuil de pente0,1 est le seuil de fixture S107, pas une calibration de production.

1. Deux sources immuables sont encodées en WPRS156 octets chacune, puis décodées
   vers des pools de segments distincts.
2. Le journal reçoit id2, puis id1 sur un pool de capacité1 : id2 est publié,
   id1 reste en attente. L'instantané WPJR fait344 octets (32+156+156).
3. Restaurer avec une seule place conserve l'attente ; `retry` rend `Full`.
   Restaurer avec deux places conserve également l'attente : aucune admission implicite.
4. Avant `retry`, préparer le champ rend `Pending`. Après `retry`, les deux sources
   sont publiées en ordre1,2 ; un second `retry` rend `Unchanged`.
5. La cuisson est issue de la recette restaurée. `Prepared::from_journal` construit
   le champ commun ; la requête monde compose B et les deux réponses de pression.

Les instantanés avant admission sont identiques en octets après restauration, y
compris sur pool encore plein. Après admission, l'instantané est identique à celui
d'un journal construit directement depuis les sources initiales, admises2 puis1.
Les contenus complets des sources sont aussi comparés, sans se limiter à leurs ids.
Les pools restaurés sont effectivement utilisés pour calculer les réponses.

## Résultats et refus

Aux23 instants de S113 (pas0,5 s sur0–8 s, plus ±1 µs autour des trois transitions),
les dix sorties de `WaterSample` sont finies et **identiques en bits** à la voie
directe. Énergie, puissance et enveloppe de pente sont également identiques en bits.
1472 points-temps par recette,2944 au total. Il s'agit d'une réception de reprise ;
la référence physique f64 reste celle de S113 et la composition B celle de S106/S107.

| Recette |Hash du lot à1,5 s|Hash des23 lots et de E/P|
|---|---|---|
|224×128|`d8f99fd1d5f9fc81`|`aecb2bca86fcc3b2`|
|256×128|`038be9a8c899ac1d`|`c53084a9dfbaf68b`|

Les assertions comparent tous les bits, pas seulement ces empreintes de diagnostic.
Le hash global parcourt les instants croissants, les64 points, les dix composantes
dans l'ordre de `values`, puis énergie et puissance de cet instant.

Refus exercés aux deux recettes :

- Chacune des344 troncatures, corruption du dernier octet réservé de la seconde
  source, époque différente, pool de sources vide, pool de segments trop court.
  Les emplacements candidats préremplis et les segments sentinelles restent intacts.
- Source restaurée avec un frame individuellement valide mais incompatible avec
  le champ commun : restauration et admission réussissent, préparation rend `Context`.
- Pression finie maximale f32 dans la source restaurée : format accepté,
  préparation numérique refusée. Le pool candidat peut être modifié ; aucune vue
  partielle n'est publiée et le champ actif sur son autre pool reste utilisable.
- Dernier point monde invalide, instant de requête décalé de1 µs, seuil de pente1e-8
  insuffisant : sortie du lot inchangée. Un appel nominal après les refus reproduit
  exactement le lot actif. Le seuil1e-8 est uniquement un témoin de refus.

## Coût local

Release, même machine locale que S113, trois échauffements puis21 mesures successives
par opération, à1,5 s. Les codecs très courts sont mesurés par lots de1000 opérations :
leurs nombres sont des moyennes de lots, pas des quantiles de latence individuelle.

| Opération |224×128, médiane|256×128, médiane|
|---|---:|---:|
|Sauvegarde WPJR|0,0836 µs|0,1018 µs|
|Restauration WPJR, attente conservée|0,6514 µs|0,4909 µs|
|Préparation du champ des deux sources|12,8233 ms|14,4900 ms|
|Restauration → retry → préparation → requête B+pression64|48,5010 ms|55,4706 ms|
|Deux WPRS → décodage → admission saturée → WPJR → restauration → retry → préparation → requête|47,9483 ms|55,5501 ms|

Cycle de reprise : min/max47,1346/50,2276 ms et53,7642/59,3182 ms.
Cycle depuis WPRS : min/max47,0057/54,8923 ms et54,1046/58,5113 ms.
Séries successives : le cycle plus long apparaît parfois plus rapide par variabilité
locale ; ne pas soustraire ces médianes pour estimer le coût des codecs.
Les sorties finales des deux cycles chronométrés sont comparées en bits au témoin
à1,5 s, en dehors de la mesure.

La configuration de B, la construction des sources initiales, la cuisson et les
allocations des pools sont hors chronométrie. Les étapes de décodage, restauration,
admission, préparation et requête indiquées sont incluses. Pas de lecture disque,
de réception réseau ni d'authentification dans ce scénario. L'allocateur hôte scellé
n'observe aucun refus ; il n'est pas un compteur global d'allocations Rust.
Ces coûts ne certifient aucun budget de jeu.

```text
cargo run --release --manifest-path code/Cargo.toml -p water-core --example restart_multisource
```

Campagne finale avec assertions réussie ; formatage vérifié. La bibliothèque est
inchangée : suite240 réussis/cinq ignorés exécutée en S112, non relancée en S114.

## Suite de construction

**S114-1, S115 :** construire une requête commune B+impacts+pressions dans un même
repère, au même instant : contribution B unique, somme des pentes avant normale,
enveloppe totale et refus transactionnel dès qu'une composante est indisponible.
Recevoir un montage mixte, ses réductions aux chemins existants et ses refus.
Aujourd'hui les deux familles de W ont chacune leur chemin B+W ; le scénario présent
ne les réunit pas. B est reconfiguré par l'hôte, pas sauvegardé dans WPJR.
Renouvellement du champ de pression, durabilité et conformité interplateforme restent ouverts.
76 ADR,193 angles,17 invariants,6 spécifications,23 cas inchangés.
