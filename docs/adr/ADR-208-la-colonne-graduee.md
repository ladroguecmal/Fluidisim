# ADR-208 — La colonne graduée : une pression linéaire par morceaux sous les couches cubiques

- **Statut : actée**, S386, 2026-09-26 ; autonomie technique (S71). Campagne du solveur volumique 3D, **C2**.
- **Remplace [ADR-207](ADR-207-la-campagne-du-solveur-volumique-3d.md) D2** (une colonne haute unique, à profil de pression
  linéaire, sous `k` couches cubiques), et **seulement D2** : D1, D3, D4 et D5 restent.
- **Mesure** : `outils/colonnes_hautes.py` (S386, P3–P4) et ses essais ; détail dans la preuve de C2
  ([COLONNES-HAUTES-S386](../validation/COLONNES-HAUTES-S386.md)).

## 1. Ce qui a été mesuré

La dispersion du pas linéaire de δ **se calcule** colonne par colonne (`scheme_frequency`, S295) ; une colonne haute est
une **restriction de Galerkin** du schéma fin, `Aᵣ = Pᵀ·A·P`, dont la dispersion se calcule de même. Critère d'usage écrit
avant la mesure : sur la configuration de la porte B (profondeur 7 m, `dx` 25 cm, `dt` 1/30 s) et pour λ de `4·dx` à
`2·profondeur`, l'erreur de fréquence **ajoutée** ne dépasse pas `max(|Ω_fin/ω − 1|, 10⁻⁴)`.

| représentation verticale | inconnues par colonne (28 fines) | division |
|---|---:|---:|
| colonne haute unique, linéaire (ADR-207 D2) | 19 | ÷1,47 |
| grille étirée en volumes finis | 14 | ÷2,00 |
| **pression linéaire par morceaux, nœuds étirés** | **11** | **÷2,55** |

Le pire cas est toujours la plus longue vague : une pression en `cosh` sur plusieurs mètres ne se laisse pas remplacer par
une droite. L'estimation de S384 — ÷3,1 avec huit couches cubiques et une colonne haute — ajoute jusqu'à 1,4·10⁻² d'erreur de
fréquence à λ = 14 m, environ seize fois le permis.

## 2. Décisions

**D1 — La colonne graduée.** Sous `K` couches cubiques, la pression n'est portée qu'en des **nœuds** — des centres de
mailles fines — dont les écarts croissent d'un rapport `r` vers le fond ; entre deux nœuds, elle est **linéaire**. Le
schéma est la restriction de Galerkin du schéma fin, `Aᵣ = Pᵀ·A·P` : symétrique défini positif par construction, fréquence
jamais plus basse que celle du schéma fin (Ritz), et la colonne haute unique en est le cas à deux nœuds.

**D2 — Stockage compact exact.** Le gradient d'une pression linéaire par morceaux est linéaire par morceaux à l'horizontale
et constant par segment à la verticale : vitesse horizontale **aux nœuds**, vitesse verticale **par segment**. C'est une
grille décalée à nœuds étirés ; elle ne perd rien par rapport à la restriction écrite sur la grille fine, qui en est la
référence de réception.

**D3 — `K` et `r` sont des réglages du profil** (des allocations : nœuds par colonne, I-16), fixés par domaine sur le
critère du §1, que l'outil calcule. Mesurés : porte B `r` = 1,25, `K` = 3 ; bassin de 3 m à 10 cm `r` = 1,25, `K` = 6.

**D4 — En mode mobile, `K` a un second plancher** : les couches cubiques doivent contenir **toute la course de la
surface**, car la condition de surface libre (fluide fantôme) n'est écrite que dans les mailles cubiques. Sous une houle
d'un mètre à 25 cm, c'est au moins huit couches ; le gain réel se mesurera là (C2b), et il sera **plus petit** que ÷2,55.

## 3. Ce que la décision ne fait pas

Elle ne construit rien : la 3D est l'étape suivante de S386, en mode linéaire d'abord. Elle ne dit pas comment la multigrille
de C1 grossit une colonne graduée (fusion de nœuds : C2b). Elle ne touche ni aux invariants, ni aux portes, ni à l'ordre
C1–C11, et ne retire rien du périmètre.

Invariants relus : I-05, I-06, I-14, I-16 ; aucun amendé.
