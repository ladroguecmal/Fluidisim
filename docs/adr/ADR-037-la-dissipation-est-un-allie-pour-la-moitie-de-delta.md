# ADR-037 — La dissipation est un allié pour la moitié du contenu de δ, et le dimensionnant pour l'autre

- **Statut** : proposée
- **Session** : S32
- **Tranche** : ADR-001 §2 — contenu des couches ; angle mort **A139**
- **Corrige** : rien n'est réécrit.
  [`ADR-036`](ADR-036-delta-ne-porte-pas-la-houle-il-porte-l-ecart.md) §3 reçoit une note corrective
  datée — son objet, le sillage, n'appartient pas à δ.
- **Produit** : la partition entretenu/transitoire, et le critère `dx ≤ K·L^1,5/√(2h)`
- **Vérifié** : §2.1 mesuré en **S33** — `L½ = K·λ²/dx`, `R²` jusqu'à 1,0000, A142 levé
- **Clôt** : action **S31-1** et angle mort **A139** — par dissolution

---

## 1. La réponse était écrite depuis S01, et je ne l'avais pas lue

**A139**, ouverte en S31 : *« le sillage est peut-être un objet de W et non de δ, et rien ne le
dit »*, sévérité 1, avec la mention « aucun document ne tranche ».

**C'est faux.** ADR-001 §2 tranche explicitement, en toutes lettres :

| Couche | Contenu, verbatim |
|---|---|
| **W** | « **sillages**, anneaux d'impact, ondes d'explosion, tsunamis, déferlement, réfraction bathymétrique » |
| **δ** | « proche-coque, gerbe d'étrave, éclaboussure, cavité d'impact, poche d'air, remous sur rocher » |

ADR-011 §4 ne contredit rien : il précise que le *générateur* de sillage, dans W, doit prendre la
profondeur en entrée. En S31 je n'avais lu que lui.

> **Deux sessions de suite, le même défaut, sur le même document.** A122 s'est dissoute en S31 en
> relisant ADR-001 ; A139 se dissout en S32 en relisant ADR-001. Le document fondateur a répondu
> deux fois à une question qualifiée d'ouverte — et la seconde fois, c'est la session précédente qui
> avait posé la question sans le consulter.

**Conséquence : ADR-036 §3 est sans objet.** Ses chiffres restent exacts — un objet de longueur
d'onde `λ` porté par δ meurt selon la loi — mais **le sillage n'est pas cet objet**, et la table des
distances (2,1 m derrière une barque) décrit un cas qui n'existe pas.

## 2. Ce qu'ADR-001 demande de δ, et que S31 n'avait pas lu non plus

> *« δ tend vers 0 en s'éloignant de sa source. Ce n'est pas une contrainte imposée de l'extérieur :
> c'est la **définition de la couche**. »* — ADR-001 §2

**La décroissance de δ est voulue.** Mais elle est voulue **en espace**, et la dissipation numérique
agit **en temps**. Toute la question tient dans cet écart.

> **Décision. Le contenu de δ se partitionne en *entretenus* et *transitoires*, et la dissipation
> numérique n'a pas le même statut pour les deux.**
>
> | | Phénomènes (ADR-001) | Ce que la dissipation fait | Statut |
> |---|---|---|---|
> | **entretenus** | proche-coque, gerbe d'étrave, remous sur rocher | la source réalimente, la dissipation atténue avec le temps de trajet → **équilibre spatial** | **mécanisme voulu** : elle produit la décroissance qu'ADR-001 exige |
> | **transitoires** | éclaboussure, cavité d'impact, poche d'air libérée | rien ne réalimente → **mort avant la fin physique** | **défaut**, et c'est lui qui dimensionne δ |

**La dissipation réalise la définition de δ pour les entretenus et la trahit pour les transitoires.**
C'est le même mécanisme qui produit la propriété voulue d'un côté et le défaut de l'autre.

### 2.1 Et la longueur de décroissance tombe juste

Pour un entretenu, δ s'éteint sur `L_d = c·t_num` :

| Phénomène | échelle | `L_d` à `dx = 0,25 m`, `ν = 0,45` |
|---|---|---|
| gerbe d'étrave | 2 m | 1,0 m |
| remous sur rocher | 5 m | 6,4 m |
| **proche-coque** | 10 m | **25,6 m** |

ADR-001 donne à un domaine δ une portée de « quelques dizaines de mètres ». **Le proche-coque
s'éteint naturellement à 25,6 m** — la dissipation produit gratuitement la décroissance exigée, à la
bonne échelle. Les deux dépendent de la même grandeur, la portée du domaine ; la concordance n'est
donc pas fortuite, mais elle n'avait pas été remarquée.

**Conséquence pratique** : il n'est pas nécessaire d'imposer la décroissance de δ par une éponge ou
un masque de bord pour les phénomènes entretenus. Elle est déjà là.

