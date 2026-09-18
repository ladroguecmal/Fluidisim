# S284 — préparation progressive du rétrécissement

Suite du refus de [S283](RETRECISSEMENT-S283.md). La demande à 1,024 s ne pouvait pas être
publiée : saut instantané de 33,447 mm contre 3 mm. La nouvelle commande **M** prépare le
champ perturbatif avant de tenter le même garde ; N conserve l'essai direct.

## Construction et critères

Le cœur reçoit fenêtre future, bande et coefficient de décroissance dans [0,1]. La couronne
hors fenêtre est amortie, l'intérieur est conservé au bit. Avec le poids spatial smoothstep
`w` de S283, chaque champ est multiplié par `w + (1-w)·decay` ; la hauteur est amortie autour du
niveau de repos, sa compensation également. Un coefficient répété donne une décroissance
géométrique dans la couronne extérieure. Pression invalidée après modification ; pas de
transduction ni conservation de masse perturbative. Ni B/W ni V ne sont amortis.

`Live` consomme cette préparation **après chaque vrai pas couplé**. Il calcule le plus grand
écart nodal susceptible d'être retiré et choisit `decay` pour limiter la correction à **1,5 mm**.
C'est la moitié de la tolérance de hauteur S201, réservant l'autre moitié à l'interpolation,
**un paramétrage de banc non reçu comme borne sur le profil interpolé ou sur les reflets**.
L'arrondi à hauteur 96 m permet un dépassement d'au plus un ulp (2^-17 m). Le reste spatial
de la correction, les vitesses et la suite temporelle doivent encore être qualifiés.

`Layer` retente la permutation toutes les 16 étapes réussies (256 ms ; choix de cadence de banc,
pas seuil physique). Le garde de hauteur de S283 reste à 3 mm. Un refus ordinaire ne construit
pas de message ni de mémoire temporaire allouée. Une pause ne prépare ni ne retente. Le coût
réinjecté inclut maintenant préparation, publication et tentatives du garde, sauf lors de la
permutation qui invalide l'historique de l'ancien domaine. Le banc publie aussi le coût complet
de cette image ; il n'est donc pas caché dans la réception.

## Mesure du chemin réel

```powershell
cargo run --release --offline --locked --manifest-path viewer/Cargo.toml -- --delta-progressif
```

Même onde 0,6 m et houle S275 que S283. Demande après 64 pas (1,024 s), observation jusqu'à
5,120 s ; témoin large aux mêmes instants. Deux `Layer` exécutés séquentiellement, GPU absent.
Ryzen AI 7 350, Windows, release ; secteur constaté aux bornes (BatteryStatus=2, batterie 98 %).

| grandeur | résultat |
|---|---:|
| permutation gardée | 1,792 s, soit 0,768 s après demande |
| correction nodale maximale d'un pas de préparation | 1,503 mm (arrondi f32 compris) |
| écart central maximal au témoin large, pendant les 4,096 s après demande | **66,994 mm** |
| allocations dans les 321 appels `Layer::update` | **0** |
| médiane du coût complet des 256 images après demande | 12,3857 ms |
| p99 observé / maximum | 42,0321 / 43,1006 ms |

Le passage préliminaire donne le même instant de permutation et les mêmes écarts ; seul le coût
varie. Présents : grille du fond, multigrille mobile, amorce par pression hors préparation,
préparation physique et garde Hermite. Absents : cadence découplée, GPU δ, 3D, choix non focal,
agrandissement, transition reçue perceptivement. Préparation et garde paient encore le grand
domaine ; **pas de réception du budget 2 ms**.

## Ce qui est reçu, ce qui ne l'est pas

Reçu : une demande tardive auparavant refusée finit par passer le garde inchangé dans le chemin
consommé ; centre au bit lors de l'amortissement seul, refus atomiques du noyau, décroissance
répétée, aucune allocation, pause sans progrès et poursuite réelle après permutation. Les tests
incluent les refus de coefficients et de fenêtres, et le témoin identique pour `decay=1`.

Suites release : **507 réussis** dans `code/` (393 cœur, 16 + 2 + 1 intégrations, 95 harnais),
**34 réussis** dans `viewer/`, aucun échec ; 19 tests ignorés au total, avertissements antérieurs
conservés. Ces tests ne reçoivent pas la qualité temporelle refusée ci-dessous.

**La fidélité dans la durée n'est pas reçue.** 66,994 mm au centre interdisent de présenter le
seul garde instantané comme une transition sans perte. Cette mesure mélange les effets de
l'amortissement préparatoire et du domaine raccourci ; leur attribution demande un témoin
préparé mais conservé large. Les deux fenêtres de S283/S284 diffèrent : ne pas conclure à une
dégradation de 21 à 67 mm à protocole identique. A290 reste ouvert.

La correction nodale de 1,5 mm ne constitue pas un contrôle de toute la surface interpolée
pendant la préparation ; le garde S283 porte seulement sur la **permutation**. Ni pente,
réflexion, énergie, fidélité après permutation ni I-12 perceptif ne sont reçus. La commande
reste manuelle et expérimentale. Le budget ne l'enclenche pas, et la famine demeure possible.

Suite utile : comparaison sur même fenêtre entre large intact, large préparé et réduit préparé,
pour attribuer l'écart central avant modification du modèle ; puis réception de la trajectoire
complète et du coût avant déclenchement non focal. Avant une troisième session spatiale,
comparer ce lot au blocage du coût δ A276 et à la 3D, conformément à MÉTHODE.
