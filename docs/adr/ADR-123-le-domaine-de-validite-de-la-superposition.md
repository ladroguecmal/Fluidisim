# ADR-123 — Le domaine de validité de la superposition perturbative

- **Statut : actée**, S194, 2026-09-12 ; autonomie technique S71.
- **Ne remplace aucun ADR.** Donne à [ADR-112](ADR-112-la-superposition-independante-ne-recoit-pas-le-couplage.md)
  la **grandeur** qui lui manquait : ADR-112 interdisait de recevoir le couplage par une
  superposition ; il ne disait pas de combien elle s'en écarte.
- Conserve le seuil de **2 %** d'[ADR-120](ADR-120-b4-tolerance-de-deux-pour-cent.md) et
  l'ordre en amplitude d'[ADR-122](ADR-122-l-ordre-en-amplitude-d-un-vehicule-non-lineaire.md).
- Ne choisit **aucune** technologie δ et **ne fixe aucun seuil de bascule W/δ**.
- Mesures : [COUPLAGE-DEUX-TRAINS-S194](../validation/COUPLAGE-DEUX-TRAINS-S194.md) §7.

## Décision

**Additionner les champs de deux sources évoluées indépendamment satisfait le critère de
2 % seulement en dessous d'une cambrure d'environ `0,009` par train, en eau profonde et
sur un couple colinéaire.** Au-dessus, la validité dépend de la durée d'observation, et
elle cesse très vite :

| cambrure par train | durée pendant laquelle l'écart reste sous 2 % |
|---|---|
| `s ≤ 0,008` | **au moins 20 périodes** (horizon mesuré) |
| `s = 0,009` | 19,8 périodes |
| `s = 0,010` | 11,7 périodes |
| `s = 0,0125` | 5,4 périodes |
| `s ≥ 0,014` | **moins d'une période** |

**Le domaine est donc un domaine en (cambrure × durée), mais le levier de la durée est
étroit** : tout ce qui dépend du temps est enfermé dans la bande `0,009 ≤ s ≤ 0,014`, un
facteur 1,6 en cambrure. En dessous, la superposition tient sur tout l'horizon mesuré ;
au-dessus, elle est fautive avant la fin de la première période. **On n'achète pas la
validité en regardant brièvement.**

## Ce que la décision énonce, au-delà du chiffre

### 1. La variable qui gouverne l'addition en eau profonde est la cambrure — A217 a sa réponse

A217 posait la question depuis S161 : en eau profonde, quelle variable gouverne la validité
de l'addition ? Elle supposait « vraisemblablement la cambrure, et rien ne le vérifie ».
**C'est la cambrure**, et la mesure donne sa loi :

```
écart / A  ≈  α·s + β·s²·N        α = 1,302602   β = 5,898728
```

résidu d'ajustement à **1,82 %** de l'écart maximal. Mais A217 ignorait **deux** variables
supplémentaires, que la dérivation a nommées avant la mesure et que la mesure a confirmées :
la **durée** `N`, et le **désaccord de triade** — donc la profondeur.

### 2. Deux mécanismes, deux lois, et ils ne se remplacent pas

| part | mécanisme | modes | loi | croissance mesurée |
|---|---|---|---|---|
| `α·s` | harmoniques liées croisées, forçage quadratique | `k₁±k₂` | pente **1,0041 / 1,0365** | **stationnaire**, rapport 1,002 à 1,023 |
| `β·s²N` | modulation croisée de fréquence, forçage cubique résonant | `k₁`, `k₂` | pente **2,0178 / 2,0627** | **×4,1** sur vingt périodes |

La part quadratique ne croît pas parce qu'**aucune triade n'est résonante en eau profonde** —
`√(k₁+k₂) = √k₁+√k₂` impose `√(k₁k₂)=0`, et l'argument est vectoriel, donc il survit à
l'obliquité. La part cubique croît parce qu'elle forçe le mode libre lui-même. Les deux
mécanismes vivent sur des **modes différents** et ont été mesurés séparément, sans
ajustement.

### 3. Le couple importe peu, la profondeur beaucoup

L'écart varie de **13 % au plus** sur quatre géométries de couple — rapports de nombres
d'onde 1,5, 2 et 3, et un renversement du sens de propagation. **Pour décider si deux
sources peuvent être superposées, il suffit donc de connaître leurs cambrures, pas leur
géométrie relative** — dans cette famille colinéaire.

La profondeur, elle, est décisive. À cambrure fixée, le coefficient `α` vaut `1,383` à
`h=8 m` et **`11,855`** à `h=0,25 m`, un facteur **8,6**, quand le désaccord de triade
`Δ = ω₁+ω₂−ω(k₁+k₂)` tombe de `2,470` à `0,508`. Le mécanisme est celui du §2.1 de la
mesure : en faible profondeur la dispersion s'affaiblit, les triades approchent la
résonance, et la part quadratique — la plus grosse — cesse d'être bornée. **Une frontière
de validité établie en eau profonde ne se transporte pas vers le rivage.**

