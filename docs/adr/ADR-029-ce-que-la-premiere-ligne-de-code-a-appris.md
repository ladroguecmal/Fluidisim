# ADR-029 — Le langage, et ce que la première ligne de code a appris

- **Statut** : proposée
- **Session** : S20
- **Tranche** : ADR-020 §7.1 — langage et cible du cœur
- **Corrige** : ADR-003 §2 (liste incomplète) · SPEC-004 §8.2 (corollaire sous condition) ·
  ADR-028 §4 (règle trop large)
- **Produit** : `code/water-core` et `code/water-harness`, étage **H1** de SPEC-003 §10

---

## 1. Le langage : Rust

ADR-020 §7.1 laissait le langage ouvert, « décision de l'équipe technique ». Il n'y a pas d'équipe
technique (ADR-028 §2) ; la décision se prend ici.

> **Décision. Le cœur et le harnais sont écrits en Rust, sans aucune dépendance externe.**

Trois motifs, dont le premier est empirique et décisif.

### 1.1 Ce que la machine permet de vérifier

Inventaire de l'environnement de travail : `rustc 1.97` et `cargo` présents ; **aucun compilateur
C++** — ni `cl`, ni `g++`, ni `clang`, ni `cmake`.

Écrire le cœur en C++ produirait du code que je ne peux **ni compiler ni exécuter** : c'est-à-dire
exactement ce que H1 doit cesser de produire. Un projet qui a écrit dix-neuf sessions de conception
sans rien exécuter ne peut pas se permettre une vingtième où le livrable est encore invérifiable.

### 1.2 Le déterminisme survit à la négligence

I-03 exige que `B` soit déterministe bit à bit **entre plateformes**. En C++, `a*b + c` est fusionné
en FMA **par défaut** chez GCC et Clang — `-ffp-contract=fast` — ce qui change le résultat dans les
derniers bits. La propriété reste atteignable, à condition qu'un drapeau ne soit jamais oublié.

Rust ne contracte pas les opérations flottantes : la fusion n'a lieu que sur appel explicite à
`mul_add`. La propriété critique **ne dépend donc de la vigilance de personne**.

C'est **L64**, écrite en S19 : entre deux options également capables, choisir celle dont la propriété
critique survit à la négligence.

### 1.3 L'absence d'allocation est lisible

I-06 interdit toute allocation à l'exécution. En Rust, chaque allocation est visible dans le texte —
il n'existe pas d'équivalent d'un conteneur de bibliothèque standard qui alloue sans qu'on l'ait
demandé. L'invariant se relit au lieu de se surveiller.

### 1.4 Ce que cela coûte, honnêtement

Un moteur écrit en C++ intégrera le cœur par une frontière C. Ce n'est pas un surcoût : cette
frontière était **déjà conçue** — SPEC-004 §1.4 impose un `ABI_VERSION` par interface et des
structures qui ne grandissent que par ajout en fin, avec un champ `size` en tête. C'est une ABI C,
décrite comme telle depuis S04.

### 1.5 Si la décision devait être inversée

Le corpus n'en dépend pas : SPEC-004 est écrite en pseudo-C++ « qui se transpose sans perte ». Ce
qu'il faudrait ajouter en repassant à C++ est la discipline de §1.2 — `-ffp-contract=off`, pas de x87
étendu, ordre fixé — et sa vérification au hash inter-plateformes du cas C18.

---

## 2. ADR-003 §2 est incomplet : les fonctions transcendantes

**C'est la trouvaille de la session, et elle est apparue à la première ligne de code.**

ADR-003 §2 énumère les disciplines qui assurent le déterminisme bit à bit : « toute réduction
parallèle est déterministe, toute source d'aléa est un PRNG à état entier, le nombre de fils ne
change pas le résultat ». La liste est juste et elle est **incomplète**.

> `B` est une somme de sinusoïdes, et **`sin` n'est pas spécifié bit à bit**. IEEE 754 impose
> l'exactitude des quatre opérations et de la racine carrée ; **jamais celle des transcendantes**.
> Deux `libm` — deux plateformes, ou deux versions de la même — diffèrent dans les derniers bits.

Un hash de conformité les aurait distingués à chaque frame, et le défaut se serait présenté comme
une divergence de plateforme sans cause apparente — la catégorie la plus coûteuse à diagnostiquer.

### 2.1 La correction n'invente rien

ADR-003 §2.2 pose déjà que « le temps ne transite jamais en `f32` : seules des **phases repliées**
passent au GPU ». Le même mécanisme résout le problème :

- une phase est un `u32` valant une **fraction de tour**, `2³²` unités par tour. Le repliement modulo
  un tour est le **débordement naturel de l'entier** : exact, et gratuit ;
- la part temporelle se calcule **entièrement en entiers** depuis `SimTime` en microsecondes —
  `freq_q32 · t_µs / 10⁶` sur 128 bits. Aucun flottant ne voit le temps, ce qu'I-08 exigeait déjà ;
