# ADR-033 — `λ_cut` a deux définitions, et la dissipative est mesurable aujourd'hui

- **Statut** : proposée
- **Session** : S25
- **Tranche** : ADR-005 §2.1 — frontière W/δ ; `CAS-CANONIQUES` §C03
- **Corrige** : rien n'est réécrit. `CAS-CANONIQUES` §C03 reçoit une note corrective datée, et
  [`ADR-030`](ADR-030-l-equilibrage-est-un-critere-d-elimination.md) §5 un renvoi qui **complète**
  sa conclusion sans l'infirmer.
- **Produit** : l'exécution de **C03**, et la loi de dissipation du §2
- **Clôt** : action **S22-3**, par requalification

---

## 1. D'abord, une recommandation héritée qui était fausse

Trois sessions — S22, S23, S24 — ont recommandé C03 « avec la friction de fond », et l'action
**S22-3** existait pour ça.

**C'est l'inverse qu'il faut faire.** C03 mesure la **demi-vie d'amplitude**, c'est-à-dire la
dissipation **numérique**. Une friction **physique** ajouterait une seconde source d'amortissement,
et la mesure ne dirait plus laquelle des deux éteint la vague. L'énoncé ne demande aucune friction,
et la grandeur qu'il mesure exige qu'il n'y en ait pas.

L'erreur vient d'un raccourci compréhensible : « C03 est un cas de dissipation, la friction est de
la dissipation, donc il faut la friction ». Le mot est le même, la grandeur non.

> **S22-3 est close par requalification.** La friction de fond reste utile au véhicule — elle n'a
> simplement **aucun cas qui la réclame**. Elle est reversée aux points ouverts sans échéance.

## 2. La loi de dissipation, et sa vérification

### 2.1 C03 passe, et c'est le montage qui est en cause

| Mesure | Rampe (énoncé) | Mode propre | Seuil |
|---|---|---|---|
| période | 9,0302 s | 9,0302 s | 9,0305 s ± 1 % → **0,003 %** |
| demi-vie d'amplitude | **20,7 périodes** | **24,4 périodes** | > 15 |
| R² de l'ajustement exponentiel | 0,9920 | 0,9998 | > 0,9 |

Le montage pose `L = 20 m` et `dx = 0,1 m`. Le fondamental d'une seiche a pour longueur d'onde
`λ = 2L = 40 m` : **400 points par longueur d'onde**. Aucun domaine de jeu n'aura cette résolution.

> **C03 passe parce qu'il ne teste pas le régime dans lequel le système vivra.** C'est le même
> défaut qu'A105 pour C01 — un cas plus faible que sa réputation — et pour la même raison : son
> montage a été choisi pour être lisible, pas pour être représentatif.

### 2.2 La formule

Pour un flux de Rusanov, la diffusion numérique vaut `D = c·dx·(1−ν)/2`, où `ν` est le nombre de
Courant. L'atténuation d'un mode `k = 2π/λ` sur une période `T = λ/c` :

```
D·k²·T = [c·(λ/N)(1−ν)/2] · (2π/λ)² · (λ/c) = 2π²(1−ν)/N
```

où `N = λ/dx` est le nombre de points par longueur d'onde. **`c`, `λ` et `T` disparaissent tous les
trois** :

> **demi-vie (périodes) = ln 2 · N / (2π²(1−ν))**
>
> L'amortissement d'une onde, exprimé en périodes de cette onde, ne dépend **que** de sa résolution
> et du nombre de Courant. Ni de sa longueur, ni de sa vitesse, ni de la profondeur.