### 4. Un ordre de troncature trop bas fait paraître la superposition meilleure qu'elle n'est

Mesuré à `M=2` : `β = 1,744764` contre `5,898728` à `M=3`, un facteur **3,4** ; lu
directement sur la part de train, facteur 3,09. **Un véhicule tronqué à l'ordre deux
sous-estime la part cumulative du couplage d'un facteur trois**, dans le sens rassurant.
C'est un argument indépendant pour ADR-122, obtenu sur une grandeur — le couplage de deux
trains — que ADR-122 n'avait pas mesurée.

## Le chiffre qui met la décision en perspective

À `s = 0,0125`, S193 recevait un train **unique** contre Stokes à **0,4555 %** d'erreur.
**Deux** trains de la même cambrure, superposés indépendamment, franchissent le même budget
de 2 % en **5,4 périodes**. Le modèle n'est pas en cause : c'est l'addition qui est fausse,
d'un facteur **quarante** à cambrure égale. ADR-112 avait raison ; voilà son ordre de
grandeur.

## Ce que la décision ne fait pas

1. **Aucun seuil de bascule W/δ.** A216 reste inexpliquée ; ni `0,02` ni `0,24` ne se
   dérivent d'ici. Une frontière de validité de la superposition ne dit rien du coût, de la
   latence ni du raccord entre couches.
2. **Deux trains ne sont pas `n` sources.** Le nombre de paires croît comme `n²`, et rien
   de ce qui est mesuré ici ne s'extrapole. C'est l'objet de **A240**, et c'est la limite la
   plus lourde de cet ADR.
3. **Une seule dimension horizontale, un couple colinéaire.** La dépendance angulaire de
   `α` n'est pas mesurée.
4. **Fond plat.** La ligne de profondeur mesure `α` à fond constant ; la bathymétrie
   variable reste entière.
5. **La source B+W de S190/S191 n'est pas branchée**, et ces écarts ne s'additionnent à
   aucun budget mesuré sur une autre référence.
6. **Aucun contrat runtime** : `f64`, allocations de banc, aucune prétention
   I-03/I-06/I-08 ; `water-core` et le support S193 sont inchangés.
7. **Aucune seconde cible** (A98), aucune mesure de coût CPU.

## Ce qu'il faudrait pour l'inverser

Trois voies, et elles ne se valent pas.

- **Un terme de correction croisée.** L'écart n'est pas une erreur aléatoire : c'est la
  réponse à un forçage croisé **explicite**, `2Q(a,b) + 3C(a,a,b) + 3C(a,b,b)`. Une
  couche perturbative qui l'évaluerait — au moins sa part quadratique, qui est bornée et
  non cumulative — récupérerait `α·s`, c'est-à-dire l'essentiel près de la frontière. C'est
  la voie la plus prometteuse, et elle est chiffrée : à `s=0,0125` la part croisée vaut
  `1,478·10⁻²` et la part de train `1,166·10⁻²`, donc corriger la première ramènerait déjà
  l'écart sous le budget sur toute la fenêtre.
- **Un budget d'erreur plus large pour les sources multiples.** Décision de l'utilisateur,
  non de session ; ADR-120 fixe 2 % et n'est pas redemandé.
- **Une observation brève.** Écartée par la mesure : au-dessus de `s=0,014` le budget est
  franchi avant la fin de la première période.

---

**Note S197 — 2026-09-12 : audit de résolution, la décision est confirmée.**
A242 a montré que le critère d'énergie employé par S194 ne détecte pas la sous-résolution.
La table mesurée du §7.8 de S194, qui porte cette décision, a donc été **rejouée** en ne
changeant que le nombre de niveaux verticaux `K` — la seule chose dont dépende le symbole de
dispersion. À `K = 64` elle reproduit exactement le publié ; à `K = 256` et `K = 1024`, deux
résolutions qui donnent le même résultat, elle se déplace de **3,3 % au maximum**. Toutes les
lignes du tableau ci-dessus tiennent : « jamais » sous `0,008`, ~19 périodes à `0,009`, ~5 à
`0,0125`, sous une période à `0,014`. **Rien n'est réécrit ; la décision est confirmée.**

Une réserve, et elle est neuve : l'ajustement `α·s + β·s²·N` se déplace davantage que la
table — `α` de 3,7 % — et ses **extrapolations hors de sa plage de calibration**
(`s ∈ [0,0125 ; 0,1]`) bougent jusqu'à 35 % à `s = 0,014`. Cette décision ne repose pas sur
elles, mais une session qui emploierait la loi loin de sa calibration devrait le savoir.
Voir [AUDIT-RESOLUTION-S197](../validation/AUDIT-RESOLUTION-S197.md) §8.1.