- le sinus est un polynôme à coefficients fixes, évalué en `f32` dans un ordre explicite, n'employant
  que `+`, `−` et `×`. Ces trois opérations **sont** spécifiées exactement.

Fidélité mesurée contre la référence `f64` : écart maximal **inférieur à 10⁻⁷**, soit sous l'ulp d'un
`f32` d'ordre 1. Le test est dans `phase.rs`.

### 2.2 Portée

La règle vaut pour **toute** fonction transcendante du système, pas seulement le sinus : exponentielle
d'atténuation optique, `tanh` du ballottement, racines de la dispersion. Chacune devra recevoir le
même traitement lorsqu'elle entrera dans une grandeur répliquée.

**Note corrective à porter dans ADR-003 §2**, et invariant I-03 inchangé — c'est sa mise en œuvre qui
était incomplète, pas son énoncé.

---

## 3. `grain` appartient au contrat de la réduction ordonnée

SPEC-004 §8.2 énonce : « **changer `worker_count` change la vitesse, jamais le résultat.** C'est une
assertion du harnais, pas une intention. »

L'assertion est vraie **à condition que le découpage soit fixé**. L'addition flottante n'étant pas
associative, deux grains différents donnent deux sommes différentes :

```text
valeurs = [1,0 ; 10¹⁶ ; −10¹⁶ ; 1,0]

grain 1 : ((1 + 10¹⁶) − 10¹⁶) + 1  =  1,0
grain 2 : (1 + 10¹⁶) + (−10¹⁶ + 1)  =  0,0
```

Si un système de tâches choisit son grain en fonction du nombre de fils disponibles — ce qui est le
réglage naturel —, alors changer `worker_count` **change le résultat**, et le corollaire est faux
silencieusement.

> **Décision. `grain` est une donnée du contrat, fixée par l'appelant, et jamais dérivée du nombre de
> fils de la machine.** L'assertion du harnais devient : *à `n` et `grain` égaux, le résultat est
> identique quel que soit `worker_count`.*

Le cas est consigné sous forme exécutable dans `host_impl.rs` — un test qui échouerait si la
propriété cessait d'être démontrée par ses propres données.

---

## 4. Seuil d'acceptation et référence de non-régression ne suivent pas la même règle

ADR-028 §4, écrit la veille, pose qu'« un seuil d'acceptation s'écrit **avant** la mesure qu'il juge,
dans un commit qui la précède ». L'usage a montré que la règle est **trop large**.

| | Ce que c'est | Peut-on l'écrire avant ? |
|---|---|---|
| **Seuil d'acceptation** — `derive_masse ≤ 0,001`, `budget_p99 ≤ 2 ms` | une barre que le travail doit franchir | **oui, et il le faut** : c'est la barre qu'on est tenté d'abaisser |
| **Référence de non-régression** — un hash de conformité | la valeur qu'on obtient aujourd'hui | **non, par construction** : elle *est* la mesure |

Une référence de non-régression est nécessairement postérieure. Ce qui la protège n'est donc pas
l'antériorité mais la **visibilité** :

> **Décision. Une référence de non-régression est inscrite par un commit qui ne contient rien
> d'autre.** L'outil l'imprime et ne l'écrit jamais lui-même : `bless` affiche la valeur, un humain
> la recopie. Le confort d'une réécriture automatique coûterait exactement la propriété recherchée —
> qu'un déplacement de référence soit un **acte**, et non un effet de bord d'une exécution.

La session a suivi sa propre règle : le commit qui produit le code et celui qui inscrit les hashs
sont séparés, et l'historique le montre.

---

## 5. Ce que H1 fait, et ce qu'il ne fait pas

**Fait** — `cargo test` : 14 tests. `water-harness check scenarios/*.toml` : deux scénarios, hash de
conformité, allocations après scellement, reproductibilité entre deux constructions.

| Propriété vérifiée | Invariant | Comment |
|---|---|---|
| `seal()` fait **échouer** l'allocation | I-06 | le harnais en tente une délibérément et compte le refus |
| Le hash de conformité est stable | I-03 | deux constructions indépendantes, comparées |
| `B` est une fonction pure de `(x, t)` | I-02 | deux évaluations du même point, comparées bit à bit |
| La résolution monde égale l'ulp d'un `f32` au rayon | ADR-028 §3 | test d'égalité exacte |
| Une différence d'un ulp change le hash | I-03 | test dédié |
| La batterie tient dans son budget | SPEC-003 §1 | mesuré et rapporté : **0,04 s** contre 60 |

**Ne fait pas.** Ni `W`, ni `δ`, ni `V`. Le `B` implémenté est **minimal et le dit** : le nombre de
composantes, le découpage en bandes et le choix Gerstner contre tuile FFT restent ouverts et se
tranchent au banc B1. Rien dans le code ne préjuge de ce résultat.

