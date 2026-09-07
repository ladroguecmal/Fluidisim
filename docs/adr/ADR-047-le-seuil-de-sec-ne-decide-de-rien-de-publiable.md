# ADR-047 — Le seuil de sec ne décide de rien de publiable, et la question était mal posée

- **Statut** : proposée
- **Session** : S40
- **Tranche** : angle mort **A163** *(sévérité 1)*, action **S37-1**, reportée quatre fois
- **Corrige** : rien n'est réécrit. [`ADR-044`](ADR-044-ce-que-l-oracle-croise-peut-dire.md) §7 reçoit
  une note corrective datée — sa prudence reposait sur une conséquence qui n'a pas lieu.
- **Confirme et étend** : [`ADR-031`](ADR-031-le-front-de-mouillage-elimine-l-ordre-un.md) §4
- **Produit** : le seuil de `shallow.rs` rendu réglable ; le balayage sur sept décades et deux
  véhicules ; la règle sur ce qui est publiable
- **Clôt** : action **S37-1**. **A163 est requalifié**, non tranché par un choix de valeur.
- **Ouvre** : **A169**

---

## 1. La question qu'on posait, et celle qu'il fallait poser

Depuis S37, le corpus porte un angle mort de sévérité 1 : `delta.rs` déclare une cellule sèche sous
`H_SEC = 10⁻⁶ m`, `shallow.rs` sous `10⁻¹⁰`. Quatre ordres de grandeur, et une seule des deux valeurs
a une provenance écrite. Quatre sessions ont croisé ce fait sans le trancher, chacune ajoutant une
raison de le faire.

**Mais le corpus contenait déjà deux mesures qui semblaient se contredire.**

| | ce qui est mesuré | résultat |
|---|---|---|
| `ADR-031` §4 *(S23)* | position du front, `H_SEC` sur six décades | **0,25 point sur seize** — *« la valeur est libre »* |
| `ADR-044` §5 *(S37)* | vitesse dans une cellule du film | **6,16 m/s**, soit **98 % de `2c₀`** |

Les deux sont justes. **Elles ne parlent pas de la même grandeur** — et c'est cela qui devait être
établi, pas une valeur.

## 2. La mesure : sept décades, deux véhicules, toutes les grandeurs

`shallow.rs` portait son seuil **en dur à huit endroits** ; il est réglable depuis cette session,
sans changer sa valeur par défaut. Rendre une constante mesurable précède de la décider.

C04, `t = 2 s`, 800 cellules, seuil balayé de `10⁻³` à `10⁻¹⁰` :

| | `delta.rs` (`f32`) | `shallow.rs` (`f64`) |
|---|---|---|
| **position du front** | 9,5416 → 9,5274 m — **0,148 %** | 9,52500 → 9,52500 — **0,000 %** |
| **`h(0)`** | 0,45165 → 0,45166 | 0,45118 → 0,45119 |
| **volume** | 20,000000 partout | 20,000000 partout |
| **longueur du film** | 3 → **15** cellules | 7 → **18** cellules |
| **`max\|u\|`** | **5,18 à 11,06 m/s** | **5,10 à 12,86 m/s** |

**Trois lectures, et la troisième décide.**

1. **Toutes les grandeurs publiées de C04 sont insensibles.** Le front bouge de 0,148 % au pire, pour
   une tolérance de **3 %** ; `h(0)` de deux pour cent mille ; le volume ne bouge pas. Sur sept
   décades. `ADR-031` §4 avait raison, et sa conclusion s'étend à `shallow.rs` et à une décade de
   plus.
2. **Le film s'allonge régulièrement** quand le seuil baisse, des deux côtés. C'est **A165** —
   *un seuil de sec coupe la vitesse, pas le flux de masse*.
3. **`max|u|` dépasse `2c₀ = 6,264 m/s`**, la vitesse du front de Ritter, **la plus grande que cette
   solution contienne**. Elle varie d'un facteur **2,1** à **2,5** avec le seuil, **sans tendance
   monotone**.

> **Une quantité qui dépasse la borne physique de son propre montage et qui varie sans tendance avec
> un réglage arbitraire n'est pas une mesure. C'est `hu/h` sur un film dont l'épaisseur est un
> paramètre.**

## 3. Ce que l'écart entre les deux seuils produit réellement

