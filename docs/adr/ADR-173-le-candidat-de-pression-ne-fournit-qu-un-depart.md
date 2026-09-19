# ADR-173 — un candidat de pression externe ne fournit qu'un départ

Actée S289, 2026-09-19, autonomie technique S71. Rend activable ce qu'ADR-172 avait ouvert
en candidat expérimental. Applique ADR-020/130/143/144/169, I-04/I-06/I-13/I-17.

## Décision

Le cœur accepte qu'un solveur **externe** — GPU en pratique — lui propose une pression.
Cette proposition n'est qu'un **point de départ**. Le cœur ne délègue rien d'autre :

1. Il assemble lui-même le second membre et exporte son opérateur figé (ADR-172).
2. Il recalcule `r = b − A·p` avec **son** opérateur, sur CPU, après réception de la
   proposition — jamais le résidu que le solveur externe annonce.
3. Il poursuit son gradient conjugué, et l'acceptation reste celle d'ADR-143 (plancher
   d'arrondi, cycle certifié) et d'ADR-144 (tolérance physique de S199 sur la divergence
   des lignes franches), sans le moindre déplacement.
4. Une proposition non finie, de mauvaise forme, ou déclinée, est **refusée atomiquement** :
   le départ du cœur est restauré au bit et le pas se déroule comme si rien n'avait été
   proposé. Aucune publication partielle, aucun état intermédiaire conservé.

Le point d'entrée est nommé — `step_surface_mobile_with`, `project_with` — et son absence de
candidat reproduit le pas historique **au bit**. La réserve où le cœur écrit son opérateur
appartient à l'hôte : le cœur n'alloue pas, et ne dépend d'aucune bibliothèque graphique.

Ce qu'un candidat ne peut pas obtenir, quelle que soit sa qualité : faire recevoir un pas qui
ne tient pas la tolérance physique, raccourcir un critère d'arrêt, publier une pression que le
cœur n'a pas re-vérifiée, ou sortir du mode mobile — le crochet n'existe que là, et seulement
sur le chemin de départ chaud d'ADR-169.

## Pourquoi cette forme, et pas une délégation

Un solveur GPU ne peut pas honnêtement porter les portes du cœur. La tolérance d'ADR-144 se
mesure sur la divergence du champ **corrigé** ; le plancher d'ADR-143 demande une erreur
inverse composante par composante ; la détection de cycle d'ADR-143 demande une empreinte au
bit de la pression. Chacune de ces trois mesures exige un rapatriement par test — exactement
ce qu'un solveur résident existe pour supprimer. Déplacer les portes sur la carte aurait donc
coûté ce que le déplacement du calcul faisait gagner, **et** aurait mis la réception physique
du système sous la dépendance d'un pilote graphique. ADR-143 §1 et l'invariant I-13 l'excluent.

Reste l'autre découpage : le GPU fait la partie coûteuse et sans conséquence — approcher la
solution — et le CPU garde la partie décisive et bon marché — constater. Le coût de la
constatation est `O(n)` par vérification contre `O(n)` par **itération** ; la mesure de S289 le
confirme, le cœur passant de 427 itérations à 19 sur 60 pas sans qu'aucune porte ne bouge.

Ce découpage a une propriété qu'une délégation n'a pas : **un candidat faux n'est pas dangereux,
seulement inutile.** Il ne peut coûter que des itérations. C'est ce qui permet d'activer un
solveur GPU sans réception physique du GPU lui-même, sans exigence d'identité inter-GPU, et
sans que δ cesse d'être cosmétique et non répliqué.

## Ce que cette décision ne reçoit pas

Le budget eau de 2 ms d'ADR-125 : S289 mesure 6,62 ms par pas à 6 656 mailles, soit ×3,3.
I-06 sur le chemin d'image : le cycle alloue ≈ 7 fois par itération dans l'encodage de la pile
graphique ; l'activation dans la boucle d'image reste donc à construire.

> **Note corrective, S290, 2026-09-19.** La rédaction d'origine ajoutait « et ADR-145 ne
> l'admet pas ». C'est une erreur de fait : ADR-145 §1 lit I-06 sur **le code du projet** pour
> `viewer/`, et §2 décide que les allocations des dépendances verrouillées sont **comptées et
> publiées, non interdites**. Les allocations de l'encodage wgpu sont donc permises, et elles
> sont publiées. Ce qui empêche la boucle d'image est le **temps** d'enregistrement — 1,86 µs
> et une allocation par `dispatch_workgroups`, mesurés en S290 — et non une règle violée.
> La décision d'ADR-173 n'est pas affectée ; seule cette justification l'était.

La longueur du cycle est un réglage de l'hôte, non calibré et non automatique :
S289 mesure qu'elle perd autant qu'elle gagne si elle est mal choisie. Enfin, aucune identité
inter-GPU n'est introduite, et la trajectoire mesurée reste celle du cœur à l'arrondi de `f32`
— si une version future la déplaçait visiblement, la revue visuelle de S254 s'appliquerait.
