# S117 — Publication temporelle de la pression sur deux pools

2026-09-09. [ADR-078](../adr/ADR-078-controleur-de-publication-pression.md), S116-1 réalisée.

## Construction et réception

Le contrôleur expose `new`, `update`, `published_time`, `state` et `current`.
Il conserve le contexte, le spectre et le journal immuables, les deux pools et les
métadonnées du dernier champ validé. Il échange les pools après succès, sans recopier
les coefficients ; une vue courante empêche la mutation du contrôleur par emprunt.

Trois nouveaux tests du montage S112, recette16×24 (192 modes réduits), vérifient :

- Publication à0,499999,500000,500001,1500000,2000000,2500000,8000000 µs, puis retour
  à0 et1500000 µs. Quatre positions par date. Les sept champs, énergie, puissance et
  enveloppe sont identiques en bits à une préparation directe du même journal.
  Les deux pools alternent ; la demande répétée au même instant rend `Unchanged`.
  Une vue demandée à un instant non publié rend `Time`, avec état `NeedsUpdate`.
- Une pression de1e30 Pa, finie et représentable à la construction, produit un champ
  nul à0 puis déborde numériquement à1,5 et2 s. Les deux tentatives sont refusées ;
  la publication à0 garde son énergie et son champ nuls. Une demande aux instants
  refusés ne récupère pas ce champ sous une date incorrecte. Cette pression est un
  témoin de refus numérique, sans interprétation physique.
- Pool actif trop court, pool de réserve trop court et journal bloqué : refus.
  Le constructeur nominal fonctionne ensuite sur les pools suffisants.
  Hors fenêtre et date extrême sont distingués ; un refus ne change pas la date publiée.

Les trois tests mixtes S115 passent désormais par le contrôleur : construction à0,
mise à jour à1,5 s, tentative hors fenêtre refusée, puis requête B+impact+pressions
sur la publication à1,5 s. Les références de somme, réductions en bits et refus tardifs
de S115 sont conservés ; ce n'est pas un nouveau relâchement de leurs critères.

## Vérification

Suite complète :153 core +93 harnais =246 tests réussis, cinq ignorés ; quatre avertissements préexistants. Les trois nouveaux tests du contrôleur et les trois tests mixtes passent aussi en release (le filtre controller exécute également quatre tests historiques).
La recette16×24 reçoit le mécanisme de publication ; sa précision spatiale n'est
pas revendiquée. Aucun changement d'ordre de sommation ni de formule du noyau.
Les métadonnées spectrales sont séparées des slices uniquement pour le stockage du
contrôleur ; aucun format WPRS/WPJR ni structure de source n'est modifié.

## Suite

**S117-1, S118 :** exercer le contrôleur dans un cycle hôte temporel mixte aux recettes
224×128 et256×128, plusieurs changements d'instant, comparaison en bits à la voie
directe et coût mise à jour+requête64. Vérifier aussi le chemin `Unchanged` et les refus.
Admission dynamique dans le contrôleur, renouvellement de fenêtre, profondeur finie
S116-2, bilan mixte et durabilité restent ouverts.
78 ADR,193 angles,17 invariants,6 spécifications,23 cas.
