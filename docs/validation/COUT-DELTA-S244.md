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

## 2. La décomposition, mesurée avant construction

`delta_step_decomposition_s244`, release, un fil, domaine de S230, médiane de 21 relevés.

| grille | mailles | itérations | pas mesuré | `apply` | axpy | `dir` | `dot` | `norm2` |
|---|---:|---:|---:|---:|---:|---:|---:|---:|
| 16×8 | 128 | 30 | 0,0891 ms | 0,00141 | 0,00038 | 0,00024 | 0,00019 | 0,00019 |
| 32×16 | 512 | 61 | 0,6994 ms | 0,00528 | 0,00141 | 0,00093 | 0,00074 | 0,00074 |
| 64×32 | 2 048 | 114 | **5,2560 ms** | **0,02172** | 0,00550 | 0,00375 | 0,00274 | 0,00284 |

*(millisecondes par appel de la pass)*

### 2.1 Le contrôle de somme passe

| grille | écritures × itérations | réductions × itérations | prédit | mesuré | rapport |
|---|---:|---:|---:|---:|---:|
| 128 | 0,0612 ms | 0,0115 ms | 0,0726 | 0,0891 | **0,815** |
| 512 | 0,4652 ms | 0,0908 ms | 0,5560 | 0,6994 | **0,795** |
| 2 048 | **3,5308 ms** | 0,6367 ms | 4,1675 | 5,2560 | **0,793** |

Le prédit rend **79 à 82 %** du pas, et le rapport est **stable sur trois tailles** — le reste est
hors de la boucle : relances du gradient conjugué, advection, diagnostic de divergence, certificats
d'ADR-143/144. La décomposition est donc juste, et ce qu'elle n'explique pas, elle le borne.

### 2.2 Ce que la mesure désigne

À 2 048 mailles, sur les 5,256 ms d'un pas :

- **les écritures disjointes valent 3,53 ms — 67 %**, dont **`apply` seule 2,48 ms, soit 47 %** ;
- les réductions valent 0,64 ms — **12 %** ;
- le reste, hors boucle, 1,09 ms — 21 %.

**La thèse du §1.3 tient** : le pas est dominé par des passes à écriture disjointe, celles dont
ADR-146 garantit le déterminisme **inconditionnellement**. Ce n'est donc pas le nombre d'itérations
qu'il faut attaquer en premier — il le faudra, et c'est la multigrille —, mais le coût de chaque
itération, qui se divise sans changer un bit.

## 3. Le prix d'un appel parallèle — et la route qu'il referme

La thèse du §1.3 supposait que la primitive de S243 pouvait servir ces passes. **Avant de la
construire, on mesure ce qu'un appel coûte à vide** : `prix_d_un_appel_parallele_s244`, remplissage
trivial, release, médiane de 41 relevés.

| éléments | 1 fil | 2 fils | 4 fils | 8 fils |
|---:|---:|---:|---:|---:|
| 2 048 | 1,60 µs | **314,7 µs** | 518,8 µs | 947,9 µs |
| 16 384 | 11,70 µs | 312,3 µs | 546,9 µs | 883,2 µs |
| 262 144 | 190,4 µs | 392,6 µs | 576,4 µs | 987,9 µs |

Le coût **ne dépend presque pas du travail** : c'est celui de créer et joindre les fils, environ
**125 µs par fil** sur cette machine.

**Conséquence, et elle est nette.** Les passes de δ valent **21,7 µs** (`apply`), **5,5 µs** (axpy) et
**3,8 µs** (`dir`) à 2 048 mailles. Un appel à deux fils coûte **314 µs** : quatorze fois la pass
qu'il découperait. **La primitive de S243 ne peut pas servir la boucle de δ** — non parce qu'elle
serait mauvaise, mais parce qu'elle crée ses fils à chaque appel, et que cette boucle appelle 114
fois par pas des passes de quelques microsecondes.

Le modèle rend compte des deux lots : pour le sillage (S243), le travail d'un appel valait 3 270 µs,
donc `3270/8 + 950 ≈ 1 360 µs` contre 1 243 µs mesurés. Pour δ, il vaut 21,7 µs, et aucun découpage
ne rattrape 125 µs de fil.

> **Note corrective sur S243, datée du 2026-09-15.** [PARALLELISME-S243](PARALLELISME-S243.md) §3.3 et
> [ADR-146](../adr/ADR-146-l-ecriture-disjointe-est-inconditionnellement-deterministe.md) chiffrent un
> fil à **≈ 67 µs**, déduit d'une **différence** entre deux fenêtres du banc du sillage. La mesure
> directe ci-dessus, à travail trivial, donne **≈ 125 µs**. C'est ce second chiffre qui fait foi : il
> est mesuré, l'autre était inféré. Rien d'autre ne change dans S243 — le seuil au-delà duquel le
> parallélisme paie s'en trouve seulement relevé, et sa conclusion (« le chemin d'image reste à un
> fil ») en sort renforcée.

### 3.1 Ce que cela laisse

La décomposition du §2 dit ce que chaque technique restante achèterait, à 2 048 mailles :

