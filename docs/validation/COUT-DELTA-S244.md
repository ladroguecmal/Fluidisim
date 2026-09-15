# Le coût d'un pas de δ, décomposé — S244, 2026-09-15

Traite **A276**, premier lot de coût de δ. Mesuré par nos propres bancs :
**5,5125 ms par pas à 2 048 mailles** ([BUDGET-DELTA-S230](BUDGET-DELTA-S230.md), un pas avance
exactement une image), contre **2 ms pour toute l'eau** — B, W, δ et le rendu associé
([ADR-125](../adr/ADR-125-budget-image-60hz-deux-ms.md)). **δ seul vaut 2,8 fois le budget entier**,
et **aucune technique de coût ne lui a jamais été appliquée** (ADR-131 : présentes — Jacobi diagonal,
f32 ; absentes — GPU, parallélisme, multigrille, factorisation incomplète, itérations fixes, cuisson).

## 1. Protocole, écrit avant mesure et construction

### 1.1 Ce que la lecture du code établit

Une itération du gradient conjugué, couvercle fixe, fait exactement :

| pass | nature | écriture |
|---|---|---|
| `apply(dir → tmp)` | stencil à cinq points, pondéré par les ouvertures | **disjointe** |
| `dot(dir, tmp)` | réduction | — |
| `p += α·dir ; res −= α·tmp` | deux axpy fusionnées | **disjointe** |
| `norm2(res)` | réduction | — |
| `dir = res + β·dir` | axpy | **disjointe** |

Soit **trois passes d'écriture disjointe et deux réductions** par itération, plus une relance
périodique (`apply(p)`, `res = rhs − Ap`, `norm2`, certificats). Le compte d'itérations croît en
`O(√N)` : 28 / 58 / 112 / 219 / 417 mesurés en S239 de 128 à 32 768 mailles.

Les deux réductions passent **déjà** par `parallel_reduce_ordered_f64` — mais sur un hôte
séquentiel, et [ADR-146 §6](../adr/ADR-146-l-ecriture-disjointe-est-inconditionnellement-deterministe.md)
les laisse séquentielles : leur ordre de fusion *est* la référence.

### 1.2 L'instrument : mesurer chaque pass, puis vérifier que leur somme rend le pas

Poser des horloges dans la boucle du gradient conjugué déplacerait ce qu'on mesure. On mesure donc
**chaque pass isolément**, sur le même domaine et les mêmes tampons, puis on confronte :

```
pas prédit  ≈  itérations × (apply + dot + axpy + norm2 + dir)  +  relances × (apply + res + norm2)
```

**Si le prédit ne rend pas le mesuré, la décomposition est fausse et le document le dit.** C'est le
contrôle qui empêche d'attribuer un coût à la pass qu'on avait envie d'accuser.

### 1.3 Thèse, et ce qui la réfuterait

Les passes d'écriture disjointe dominent le pas ; les paralléliser **sur le chemin sans budget**
(`Control::unlimited`, celui des bancs et de `step`) le réduit **sans changer un bit** — les
réductions, seules sensibles à l'ordre, restent séquentielles, et l'écriture disjointe a une garantie
inconditionnelle (ADR-146).

**Ce qui la réfuterait** : que les réductions, ou le simple nombre d'itérations, portent l'essentiel.
Dans ce cas la technique à appliquer n'est pas le parallélisme — ce serait la **multigrille** ou un
préconditionneur plus fort, qui attaquent le compte d'itérations — et la session le dira sans forcer.

### 1.4 Le budget coopératif n'est pas parallélisé, et c'est déjà l'usage

`Control::poll` rend `Ok` immédiatement quand il n'y a pas d'horloge, et `budget::copy` traite déjà
le cas sans budget à part. Le budget (I-05) est un **état partagé** : on ne le distribue pas entre
fils. Le chemin **coopératif** reste donc exactement celui d'aujourd'hui, au bit et à la
milliseconde ; seul le chemin sans budget se parallélise.

### 1.5 Critères de réception, déclarés avant construction

1. **Décomposition mesurée avant toute modification**, avec son contrôle de somme (§1.2).
2. **Au bit, à tout nombre de fils** : empreinte `delta_filters` **`0xfb12b2092df4ee6d`**, les dix
   cas de `delta_precision`, et la suite complète.
3. **Coût publié contre le nombre de fils**, sur les grilles de S230 (128, 512, 2 048 mailles).
4. **Ni le chemin coopératif ni le chemin à un fil ne sont ralentis** ; aucune allocation ajoutée.
5. **Coût** (ADR-131) : techniques présentes, absentes, domaine — **et le facteur qui reste à
   trouver, écrit en clair**. Un lot de coût qui ne dit pas ce qu'il ne gagne pas ment par omission.

### 1.6 Arrêt

La décomposition publiée et la technique qu'elle désigne appliquée et reçue. **Ou** la décomposition
publiée seule, si elle désigne un lot qui dépasse la session — nommé, avec son déclencheur.
