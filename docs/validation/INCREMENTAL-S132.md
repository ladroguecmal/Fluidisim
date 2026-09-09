# S132 — Admission incrémentale : exacte ou pas du tout

2026-09-10. [ADR-088](../adr/ADR-088-admission-incrementale-exacte.md) actée. S131-1 réalisée.

## 1. Les deux mesures qui décidaient

### Le coût est linéaire en nombre de segments

`code/water-core/examples/incremental_gain.rs`, après mise en régime (A195) :

| segments | 224×128 | 256×128 |
|---|---|---|
| 1 | 5,00 ms | 5,88 ms |
| 2 | 9,58 ms (×1,92) | 11,28 ms (×1,92) |
| 4 | 19,80 ms (×3,96) | 22,20 ms (×3,77) |
| 8 | 35,06 ms (×7,01) | 40,29 ms (×6,85) |

Une admission recalcule donc tout ce qui était déjà publié : à huit segments, sept huitièmes du
travail sont refaits pour rien.

*(Le premier montage de cette sonde a été refusé par la préparation : `prepare` exige un chemin
**contigu** — chaque segment commence là et quand le précédent finit — et une suite de segments
identiques ne l'est pas. La mesure porte donc sur une trajectoire réelle découpée en huit.)*

### L'ajout après coup est exact, sous une condition

`prepare_segments` accumule par nœud, segment après segment, en `f32`. Ajouter la contribution
d'une source à un champ déjà préparé reproduit donc l'ordre d'addition **si cette source vient
en dernier** dans l'ordre canonique.

| position de la source ajoutée | points de contrôle différents |
|---|---|
| en dernier | **0 sur 8** |
| au milieu | **8 sur 8** |

**La première version de cette sonde était trop faible et concluait le contraire.** Elle
n'utilisait que deux segments — et avec deux termes, l'addition `f32` est commutative : l'ordre
n'y change rien. Il en faut trois pour que l'associativité entre en jeu. Une sonde à deux
sources aurait autorisé un raccourci faux dans tous les cas.

## 2. Décision

[ADR-088](../adr/ADR-088-admission-incrementale-exacte.md) : le chemin incrémental s'applique
**si et seulement si** la source s'insère en dernier ; sinon l'admission recalcule tout. Le
résultat est celui de la voie directe dans les deux cas, au bit près.

C'est la seule forme acceptable. Un champ qui dépendrait de l'ordre historique des admissions
donnerait deux résultats pour un même journal, et le déterminisme bit à bit d'**I-03** ne
survivrait pas à deux hôtes ayant admis les mêmes sources dans un ordre différent.

**L'optimisation est donc invisible** : aucune API ne change, rien n'est annoncé, l'appelant n'a
pas à savoir quel chemin a été pris. C'est l'inverse de la discipline d'ADR-079 et ADR-080 — ici
c'est de ne rien exposer qui protège la propriété.

## 3. Construction

Le `Slot` porte désormais la **pression modale cumulée**, deux `f32` de plus. L'énergie se
recalcule depuis la réponse cumulée, déjà stockée ; la puissance a besoin de la pression, qui ne
l'était pas, et la reconstituer aurait exigé une réponse modale par segment — c'est-à-dire le
coût qu'on cherche à éviter. Prix : +18 % sur les pools de coefficients, aucun format sérialisé
touché.

`spectral_pressure::add_segments` reprend la boucle de `prepare_segments` en partant des
coefficients présents, et **refait les bilans en entier** — sommation de Kahan sur les nœuds,
dans le même ordre — donc identiques à ceux d'une préparation complète.

La transaction d'ADR-086 est préservée : le chemin incrémental recopie le pool actif dans la
réserve avant d'y ajouter, si bien qu'un échec laisse la publication intacte. **Cette recopie
coûte 28,5 µs**, soit 0,6 % d'un segment — sans commune mesure avec la réponse modale qu'elle
évite.

## 4. Réception

**Le champ publié est celui de la voie directe, où que la source tombe.** Le test admet une
troisième source dans deux ordres d'arrivée différents — identifiant 3 après {1, 2}, puis
identifiant 2 après {1, 3} — vérifie la position réelle obtenue plutôt que de la supposer, et
compare cinq points, l'énergie, la puissance et l'enveloppe à une préparation directe du journal.

**Ce test a été vérifié comme témoin** : en forçant le raccourci quelle que soit la position, il
échoue sur le cas du milieu — « source 2 en position 1 ». Sans cette vérification, il aurait pu
passer pour une raison sans rapport avec ce qu'il prétend établir.

Les tests d'ADR-086 restent valides tels quels, et l'un d'eux compare déjà le champ après
admission à une préparation directe : il exerce désormais le chemin incrémental sans avoir été
modifié.

169 core + 93 harnais = **262 tests réussis, cinq ignorés** ; ciblés aussi en release.
**Hachages de la campagne `cycle_mixed` identiques** à ceux de S118 — le résultat n'a pas bougé,
ce qui est la propriété centrale de cette décision.

## 5. Ce que cela rapporte

Une admission incrémentale coûte le prix d'**un** segment, plus la recopie : 5,03 ms au lieu de
35,06 ms quand sept segments sont déjà publiés, soit un facteur **6,9** à 224×128. Le gain croît
avec la charge, puisque c'est exactement le travail répété qui disparaît.

Sur la campagne à deux sources d'un segment, le gain est d'un facteur 2 — et les hachages le
confirment inchangés.

## 6. Ce qui n'est pas revendiqué

La reconstruction après élargissement (ADR-087) n'est pas accélérée : le nouveau contrôleur part
d'un pool vide, et rien ne lui transmet les coefficients de l'ancien. La fenêtre sans champ de
S131 reste ce qu'elle est.

Aucun mode dégradé, aucune option, aucun réglage. Le retrait d'une source n'existe toujours pas,
et la réadmission d'une source modifiée reste un conflit refusé par ADR-075.

## 7. Suite

**S132-1, S133 :** la reconstruction d'ADR-087 pourrait maintenant repartir des coefficients de
l'ancien contrôleur au lieu d'un pool vide — les sources publiées sont les mêmes, seule l'attente
s'ajoute, et elle s'insère en dernier si son identifiant est le plus grand. La fenêtre sans champ
tomberait alors au coût d'un segment. **À mesurer avant de décider** : il faut transporter des
coefficients d'un pool à un autre, ce qu'aucune API ne fait aujourd'hui, et vérifier que la
condition d'ordre est satisfaite dans les cas réels plutôt que supposée.

Restent ouverts : la transaction mixte, l'extension de fenêtre, la profondeur finie de pression
(S116-2), le bilan mixte, la durabilité disque, le générateur physique d'ADR-055.

88 ADR, 204 angles, 17 invariants, 6 spécifications, 23 cas.
