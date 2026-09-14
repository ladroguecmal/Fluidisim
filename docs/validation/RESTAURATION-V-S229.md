# Restauration du réseau hydraulique V — S229, 2026-09-14

**Capacité** : reprendre le graphe du noyau V depuis une charge utile versionnée, puis continuer
le pas réel à l'identique. Contrat : [ADR-140](../adr/ADR-140-restauration-du-graphe-V.md).
Implémentation : `hydro_network::snapshot` dans `code/water-core/src/hydro_snapshot.rs`.

## Consommation

1. Construire les formes, les nœuds et les ouvertures d'auteur, puis `Baseline::new` avec
   l'identité du graphe/référentiel et la révision des assets. Conserver cette base immuable.
2. Faire évoluer des copies des nœuds et ouvertures par `hydro_network::step`.
3. Entre deux pas, dimensionner le tampon par `snapshot_len`, puis appeler `snapshot_into` avec
   l'instant courant, le prochain `dt` et `g_eff`. Réutiliser un tampon réservé selon le profil.
4. Charger la même base, appeler `restore_into` sur des pools fournis par l'hôte, reprendre
   depuis le `Context` retourné et les mêmes entrées futures. Aucun pas implicite ni plan stocké.

La restauration ne dépend ni des anciennes valeurs de destination ni du tampon après le retour.
Les emplacements sans écart sont remis à l'état d'auteur, restes compris. Les suffixes des pools
et du tampon de sortie restent intacts. Une modification de base nécessite une migration
explicite ; le format refuse de réinterpréter les indices ou de tronquer un volume trop grand.

## Preuves reçues

| montage | observation et assertion |
|---|---|
| Deux contenants tabulés, ouverture de 1 mm² ; capture après 3 pas | 1 000 pas supplémentaires identiques, volumes et restes entiers, horloge et masse exacte. Le témoin qui oublie les restes diverge sur les volumes : le test distingue réellement cette omission |
| Trois contenants polyédriques de 8 m³, 4 ouvertures, orifice et déversoir, rejet extérieur | Capture après 3 pas ; 200 pas sous 5 directions de gravité, 3 restaurations au total. À chaque pas : mêmes transferts, volumes, restes, bits des plans et octets de capture |
| Même montage orienté | Capture initiale de **148 octets**, **3 492 ml** rejetés après capture, masse restante + rejets exactement égale à la masse capturée |
| Destination sale, nœud sans écart et reste nul | Reconstruction complète depuis la base, aucune contamination des emplacements omis ; contexte conserve les bits de gravité, y compris zéro signé, ainsi que `u64::MAX` comme instant |
| Toutes les troncatures et altération de chaque octet d'une capture | Refus sans mutation ; octet supplémentaire refusé aussi |
| Erreurs sémantiques, contrôle d'intégrité recalculé | Indices hors limite/doublons, volume négatif/excessif/redondant, reste nul ou ≥1 ml, comptes impossibles, gravité et durée invalides refusés |
| Base incompatible | Identité, révision, valeur d'auteur, capacité, origine, destination, coefficient, loi, position d'ouverture, entrée tabulée et géométrie de même volume mais translatée : refus |
| Compteur du tas avec témoin positif | **Zéro allocation** lors de la capture, restauration, continuation orientée et refus testés ; les buffers sont réservés auparavant |

Le premier montage de rejet utilisait un receveur initialement vide et un orifice de 1 mm² :
aucun millilitre ne sortait pendant la fenêtre. Le témoin a été corrigé pour exercer une vraie
sortie (250 000 ml initiaux et 1 000 mm²). **Aucun code de production ni seuil modifié pour ce
résultat.** Les assertions de continuation étaient déjà satisfaites.

## Format et limites

WVST V1 écrit `88 + 12 × (nœuds modifiés + restes non nuls)` octets. Pas de copie des assets :
l'empreinte couvre leur contenu ordonné, recalculé à la construction de la base. FNV-1a 64 bits
est un diagnostic d'incompatibilité/intégrité accidentelle, **pas une authentification ni une
garantie sans collision**. Un transport hostile demande son propre mécanisme d'autorité.

La branche **V de C19** est reçue localement sur ces montages. Ce résultat ne reçoit pas le
montage B/W/V complet, le protocole réseau, la durabilité disque, la migration de topologie,
les nœuds dynamiques, `liquid_id`/flags, le couplage V↔δ ni une seconde cible I-03. Les données
futures (gravité et commandes) restent de la responsabilité de l'hôte. Aucun budget temporel
du codec ou du pas n'est déduit de l'absence d'allocation.

Tests : `tests_hydro_snapshot.rs` (cinq tests) et `tests/hydro_geometry_runtime.rs` (réception
orientée S229 avec compteur). Commandes depuis `code/` :

```text
cargo test -p water-core --offline hydro_network::snapshot
cargo test -p water-core --offline --test hydro_geometry_runtime c19_oriented -- --nocapture
cargo test --workspace --release --offline --quiet
```

Suite finale release hors réseau : **404 réussis, 5 ignorés**, aucun échec, six tests de plus que S228. Avertissements préexistants du harnais/exemples uniquement. Quatre tests de l’inventaire passent ; navigation active sans erreur. Validation sur Rust 1.97 local ; Rust 1.75 déclaré conservé, sans réception de cette chaîne ancienne.
