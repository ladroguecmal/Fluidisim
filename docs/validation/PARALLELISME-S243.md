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

## 2. Ce qui a été construit

| pièce | où | ce qu'elle est |
|---|---|---|
| `JobSystem::parallel_fill_f32` | `water-core/src/host.rs` | la primitive, **avec son implémentation par défaut séquentielle** : aucun hôte existant ne casse, et cette implémentation **est** la référence de bits |
| `ScopedJobs` | `water-harness/src/host_impl.rs` | l'hôte parallèle : `std::thread::scope` + `chunks_mut`, sans dépendance et **sans `unsafe`** — les emprunts restent vérifiés par le compilateur ; à un fil, il rend le chemin séquentiel sans créer de fil |
| assertions | `host_impl` (tests) | l'écriture disjointe rend **les mêmes bits** sur 6 nombres de fils × 7 grains (1 à 20 000), et via le défaut du trait ; chaque tranche écrite une fois et une seule |
| consommateur | `Timeline::render_components` | chemin rapide parallèle ; `render_sequential` conserve la boucle d'origine comme **référence de bits et seul porteur du verdict** |

La **réduction n'est pas parallélisée** : son ordre de fusion *est* la référence (ADR-029 §3), et
rien ne la consomme en parallèle aujourd'hui. `ScopedJobs` la délègue à `SequentialJobs`.

L'accumulateur d'un nœud vit dans les **deux premiers `f32` de son quadruplet de sortie** — ce sont
exactement les `f32` que l'ancien champ `current` portait, donc exactement les mêmes bits.

## 3. Réception

### 3.1 Au bit — la garantie du §1.2, éprouvée comme inconditionnelle

Les **six empreintes** du banc `sillage_troncons` sont identiques à **1, 2, 4, 8 et 16 fils**, et
identiques à celles de S242 :

```
0x95a8239f6f9cc139  0x0af49e8f4c2aa21c  0xe14cee491a007edc
0x24b5f8e1f47300d0  0x393d0b9d526daf84  0xdf150d01e0afe270
```

| contrôle | résultat |
|---|---|
| `--multi --verify`, 23 âges | **0,368476 mm** à 12 s avec 4 impacts — valeur de S235 |
| `--multi --retour` | 150 images cachées, 31 comparées, **0 différente au bit** |
| oracle de chemin indépendant | `timeline_matches_prepared_across_instants_and_jumps_s213` passe |
| assertion du harnais | mêmes bits sur 6 × 7 combinaisons de fils et de grains |
| suite `code/` complète | **433 réussis, 0 échec, 12 ignorés** (431 en S242, plus les deux assertions) |

**Sur l'hôte**, chemin d'image à un fil (§3.4) : CPU médian **4,0581 ms** contre 3,9877 en S242,
sillage **3,1947** contre 3,1110 — soit +1,8 % et +2,7 %, **sous le témoin de bruit de ce banc**
(L321 : une variante sémantiquement neutre y mesure +2,6 à +7,7 %). Allocations de `update` :
**0**, image 133 / 18 509 octets, inchangées.

### 3.2 Coût — le levier fonctionne, et son prix se voit

Banc `sillage_troncons`, trois sillages (24 segments), 4 096 nœuds, release :

| fils | forçage (3 tronçons actifs) | après forçage (aucun) |
|---:|---:|---:|
| 1 | 3,2719 ms | 0,2973 ms |
| 2 | 2,0001 ms | 0,2975 ms |
| 4 | 1,2871 ms | 0,2985 ms |
| **8** | **1,2432 ms** | 0,2954 ms |
| 16 | 1,7592 ms | 0,3443 ms |

**Sur le poste dominant, huit fils rendent ×2,63** — 3,27 ms deviennent 1,24. Au-delà, le prix de
création des fils reprend le dessus : seize fils sont **plus lents** que huit.

### 3.3 Deux défauts attrapés par les critères déclarés, et corrigés

Le protocole exigeait qu'aucune allocation ne soit ajoutée au chemin d'image (5) et que le chemin à
un fil ne soit pas ralenti (4). Les deux ont mordu.

1. **`ScopedJobs` construisait un `Vec` de tranches à chaque appel** : `update` est passé de **0 à
   50 allocations par image**, ce qu'ADR-145 interdit. Corrigé — chaque fil reçoit une **portion
   contiguë**, multiple du grain, qu'il parcourt lui-même ; aucun tampon intermédiaire.
2. **Lancer des fils quand aucun tronçon ne force faisait monter 0,30 ms à 0,76 ms.** Le travail par
   nœud vaut 76 ns sans tronçon actif et 211 ns de plus par tronçon actif (S242) ; un fil coûte
   **≈ 67 µs** à créer et joindre. Corrigé par le seul moyen qui ait une provenance : **le grain
   encode le travail par élément, et seul l'appelant le connaît** (ADR-029 §3). Sans forçage, le
   grain vaut le total — une tranche, aucun fil, et les mêmes bits.

### 3.4 Le blocage qui reste, nommé précisément

**Le chemin d'image de l'hôte reste à un fil**, et ce n'est pas un oubli.

Créer les fils à chaque appel coûte ≈ 67 µs pièce **et alloue**. Un vivier persistant supprimerait
les deux — mais partager une tranche `&mut` empruntée avec des fils qui survivent à l'appel
**n'existe pas en Rust sûr** : `std::thread::scope` est la seule voie sans `unsafe`, et elle
rejoint les fils avant de rendre la main. C'est précisément pourquoi les bibliothèques de
parallélisme en contiennent.

Donc : **le parallélisme sert aujourd'hui les bancs hors ligne**, où ni la latence ni l'allocation ne
mordent, et où il vaut déjà ×2,6 sur le poste dominant. Le porter au chemin d'image demande
`unsafe` dans l'hôte — le harnais n'en a aucun, le cœur l'interdit. **C'est une décision, pas un
détail d'implémentation**, et elle n'est pas prise ici.

### 3.5 Coût de la mesure (ADR-131)

- **Techniques présentes** : écriture disjointe parallèle (`std::thread::scope`), grain choisi par
  l'appelant selon le travail par élément, repli des tronçons achevés (S213), sélection hissée (S242).
- **Techniques absentes** : vivier persistant de fils, SIMD explicite, LOD temporel, réduction
  parallèle, GPU pour cette passe, mutualisation entre journaux.
- **Domaine** : fixture S212/S235, 4 096 nœuds, 24 segments, release, une machine ; 1 à 16 fils.

### 3.6 Ce qui n'est pas reçu

- **Le chemin d'image ne profite de rien** : il reste à un fil, pour les raisons du §3.4.
- **Un seul consommateur.** δ (A276) a `jobs` en main mais ses boucles chaudes interrogent un budget
  coopératif (I-05) à chaque poll — une boucle qui parallélise un compteur partagé est un autre lot.
- **Un seul nombre de nœuds, une machine, un système d'exploitation.** Les 67 µs par fil sont ceux
  de cette machine ; ailleurs, l'arbitrage entre 8 et 16 fils peut se déplacer.
- **La réduction reste séquentielle**, et le corollaire de SPEC-004 §8.2 n'est donc éprouvé que pour
  l'écriture disjointe.