Rien, sur tout ce que le corpus publie. Le désaccord de 6,16 m/s trouvé par l'oracle croisé en S37
était **réel et sans portée** : il portait sur une grandeur qu'aucun cas canonique ne mesure,
qu'aucune assertion ne lit, et dont le §2 vient d'établir qu'elle n'a pas de sens.

C'est exactement **A166**, importé de la lignée B trois heures plus tôt : *une mesure peut être juste
et sans portée, et rien dans la mesure ne le dit*.

## 4. Les décisions

**D1 — le seuil de sec n'est pas un paramètre physique, et sa valeur est libre sur au moins sept
décades.** `ADR-031` §4 le disait pour `delta.rs` ; c'est vrai aussi de `shallow.rs`, et d'une décade
de plus. La provenance des deux valeurs est désormais cette mesure — *une constante dont l'effet a
été mesuré en a une, même quand l'effet est nul* (`ADR-031` §4).

**D2 — les deux valeurs ne sont PAS alignées.** Aligner coûterait la reproductibilité de tous les
chiffres publiés par la lignée B, qui ont été mesurés à `10⁻¹⁰` — et n'achèterait rien, puisque
l'écart ne déplace aucune grandeur publiable. **Une différence sans conséquence ne se corrige pas :
elle se documente.** Chaque véhicule garde la sienne, désormais réglable et rapportée.

**D3 — `u` dans une cellule sous le seuil n'est pas une grandeur publiable.** Aucune assertion, aucun
rapport, aucun oracle ne doit la lire. C'est la décision de fond, et elle vaut au-delà de ce seuil :
*une grandeur définie par une division dont le dénominateur est un réglage n'est pas mesurable.*

**D4 — l'oracle croisé écarte les cellules litigieuses par construction.** S37 le faisait à la main
pour expliquer un désaccord ; c'est désormais la règle, et `ecart_vitesse_hors_zone_seche` la porte.
Un oracle qui compare deux quantités non publiables compare deux bruits.

**D5 — A163 est requalifié, pas tranché.** Son énoncé — *deux seuils incompatibles* — supposait une
incompatibilité qui n'existe pas. Ce qui reste, et qui justifiait la sévérité 1, est le §5.

## 5. Ce qui justifiait la sévérité 1, et qui demeure

**Une seule des deux valeurs avait une provenance.** Ce n'est pas la différence qui était grave,
c'est qu'un des deux véhicules portait une constante posée au jugé, en dur, à huit endroits, sans
que rien ne dise ce qu'elle commandait. **Le défaut était l'ignorance, pas l'écart** — et il est
levé par la mesure, pas par un alignement.

> *Ce qui rendait ce seuil dangereux n'est pas qu'il valait `10⁻¹⁰` plutôt que `10⁻⁶`, c'est que
> personne ne pouvait dire ce qui changerait s'il valait autre chose.*

## 6. Ce que cette session a corrigé dans ce qu'elle a lu

`ADR-044` §7 refusait d'aligner les deux seuils, et donnait pour raison qu'un alignement
*« déplace la position du front, donc le verdict de C04, donc le critère d'entrée au banc B3 »*.
**La prudence était bonne ; sa justification était fausse.** Le front bouge de 0,148 % au pire, pour
une tolérance de 3 % — le verdict de C04 ne bouge pas.

Cette prudence a coûté quatre reports. Elle en valait la peine — mais pour la raison du §5, pas pour
celle qui était écrite. Angle mort **A169** : *une prudence justifiée par une mauvaise raison se
défend mal et se reporte longtemps.*

## 7. Ce qui reste ouvert

1. **Le seuil de mesure du front** — `10⁻²·h₀`, révisé en B-S25 — n'est pas celui-ci et n'a pas été
   réexaminé. `ADR-031` §5.3 demandait qu'il soit *fixé par C04 lui-même* plutôt que conventionnel
   (**A154**). Toujours ouvert.
2. **La quantité de mouvement dans le film** n'a pas été regardée. Si `u` n'y est pas publiable,
   `hu` non plus, et rien ne le dit encore.
3. **`max|u|` est utilisée ailleurs** — `vitesse_max()`, `u_max()`, la borne de pas de temps
   d'`ADR-035`. Le §2 dit qu'elle n'est pas publiable ; il ne dit pas qu'elle est inutilisable comme
   **borne interne**, et la différence n'a pas été instruite.
4. **Aucun cas canonique ne mesure de grandeur dans le film.** C'est une bonne nouvelle
   aujourd'hui ; ce serait un piège le jour où l'on écrira un cas de plage ou de mouillage, où le
   film **est** le sujet.
