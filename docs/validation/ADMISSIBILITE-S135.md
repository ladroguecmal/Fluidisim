# S135 — Admissibilité annoncée : ce qui manquait n'était pas une transaction

2026-09-10. [ADR-091](../adr/ADR-091-admissibilite-annoncee-entre-couches.md) actée. S134-1
réalisée, après recadrage de ce qu'elle demandait.

## 1. Ce qui était déjà garanti

La suite s'intitulait « transaction mixte ». L'inventaire montre que trois des quatre pièces
existaient déjà, et que la quatrième n'est pas celle qu'on croyait.

**Pendant une requête mixte, rien ne peut bouger.** `sample_world_batch` prend deux vues
immuables ; `LiveWater::admit` et `Controller::admit` exigent `&mut`. Le compilateur interdit
déjà d'admettre pendant qu'on échantillonne — la même garantie structurelle qu'en S130, et il
suffisait de la constater plutôt que de la construire.

**Chaque couche a son admission transactionnelle** : `Controller` côté pression depuis ADR-086,
`LiveWater` côté impacts, qui publie journal et champs ensemble et bloque la vue courante tant
qu'une commande est en attente.

**La cause est déjà commune** : `wave_journal::Cause` sert au journal W et aux métadonnées des
sources de pression. Deux effets d'un même événement de jeu portent la même identité, et les
deux journaux l'exposent — `Record { cause, state }` d'un côté, `metadata().cause` de l'autre.

## 2. Ce qui ne l'était pas, et que personne n'avait constaté

**Aucune admission n'est annulable.** `wave_journal::reject` sur une cause déjà confirmée rend
`Conflict` : seule une prédiction peut être retirée. Et aucune source de pression publiée n'est
retirable — ADR-086 l'avait laissé ouvert sans en tirer la conséquence.

Cela **rend la transaction inter-couches irréalisable par l'hôte** : si sa première admission
réussit et la seconde échoue, rien ne le ramène à l'état antérieur. Ce n'est pas un manque de
coordinateur, c'est l'absence de retour en arrière.

## 3. Décision

[ADR-091](../adr/ADR-091-admissibilite-annoncee-entre-couches.md) : **pas de coordinateur.** Le
construire demanderait soit de rouvrir le retrait que deux journaux ont délibérément fermé, soit
un tiers tenant les deux couches — le couplage qu'ADR-086 a refusé, dont le motif vaut toujours.

À la place, **l'admissibilité s'annonce** : `pressure_journal::Journal::would_admit` et
`wave_journal::Journal::would_confirm` disent ce que l'admission déciderait, sans rien changer.
L'hôte interroge les deux couches, et n'admet que si les deux répondent oui. Il ne lui reste
comme risque qu'un échec **numérique** à la préparation — que chaque couche traite déjà de façon
transactionnelle, et qui ne dépend pas de l'autre.

**Une seule implémentation, pour la troisième fois.** `admit_authenticated` appelle `would_admit`
et n'insère qu'ensuite ; `confirm` et `insert` appellent `check_confirm` et `would_insert`. Après
ADR-079 et ADR-080, c'est le même critère de construction : deux implémentations du même contrôle
divergent (**L137**).

## 4. Réception

**L'annonce dit exactement ce que l'admission fait.** Le test enchaîne cinq cas — première
source, réadmission identique, seconde source, conflit d'identité, époque étrangère — et compare
à chaque fois l'annonce au verdict. Il vérifie aussi que l'annonce est **stable** (deux appels
successifs disent la même chose) et qu'elle **n'admet rien** d'elle-même, en recomptant les
sources publiées.

**La seule différence de comportement est voulue et testée** : à saturation, l'annonce rend
`Full` sans mettre la source en attente, là où l'admission la conserve. Côté W, `would_insert`
rend `Full` sans marquer la perte connue, pour la même raison.

**Le scénario inter-couches est joué.** Un événement produit un impact et une source de pression
portant la même cause ; le journal de pression est plein. L'hôte interroge les deux couches,
obtient un refus d'un côté, et **n'admet rien** — le test vérifie qu'aucun des deux journaux n'a
bougé. Avec de la place, les deux annonces passent, les deux admissions aussi, et la cause est
portée des deux côtés.

173 core + 93 harnais = **266 tests réussis, cinq ignorés** ; les deux tests neufs passent aussi
en release. Hachages de la campagne `cycle_mixed` identiques à ceux de S118.

## 5. Ce qui n'est pas revendiqué

Le risque d'état partiel n'est pas supprimé : un échec numérique de la seconde admission le
produit encore, et l'hôte n'a alors aucun retour en arrière. Il est **réduit aux causes qui
demandent un calcul**, et nommé.

Le retrait n'est pas ouvert, les couches ne sont pas couplées, et ce que signifie l'appartenance
conjointe de deux effets à une cause reste au gameplay — le système ne définit pas de « cause
complète ».

L'ordre d'admission ne rattrape rien, puisque aucune des deux couches n'est annulable : ADR-091
ne le prescrit donc pas, contrairement à ce qu'une lecture rapide de l'asymétrie pourrait
suggérer.

## 6. Suite

**S135-1, S136 :** la couche pression est complète, et son admission est désormais annonçable
comme son échantillonnage l'était depuis ADR-079. Ce qui reste ouvert de ce côté ne relève plus
de la mécanique mais du **contenu** : le générateur physique d'ADR-055 — quelle énergie, quel
`α` — que trois sessions ont maintenant cité comme manquant, et sans lequel `wavelength_m` et
`energy_j` restent des nombres que personne ne sait produire (A200, sévérité 1).

Restent ouverts : l'extension de fenêtre, la profondeur finie de pression (S116-2), le bilan
mixte, la durabilité disque, la calibration B2.

91 ADR, 204 angles, 17 invariants, 6 spécifications, 23 cas.
