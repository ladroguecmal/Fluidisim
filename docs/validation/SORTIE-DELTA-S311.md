# Ce qui sort d'un domaine δ — identification, S311

2026-09-20. **Lot 2 d'[ADR-178](../adr/ADR-178-strategie-en-trois-systemes-physiques.md) D7**,
lancé par l'utilisateur, sous les critères d'[ADR-179](../adr/ADR-179-tolerances-de-conservation-et-grandeur-restituee.md).
Cette session fait le **point 1** de la liste d'ADR-179 D8 — « une perturbation sortante est
correctement identifiée à la frontière de δ » — et les instruments des points 4 et 5. Elle ne
transfère rien vers W : c'est le point 2, et il vient après.

Machine de référence (ADR-174 D1). Aucune revendication d'énergie ni de quantité de mouvement
(ADR-179 D7).

---

## 1. La frontière d'un domaine δ est une **paroi**, pas une sortie

Le plan de la session posait la question avant d'y répondre : le flux sortant existe-t-il déjà,
calculé puis jeté par la garde `a > 0 && a < n` du transport ?

**Non.** Mesuré sur la scène couplée à fond spectral réel : la **vitesse normale aux faces
extérieures vaut 0 exactement**, quand le champ atteint 0,605 m/s à l'intérieur. `close_walls`
ferme ces faces, et rien dans le pas ne les rouvre — la projection à Neumann préserve ce zéro. Le
transport ne jette rien : il n'y a rien à jeter.

**Conséquence structurelle.** Un domaine δ est une **boîte fermée**, et l'éponge n'est pas une
frontière absorbante au sens des ondes : c'est une **région d'amortissement à l'intérieur de la
boîte**. « δ ne ressort pas vers W » est donc plus fort que ce que S310 disait — ce n'est pas un
chemin manquant, c'est une paroi. `Balance3::outgoing` publie ce zéro à chaque pas, pour que la
lecture du code soit vérifiée et non crue.

**Deux voies s'ouvrent, et la seconde est celle d'un premier cas** : ouvrir la frontière par une
condition de radiation — gros lot, qui change le schéma —, ou **lire la perturbation sortante sur
une surface de contrôle intérieure**, la ligne intérieure de la bande d'éponge, où l'onde est
encore intacte. Cette ligne est une face **intérieure** : `transport_coupled3` calcule déjà son
flux, avec les mêmes mouillures et les mêmes vitesses que le transport applique.
`Volume3::control_flux_x` le **lit** ; il n'ajoute aucune physique.

---

## 2. Le cas contrôlé

`examples/sortie_canal.rs`. Un canal long en `x`, étroit en `y`, invariant en `y` ; profondeur
`h₀` = 1 m ; **fond nul**, donc aucune bande B/W n'entre et rien ne se mélange à ce qui sort
(ADR-179 D4). Une **bosse gaussienne** de surface, avec la vitesse qui en fait une onde purement
progressive vers les `x` croissants :

```
η(x) = a·exp(−(x−x₀)²/2σ²)     u = (c/h₀)·η     w = −(c/h₀)·η′·z     c = √(g h₀)
```

- **grandeur de référence non nulle** : le **volume de l'onde**, `V = ∫(η−repos)dA` à `t = 0`. Une
  onde progressive transporte son volume ; tout doit traverser la ligne de contrôle.
- **erreur normalisée** : `|Q_traversé − V| / V`.
- **réflexion en énergie** : une **jauge** sur la ligne enregistre `η−repos` ; les deux fenêtres se
  déduisent de la géométrie — passage jusqu'à `t_arrivée + 4σ/c`, retour à partir de
  `t_arrivée + 2·largeur_éponge/c − 4σ/c`. C'est un rapport de deux intégrales de `η²` au même
  point, **pas un bilan d'énergie** (ADR-179 D7), exactement comme la mesure 2D de S269.

### Deux erreurs de montage, que seule la mesure a dites

1. **`λ/h₀ = 4` n'est pas une onde longue.** La condition initiale est une solution d'**eau peu
   profonde** ; le solveur est complet et **dispersif**. La bosse se sépare en une onde progressive
   et une **traîne** qui traverse la ligne dans les deux sens : retour de 36 %, **insensible au
   taux de l'éponge** entre 2 et 20 — c'est ce qui a disculpé l'éponge et accusé la dispersion.
2. **L'éponge est symétrique.** `width_x` s'applique aux **deux** bords ; la bosse, placée à `3σ`,
   démarrait *dedans*, et l'éponge de gauche en effaçait **14,7 %** avant le premier pas. Signature :
   une erreur de restitution constante à 14,7 % **quel que soit** `λ/h₀`.

Ni l'une ni l'autre n'aurait été trouvée par relecture. La première s'est révélée par un balayage
du taux d'éponge, la seconde par un balayage de `λ/h₀` — **deux paramètres, deux coupables**.

---

## 3. Les mesures

Éponge de `4σ`, balayage de `λ/h₀`, maille 25 cm — le retour s'effondre avec la dispersion :

| `λ/h₀` | retour relatif (volume) | erreur de restitution |
|---:|---:|---:|
| 8 | 7,30 % | 1,28 % |
| 16 | 0,646 % | 0,328 % |
| 24 | 0,215 % | 0,148 % |

**Cas de réception de T3** — `λ/h₀` = 12, éponge de `8σ` (assez large pour que les deux fenêtres de
jauge se séparent : 7,66 s contre 15,33 s), taux `10·c/largeur`. **Deux mailles**, et c'est la
réponse à la question de la convergence :