> **Note S33 — mesuré, et confirmé.** Le §2.1 était **dérivé** : il supposait qu'une source
> constante et une dissipation exponentielle en temps donnent une décroissance exponentielle en
> espace, ce qui est vrai en régime linéaire — et le solveur ne l'est pas. C'était l'angle mort
> **A142**, et il portait la conclusion la plus rassurante du corpus récent.
>
> Un **batteur oscillant** au bord d'un domaine long produit un train entretenu ; l'enveloppe relevée
> en régime établi, hors champ proche et avant toute réflexion, donne :
>
> | `λ` | `L½` mesurée | `L½` prédite par `K·λ²/dx` | écart | `R²` |
> |---|---|---|---|---|
> | 10 m | 27,03 m | 25,54 m | +5,9 % | 0,9990 |
> | 14 m | 51,40 m | 50,06 m | +2,7 % | 0,9990 |
> | 20 m | 101,81 m | 102,15 m | **−0,3 %** | **0,9999** |
> | 28 m | 195,34 m | 200,22 m | −2,4 % | **1,0000** |
>
> **La décroissance est exponentielle pure** — `R²` atteint 1,0000 — et la loi tient sur un facteur
> 8 en `L½`. L'écart **change de signe** avec `λ`, ce qui est la signature des termes d'ordre
> supérieur en `k·dx` et non d'un biais.
>
> **`L½ = K·λ²/dx`, et `c` disparaît** : la longueur de demi-décroissance ne dépend que de la
> longueur d'onde, du pas d'espace et du nombre de Courant. C'est la troisième annulation du corpus.
>
> **A142 est levé — dans le régime linéaire.** L'amplitude du batteur est faible et `a/h ≪ 1 %` : la
> mesure valide la dérivation **là où elle était supposée valide** (A127), et ne dit rien du régime
> non linéaire.

## 3. Le critère de dimensionnement, pour les transitoires

Une éclaboussure d'échelle `L` retombe en `t_phys ≈ √(2L/g)` — le temps de chute gravitaire. La
dissipation lui laisse `t_num = K·L²/(dx·c)` avec `K = ln2/(2π²(1−ν))` et `c = √(g·h)`.

La condition `t_num ≥ t_phys` se résout, et **`g` disparaît** :

> ```
> dx  ≤  K · L^1,5 / √(2h)
> ```

| `L` | `dx_max` à `ν = 0,45` | `dx_max` à `ν = 0,70` |
|---|---|---|
| 0,5 m | **1,1 cm** | 2,1 cm |
| 1 m | **3,2 cm** | 5,9 cm |
| 2 m | 9,0 cm | 16,6 cm |
| 5 m | 35,7 cm | 65,5 cm |

**Une éclaboussure d'un mètre demande `dx = 3,2 cm`**, et δ est un solveur **3D** (ADR-001). Sur un
domaine de 20 m de côté, cela fait 625 cellules par direction.

**Le rapport `t_num/t_phys` croît comme `L^1,5/dx`** : ce sont les **plus petits** phénomènes qui
sont détruits, alors que ce sont eux que δ existe pour montrer. À `dx = 0,25 m`, une éclaboussure
d'un mètre s'éteint **huit fois trop tôt**, une de cinquante centimètres **vingt fois trop tôt**.

### 3.1 Ce que cela ajoute à ADR-035

Passer de `ν = 0,45` à `ν = 0,70` multiplie `dx_max` par **1,83**, donc divise le nombre de cellules
3D par **6,1**.

> **Le levier du nombre de Courant ne se mesure plus en portée d'onde — il se mesure en taille de
> maille pour une fidélité de transitoire donnée**, et c'est la contrainte dimensionnante de δ.
> [ADR-035](ADR-035-le-nombre-de-courant-definition-borne-valeur.md) §4 chiffrait le gain à ×1,77 de
> portée ; il vaut aussi **×6,1 en coût de domaine**.

## 4. Ce qui reste ouvert

1. **`t_phys ≈ √(2L/g)` est une estimation, pas une référence.** Le temps de retombée gravitaire
   d'une éclaboussure est le bon ordre de grandeur, mais la durée *perçue* d'une éclaboussure inclut
   l'écume et le spray, qui relèvent d'ADR-014 et vivent plus longtemps que la déformation de
   surface. **Le critère du §3 est donc conservateur ou optimiste selon ce qu'on veut voir**, et
   rien ne le dit. Angle mort **A141**.
2. **La partition entretenu/transitoire n'est pas dans ADR-001**, qui liste six contenus sans les
   distinguer. Elle est proposée ici et devrait y être portée — c'est une lecture, pas une décision
   nouvelle, mais elle change le dimensionnement.
3. ~~**Aucune mesure n'a été faite sur un phénomène entretenu.**~~ **Fait en S33** — voir la note du
   §2.1. `L½ = K·λ²/dx` vérifiée sur quatre longueurs d'onde, `R²` jusqu'à 1,0000, A142 levé **dans
   le régime linéaire**. Le régime non linéaire reste non mesuré.
4. **δ est 3D et toutes les mesures sont 1D.** Le critère du §3 se transporte en forme ; le
   coefficient `K` est celui du véhicule 1D.
