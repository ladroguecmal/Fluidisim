# ADR-091 — Annoncer l'admissibilité plutôt que coordonner les couches

- **Statut : actée**, S135, 2026-09-10, autonomie technique S71.
- **Prolonge :** journal W ADR-056, journal de pression ADR-075, requête mixte ADR-077,
  admission dynamique ADR-086.
- **Résout :** S134-1 — en établissant que la transaction inter-couches n'est pas la bonne
  réponse, et ce qui l'est.

## Ce que l'inventaire a montré

**Trois choses étaient déjà vraies, et une quatrième ne l'était pas.**

1. **Pendant une requête mixte, rien ne peut bouger.** `sample_world_batch` prend deux vues
   immuables ; `LiveWater::admit` et `Controller::admit` exigent `&mut`. Le compilateur interdit
   déjà d'admettre pendant qu'on échantillonne — même garantie structurelle qu'en S130.
2. **Chaque couche a son admission transactionnelle** : `Controller` côté pression (ADR-086),
   `LiveWater` côté impacts, qui publie journal et champs ensemble et bloque la vue courante
   tant qu'une commande est en attente.
3. **La cause est déjà commune** : `wave_journal::Cause` sert au journal W et aux métadonnées
   des sources de pression. Deux effets d'un même événement de jeu portent la même identité, et
   les deux journaux l'exposent.
4. **Mais aucune admission n'est annulable.** `wave_journal::reject` sur une cause déjà
   confirmée rend `Conflict` — seule une prédiction peut être retirée. Et aucune source de
   pression publiée n'est retirable, ADR-086 l'a laissé ouvert.

Le point 4 est celui qui décide, et il n'avait pas été constaté. **Il rend la transaction
inter-couches irréalisable par l'hôte** : si sa première admission réussit et la seconde échoue,
rien ne le ramène à l'état antérieur.

## Décision

**Pas de coordinateur entre couches.** Le construire demanderait soit de rendre les admissions
annulables — donc de rouvrir ce que deux journaux ont délibérément fermé — soit un tiers qui
tienne les deux couches, c'est-à-dire exactement le couplage qu'ADR-086 a refusé et dont le
motif vaut toujours.

**À la place, l'admissibilité s'annonce.** Chaque journal expose ce que son admission déciderait,
sans rien changer :

- `pressure_journal::Journal::would_admit(&source)` ;
- `wave_journal::Journal::would_confirm(epoch, cause, event)`.

L'hôte qui veut admettre deux effets d'une même cause interroge les deux, et n'admet que si les
deux répondent oui. Il ne lui reste alors comme risque qu'un **échec numérique** à la
préparation — que chaque couche traite déjà de façon transactionnelle, et qui ne dépend pas de
l'autre.

**Une seule implémentation, comme toujours ici.** `would_admit` n'est pas une copie des
contrôles : `admit_authenticated` l'appelle et n'insère qu'ensuite. Deux implémentations du même
contrôle divergent (**L137**), et c'est la troisième fois que ce dépôt s'en sert comme critère de
construction — après ADR-079 et ADR-080.

## Ce que cette décision ne fait pas

Elle ne supprime pas le risque d'état partiel : un échec numérique de la seconde admission le
produit encore, et l'hôte n'a alors aucun retour en arrière. Elle le **réduit aux causes qui
demandent un calcul**, et le nomme.

Elle n'ouvre pas le retrait, ne couple pas les couches, ne définit pas de « cause complète » —
ce que signifie l'appartenance conjointe de deux effets à une cause reste au gameplay, pas au
système.

Elle ne dit rien de l'ordre dans lequel admettre : l'ordre ne rattrape rien puisque aucune des
deux couches n'est annulable.

## Réception

[ADMISSIBILITE-S135](../validation/ADMISSIBILITE-S135.md). L'annonce est confrontée au verdict
réel de l'admission sur chaque cause de refus — époque, conflit, saturation — et l'égalité est
vérifiée dans les deux sens. Le scénario complet est joué : vérifier les deux couches, puis
admettre les deux, et constater que le champ mixte contient bien les deux effets.
