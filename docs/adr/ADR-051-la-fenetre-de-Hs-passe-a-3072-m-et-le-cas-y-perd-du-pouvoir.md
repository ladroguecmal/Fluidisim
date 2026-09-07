# ADR-051 — La fenêtre de `Hs` passe à 3072 m, et le cas y perd du pouvoir de détection

- **Statut** : proposée
- **Session** : S64
- **Tranche** : action **S62-1**
- **Corrige** : rien n'est réécrit. L'action **S62-1** avançait **deux** raisons de ne pas élargir
  la fenêtre — une correction de sommation à faire d'abord, et un coût multiplié par 64. **Les
  deux étaient fausses**, et aucune n'avait été éprouvée.
- **Applique** : **A102** mesuré en S62, **A184** corrigé en S62
- **Produit** : la fenêtre portée à 3072 m dans `C18-invariants.toml` ; le comptage des points
  hors de portée de l'ancre
- **Clôt** : **S62-1**. **Ouvre** : **A186**, **A187**, actions **S64-1** et **S64-2**

---

## 1. Les deux raisons de ne pas le faire, et pourquoi elles ne tenaient pas

**S62-1 disait** : *« corriger d'abord la sommation (Welford), puis décider la fenêtre — 54,7 λ la
rendraient juste à 0,28 %, mais pour 64 fois le coût, et à 12288 m la variance en une passe rend
`NaN` »*.

### 1.1 Le `NaN` ne venait pas de la sommation

Il vient de `to_local` (`types.rs`), qui refuse tout point à plus de **4096 m de l'ancre**. Mesuré
à six mètres près : demi-fenêtre **4092 m → 0,505 %**, demi-fenêtre **4098 m → `NaN`**. La
correction de sommation prescrite n'avait pas lieu d'être ; l'ordre de grandeur ne soutenait pas
l'hypothèse non plus — à 16,7 millions de points, `somme2/n ≈ 0,09` et `moyenne² ≈ 0`.

Ce qui manquait n'était pas la précision, c'était le **motif du refus** : le cas échouait
correctement, sans dire pourquoi. Il compte désormais les points hors de portée et l'écrit dans son
libellé. **La mesure reste refusée** — écarter les points invalides et mesurer sur le reste
fabriquerait une variance sur un domaine amputé, la faute exacte corrigée en S45 sur C10 (**A173**).

### 1.2 Le coût n'est pas multiplié par 64

| | mode `physics` complet |
|---|---:|
| fenêtre nominale, 128 × 3 m | **33,9 s** |
| fenêtre 1024 × 3 m — 64 fois plus de points | **35,9 s** |

**+2 secondes, soit +6 %.** L'échantillonnage de `Hs` est marginal devant les solveurs `δ` que le
mode exécute par ailleurs. Le facteur 64 était exact — sur la mesure seule, qui ne pèse rien.

> Une action a donc avancé deux motifs techniques, tous deux faux, tous deux écrits par la session
> qui ne les exécutait pas. C'est le défaut recensé en **S63** (**A181**, **L177**) — **commis par
> la session immédiatement précédente, dans l'action même que celle-ci exécute.**

## 2. La décision

### D1 — La fenêtre passe à 3072 m (1024 × 3 m)

Elle est déclarée dans `C18-invariants.toml`, section `[physics]`, depuis que S62 a rendu ces
paramètres déclarables (**A184**). Le chiffre publié pour `Hs` passe de **8,528 %** à **0,282 %**.

Six configurations mesurées à cette fenêtre :

| | `tp` = 4 | `tp` = 9 | nominal | `hs` = 4,8 | 64 comp. | **256 comp.** |
|---|---:|---:|---:|---:|---:|---:|
| écart | 0,152 % | 0,184 % | 0,282 % | 0,282 % | 0,367 % | **6,612 %** |

La deuxième colonne est le témoin qui décide : à `tp = 9 s`, le cas **échouait** à la fenêtre
nominale — 15,58 % — et rend 0,184 % à 3072 m. La correction ne rend pas seulement le chiffre plus
joli sur la configuration retenue : **elle répare un cas qui échouait ailleurs.**

