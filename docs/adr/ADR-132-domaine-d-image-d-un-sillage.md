# ADR-132 — Le domaine d'image d'un sillage se calcule depuis sa recette, et l'hôte l'annonce

- Statut : **actée**, S214, 2026-09-13 ; autonomie technique S71.
- Prolonge [ADR-107](ADR-107-domaine-d-un-sillage-depuis-sa-recette.md) (le domaine se déduit de la
  recette) et applique à W ce qu'[ADR-126](ADR-126-emprise-d-un-impact-visible.md) a fait pour
  l'impact : une emprise d'image reçue **par ses coutures**.
- Traite **A251** ; apporte un troisième point de calibration à **A214**, sans la clore.
- Ne réécrit aucun ADR. Ne modifie ni `water-core`, ni une interface publiée, ni un refus.
- Mesures : [COMPOSITION-J1-S214](../validation/COMPOSITION-J1-S214.md) §3.

## Constat

ADR-107 a établi que le domaine d'un sillage est un couple `(rayon, durée)` déduit de sa recette,
et a **refusé d'en encoder la loi** : celle que S156 avait écrite,
`2π/(c_g,max·dk)`, donnait 13,1 s là où la mesure encadrait 15–20 s, et se trompait d'un facteur
2,5 à radial 256. « Une mesure qui manque, pas une décision » — **A214**.

Depuis, une fixture d'image est née sans domaine (**A251**) : le sillage prescrit de S212 déclare
un contexte de **40 s** sur une recette 64×128 dont S212 mesurait qu'elle tient 2 % pendant 16 s,
9,5 % à 24 s et 28 % à 39 s. L'hôte affichait donc, sans le dire, un champ hors de son domaine —
l'inverse exact de ce qu'ADR-126 a obtenu pour l'impact.

## Les deux lois

Elles ne sont pas nouvelles : ce sont les deux mécanismes d'ADR-107, écrits en formule et
**recalibrés**.

**Rayon** — l'échantillonnage de `exp(i k·x)` sur le cercle :

```
rayon_honnête = 2π · angular / (3 · cutoff)
```

C'est exactement la colonne « théorique » de [SILLAGE-DOMAINE-S156](../validation/SILLAGE-DOMAINE-S156.md) :
22 / 45 / 90 m pour angular 64 / 128 / 256 à cutoff 6, contre 20 / 45 / « au-delà de 200 m »
mesurés. Juste à 128, optimiste de 10 % à 64, conservatrice à 256.

**Durée** — la périodicité spatiale que le pas radial impose, `L = 2π/dk` avec `dk = cutoff/radial`,
parcourue par la composante la plus rapide représentée, `c_g,max = ½√(g/dk)` :

```
durée_honnête = L / c_g,max = 4π / √(g · dk)
```

**C'est la récurrence de S156 avec une autre constante**, et c'est ce changement qui la rend
utilisable : 18,53 s à radial 128 / cutoff 6, dans l'encadrement mesuré **15–20 s**, là où S156
publiait 13,1 s ; 26,2 s à radial 256, conservatrice contre 45–50 s.

## Décision

**1. Ces deux lois sont le domaine d'image d'un sillage**, et elles se calculent depuis la recette
seule. Elles vivent dans l'hôte (`scene::wake_honest_radius`, `wake_honest_duration`), pas dans
`water-core` : voir §« ce que cette décision ne fait pas ».

**2. Une durée honnête porte le critère qui l'a calibrée.** Sur la fixture S212 — recette 64×128,
cutoff 3, σ 2 m, huit tronçons de 2 s à 3 m/s — la loi donne **18,53 s**, et les trois critères
mesurés en S214 l'encadrent chacun à leur façon :

| critère | franchi entre | la loi est |
|---|---|---|
| écart à la recette fine ≤ 2 % (ADR-120) | **16 et 18 s** | optimiste d'au moins 3 % |
| couture au bord d'emprise ≤ 3 mm (S201) | **18 et 20 s** | juste |
| rayon d'accord à 10 % (critère S156) | **24 et 30 s** | conservatrice de 20 à 40 % |

**Une garde bâtie sur cette loi porte donc une marge**, et elle nomme son critère. Annoncer sans
marge, comme ici, est licite ; refuser sans marge ne le serait pas.

**3. L'hôte annonce, il ne refuse pas.** `FrameData::update` publie une ligne unique
`WAKE_HORS_DOMAINE` au premier instant qui dépasse la durée honnête, et le domaine calculé est
publié avec la fixture (`WAKE_LOI`). C'est l'habitude d'ADR-091 — annoncer l'admissibilité plutôt
que mentir en silence — appliquée là où elle peut l'être sans décider pour la bibliothèque. Le
chemin reste cosmétique (ADR-129 §3, I-04).

**4. La fixture S212 est déclarée hors domaine, et conservée telle quelle.** Son contexte de 40 s
vaut **2,2 fois** sa durée honnête ; son coin d'emprise le plus lointain est à 102,22 m pour un
rayon honnête de 89,36 m. Elle garde sa valeur — c'est elle qui a servi à recevoir S211, S212,
S213 et S214, et la raccourcir invaliderait ces réceptions. Ce qui change est qu'elle **le dit**.

## Ce que cette décision ne fait pas

- **Elle n'encode aucun garde-fou dans `water-core`.** A214 le demandait sur une loi non vérifiée ;
  la loi a désormais **trois** points de calibration (S156 à radial 128 et 256, S214 à radial 64),
  ce qui est mieux mais pas suffisant : la dépendance à `sigma` reste non mesurée (S156 §fin), et
  aucun point n'existe entre radial 256 et 512. Refuser un contexte à l'admission d'une source est
  une décision de bibliothèque, avec un coût de compatibilité — la fixture S212 elle-même serait
  refusée. **A214 reste ouverte**, avec un point de plus.
- **Elle ne fixe pas le rayon comme borne active.** Sur cette fixture le rayon ne mord jamais :
  l'accord tient sur toute l'emprise jusqu'à 24 s. C'est la durée qui borne, et ADR-107 avait
  prévenu que laquelle des deux mord **dépend de l'instant**.
- **Elle ne dit rien du coût.** Réduire la durée honnête d'un LOD spectral (J1-bis) la déplacerait ;
  c'est ce qu'A251 notait déjà en S213.
- **Elle ne transporte pas hors de la famille gaussienne.** Les deux lois sont celles d'une
  quadrature uniforme en `k` sur un demi-disque ; une quadrature non uniforme (le remède qu'ADR-107
  laisse ouvert) les changerait toutes les deux.

## Réversibilité

Les deux lois sont deux fonctions de l'hôte et une ligne d'annonce. Les retirer ne touche aucun
bit publié ni aucun refus. Les remplacer par une loi mieux calibrée est le chemin attendu, et
demande alors un ADR qui remplace celui-ci — pas une réécriture.
