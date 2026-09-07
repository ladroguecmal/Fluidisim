# ADR-052 — Séparer la précision de phase et la statistique locale

- **Statut : proposée**
- **Session : S68, 2026-09-08**
- **Action : S66-1**, A188 ; précise le contrôle issu de H3, sans modifier ADR-051 D1/D2.

## 1. Problème mesuré

[HOMOGENEITE-S66](../validation/HOMOGENEITE-S66.md) reproduit le ratio nominal 1,397507
en f64 : les interférences sur 144 m expliquent le refus du contrôle présenté comme surveillant
la précision. [SPECTRE-DENSE-S67](../validation/SPECTRE-DENSE-S67.md) établit le même mécanisme
pour Hs à 256 composantes. La précision et la variance locale ne portent pas le même contrat.
Le seuil historique de 15 % ne permet pas d'attribuer un échec à la précision spatiale.

## 2. Décision

### D1 — Conserver la statistique comme diagnostic

Le ratio de variance à 3000 m reste calculé sur les mêmes fenêtres 48×48, pas 3 m, avec les
mêmes phases. Il est affiché et compté comme **un diagnostic sans verdict statistique**.
Sa valeur 1,397507 ne devient pas un succès. Une mesure non finie reste un **refus** compté
dans les échecs de la commande ; elle n'est ni omise ni remplacée par zéro.
La tolérance historique de 15 % reste dans la structure héritée mais n'est plus appliquée
par ce chemin de rapport. Aucune tolérance statistique de remplacement n'est inventée.

### D2 — Contrôler directement la phase spatiale

Une primitive partagée calcule la phase utilisée par eval et par le contrôle. Pour chaque
composante et 49 positions (produit des axes −4095,−3000,−72,0,72,3000,4095 m), comparer
sa phase Q32 à la référence f64 issue des **mêmes coefficients f32**, en distance circulaire
modulo un tour. Aucun besoin de comparer deux réalisations ni d'estimer une variance.

Avec u=2^-24, gamma3=3u/(1−3u), et x,y les deux contributions k·coordonnée·direction
évaluées en f64, utiliser la borne en tours :

`B = gamma3 (|x|+|y|) + u + 2^-32`.

Chaque contribution traverse trois arrondis f32 : multiplication coordonnée×direction,
addition, multiplication par k. La borne gamma3 majore leur erreur cumulée absolue, même
en cas d'annulation dans la somme. Le terme u couvre le repliement de la partie fractionnaire
en f32 ; 2^-32 couvre la troncature Q32. Les très petits termes sous-normaux sont également
couverts par le terme absolu u, très supérieur à leurs erreurs absolues. La référence f64
n'est pas exacte en général ; sur ces paramètres son erreur est négligeable devant cette
borne conservatrice. Les non-finis et B≥0,5 tour (borne devenue non informative) sont refusés.

Le score est le maximum de erreur/B ; l'assertion exige **score≤1**, dérivé avant les mesures.
Pas de seuil choisi pour faire passer le nominal. Un échantillon invalide refuse tout le
contrôle. La liste vide est refusée. Les positions sont strictement dans ±4096 m par axe.

### D3 — Témoins et portée

Un témoin à 256 composantes passe ; une phase volontairement réduite à quatre bits échoue
avec la même borne. Le défaut est injecté dans le chemin de test avant la comparaison,
sans modifier la production. Le test exerce aussi entrée vide, position invalide et coefficient
non fini. Le rapport vérifie un témoin fini hors de l'ancien seuil statistique et un refus NaN.

Ce contrôle vérifie l'arrondi de la phase **sur les composantes et positions échantillonnées**.
Il ne prouve ni le modèle spectral, ni les coefficients cuits, ni le calcul temporel, ni le
sinus, ni la sommation du champ, ni la conformité interplateforme de toute la chaîne.
Il ne remplace pas C18, les tests de phase existants ou les contrôles physiques.
Un mauvais spectre partagé par les deux chemins peut lui échapper.

## 3. Conséquences et réversibilité

Une assertion de précision remplace une assertion statistique mal attribuée ; un diagnostic
statistique explicite s'ajoute au bilan. Le nombre de mesures visibles augmente, la couverture
statistique n'est pas déclarée validée. Le code eval utilise la même opération qu'avant,
extraite sans changement d'ordre ; les hashs doivent rester identiques.

Pour rétablir un verdict d'homogénéité, spécifier un contrat d'ensemble et son domaine,
calibrer sur plusieurs réalisations, puis éprouver un défaut que ce contrat doit détecter.
Pour modifier la borne de précision, fournir une autre analyse d'erreur et conserver témoin
et défaut injecté. Pour étendre la couverture spatiale, définir les points supplémentaires ;
les 49 points présents ne sont pas une preuve exhaustive.

Hs reste sous ADR-051, tolérance 10 % inchangée ; S64-2 reste à instruire. S63-1 reste ouverte.