### D2 — La tolérance reste à 10 %, et le cas y perd du pouvoir de détection

C'est la conséquence gênante, et elle doit être écrite plutôt que découverte plus tard.

**Avant** : écart nominal 8,53 %, tolérance 10 %. Une erreur de chaîne de 2 % portait l'écart à
10,5 % — **le cas tombait**. **Après** : écart nominal 0,28 %. La même erreur de 2 % porte l'écart
à 2,3 % — **le cas passe**. En corrigeant la justesse sans toucher la tolérance, on fait passer la
marge réelle de 1,5 point à 9,7 points : *le cas devient plus juste et moins sévère.*

**La tolérance n'est pourtant pas resserrée**, pour deux raisons mesurées :

1. **256 composantes rendent 6,612 %**, et cet écart ne vient ni de la fenêtre ni du pas — mesuré
   à pas 3,0, 1,5 et 1,0 m à fenêtre égale : 6,612 %, 6,614 %, 6,615 %. **La cause n'est pas
   identifiée** (**A187**). Toute tolérance inférieure à 7 % exclurait une configuration que rien
   ne permet aujourd'hui de déclarer invalide.
2. **Aucune barre d'erreur n'est mesurable.** Le générateur ne produit **qu'une réalisation par
   état de mer** : `phase0 = i × 0x9E3779B9`, sans PRNG, et le paramètre `graine` du scénario —
   obligatoire — n'est utilisé **nulle part** (**A186**). Six graines rendent six fois le même
   chiffre. Une tolérance calibrée sur un échantillon unique n'aurait pas de provenance (I-14,
   **A106**).

**Resserrer la tolérance est donc bloqué par deux faits, pas par prudence** : l'un est un défaut à
élucider (A187), l'autre une pièce manquante du générateur (A186). Les deux sont ouverts en
actions, et **S64-1 débloque S64-2**.

### D3 — Ce que la décision ne change pas

Les deux hashs `check` sont inchangés — `physics` et `check` ne partagent pas ce chemin. Aucune
autre mesure du corpus ne bouge. La tolérance, la référence et la définition `Hs = 4√m₀` sont
intactes.

## 3. Ce qu'il faudrait pour inverser cette décision

| Inverser | Ce qu'il faudrait |
|---|---|
| **D1** (revenir à 384 m) | que le coût de `physics` devienne critique — il est aujourd'hui de +6 % — ou qu'un usage exige une fenêtre courte. À `tp ≥ 9 s`, revenir en arrière **rouvrirait un échec**. |
| **D2** (resserrer la tolérance) | élucider A187, **ou** brancher la graine et mesurer la dispersion sur des réalisations indépendantes. C'est l'ordre naturel : S64-1 puis S64-2. |

> **Note corrective S65 — 2026-09-08.** La graine commande désormais les phases. Les valeurs
> S64 décrivent la réalisation historique : au nominal courant Hs vaut 1,216660 m (+1,388 %),
> et six graines à 256 composantes donnent −7,270 à +5,844 %. A186 corrigé pour le paramètre
> mort ; calibration et A187 restent ouverts. Fenêtre et tolérance inchangées.
> Voir [GRAINES-S65](../validation/GRAINES-S65.md).

> **Note corrective S67 — 2026-09-08.** A187 est expliqué : les covariances sur fenêtre finie
> reproduisent les 6,612 % historiques. La somme des variances individuelles rend 1,200001971 m ;
> à fenêtre doublée, Hs total vaut 1,218126498 m. Les battements entre composantes voisines
> atteignent 20,208 km. L’affirmation excluant la fenêtre était trop forte ; seul le pas avait
> été éprouvé à fenêtre constante. S64-3 close, S64-2 à instruire avec calibration d’ensemble.
> Fenêtre nominale et tolérance inchangées. [SPECTRE-DENSE-S67](../validation/SPECTRE-DENSE-S67.md).
