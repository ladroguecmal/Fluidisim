# S130 — Admettre une source sans mentir sur le champ publié

2026-09-10. Suite de S129-1. [ADR-086](../adr/ADR-086-admission-dynamique-de-la-pression.md).

## 1. Inventaire : ce que l'état actuel impose

Le contrôleur d'ADR-078 emprunte un journal **figé** : `journal: &'v Journal`. Pour changer
l'admission, il faut libérer le contrôleur, donc perdre la publication en cours. Ce que la
lecture établit avant toute décision :

| fait | conséquence pour l'admission dynamique |
|---|---|
| `Controller` détient `&Journal` | admettre exige `&mut Journal` ; sept appelants, tous en tests ou exemples |
| `admit_authenticated` rend `Added`, `Unchanged`, `Epoch`, `Conflict`, `Pending`, `Full` | quatre refus et **deux succès de nature différente** |
| `Full` met la source **en attente** et la conserve | l'attente survit à l'échec ; elle n'est pas perdue |
| `from_journal` refuse tout journal dont l'attente est non vide | **une saturation bloque aussi les changements d'instant** |
| le journal n'offre aucun retrait | après une admission réussie, revenir en arrière demande une opération qui n'existe pas |

Deux points méritent d'être soulignés parce qu'ils décident de la forme de la réponse.

**L'emprunt mutable garantit l'invariant demandé, structurellement.** La consigne exige
« jamais `Unchanged` sur un journal différent ». Un compteur de version le détecterait après
coup ; l'emprunt mutable l'interdit d'avance, puisque personne d'autre ne peut muter le journal
pendant la vie du contrôleur. C'est la même préférence qu'ADR-079 et ADR-080 — tenir une
propriété par structure plutôt que par vigilance.

**Les deux succès d'`admit_authenticated` n'ont pas les mêmes conséquences.** `Added` change le
journal, donc le champ publié ne le représente plus : il faut recalculer. `Unchanged` signifie
que la source était déjà là, à l'octet près — le champ reste exact, et recalculer serait du
travail pur. La transaction doit distinguer les deux, sans quoi elle paierait une préparation
complète à chaque réadmission d'une source connue.

## 2. Décision

Voir [ADR-086](../adr/ADR-086-admission-dynamique-de-la-pression.md).

## 3. Construction

`Controller::new` prend désormais `&mut Journal`. `admit(source)` enchaîne l'admission et le
recalcul, et rend `Admission::AlreadyPresent`, `Admission::Republished`, ou l'une des trois
formes de refus — `AdmitError::Journal`, `Saturated`, `Field`. Le retour en arrière s'appuie sur
`Journal::insertion_index`, calculé **avant** l'insertion, et sur `undo_last_admit`, tous deux
`pub(crate)` : ce n'est pas un retrait par identifiant, et il n'en existe toujours pas.

`admit_authenticated` utilise maintenant `insertion_index` pour choisir sa position, au lieu de
recalculer la même expression sur place. C'était la condition pour que le retour en arrière
défasse exactement ce que l'insertion a fait, et non ce qu'on croit qu'elle a fait.

**Une conséquence de l'emprunt n'avait pas été anticipée.** Le journal n'est plus lisible
directement pendant la vie du contrôleur — or la campagne et les tests s'en servaient pour la
voie directe témoin. D'où `Controller::journal()`, accesseur en lecture seule : il manquait, et
c'est le seul chemin pour inspecter le journal sans pouvoir le modifier dans le dos d'une
publication qui en dépend.

## 4. Réception

**Les cinq issues, avec l'état comparé avant et après.** Le test n'inspecte pas seulement le
code de retour : il échantillonne le champ en quatre points et relève énergie, puissance et
enveloppe, avant et après chaque refus.

| issue exercée | journal | champ |
|---|---|---|
| réadmission à l'octet près | 1 source | identique en bits, aucun recalcul |
| source nouvelle | 2 sources | republié, **identique en bits à une préparation directe** |
| conflit d'identité | 2 sources | intact |
| époque étrangère | 2 sources | intact |
| saturation | 2 sources + 1 en attente | intact, **et `update` refuse désormais** |

La dernière ligne est le point que la consigne demandait de recevoir : après un `Full`, le
contrôleur conserve sa publication mais ne peut plus changer d'instant, `from_journal` refusant
tout journal en attente. La transaction le dit par `Saturated` au lieu de le laisser découvrir.

**Le retour en arrière après refus du champ**, sur le témoin de S117 — une pression de 10³⁰ Pa,
représentable à la construction, dont le champ déborde. Le journal retrouve exactement une
source, l'attente est vide, la date publiée et l'énergie sont inchangées, et le contrôleur
reste utilisable : `update` puis `admit` fonctionnent ensuite.

**Ce test a été vérifié comme témoin.** Rollback désactivé, il échoue — le journal reste à deux
sources au lieu d'une. Réactivé, il passe. Sans cette vérification, un test d'état peut être
creux exactement comme un nom d'erreur inatteignable (L211).

## 5. Ce qui n'est pas revendiqué

La saturation n'est pas résolue : élargir le pool passe par `copy_into` sur un stockage plus
grand, donc par la libération du contrôleur. Aucune source publiée n'est retirable, l'ordre
canonique est inchangé, et ni les formats WPRS/WPJR ni l'époque ne sont touchés.

L'admission n'est pas coordonnée avec les autres couches d'un montage mixte : ADR-086 s'arrête
au chemin pression, ce que la consigne de S129-1 demandait précisément avant toute revendication
de transaction mixte.

Aucun coût nouveau n'est certifié. Une admission qui republie coûte une préparation, déjà
mesurée depuis S118 ; une réadmission identique ne coûte rien.

## 6. Vérification

166 core + 93 harnais = **259 tests réussis, cinq ignorés** ; les trois tests ciblés du
contrôleur passent aussi en release. Hachages de la campagne `cycle_mixed` identiques à ceux de
S118. Cinq avertissements préexistants dans le harnais, aucun nouveau.

## 7. Suite

**S130-1, S131 :** la saturation reste un état terminal pour le contrôleur. Le chemin de sortie
existe déjà — `copy_into` vers un pool élargi, puis `retry` — mais il oblige à détruire la
publication en cours, donc à perdre le champ pendant l'élargissement. Recevoir ce cycle complet,
et mesurer ce qu'il coûte, est le prolongement direct.

Restent ouverts : la transaction mixte proprement dite, l'extension de fenêtre, la profondeur
finie de pression (S116-2), le bilan mixte, la durabilité disque, le générateur physique
d'ADR-055 et la calibration B2.

86 ADR, 204 angles, 17 invariants, 6 spécifications, 23 cas.