**Et le budget est tenu avec une marge trompeuse** : 0,04 s pour deux scénarios n'annonce rien de ce
que coûteront les seize cas canoniques avec `W` et `δ`. La marge est réelle, la mesure est petite, et
il faut dire les deux.

---

## 6. Ce qui reste ouvert

1. **Le `B` de B1.** Ce qui est écrit exerce la chaîne ; il ne prétend pas être le champ de fond.
2. **Les autres transcendantes** (§2.2) — chacune devra recevoir le même traitement en entrant dans
   une grandeur répliquée. Aucune n'est encore employée.
3. **Le hash inter-plateformes n'est pas vérifié.** Une seule plateforme a exécuté ce code. Le hash
   est *déterministe par construction*, ce qui n'est pas la même chose que *vérifié identique
   ailleurs* — et c'est précisément ce que le cas C18 demande. La vérification attend une seconde
   machine.
4. **H2 à H6** — métriques et séries temporelles, cas canoniques analytiques, oracle lent,
   `capture`, `replay` et `starve`. H1 débloque la CI par commit, rien de plus.
5. **La frontière C** de §1.4 n'est pas écrite. Elle n'a pas d'objet tant qu'aucun moteur n'intègre
   le cœur.


---

## Note S21 — ce que le premier cas analytique a trouvé

Le tableau du §5 énumère six propriétés vérifiées par H1. **Les six portent sur la
reproductibilité et sur la discipline d'exécution ; aucune ne porte sur la justesse du champ.** Cette
absence n'était pas énoncée comme une limite — le « ne fait pas » du §5 parlait des couches manquantes,
`W`, `δ`, `V`, pas de ce que H1 laisse passer sur la couche qu'il couvre. S21 l'a rendue explicite, à
ses dépens.

**Le défaut.** Dans `background.rs`, la vitesse orbitale de surface était calculée **en quadrature**
avec l'élévation :

```text
écrit    :  u = a·ω·cos(φ)   w = a·ω·sin(φ)
correct  :  u = a·ω·sin(φ)   w = a·ω·cos(φ)      (Airy, eau profonde, avec η = a·sin(φ))
```

Conséquence physique : **sous une crête, l'eau n'avançait pas**, elle montait. Un bateau posé sur ce
champ aurait été soulevé sans être entraîné — précisément le défaut qu'ADR-008 §2 signale comme
« immédiatement perceptible » et que la vitesse orbitale existe pour éviter.

**Ce qui ne l'a pas trouvé.** Dix-neuf sessions de conception. Six audits du corpus. Le hash de
conformité de H1, parfaitement stable d'une exécution à l'autre — parce qu'il l'était : le champ
était reproductible, et faux.

**Ce qui l'a trouvé, en un passage.** Une identité fermée qui ne dépend d'aucun paramètre du code :
pour une composante unique en eau profonde, `u_horizontal = ω · η` **en tout point**. Le cas mesure
le rapport au point d'élévation maximale et le compare à `ω`. Sous une crête, `sin(φ) = 1` et donc
`cos(φ) = 0` : le code inversé rendait une vitesse horizontale **nulle** là où la référence vaut `ω`.
L'écart mesuré était de 100 %, soit l'écart maximal qu'une telle comparaison puisse produire. Après
correction : **0,000 %**.

**Ce que cela ajoute au §5.** Le tableau des propriétés vérifiées doit se lire avec sa ligne
manquante : **rien dans H1 ne juge la physique**. La frontière entre H1 et H3 n'est pas une gradation
de rigueur mais une séparation de nature — aucun raffinement du premier n'approche ce que fait le
second. À rajouter au « ne fait pas » du §5, qui ne nommait que les couches absentes.

**Trois autres enseignements du même passage**, plus modestes :

- **deux des quatre échecs venaient de mes tests, pas du champ** — un temps rendu dans la mauvaise
  unité, et un échantillonnage au-delà du rayon de référentiel. Le second est instructif : le champ
  refusait correctement, `eval` renvoyant `None` conformément à I-08. **Une erreur de test qui
  révèle une propriété mérite que cette propriété devienne un cas** ; elle en a un ;
- **une référence tirée des paramètres ne prouve rien.** `configure` calcule `k = ω²/g` ; comparer
  `k` aux paramètres n'aurait vérifié que ma propre arithmétique. Les cas mesurent donc des
  longueurs d'onde, des périodes et des variances **dans le champ échantillonné** ;
- **une mesure statistique a besoin de sa fenêtre.** La restitution de `Hs` par la variance donne
  8,5 % d'écart sur le scénario à 32 composantes : la plus longue fait 225 m et la fenêtre 384 m,
  soit 1,7 longueur d'onde. Ce n'est pas un défaut du champ mais un piège de mesure, de la même
  famille que les six de SPEC-003 §6.