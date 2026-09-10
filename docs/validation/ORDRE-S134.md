# S134 — La condition d'ordre : ce qu'elle coûte, ce que la lever coûterait

2026-09-10. [ADR-090](../adr/ADR-090-la-condition-d-ordre-reste-et-s-ecrit.md) actée. S133-1
réalisée — **en refusant de construire**, ce qui demandait de chiffrer les quatre voies.

## 1. La question

Trois sessions ont buté sur la même limite : l'ajout incrémental n'est exact que si la source
s'insère **en dernier** dans l'ordre canonique. Les identifiants viennent de l'hôte, et rien ne
garantit qu'ils croissent. S133 demandait de mesurer ce que coûterait de s'en affranchir.

Quatre voies, dont la dernière n'était pas dans les notes de S133 :

1. accumuler en `f64` puis arrondir ;
2. sommation compensée par nœud ;
3. sommation exacte ;
4. stocker la contribution de chaque source séparément et recomposer dans l'ordre canonique.

## 2. Ce que la condition masque : rien

**C'est la mesure qui décide, et elle porte sur les contributions modales réelles.** Permuter
l'ordre des segments et comparer le champ :

| segments | identiques en bits | écart relatif maximal |
|---|---|---|
| 2 | oui | 0 |
| 4 | non | 5,6 × 10⁻⁷ |
| 8 | non | 7,1 × 10⁻⁶ |
| 16 | non | 4,9 × 10⁻⁷ |

L'ulp d'un `f32` vaut 6 × 10⁻⁸ : l'écart est donc du niveau de l'arrondi, dix à cent fois
l'ulp. **Le champ ne dépend pas de l'ordre au sens où un résultat en dépendrait** ; il en dépend
au sens où deux arrondis diffèrent. La condition ne cache aucun défaut de justesse.

*(Une première sonde, sur valeurs synthétiques aux amplitudes réparties sur six décades, donnait
jusqu'à 1,5 × 10⁻² à soixante-quatre termes. Ce n'est pas le régime des contributions réelles.
Prendre ce chiffre pour une mesure du problème aurait fait traiter comme un défaut de justesse
ce qui est un bruit d'arrondi — et probablement conduit à renouveler toutes les références du
projet pour rien.)*

## 3. Ce que chaque voie coûterait

**Voie 1, accumuler en `f64`.** Elle supprime la sensibilité : sur six mille jeux de valeurs, la
somme `f64` arrondie en `f32` donne un résultat unique quel que soit l'ordre, là où la somme
`f32` est sensible dans **288 cas sur 1000 dès trois termes** et 1000 sur 1000 à soixante-quatre.
Le surcoût en temps est faible — ×1,1 à ×2,1 sur la somme seule, et cette somme est noyée dans
les trigonométries de la réponse modale.

**Mais son prix n'est pas le temps** : les hachages de campagne et les réceptions numériques
accumulées depuis S113 sont des sommes `f32`. Changer l'accumulation les rend toutes non
reproductibles. On perdrait la comparabilité de tout ce qui a été reçu, pour supprimer une
condition dont la mesure vient de montrer qu'elle ne masque rien.

**Voies 2 et 3.** La sommation compensée a la même nature que la voie 1 sans en avoir la force ;
la sommation exacte serait indépendante de l'ordre par construction, au prix d'un accumulateur
par nœud et d'un coût par terme sans rapport avec ce qu'il évite. Ni l'une ni l'autre ne change
le fait que les références bougeraient.

**Voie 4, une contribution par source.** La seule qui rendrait la condition inutile **sans
toucher aux résultats** : recomposer dans l'ordre canonique à chaque admission, sans refaire les
réponses modales. Son prix est la mémoire.

| sources | pools par contrôleur | mémoire | avec la transition d'ADR-089 |
|---|---|---|---|
| 1 | 2 | 1,31 Mo | 2,62 Mo |
| 2 | 4 | 2,62 Mo | 5,25 Mo |
| 4 | 8 | 5,25 Mo | 10,50 Mo |
| 8 | 16 | **10,50 Mo** | **21,00 Mo** |

Hors de proportion avec ce qu'elle évite — une préparation complète occasionnelle.

## 4. Décision

[ADR-090](../adr/ADR-090-la-condition-d-ordre-reste-et-s-ecrit.md) : **la condition reste, et
devient une contrainte d'usage écrite.** Un hôte qui attribue des identifiants croissants — un
compteur suffit — reste sur le chemin incrémental. Un hôte qui ne le fait pas obtient un champ
exactement aussi juste, calculé plus lentement.

Ce qui manquait n'était pas de lever la condition mais de **la dire** : elle est portée dans la
documentation de `admit` et d'`extend_into`, à l'endroit où un appelant la rencontre.

L'accumulation `f64` n'est pas retenue **aujourd'hui**, et le motif est daté : si les références
numériques devaient être renouvelées pour une autre raison — spectre, quadrature, milieu — c'est
à ce moment qu'il faudrait la reconsidérer, le renouvellement étant alors déjà payé.

## 5. Réception

La sonde qui mesure l'écart dû à l'ordre est **conservée comme test**, avec une borne large —
10⁻⁴ — qui sépare « bruit d'arrondi » de « défaut de justesse » plutôt que deux valeurs voisines.
Si l'écart changeait de nature, la décision devrait être reprise, et le test le dirait.

Elle fige aussi qu'à deux segments le résultat ne peut pas dépendre de l'ordre : l'addition
`f32` est commutative. C'est ce qui avait rendu muette la première sonde de S132, et il vaut
mieux l'avoir écrit que de le redécouvrir.

171 core + 93 harnais = **264 tests réussis, cinq ignorés**. Aucun code de calcul n'a changé :
la session ajoute une contrainte écrite, un test et deux sondes.

## 6. Ce qui n'est pas revendiqué

Six mille jeux de valeurs ne démontrent pas que la somme `f64` serait indépendante de l'ordre
dans tous les cas : c'est une mesure, pas une preuve. L'écart mesuré sur les contributions
réelles porte sur un montage et une recette ; un autre régime pourrait donner autre chose, et
c'est précisément ce que le test conservé surveille.

La contrainte n'est pas vérifiable par l'appelant, et ce n'est pas un oubli : ADR-088 a décidé
que l'optimisation resterait invisible, et exposer un prédicat « cette admission sera-t-elle
rapide ? » ferait dépendre le code de l'hôte d'une propriété de performance.

## 7. Suite

**S134-1, S135 :** la couche pression a maintenant son cycle complet — admission, saturation,
sortie, extension. Ce qui reste ouvert de son côté est la **transaction mixte** : ADR-086 s'est
explicitement arrêtée au chemin pression, et rien ne coordonne encore l'admission d'une source
avec les autres couches d'un montage. C'est la suite naturelle, et la seule qui reste avant de
revenir aux couches que le bilan S69 signalait absentes.

Restent ouverts : l'extension de fenêtre, la profondeur finie de pression (S116-2), le bilan
mixte, la durabilité disque, le générateur physique d'ADR-055 et la calibration B2.

90 ADR, 204 angles, 17 invariants, 6 spécifications, 23 cas.
