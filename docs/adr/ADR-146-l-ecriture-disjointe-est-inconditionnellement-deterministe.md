# ADR-146 — L'écriture disjointe est inconditionnellement déterministe, et c'est elle qu'on parallélise

- Statut : **actée**, S243, 2026-09-15 ; autonomie technique S71.
- Construit la seconde primitive de [SPEC-004 §8.2](../specs/SPEC-004-interfaces.md), spécifiée
  depuis l'origine et jamais écrite. Précise la portée de
  [ADR-029 §3](ADR-029-ce-que-la-premiere-ligne-de-code-a-appris.md) sans la contredire.
- Mesures et réception : [PARALLELISME-S243](../validation/PARALLELISME-S243.md).

## Constat

Le projet n'avait **aucun fil d'exécution**. Le poste dominant du budget de l'hôte est
`ModalPressure::sample` — 87 % de la préparation du sillage (S242) — et le coût de δ dépasse le
profil de deux à trois ordres de grandeur dès 2 048 mailles (A276) ; dans les deux cas, la seule
technique listée qui les attaque est le parallélisme CPU.

SPEC-004 §8.2 énonçait **deux** primitives : `parallel_reduce_ordered` et `parallel_for`, « réservé
aux écritures disjointes ». Seule la première existait. Et `SequentialJobs` portait depuis S20 la
phrase qu'il fallait rendre vérifiable : *le jour où une version parallèle existera, l'assertion
« changer `worker_count` change la vitesse, jamais le résultat » se vérifiera contre celle-ci*.

## Décision

1. **La primitive parallélisée est l'écriture disjointe**, et sa garantie est **inconditionnelle**.
   Pour une réduction, ADR-029 §3 a dû inscrire `grain` dans le contrat : l'addition flottante n'est
   pas associative, donc deux découpages donnent deux sommes, et la garantie s'énonce *à `n` et
   `grain` égaux*. Pour une écriture disjointe, **aucune accumulation ne passe d'une tâche à
   l'autre** : chaque élément de sortie est écrit une fois, depuis des entrées en lecture seule. La
   suite d'opérations flottantes qui produit un élément ne dépend donc **ni du grain, ni de
   `worker_count`, ni de l'ordre d'exécution**. Cette garantie est **plus forte** que celle de sa
   voisine ; elle se démontre, elle ne se recopie pas.
2. **Signature** : `parallel_fill_f32(&self, out: &mut [f32], grain: usize,
   fill: &(dyn Fn(usize, &mut [f32]) + Sync))`. Objet-sûre — `HostServices` porte un `&dyn
   JobSystem` —, sans allocation dans le cœur, sur le seul type de tampon flottant que le système
   publie. **L'hôte découpe et exécute ; le cœur ne crée aucun fil** (ADR-020).
3. **L'implémentation par défaut du trait est la référence séquentielle.** Un hôte qui ne surcharge
   rien la rend déjà, et un hôte parallèle se compare à elle au bit.
4. **`grain` encode le travail par élément, et seul l'appelant le connaît.** Il reste une donnée du
   contrat, jamais dérivée de la machine (ADR-029 §3). Mesuré : un nœud du sillage coûte 76 ns sans
   tronçon actif, 211 ns de plus par tronçon actif ; un fil coûte ≈ 67 µs à créer et joindre. Un
   appelant qui découperait sans regarder son propre travail **ralentirait** son chemin — c'est
   arrivé en construisant ce lot, et le grain adaptatif est la réponse qui a une provenance.
5. **Un chemin parallèle ne porte pas de verdict.** Une tâche ne propage pas de `Result` : elle
   écrit un non-fini, et la **boucle séquentielle est rejouée** pour rendre le variant d'erreur et
   le rang exacts. Le chemin rapide n'a donc aucune sémantique propre.
6. **La réduction n'est pas parallélisée.** Son ordre de fusion *est* la référence, et rien ne la
   consomme en parallèle aujourd'hui.

## Conséquences

- **Ce que cela débloque.** Toute passe à écritures disjointes peut être parallélisée avec une
  garantie de bits démontrée, sans `unsafe` dans le cœur. Mesuré sur le premier consommateur :
  **×2,63 à huit fils** sur le poste dominant (3,27 → 1,24 ms).
- **Ce que cela ne débloque pas, et il faut le dire.** Le **chemin d'image reste à un fil**. Créer
  les fils à chaque appel coûte ≈ 67 µs pièce **et alloue**, ce qu'ADR-145 interdit à 60 Hz. Un
  vivier persistant supprimerait les deux, mais partager une tranche `&mut` empruntée avec des fils
  qui survivent à l'appel **n'existe pas en Rust sûr** : `std::thread::scope` est la seule voie sans
  `unsafe`, et elle rejoint avant de rendre la main. Le parallélisme sert donc **les bancs hors
  ligne**, où ni la latence ni l'allocation ne mordent.
- **La question qui reste est une décision, pas un réglage** : autoriser `unsafe` dans l'hôte pour
  un vivier persistant. Le cœur l'interdit et continuera ; le harnais n'en a aucun aujourd'hui.
  Ouverte sous A278, non tranchée ici.
- Une machine, un système, 1 à 16 fils : les 67 µs par fil sont ceux de cette machine, et l'arbitrage
  entre huit et seize fils peut se déplacer ailleurs.

## Réversibilité

Retirer `parallel_fill_f32` du trait et rendre à `render_components` sa boucle séquentielle — qui est
conservée intacte sous `render_sequential` — rend exactement le chemin de S242, au bit.
