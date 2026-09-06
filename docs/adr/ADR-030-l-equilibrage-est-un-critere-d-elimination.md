# ADR-030 — L'équilibrage sur fond variable est un critère d'élimination, pas un réglage

- **Statut** : proposée
- **Session** : S22
- **Tranche** : ADR-007 §5.1 — candidats à évaluer pour δ (banc B3)
- **Corrige** : rien. Complète `CAS-CANONIQUES` §C01 et ADR-007 §5.
- **Produit** : `code/water-core/src/delta.rs`, et l'exécution de **C01** dans le mode `physics`

---

## 1. Ce que cette décision tranche, et ce qu'elle ne tranche pas

**Elle ne choisit pas le solveur δ.** Ce choix reste le banc **B3**, et les cinq candidats
d'ADR-007 §5.1 restent tous en lice.

> **Décision. Un candidat δ qui ne préserve pas exactement l'eau au repos sur un fond variable est
> éliminé avant d'entrer au banc B3, quel que soit son coût par cellule.**
>
> La propriété se vérifie par C01, elle se vérifie en moins d'une seconde, et elle ne se rachète
> par aucun réglage.

Ce qui suit est la mesure qui justifie le mot « éliminé » plutôt que « pénalisé ».

## 2. La mesure

Montage de C01 : bassin de 40 m, fond en pente 1:20, eau au repos, 60 s (`CAS-CANONIQUES` §C01).
Véhicule d'essai : Saint-Venant 1D, volumes finis, flux de Rusanov, Euler explicite — le schéma que
l'on écrit sans y penser, choisi pour cette raison.

| `dx` | premier jet : `max\|u\|` | premier jet : `max\|η−η₀\|` | équilibré : `max\|u\|` | équilibré : `max\|η−η₀\|` |
|---|---|---|---|---|
| 1,0000 m | 1,98 mm/s | 85,0 mm | 0,0018 mm/s | 0,00048 mm |
| 0,5000 m | 1,04 mm/s | 43,0 mm | 0,0026 mm/s | 0,00048 mm |
| **0,2500 m** | **0,53 mm/s** | **21,6 mm** | **0,0068 mm/s** | **0,00072 mm** |
| 0,1250 m | 0,29 mm/s | 10,8 mm | 0,0062 mm/s | 0,00119 mm |
| 0,0625 m | 0,10 mm/s | 5,5 mm | 0,0070 mm/s | 0,00131 mm |

Seuils de C01 : `max|u| < 1 mm/s` et `max|η−η₀| < 1 mm`.

**Le défaut du premier jet est du premier ordre exact en `dx`.** Le rapport `max|η−η₀|/dx` vaut
0,0850 · 0,0860 · 0,0864 · 0,0862 · 0,0875 sur les cinq grilles : une constante, `C ≈ 0,0864`.

**L'erreur du schéma équilibré ne dépend pas de `dx`.** Elle reste entre 0,0005 et 0,0013 mm, et
l'ulp d'un `f32` à 3 m vaut 0,00024 mm. Ce n'est pas une erreur de discrétisation : c'est le bruit
d'arrondi. Le repos est préservé par **identité algébrique**, et il le serait sur trois cellules.

### 2.1 Le chiffre qui fait de l'équilibrage un critère d'élimination

Atteindre la tolérance de C01 par raffinement seul demanderait `dx = 10⁻³ / 0,0864 = 11,4 mm`, soit
**21,9 fois plus fin** que le montage nominal. En 2D, le coût va comme `dx⁻²` en cellules et
`dx⁻¹` en pas de temps (CFL) :

```
coût relatif = 21,9³ ≈ 10 500
```

**Quatre ordres de grandeur pour obtenir par la force brute ce qu'une reconstruction correcte donne
gratuitement.** Un solveur non équilibré n'est pas un solveur un peu moins bon : c'est un solveur
dont le domaine utile est réduit d'un facteur dix mille à budget constant. Aucun avantage de coût
par cellule mesurable au banc B3 ne compense cela, ce qui autorise à éliminer avant de mesurer.

### 2.2 Ce que le défaut donne à voir en jeu

`CAS-CANONIQUES` §C01 annonçait « un lac qui frissonne sans raison et une eau qui coule lentement
vers le bas d'une plage ». La mesure précise le symptôme : à `dx = 0,25 m`, la surface libre s'écarte
de **21,6 mm** de l'horizontale et n'y revient pas. Sur une plage en pente 1:20, un écart de 2 cm de
niveau d'eau **déplace le trait de côte de 43 cm** — visible, permanent, et sans cause dans le monde.

## 3. Les deux assertions de C01 ne sont pas redondantes

L'énoncé de C01 porte deux seuils, `max|u|` et `max|η−η₀|`, et rien n'y disait lequel travaille.

**Le schéma au premier jet passe le premier et échoue le second** : 0,53 mm/s contre 1 mm/s admis,
et 21,6 mm contre 1 mm admis. Un harnais qui n'aurait mesuré que la vitesse — la grandeur que le
nom du symptôme, « courants parasites », désigne pourtant explicitement — **aurait déclaré ce
schéma conforme**.

