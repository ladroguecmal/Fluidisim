# Le parallélisme déterministe du système — S243, 2026-09-15

Le projet n'a **aucun fil d'exécution**. Le poste dominant du budget de l'hôte est
`ModalPressure::sample` — 87 % de la préparation du sillage ([S242](PREPARATION-SILLAGE-S242.md)) —
et la seule technique qui l'attaque est le parallélisme CPU, absente de J1-bis. Le coût de δ (A276)
attend la même chose. C'est un prérequis **partagé**.

## 1. Protocole, écrit avant construction

### 1.1 Ce que le contrat prévoyait déjà

[SPEC-004 §8.2](../specs/SPEC-004-interfaces.md) énonce **deux** primitives, pas une :
`parallel_reduce_ordered`, et `parallel_for` « réservé aux écritures disjointes ». Seul le
`trait JobSystem` de Rust ne porte que la première. Et `SequentialJobs`, dans le harnais, écrit
depuis S20 que le jour où une version parallèle existera, l'assertion *changer `worker_count` change
la vitesse, jamais le résultat* se vérifiera contre elle.

**Cette session ne change donc pas l'interface : elle construit la moitié qui manquait**, et rend
cette phrase vérifiable.

### 1.2 La dissymétrie qui fonde la garantie

Pour une **réduction**, [ADR-029 §3](../adr/ADR-029-ce-que-la-premiere-ligne-de-code-a-appris.md) a
dû inscrire `grain` dans le contrat : l'addition flottante n'est pas associative, donc deux
découpages donnent deux sommes — sur `[1 ; 10¹⁶ ; −10¹⁶ ; 1]`, grain 1 rend `1,0` et grain 2 rend
`0,0`. La garantie y est conditionnelle : *à `n` et `grain` égaux*.

Pour une **écriture disjointe**, il n'y a **aucune accumulation d'une tâche à l'autre** : chaque
élément de sortie est écrit par exactement une tâche, à partir d'entrées en lecture seule. La suite
d'opérations flottantes qui produit un élément est donc la même quel que soit le découpage. La
garantie est **inconditionnelle** : indépendante du grain, du nombre de fils et de l'ordre
d'exécution. C'est une propriété plus forte que celle de la réduction, et elle mérite d'être écrite
comme telle plutôt que recopiée de sa voisine.

### 1.3 Ce que le langage interdit ici, et la forme retenue

`water-core` est en `#![forbid(unsafe_code)]`. Le cœur ne peut donc pas découper lui-même une tranche
mutable entre plusieurs fils par pointeur brut, ni construire sans allocation un tableau de tâches
possédant chacune sa tranche (I-06 interdit l'allocation).

**Forme retenue** — objet-sûre, sans générique, sans allocation dans le cœur, sans `unsafe` dans le
cœur :

```rust
fn parallel_fill_f32(
    &self,
    out: &mut [f32],
    grain: usize,
    fill: &(dyn Fn(usize, &mut [f32]) + Sync),
);
```

C'est **l'hôte** qui découpe `out` par `chunks_mut` et exécute ; le cœur ne crée aucun fil et ne voit
jamais deux tranches à la fois. `fill` reçoit l'indice du premier élément de sa tranche et la tranche.
Le type est `f32` parce que c'est **le seul tampon flottant que le système publie** — les
coefficients du sillage (quatre par nœud) comme les champs de δ (un par maille) ; `as_flattened_mut`,
stable depuis Rust 1.80, donne la vue plate d'un `&mut [[f32; 4]]` **sans `unsafe`**.

**Formes écartées, et pourquoi.**

| forme | pourquoi non |
|---|---|
| générique `parallel_for<T>` | un `trait` à méthode générique n'est pas objet-sûr ; `HostServices` porte un `&dyn JobSystem` |
| tâches fournies par le cœur (`&mut [&mut dyn FnMut]`) | il faudrait un tableau de taille variable, donc une allocation par image (I-06) |
| indices seuls, écriture par cellules atomiques | change le type publié et le coût, pour une sûreté que le découpage par tranches donne déjà |
| fils créés dans le cœur (`std::thread::scope`) | contredit ADR-020 : l'hôte fournit les services ; un serveur ou un harnais sans fil doit rester servi |

### 1.4 Les erreurs restent celles d'avant

`ModalPressure::sample` peut échouer, et une tâche ne peut pas propager un `Result`. Le chemin
parallèle est donc un **chemin rapide** : si une sortie est non finie, la boucle **séquentielle est
rejouée** et c'est elle qui rend le verdict. Le variant d'erreur et le rang du premier nœud fautif
sont donc exactement ceux d'avant, et le tampon de sortie reste intact au refus comme l'exige
`refusals_leave_output_and_state_usable_s213`.

### 1.5 Critères de réception, déclarés avant construction

1. **`SequentialJobs` est la référence.** La sortie parallèle lui est identique **au bit** à 1, 2, 4
   et 8 fils, et à plusieurs grains — la garantie du §1.2 est inconditionnelle, donc on l'éprouve
   comme telle.
2. **Assertion du harnais, exécutable** : à `n` et `grain` égaux, le résultat ne dépend pas de
   `worker_count`. Pour la primitive nouvelle **et** pour la réduction, dont le cas de S20 reste.
3. **Consommateur** : les six empreintes du banc `sillage_troncons` de S242, `--multi --verify`
   (0,368476 mm) et `--multi --retour` (0 image différente) inchangés.
4. **Coût publié contre le nombre de fils**, et **le chemin à un fil ne doit pas être ralenti** :
   un parallélisme qui coûte au serveur mono-fil n'est pas gratuit.
5. **Aucune allocation ajoutée au chemin d'image** (ADR-145) : `update` reste à zéro.
6. **ADR** : la primitive et son argument de déterminisme sont une décision, pas un détail.

### 1.6 Arrêt

Primitive, pool, assertion et premier consommateur reçus. **Ou**, si le lot dépasse la session, la
primitive et son assertion reçues seules, et le consommateur déclaré en file avec son déclencheur —
écrit, pas laissé à deviner.