| technique | ce qu'elle attaque | plafond de gain |
|---|---|---|
| **vivier persistant de fils** (A278) | les 3,53 ms d'écritures disjointes, **67 %** | ×3 sur ces passes si la répartition coûte des microsecondes et non 125 µs |
| **multigrille / préconditionneur fort** | les **114 itérations**, qui croissent en `O(√N)` | le facteur le plus grand, et le seul qui s'améliore avec la taille |
| **GPU** | les deux à la fois | hors du domaine de ce lot |

**Aucune ne se fait sans décision** : la première demande `unsafe` dans l'hôte (A278), la deuxième
est une construction numérique à part entière avec ses propres critères de réception, la troisième
un portage. Ce lot les chiffre ; il n'en tranche aucune.

## 4. La décomposition étendue, et le coût à 32 768 mailles

Le même instrument, aux cinq tailles. [TOLERANCE-PRESSION-S239](TOLERANCE-PRESSION-S239.md) §4.5 et
la file laissaient le coût à 32 768 mailles **non mesuré** ; il l'est ici.

| mailles | itérations | pas mesuré | `apply` | écritures | réductions | part des écritures |
|---:|---:|---:|---:|---:|---:|---:|
| 128 | 30 | 0,089 ms | 0,00141 | 0,061 | 0,012 | 69 % |
| 512 | 61 | 0,699 ms | 0,00528 | 0,465 | 0,091 | 67 % |
| 2 048 | 114 | 5,256 ms | 0,02172 | 3,531 | 0,637 | 67 % |
| 8 192 | 220 | ≈ 37–42 ms | 0,08947 | 27,75 | 4,79 | ≈ 70 % |
| 32 768 | 425 | **286,2 ms** | 0,34149 | 208,5 | 37,4 | **73 %** |

**La structure du coût est stable** : les écritures disjointes valent 67 à 73 % du pas à toutes les
tailles, les réductions 12 à 13 %. Ce qui grandit, c'est le **nombre d'itérations** — 30, 61, 114,
220, 425 — soit un doublement par raffinement, `O(√N)`, exactement la loi de S239.

**À 32 768 mailles, un pas coûte 286 ms pour une image de 16,7 ms** : δ seul est **143 fois** le
budget que ADR-125 donne à toute l'eau. Les mesures de pas aux grandes tailles sont bruitées (36,6 et
41,7 ms au même point pour 8 192 mailles, alors qu'`apply` y est stable à 0,3 % près) : ce sont les
**passes** qui font foi ici, pas le pas.

## 5. Ce qui a été tenté et n'a rien donné

`apply` parcourt ses mailles avec `i` à l'extérieur, alors que `c = k·nx + i` : la boucle interne
saute d'une rangée à chaque maille. Échanger les deux boucles est **exact au bit** — aucune maille ne
lit la sortie d'une autre, et l'ordre d'accumulation dans une maille ne change pas.

| mailles | `apply`, ordre d'origine | `apply`, boucles échangées |
|---:|---:|---:|
| 2 048 | 0,02172 ms | 0,02138 ms |
| 8 192 | 0,08947 ms | 0,08924 ms |
| 32 768 | 0,34149 ms | 0,34170 ms |

**Rien, à aucune taille** — et l'écart est sous le témoin de bruit de L321. L'échange a donc été
**annulé** : ce projet ne garde pas un changement que la mesure ne soutient pas. L'explication tient
en une ligne : le travail par maille est branchu — quatre faces, chacune avec son ouverture et son
voisin à tester — et ce sont ces branches qui tiennent le processeur, pas la distance entre deux
lectures. C'est aussi pourquoi la vectorisation automatique n'opère pas ici.

## 6. Ce que ce lot conclut

1. **La décomposition est établie et stable** : écritures 67–73 %, réductions 12–13 %, le reste hors
   boucle ; le compte d'itérations double à chaque raffinement.
2. **Le parallélisme de S243 ne peut pas servir cette boucle** : 125 µs par fil contre 21,7 µs de
   pass. Il faudrait un **vivier persistant** (A278), dont la répartition coûterait des
   microsecondes — et qui demande `unsafe` dans l'hôte.
3. **La seule technique qui attaque le terme qui grandit est la multigrille** (ou un préconditionneur
   équivalent) : elle vise les 425 itérations, pas le coût de chacune. C'est le seul levier dont le
   gain **augmente** avec la taille, et c'est donc lui qui commande le passage à la 3D.
4. **Aucun facteur n'a été gagné dans cette session**, et il faut le dire ainsi. Ce qui a été gagné
   est la carte : où va le temps, ce que chaque technique achèterait, et laquelle est fermée.

### Coût de la mesure (ADR-131)

- **Techniques présentes** : aucune de coût — Jacobi diagonal et f32, comme avant.
- **Techniques absentes** : GPU, parallélisme (fermé pour cette boucle par le §3), multigrille,
  factorisation incomplète, itérations fixes, cuisson, SIMD explicite.
- **Domaine** : 8 × 4 m, fond plat, `dt = 1/60 s`, plafond 512, release, un fil, une machine ;
  128 à 32 768 mailles.
