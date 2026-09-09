# ADR-087 — La capacité qui résout une attente s'annonce

- **Statut : actée**, S131, 2026-09-10, autonomie technique S71.
- **Prolonge :** journal de pression ADR-075, admission dynamique ADR-086.
- **Résout :** S130-1 — la sortie de saturation.

## Problème

ADR-086 a reçu la saturation comme un état terminal du contrôleur : après un `Full`, la source
est conservée en attente, et `from_journal` refusant tout journal en attente, le contrôleur ne
peut plus changer d'instant. La sortie passe par `copy_into` vers un stockage plus grand, puis
`retry`.

**Ce chemin a un piège.** `copy_into` ne refuse que si le nouveau stockage est plus petit que
la publication : `slots.len() < count`. Un stockage de taille exactement `count` passe donc la
copie — et laisse l'attente irrésolue, puisqu'il n'y a de place pour personne. `retry` y rend
`Full` de nouveau. **Un élargissement peut réussir sans sortir de la saturation**, et l'hôte ne
l'apprend qu'après avoir payé la copie, puis la reconstruction du contrôleur, pour rien.

## Décision

**`Journal::required_capacity()` rend le nombre d'emplacements nécessaires pour que la copie
**et** la reprise aboutissent** : la publication, plus l'attente s'il y en a une.

La garantie est une équivalence, dans les deux sens :

- `slots.len() >= required_capacity()` ⟹ `copy_into` réussit **et** `retry` résout l'attente ;
- `slots.len() < required_capacity()` ⟹ l'un des deux échoue — `Capacity` pour la copie si le
  stockage est plus petit que la publication, `Full` pour la reprise s'il est juste assez grand
  pour elle.

Cette exactitude tient parce qu'une source mise en attente a déjà passé les contrôles d'époque
et de conflit lors de sa première présentation, et que `copy_into` reproduit la publication à
l'identique : après la copie, seule la place peut encore manquer.

**L'ordre du cycle est prescrit, parce qu'il est mesurable.** `copy_into` prend `&self`, et le
contrôleur expose son journal en lecture seule depuis ADR-086 : élargissement et reprise se font
donc **pendant que le contrôleur sert encore**. Seule la reconstruction impose de le libérer. La
fenêtre sans champ vaut une préparation, et non le cycle entier — à condition d'élargir avant de
libérer, et non l'inverse.

## Ce que cette décision ne fait pas

Elle ne supprime pas la fenêtre sans champ : reconstruire le contrôleur sur le journal élargi
coûte une préparation complète, et il n'existe pas de chemin qui republie sans recalculer.

Elle n'alloue rien et ne choisit aucune taille : le stockage élargi vient de l'hôte, comme tous
les pools depuis I-06. `required_capacity` dit combien il en faut **maintenant**, pas combien il
en faudra — dimensionner pour ne jamais saturer reste la bonne pratique, et cette sortie un
chemin de secours.

Elle ne traite pas plusieurs sources en attente : le journal n'en retient qu'une, et ADR-075
n'est pas rouvert.

## Réception

[SORTIE-SATURATION-S131](../validation/SORTIE-SATURATION-S131.md). L'équivalence est vérifiée
par balayage sur les tailles de stockage, de trop petit à confortable, et le champ reconstruit
est comparé en bits à une préparation directe du journal élargi.
