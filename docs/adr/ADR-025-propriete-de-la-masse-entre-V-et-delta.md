# ADR-025 — La propriété de la masse ne quitte jamais la couche V

- **Statut** : proposée
- **Session** : S14
- **Remplace** : le mécanisme de **transfert de propriété de masse** d'ADR-010 §6. ADR-010 n'est pas
  réécrit ; cette décision lui succède et dit pourquoi.
- **Résout** : écart de gravité 1 de
  [`AUDIT-INVARIANTS-S14`](../registres/AUDIT-INVARIANTS-S14.md) — I-04 contredit par ADR-010 §6
- **Dépend de** : ADR-008, ADR-010, ADR-021, ADR-022, SPEC-003, SPEC-004

---

## 1. Décision

> **La masse d'un nœud V lui appartient en permanence.** Un domaine δ substitutif est **amorcé**
> depuis le volume du nœud et **forcé** vers lui ; il ne le possède jamais, ne le gèle jamais, et ne
> rend aucune valeur autoritaire.

Trois conséquences immédiates, toutes en retrait de mécanisme :

- le nœud **n'est plus gelé** pendant l'épisode substitutif : V continue de l'intégrer en entiers à
  10 Hz, chez tous les participants, serveur compris ;
- `M' − M` **reste mesuré et journalisé**, mais uniquement comme **diagnostic de fuite du solveur** —
  ce qu'ADR-010 §6 disait déjà être son intérêt principal. Il n'est plus appliqué comme une perte de
  masse réelle ;
- **rien ne transite** : V étant déterministe bit à bit entre plateformes (I-03, amendé en S10), tous
  les participants intègrent la même valeur. Il n'y a ni resynchronisation ni message.

---

## 2. Le constat

ADR-010 §6, écrit en S01 :

```
V → δ :   le nœud est gelé, sa masse M est remise au domaine, le domaine s'initialise
          au niveau donné par shape_lut(M) orienté selon g_eff
δ → V :   le domaine rend M' = masse mesurée ; l'écart M' − M est reporté comme perte
          contrôlée et journalisé
```

Le texte qualifie lui-même l'opération de « transfert de propriété de masse, explicite ». C'est cette
propriété qui pose problème, et sur trois plans distincts.

### 2.1 Une issue de jeu déterminée par δ — I-04

Le volume d'eau d'une cale décide d'un chavirement, d'une ligne de flottaison, d'une perte de
portance. Ce sont des issues de jeu. Or pendant l'épisode substitutif, ce volume est celui que rend
δ — un solveur qui **n'est jamais D1** (SPEC-003 §2) et dont le résultat diffère d'un client à
l'autre.

I-04 l'interdit sans exception, et la clarification S05 ajoute que « l'argument *exception
contrôlée* n'est pas recevable ». Tout ce qui a été écrit depuis a respecté cette règle ; ADR-010 §6
la précédait de quatre sessions.

### 2.2 Le serveur n'a pas d'histoire — I-10

Le serveur exécute la couche V (ADR-022 §5.1) et n'exécute **jamais** δ (I-10). Il ne peut donc ni
geler le nœud, ni recevoir `M'`. Pendant tout l'épisode — qui dure le temps qu'une coursive
s'inonde, soit des minutes — serveur et client tiennent deux valeurs différentes du même volume, et
aucun mécanisme ne les réconcilie.

Le mécanisme était impensable côté serveur, et il a été écrit avant que les conséquences d'I-10 ne
soient travaillées.

### 2.3 Un même écart pour deux usages incompatibles

`M' − M` est présenté comme un diagnostic : « un écart systématique révèle une fuite du solveur, et
le défaut serait autrement invisible pendant des mois ». C'est juste, et c'est précieux.

Il est **aussi** appliqué comme une perte de masse réelle. Une grandeur ne peut pas être à la fois
l'erreur qu'on mesure et l'effet qu'on applique : appliquée, elle disparaît de la mesure, et le
diagnostic qui la justifiait cesse de fonctionner.

---

## 3. La résolution

C'est la même forme qu'ADR-021 §3, et pour la même raison : **si le serveur peut le calculer, il n'a
pas à le recevoir.**

### 3.1 Amorçage

À la création d'un domaine substitutif attaché à un nœud V, le domaine est initialisé au niveau
`shape_lut(volume_ml)` orienté selon `g_eff` — exactement comme aujourd'hui. Rien ne change ici, et
c'est la partie d'ADR-010 §6 qui était juste.

### 3.2 Forçage, et pourquoi il est lent