> **Note corrective S27 — cette loi a un domaine de validité en amplitude, qui n'est pas écrit
> ci-dessus.** Le balayage qui l'établit était à `a/h = 1 %` **fixe** ; il n'était donc pas confondu
> et sa conclusion tient. Mais la pente `k = demi-vie/N` n'est constante que dans ce régime :
>
> | `a/h` | N = 80 | N = 160 | N = 320 |
> |---|---|---|---|
> | **1 %** | k = 0,0635 | 0,0628 | 0,0616 — **constant** |
> | **5 %** | k = 0,0608 | 0,0520 | **0,0373** — s'effondre de 39 % |
>
> **À `a/h = 5 %`, la loi est fausse.** Le raidissement transfère de l'énergie vers les harmoniques,
> qui s'amortissent en `n²` (ADR-034) ; l'effet domine d'autant plus que `N` est grand, la
> dissipation linéaire y devenant faible. Et `a/h = 5 %` est **ordinaire en eau peu profonde** : le
> domaine de validité exclut une part des situations que la loi est censée dimensionner. Le
> découpage exact n'est pas mesuré — angle mort **A127**. Voir
> [`ADR-035`](ADR-035-le-nombre-de-courant-definition-borne-valeur.md) §5.

**Vérification.** Prédit `0,06385·N` à `ν = 0,45` ; mesuré sur six grilles, de 20 à 640 points par
longueur d'onde : `0,0640 · 0,0638 · 0,0635 · 0,0628 · 0,0616 · 0,0606`. **Écart de 0,2 %** au point
le mieux résolu par la théorie, avec `R² > 0,999` sur chaque ajustement exponentiel.

C'est une **provenance** au sens d'I-14, et non un constat.

### 2.3 Le levier : le nombre de Courant, prédit puis vérifié

La formule dit que `1−ν` commande tout. Elle prédit donc qu'élever le nombre de Courant réduit la
dissipation — ce qui n'est pas intuitif, puisqu'on associe un grand pas de temps à moins de
précision. Vérifié à `N = 160` :

| `ν` | demi-vie mesurée | prédite | écart |
|---|---|---|---|
| 0,45 | 10,10 | 10,22 | −1,1 % |
| 0,70 | 17,91 | 18,73 | −4,4 % |
| 0,90 | **45,33** | 56,18 | −19,3 % |

**×4,5 de demi-vie entre 0,45 et 0,9 — et le pas de temps double au passage.** Moins de dissipation
*et* moins de calcul, sur le même schéma et la même grille.

La loi se dégrade près de `ν = 1` : elle néglige les termes d'ordre supérieur en `k·dx`, et
l'intégration d'Euler explicite y a sa propre erreur. **Prédictive à mieux que 5 % pour `ν ≤ 0,7`**,
ce qui est la plage exploitable — au-delà, la marge de stabilité devient le facteur limitant.

## 3. Décision : `λ_cut` a deux définitions, et il faut les nommer séparément

ADR-005 §2.1 définit `λ_cut` comme *« la plus petite longueur d'onde que δ transporte
correctement »*. Le mot **correctement** n'a jamais été qualifié, et C02 devait le faire par la
**dispersion** — l'erreur de célérité.

[ADR-030](ADR-030-l-equilibrage-est-un-critere-d-elimination.md) §5 a montré que cette mesure
demande une couche dispersive, que le véhicule Saint-Venant n'est pas. La conclusion tient. **Mais
elle n'épuisait pas la question**, parce qu'une onde peut être mal transportée de deux façons
indépendantes.

> **Décision. `λ_cut` se décompose en deux grandeurs, à mesurer et à nommer séparément :**
>
> | | Ce qu'elle borne | Comment elle se mesure | État |
> |---|---|---|---|
> | **`λ_cut` dispersif** | l'onde arrive **au mauvais moment** — erreur de célérité | C02, sur une couche dispersive | **bloqué** — ADR-030 §5 |
> | **`λ_cut` dissipatif** | l'onde **n'arrive pas** — elle s'est éteinte en route | C03, formule du §2.2 | **mesurable aujourd'hui** |
>
> **La frontière W/δ est commandée par le plus contraignant des deux**, et rien ne garantit que ce
> soit toujours le même.

### 3.1 Ce que le `λ_cut` dissipatif vaut, en chiffres

En posant l'exigence « une onde doit survivre `X` périodes », la formule du §2.2 donne directement :