> **Conséquence de méthode.** Quand un cas canonique porte plusieurs assertions, elles sont toutes
> exécutées et toutes rapportées, même quand l'une paraît impliquer l'autre. Ici la vitesse et
> l'élévation mesurent deux régimes différents : un déséquilibre **stationnaire** installe un écart
> de surface permanent avec un courant presque nul, alors que le nom du cas oriente vers le courant.

## 4. La propriété ne se découpe pas : le bord en fait partie

Le premier jet de `delta.rs` avait un intérieur exact — la reconstruction hydrostatique est une
identité algébrique — et **perdait 1,1 % du volume en 60 s**.

La cause était dans la condition de mur : elle recopiait la **hauteur d'eau** dans la cellule
fantôme, `h_fantôme = h_interne`, le miroir évident. Sur un fond en pente, le lit de la fantôme
n'est pas à la cote de sa voisine ; recopier la hauteur y installe une surface libre décalée de
`dx·pente`, c'est-à-dire **une marche d'eau permanente contre chaque mur**. Le miroir juste porte
sur la surface libre : `h_fantôme = η_interne − b_fantôme`.

> **Un intérieur équilibré et un bord qui ne l'est pas donnent un solveur non équilibré.**

C'est une exigence à porter au protocole de B3 : un candidat n'est pas « équilibré », il l'est
**avec ses conditions aux limites**, et c'est dans cette configuration qu'il doit passer C01. La
distinction n'est pas académique — le défaut de bord pesait quarante fois plus que le défaut de
schéma qu'il masquait.

### 4.1 Comment il a été trouvé, et pourquoi c'est reproductible

Il n'a pas été trouvé par relecture. Il a été trouvé parce qu'**une propriété exacte donnait un
résultat faux** : le volume fuyait alors que le schéma le conserve par construction. Une propriété
démontrée qui ne se vérifie pas ne laisse qu'une possibilité — l'erreur est en dehors de ce qu'elle
couvre.

C'est le même mécanisme qu'en S21, où l'identité `u = ω·η` a fait tomber la vitesse orbitale
(ADR-029, note S21). **Une identité fermée ne sert pas seulement à valider ; elle localise.**

## 5. Ce que le véhicule d'essai est, et ce qu'il n'est pas

`code/water-core/src/delta.rs` est un **véhicule d'essai**, au même titre que `background.rs` pour
la couche `B`. Il donne à C01 quelque chose à faire tomber.

| Il est | Il n'est pas |
|---|---|
| Saint-Venant 1D, volumes finis, Rusanov, Euler explicite | le solveur δ du projet — c'est B3 |
| équilibré par reconstruction hydrostatique (Audusse) | d'ordre élevé : il est d'ordre 1 en espace |
| assez ordinaire pour que ses défauts soient ceux de sa famille | validé sur autre chose que C01 |

**Ce qu'il ne pourra jamais mesurer : `λ_cut`.** Saint-Venant est non dispersif — `c = √(g·h)`,
SPEC-001 §1 — et C02 mesure précisément une **erreur de célérité en fonction de la longueur d'onde**.
Sur ce véhicule, C02 ne mesurerait que la dispersion **numérique** du schéma, qui n'est pas la
grandeur cherchée.

> **`λ_cut` ne sortira pas de ce solveur.** Le chemin critique de `00_INDEX.md` fait suivre C01 · C02
> puis `λ_cut` puis B2 ; C01 est fait, et C02 demande une couche dispersive, donc `W` ou un δ de
> famille différente. C'est un point à porter au chemin critique, pas un contretemps de codage.

## 6. Ce qui reste ouvert

1. **Le montage de C01 est moins discriminant qu'il n'en a l'air.** Son fond est à pente
   **constante** : `h` y varie linéairement, le saut de hauteur aux interfaces est le même partout,
   et sa divergence est donc presque nulle. Le premier jet y est *presque* équilibré par accident de
   géométrie — d'où les 0,53 mm/s qui passent. Un fond **courbe** — la bosse parabolique classique —
   le ferait tomber bien plus lourdement. **Proposition : un C01-bis à fond courbe**, à écrire avant
   que B3 ne s'en serve pour éliminer des candidats. Angle mort **A105**.
2. **L'ordre en espace du véhicule.** Il est de 1 ; les candidats de B3 seront d'ordre 2 ou plus. Le
   coefficient `C ≈ 0,0864` mesuré ici est celui de *ce* schéma, et ne se transporte pas. Ce qui se
   transporte est la **structure** du raisonnement : mesurer `C`, en déduire le raffinement requis,
   l'élever au cube.
3. **La tolérance de C01 n'a pas de provenance.** `1 mm/s` et `1 mm` viennent de `CAS-CANONIQUES` et
   n'y sont rattachés à aucune formule ni à aucun banc — I-14 demande l'un ou l'autre. La mesure du
   §2.2 en donne enfin une lecture physique — 21,6 mm de surface font 43 cm de trait de côte sur une
   pente 1:20 — mais une lecture n'est pas une justification. Angle mort **A106**.
4. **La friction de fond est absente du véhicule.** Elle ne change rien à C01, dont la référence est
   le repos, mais elle change C03 (dissipation) et C04 (front). À poser avant ces cas.
