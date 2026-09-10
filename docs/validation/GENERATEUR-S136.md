# S136 — Ce qu'un objet qui entre dans l'eau donne au modèle

2026-09-10. [ADR-092](../adr/ADR-092-generateur-d-impact.md) actée. A200 traitée. S135-1.

## 1. Deux nombres, deux statuts

`WaveEvent::impact` exige `wavelength_m` et `energy_j`, et aucun document ne disait comment un
objet qui tombe les produit. Trois sessions — S123, S132, S135 — ont cité ce manque ; c'est
A200, sévérité 1.

La session commence par séparer ce qui se **dérive** de ce qui doit être **calibré**, parce que
les deux nombres n'ont pas le même statut et que les confondre aurait produit une formule
d'apparence physique avec un facteur arbitraire dedans.

## 2. La longueur d'onde se dérive

**La forme spatiale initiale du candidat est exactement homothétique en λ.** Échantillonnée aux
mêmes `r/λ` pour λ de 0,5 à 32 m, elle donne des valeurs identiques — écart nul, pas « petit ».
C'est structurel, le modèle ne dépendant de λ que par `k0 = 2π/λ`, et c'est désormais mesuré.

Elle a donc un rayon caractéristique proportionnel à λ, et `α` du contrat `λ = α·b` n'est pas un
paramètre libre : c'est le rapport qui fait coïncider ce rayon avec la demi-largeur de l'objet.

| définition du rayon | valeur | α |
|---|---|---|
| mi-hauteur | 0,1830 λ | 5,46 |
| **premier zéro** | **0,2985 λ** | **3,35** |
| rayon de giration | 0,1636 λ | 6,11 |

**α = 3,35 est retenu** — le premier zéro délimite la perturbation centrale, celle que la cavité
creuse, et c'est la lecture qui correspond à la demi-largeur mouillée de Wagner. L'incertitude
est bornée et dite : un facteur 1,8 entre les trois lectures, que le banc B2 resserrera.

**Les valeurs α = 1 et α = 2, utilisées comme hypothèses en S123, sont hors de cette
fourchette.** Les conclusions de S123 étaient donc pessimistes — voir §5.

## 3. L'énergie ne se dérive pas, mais sa borne oui

La fraction de l'énergie d'entrée qui part en ondes de gravité — le reste allant dans la gerbe,
la cavité, la turbulence et la chaleur — est une propriété physique externe qu'aucune lecture du
modèle ne produit. Elle reste **à calibrer**, et elle est un **paramètre explicite de
l'appelant**, jamais une constante enfouie :

```
E = η · ½ · ρ · b³ · v²        η : fraction transférée, À CALIBRER — banc B2
```

**Mais le modèle borne ce qu'il accepte, et cette borne suit une loi exacte.** Par dichotomie sur
le candidat lui-même :

- `E_max / λ⁴` est **constant à 8,9401 × 10⁻²** pour λ de 0,5 à 8 m ;
- le rapport vaut **16,00 exactement** quand la pente admise quadruple.

Soit, en forme homogène : `E_max = K · ρ · g · λ⁴ · s²` avec `K ≈ 8,89 × 10⁻⁴`.

D'où une borne sur η que personne n'avait écrite :

```
η ≤ 2 K g α⁴ b s² / v²
```

**La fraction représentable décroît comme le carré de la vitesse et croît avec la taille.** Les
impacts rapides de petits objets sont ceux que le candidat peut le moins porter — ce qui explique
après coup les 10⁻⁶ mesurés en S123 pour une balle d'arme.

## 4. Construction et réception

`impact_generator` rend `(energy_j, wavelength_m)` et refuse au-delà de la borne. Il ne produit
pas l'événement : identité, référentiel, naissance et durée de vie sont des données de gameplay,
pas de la physique.

**La borne annoncée est celle que le candidat applique**, vérifié des deux côtés — construction
à 97 % de la borne, refus `Steepness` à 105 % — sur douze combinaisons de taille et de pente.
C'est ce qui valide la constante mesurée : elle prédit le seuil réel à quelques pour cent près.

**Les lois d'échelle sont vérifiées** : vitesse doublée, fraction divisée par quatre ; taille
doublée, fraction doublée. Et le générateur produit un plongeon que le candidat construit, refuse
ce qui dépasse, et nomme chaque entrée invalide pour ce qu'elle est.

176 core + 93 harnais = **269 tests réussis, cinq ignorés** ; les trois tests neufs passent aussi
en release. **Hachages de la campagne `cycle_mixed` identiques** à ceux de S118 : aucun calcul
existant n'a bougé.

## 5. Ce que cela change aux conclusions de S123

S123 concluait qu'**un seul cas de jeu sur onze** tenait dans le couloir, à α = 2. Avec l'α
dérivé, le tableau change :

| α | cas construits à la portée demandée |
|---|---|
| 2 (S123) | 1 sur 11 |
| **3,35** | **5 sur 11** |
| 5,46 | 7 sur 11 |
| 6,11 | 7 sur 11 |

Et les portées atteintes passent de 3–102 % de la portée demandée à **20–321 %**, cinq cas
dépassant 100 %. Le pas de personnage, le véhicule léger, le petit vaisseau, l'explosion de
surface et le vaisseau en haute mer sont désormais couverts.

Ce qui résiste : les très petits objets, bornés par `Resolution` — goutte, balle, pierre — et le
vaisseau en port, exclu par le régime d'eau profonde. Ces deux limites sont celles que S123 et
S124 avaient identifiées, et elles ne bougent pas.

Un **suivi daté** est porté à ENVELOPPE-IMPACTS-S123 : ses tableaux restent justes pour l'α qu'ils
supposaient, et cet α est maintenant hors du domaine dérivable.

## 6. Ce qui n'est pas revendiqué

η n'est pas calibré, et α n'est pas définitif : la session remplace un arbitraire par une
dérivation assortie d'une fourchette d'un facteur 1,8. Le banc B2 reste nécessaire pour les deux.

La constante `K` est mesurée à quelques pour cent près — le test l'encadre à 97 % et 105 % — et
sa dépendance en `ρ` et `g` est celle qui rend la formule homogène, non une mesure.

L'angle de relèvement `β` n'intervient pas : dans Wagner, il fixe la **durée** de l'impact, pas
son étendue finale, qui vaut `b` quelle que soit la carène. L'anisotropie reste refusée par
ADR-060.

## 7. Suite

**S136-1, S137 :** A200 est traitée mais deux calibrations restent ouvertes, et elles ont
maintenant un objet précis — `α` dans [3,35 ; 6,11] et `η` sous sa borne. Le banc B2 est
mentionné depuis ADR-060 sans avoir jamais été spécifié : dire **quelles mesures il devrait
produire** pour fixer ces deux nombres est le prolongement direct, et c'est de la conception, pas
du code.

Restent ouverts : l'extension de fenêtre, la profondeur finie de pression (S116-2), le bilan
mixte, la durabilité disque, et les deux limites que S123 et S124 ont posées — petits objets
bornés par la résolution, grands objets en eau peu profonde.

92 ADR, 204 angles, 17 invariants, 6 spécifications, 23 cas.
