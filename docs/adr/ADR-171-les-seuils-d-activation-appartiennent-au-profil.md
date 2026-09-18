# ADR-171 — Les seuils d'activation appartiennent au profil, et leur première calibration

Actée S279, 2026-09-18, autonomie S71. Calibre ce qu'ADR-013 §5 laissait explicitement « à
calibrer » ; ne remplace aucune décision.

## Constat

ADR-013 §5 donne `0,60` pour allumer un domaine et `0,40` pour l'éteindre, en disant que ces
valeurs sont « toutes à calibrer ». Personne ne les avait éprouvées : le banc B8 n'existe pas, et
jusqu'à S278 rien n'appelait l'ordonnanceur.

Le premier branchement réel — la bande δ de l'afficheur, S279 — les met à l'épreuve, et **elles ne
passent pas**. La part de cadre qu'occupe la bande vaut, dans les cinq poses de la revue R10 :

| pose | part de cadre |
|---|---|
| défaut | 0,5774 |
| le long des crêtes | 0,5456 |
| face à la houle | 0,5571 |
| haute | 0,3185 |
| rasante | 0,5089 |

**Aucune n'atteint 0,60.** Et la priorité étant un produit de trois fractions
(`P = W_gameplay · W_perception · W_urgence`, ADR-012 §2 borné par ADR-170), elle est **toujours
inférieure ou égale à la plus petite des trois** : un domaine maximal au gameplay et à l'urgence,
qui occupe 55 % de l'écran, n'atteint jamais le seuil. Avec les valeurs de départ, la bande
resterait éteinte dans toutes les poses où la revue R10 l'a montrée.

Le défaut n'est pas dans le mécanisme, qui fait exactement ce qu'on lui demande. Il est dans des
seuils posés en S01 sans jamais être confrontés à une valeur réelle.

## Décision

**Les seuils sont portés par le profil, pas par le cœur.** `Profile` reçoit `on` et `off` ; les
constantes `ON` et `OFF` restent comme valeurs par défaut documentées, au titre de ce qu'ADR-013 a
proposé. Chaque hôte calibre les siennes et écrit sur quoi.

**Première calibration, celle de l'afficheur : `on = 0,45`, `off = 0,35`.**

Le critère ne vient pas du résultat cherché, il vient d'une mesure antérieure et indépendante.
S275 a relevé, pixel par pixel, ce que δ change à l'image :

| pose | pixels changés de plus de 4 niveaux |
|---|---|
| le long des crêtes | 16,2 % |
| face à la houle | 10,0 % |
| rasante | 24,1 % |
| **haute** | **0 %** |

**Un domaine mérite de vivre quand sa présence change l'image, pas quand il occupe le cadre.** La
frontière mesurée passe donc entre la pose haute — où δ ne change rien — et les quatre autres. En
part de cadre, elle tombe entre `0,3185` et `0,5089` ; `0,45` s'y place en gardant de la marge sous
la plus faible des poses utiles. `0,35` éteint la pose haute sans la faire clignoter, et laisse
l'intervalle d'hystérésis à `0,10`.

## Conséquences

La bande vit dans les quatre poses où δ se voit, et **s'éteint vue d'en haut** — où S275 avait
mesuré qu'il ne change aucun pixel. Ce n'est pas un effet de bord : c'est la première fois que le
système éteint quelque chose pour une bonne raison.

Le rendu des quatre poses utiles doit rester **identique au bit** à celui d'avant le branchement.
Un branchement qui change l'image là où rien ne devait changer est un branchement faux.

**Ce que cette calibration n'est pas.** Elle vaut pour un consommateur, une emprise et une mesure
— celle de S275, sur une bande de 256 m par 200 sous une houle idéalisée. Elle ne dit rien des
seuils qu'un jeu devrait employer, ni de ceux d'un domaine de forme différente. Le banc **B8**
reste à écrire, et c'est lui qui devra donner une loi plutôt qu'un couple de nombres.

**Ce qu'elle laisse ouvert.** Qu'un produit de trois fractions soit « toujours plus petit que la
plus petite » reste une propriété gênante : elle rend l'activation d'autant plus difficile que
l'on ajoute des critères. Une moyenne géométrique — `P^(1/3)` — préserverait l'ordre du tri tout en
gardant les seuils interprétables. **Non tranché ici** : ce serait changer ADR-012 §2 sans mesure,
et le tri par `P/C` n'en a pas besoin.