```
N_min = 2π²(1−ν)·X / ln 2
```

| Exigence | `ν = 0,45` | `ν = 0,70` | `ν = 0,90` |
|---|---|---|---|
| survivre 15 périodes (seuil de C03) | **235 pts/λ** | 128 | 43 |
| survivre 5 périodes | 78 | 43 | 14 |
| survivre 2 périodes | 31 | 17 | 6 |

**Le seuil de C03 est très exigeant** : 235 points par longueur d'onde à `ν = 0,45`. À l'inverse,
une houle qui doit seulement traverser un domaine de quelques longueurs d'onde s'accommode de 30 à
80 points.

**Ce que cela dit du coût d'un domaine.** Une houle de `λ = 100 m` — période de 8 s en eau profonde
— demanderait `dx = 0,43 m` pour survivre 15 périodes à `ν = 0,45`, et `dx = 1,3 m` pour en survivre
5. Le rapport entre les deux exigences est de **trois en résolution, donc vingt-sept en coût 2D** :
c'est un arbitrage de conception, pas un détail de réglage.

## 4. Conséquences immédiates

1. **Le nombre de Courant devient un paramètre de conception**, et non un réglage de stabilité. Il
   ne figure nulle part dans le corpus comme grandeur à choisir. Angle mort **A117**.
2. **Toute exigence de portée doit être exprimée en périodes**, pas en mètres ni en secondes : la
   loi du §2.2 montre que c'est la seule formulation dont la réponse ne dépende pas de l'onde.
3. **`CAS-CANONIQUES` §C03 doit dire sa résolution.** Un cas dont le verdict dépend d'un paramètre
   que l'énoncé ne mentionne pas est un cas qui mesure ce paramètre — ici, 400 points par longueur
   d'onde.
4. **Le seuil « 15 périodes » de C03 n'a pas de provenance** au sens d'I-14, comme les seuils de
   C01 (A106) et de C04. Il en a désormais une **conséquence** chiffrée — 235 points par longueur
   d'onde — ce qui permet enfin de discuter s'il est le bon.

## 5. Ce qui reste ouvert

1. **La formule n'est vérifiée que sur ce schéma.** `D = c·dx·(1−ν)/2` est la diffusion de Rusanov ;
   un autre flux, un autre ordre, une autre intégration temporelle donneront un autre coefficient.
   **Ce qui se transporte est la forme** — `demi-vie ∝ N`, indépendante de `λ` et de `c` — et la
   méthode : mesurer le coefficient une fois, en déduire `N_min`.
2. **Le régime `ν → 1` n'est pas couvert.** L'écart de 19 % à `ν = 0,9` demande soit un terme
   correctif, soit une borne d'emploi déclarée. Angle mort **A118**.
3. **Aucune mesure n'a été faite sur un mode non fondamental.** La rampe de l'énoncé en excite, et
   l'écart de demi-vie entre rampe et mode propre — 20,7 contre 24,4 — le montre. La loi prédit que
   l'harmonique `n` s'amortit `n` fois plus vite ; ce n'est pas vérifié.

   > **Note corrective S26.** *Cet énoncé était ambigu, et faux dans la lecture la plus naturelle.*
   > « `n` fois plus vite » ne vaut qu'en **périodes propres de l'harmonique**. En **secondes**, le
   > facteur est **`n²`** : le mode `n` a `N_n = N₁/n` points par longueur d'onde *et* une période
   > `T_n = T₁/n`, et les deux effets se composent. Mesuré en S26 à `nx = 400`, `ν = 0,45` — rapports
   > de demi-vie en secondes : **3,99 · 8,85 · 15,59** pour 4 · 9 · 16 attendus. Le point est clos
   > par cette mesure ; voir [`ADR-034`](ADR-034-la-dissipation-est-un-filtre-passe-bas.md).
4. **`λ_cut` dispersif reste bloqué** sur l'absence de couche dispersive (ADR-030 §5). La décision
   du §3 ne le débloque pas : elle établit que l'autre moitié de la question, elle, est ouverte.