À chaque pas de V — 10 Hz, ADR-010 §4 — la masse totale du domaine δ est ramenée vers le volume
autoritaire du nœud. **Le forçage est une relaxation, pas une remise à zéro** :

```
correction appliquée par pas  =  (M_noeud − M_δ) · dt_V / τ        τ ≈ 1 s
```

Une correction instantanée à 10 Hz produirait un battement de niveau visible sur une surface calme.
Étalée sur `τ ≈ 1 s`, elle est un forçage doux, du même ordre que ceux que le solveur reçoit déjà à
ses frontières.

**Ce que cela coûte visuellement se calcule.** En régime établi, l'écart de niveau résiduel vaut
`dérive_par_s · τ · h`. Pour une tranche d'eau de 1 m :

| Dérive du solveur | Écart résiduel de niveau |
|---|---|
| 0,1 %/s | 1 mm — invisible |
| 1 %/s | 1 cm — à la limite du perceptible sur une surface calme |
| 3 %/s | 3 cm — visible |

### 3.3 Un critère d'admission que le banc n'avait pas

ADR-010 §6 remarquait qu'« un solveur δ qui perd 3 % de masse par seconde est inutilisable en régime
substitutif », sans en faire un critère. Le tableau ci-dessus le rend calculable :

> **Un solveur candidat n'est admis en régime substitutif que si sa dérive de masse maintient l'écart
> résiduel sous le seuil de perception.** Valeur de départ **1 %/s**, à calibrer au banc B3.

À noter, et c'est rassurant : le scénario d'exemple de SPEC-003 §3 assertait déjà
`derive_masse_par_s ≤ 0,001`, soit **dix fois plus strict** que cette borne visuelle. La contrainte
qui mord est donc physique, pas perceptuelle — le critère ci-dessus est un plancher de recevabilité,
pas l'exigence.

`mass_drift_per_s` est déjà rapporté à chaque pas par `StepResult` (SPEC-004 §4), « pas seulement en
test ». Rien à ajouter à l'interface.

### 3.4 Ce que δ rend, et ce qu'il ne rend pas

| Grandeur | Statut |
|---|---|
| Forme de la surface, vagues, ballottement visuel | **rendue** — c'est l'objet du domaine |
| Contribution à la pose de rendu du porteur | **rendue**, bornée (ADR-008 §1) |
| `mass_drift_per_s` | **rendue**, comme **diagnostic** |
| Masse, volume, niveau autoritaire | **jamais** — ils appartiennent au nœud |

Le ballottement qui fait chavirer un navire continue de passer par le modèle d'ADR-008 §4 — pendule
équivalent, période propre analytique, déterministe — et non par δ. Cette voie était déjà la bonne et
n'est pas touchée.

---

## 4. Ce que la décision rapporte

- **I-04 redevient vrai sans être amendé.** C'est le bon sens de la correction : on ne relâche pas la
  règle, on retire ce qui la violait.
- **Le serveur a une histoire, et elle est simple** : il intègre V, comme partout ailleurs, sans
  savoir qu'un client affiche un domaine δ au-dessus.
- **Aucun trafic réseau.** V étant D1, chaque participant dérive la même valeur. Le transfert
  supprimé n'est remplacé par aucun message — c'est le même bénéfice qu'ADR-021 §3, obtenu par le
  même raisonnement.
- **Le diagnostic redevient un diagnostic.** `M' − M` mesure la fuite du solveur au lieu de la
  consommer, et devient exploitable comme critère d'admission (§3.3).
- **Un cas canonique en découle**, à ajouter : un compartiment qui s'inonde, avec et sans domaine δ
  actif, doit donner **le même volume à tout instant**. C'est une assertion binaire, en régime D1,
  exécutable en mode `check` — parce que précisément δ n'y intervient plus.

---

## 5. Ce qui reste ouvert

1. **`τ` du forçage** — 1 s proposé. Trop court, le forçage se voit ; trop long, un solveur qui fuit
   laisse un écart durable. À calibrer avec la dérive mesurée du solveur retenu, banc **B3**.
2. **Seuil d'admission en régime substitutif** — 1 %/s de dérive est un plancher de recevabilité
   perceptuelle (§3.3) ; le seuil réel sortira de B3, et il sera probablement dicté par la physique.
3. **Cas d'un domaine substitutif couvrant plusieurs nœuds V** — une coursive ouverte sur deux
   compartiments. Le forçage porte alors sur une somme, et la répartition entre nœuds n'est pas
   spécifiée. Probablement au prorata des volumes, comme la scission des poches d'air (ADR-023 §5.4),
   mais cela demande d'être vérifié plutôt que supposé.