| grandeur | **maille 25 cm** | **maille 10 cm** | seuil T3 |
|---|---:|---:|---:|
| **erreur de restitution** | **0,151 %** | **0,149 %** | ≤ 5 % |
| **réflexion en énergie, en 3D** | **1,4849·10⁻⁶** | **1,4888·10⁻⁶** | < 1 % |
| retour relatif en volume | 1,41 % | 1,31 % | — |
| résidu du bilan de masse | ≤ 3·10⁻¹² m³ | ≤ 4,5·10⁻¹³ m³ | T1 : ≤ 10⁻⁶ |
| `outgoing` au bord | **0** partout | **0** partout | — |

Les deux mailles donnent les mêmes chiffres à **0,3 %** près sur les deux grandeurs de T3, pour un
rapport de maille de 2,5 : ce qui est mesuré est le comportement du raccord, pas un artefact de
grille. La réflexion est **quatre ordres de grandeur** sous le seuil, et elle est mesurée
**directement en 3D** comme ADR-179 D6 l'exige — la mesure 2D de S269 n'a pas été transportée.

*Ce qui reste hors de portée de cette session : la maille de 12,5 cm sur le cas `λ/h₀ = 20`, dont
le coût a dépassé le temps disponible et qui a été arrêté. La comparaison ci-dessus la remplace
sans la refaire : elle porte sur le cas de réception lui-même.*

---

## 4. Ce que W peut recevoir — et le point dur

W n'a que **deux primitives de production** : l'**impact** radial (`wave_event.rs` : énergie,
longueur d'onde, direction, anisotropie, TTL, volume déplacé) et le **sillage** le long d'une
trajectoire prescrite. **Aucune ne représente un front quelconque sortant d'un bord.**

Et surtout — mesuré, pas supposé : **l'impact de W porte de l'énergie, pas du volume**. L'essai
`initial_energy_in_disk_and_volume_residual` du dépôt imprimait déjà le chiffre sans jamais
l'affirmer : pour un impact de 0,01 J, le volume net du disque vaut **−3,6·10⁻⁷ m³**,
c'est-à-dire **zéro à la quadrature près**. Un impact est un anneau dispersif de moyenne nulle.

**Le cas contrôlé prouve donc deux choses d'un coup**, et la seconde n'était pas prévue :

- la perturbation sortante est **correctement identifiée** — point 1 d'ADR-179 D8, tenu ;
- **ce qu'il identifie ne peut pas être remis à W en l'état.** La grandeur de référence du cas est
  un **volume net** sortant, et c'est précisément la composante qu'aucune primitive de W ne porte.

C'est exactement l'identification qu'ADR-179 D5 exige : *« Les composantes qui ne peuvent pas être
représentées par W doivent être identifiées. Leur devenir ne doit pas être implicitement assimilé à
une restitution réussie. »*

### Ce qui est représentable, et ce qui ne l'est pas

| composante du flux sortant | W la porte ? |
|---|---|
| énergie d'un train d'ondes, avec direction et anisotropie | **approximativement** — un impact est un anneau radial, pas un front plan ; l'accord se fait sur l'énergie et la direction, pas sur la forme |
| longueur d'onde dominante | **oui** — c'est un champ de l'impact |
| **volume net** | **non** — mesuré nul sur la primitive |
| phase exacte du signal sortant | **non** — l'impact a une date de naissance et un profil radial, pas un `η(t)` arbitraire |
| variation le long de la frontière (front non uniforme en `y`) | **non** — un impact est isotrope à l'anisotropie près |

---

## 5. Ce que cela pose comme question, et qui la tranche

Un domaine δ qui émet un **volume net** n'a **pas de receveur** dans W. Trois issues, et ADR-179
§3 dit explicitement qu'aucune n'est encore tranchée :

1. **Restreindre le transfert à la part de moyenne nulle** et **déclarer le volume net perdu**,
   chiffré à chaque pas. Honnête, immédiat, et conforme à D5 — mais un domaine qui perd de l'eau en
   perd pour de bon.
2. **Étendre W** d'une primitive qui porte un volume net — une surélévation locale qui s'étale et
   se relaxe. Lot réel, et il touche à une couche jusqu'ici stable.
3. **Choisir une surface de contrôle où le volume net sortant est nul**. Physiquement, c'est le cas
   d'un train d'ondes qui passe ; ce ne l'est pas pour une masse d'eau qui s'en va.

**Recommandation** : (1) pour le premier transfert — il permet de tenir les points 2 et 3 d'ADR-179
D8 sur la part représentable, avec le déficit publié —, puis (2) si un cas de jeu exige qu'un
domaine cède réellement de l'eau. Décision de l'utilisateur.

---

## 6. Limites

Tout est mesuré sur la **référence CPU**, maille 25 cm, une seule machine (A98). La convergence en
maille n'est pas faite. Le cas est **unidirectionnel et invariant en `y`** : il ne dit rien d'un
front oblique ni d'une frontière courbe. La séparation entrant/sortant sur la ligne de contrôle est
**triviale ici parce que rien n'entre** ; sur une scène quelconque elle demande une décomposition
en caractéristiques, qui n'est pas écrite — et la confondre serait le double comptage qu'ADR-179 D4
interdit. Aucun transfert n'a été construit ; aucun banc n'est déclaré reçu.
