# Travail en cours — journal d'intention

> **Pourquoi ce fichier existe.** Une session coupée par une limite d'usage n'a *aucune* occasion
> d'écrire « j'ai été interrompue ». Tout dispositif de passation qui suppose une action au moment
> de l'arrêt est donc inutile. Seule survit une déclaration faite **avant** le travail.
>
> Ce fichier déclare ce qui va être fait, avant de le faire. Git enregistre ce qui a effectivement
> été fait. L'écart entre les deux est exactement ce qui a été interrompu.

---

## Reprise à chaud — procédure

À suivre lorsque l'état ci-dessous n'est pas `terminée`. Cinq minutes ; **ne pas lire tout le
dépôt** — la lecture complète (`REPRISE.md`) ne sert qu'au démarrage à froid.

1. **Lire l'état et le plan** de la session en cours, plus bas.
2. `git log --oneline -15` — **ce qui est committé est fait**, définitivement. Ne pas le refaire.
3. `git status --short` — les fichiers modifiés non committés appartiennent à l'étape marquée
   `[>]`. C'est elle qui a été interrompue, et elle seule.
4. `git diff` — **lire avant de décider**. Deux issues, pas trois :
   - **compléter** l'étape, si le diff est cohérent et si la thèse déclarée dans le plan est
     claire ;
   - **annuler** l'étape (`git restore <fichiers>`), si le diff est incohérent ou
     incompréhensible.

   Ne jamais laisser un état intermédiaire non tranché, et écrire dans le journal lequel des deux
   a été choisi.
5. **Lire les notes de reprise** de la session interrompue. C'est là que vivent les chiffres déjà
   calculés, les décisions prises mais pas encore écrites et les impasses déjà explorées —
   l'information la plus coûteuse à reproduire, et la seule que git ne conserve pas.
6. Reprendre au premier `[ ]`, ou à `[>]` si l'étape a été complétée.
7. **Prévenir l'utilisateur** : la session précédente a probablement été coupée avant d'avoir pu
   rendre compte de son travail. Résumer ce qu'elle avait fait — il ne l'a peut-être jamais vu.

---

## Règles pour la session qui travaille

- **Déclarer le plan complet avant la première modification**, et le committer seul. C'est
  l'écriture anticipée : sans elle, une interruption ne laisse aucune trace d'intention.
- **Aucune étape ne dépasse une quinzaine de minutes de travail.** Si elle est plus grosse, la
  découper. C'est la seule prophylaxie réelle contre une coupure — pas un confort d'organisation.
- Marquer `[>]` **avant** de commencer une étape. Basculer `[x]` **en dernière action avant le
  commit de cette étape**, jamais après : le commit doit contenir à la fois le travail et la case
  cochée, sinon l'historique ment dans un sens ou dans l'autre. Un `[x]` sans commit est un
  mensonge que la session suivante paiera ; un commit sans `[x]` fera refaire du travail déjà fait.
- **Un commit par étape**, message `S<n> P<k> — <description>`. Le plan et le journal git disent
  alors la même chose de deux façons indépendantes ; si l'un est faux, l'autre le révèle.
- Déposer dans **Notes de reprise** tout ce qui n'est pas encore dans un fichier : un chiffre
  calculé, une décision prise, une impasse explorée. **Une impasse est aussi précieuse qu'un
  résultat** — sans elle, la session suivante la réexplore intégralement.
- **Le rituel de fin (`REPRISE.md` §6) est lui-même une étape du plan.** Une session interrompue
  laisse ainsi cette étape visiblement non cochée, ce qui dit à la suivante exactement ce qui
  manque.

---

## Session en cours

Session : S311 — **terminée**. La frontière est une paroi, la sortie se lit sur une ligne
de contrôle intérieure, et le cas contrôlé est reçu contre les deux seuils de T3.
Agent : Claude Opus 5, application desktop ; fichiers, git, cargo, carte réelle, accès web.
Entrée : la **décision de l'utilisateur du 2026-09-20** sur les tolérances, et le lancement du
lot 2. T1 acceptée provisoirement, T2 comme objectif à éprouver sur la durée complète, T3 comme
objectif initial **avec une précision qui commande cette session** :

> « Il faut définir précisément la grandeur à restituer. L'éponge absorbe des perturbations dont le
> volume net signé et la quantité absolue sont différents. Ces deux mesures ne doivent pas être
> confondues. Je ne souhaite pas que le moteur crée artificiellement une nouvelle vague pour
> compenser toute l'activité de l'éponge. Le transfert doit représenter la perturbation physique
> **sortante**, compatible avec W, sans double comptage avec le fond B/W entrant. Les composantes
> qui ne peuvent pas être représentées par W doivent être **identifiées**. »

Capacité visée : **le dépôt sait ce qui sort d'un domaine δ, le mesure sur un cas contrôlé, et
sait ce que W peut en recevoir.** Consommateur : la construction du transfert lui-même, qui est la
suite immédiate. Sans cette phase, le transfert n'aurait ni grandeur de référence, ni normalisation
d'erreur, ni moyen de distinguer ce qui est transmis de ce qui est perdu — c'est-à-dire exactement
les trois choses que l'utilisateur demande.

**Ce que je ne fais pas dans cette session, et je le déclare d'avance** : écrire le transfert.
L'utilisateur demande une progression « à partir du cas physique le plus simple permettant de
démontrer un transfert réel, mesuré et reproductible », et sa liste commence par *« qu'une
perturbation sortante est correctement identifiée à la frontière de δ »*. C'est ce point 1, plus
les instruments des points 4 et 5. Les points 2 et 3 — le transfert et sa propagation dans W —
viennent ensuite, et cette session doit leur laisser un terrain mesuré.

Ce que je ne fais pas non plus : revendiquer une conservation d'énergie ou de quantité de
mouvement (leurs bilans ne sont pas fermés, S310 §4) ; ouvrir un chantier de rendu ; imposer le
budget temps réel au solveur de référence.

**La question de fond, posée avant de mesurer.** Le pas calcule une vitesse normale aux faces
extérieures, et la garde `a > 0 && a < n` de `transport_coupled3` **jette** le flux de colonne
qu'elle porterait. Si cette vitesse est non nulle, alors le flux sortant *existe déjà*, il est
calculé, et il est perdu à chaque pas — et c'est lui, et non l'activité de l'éponge, qui est « la
perturbation physique sortante » que l'utilisateur décrit. Si elle est nulle, la sortie devra être
construite autrement. **Je ne sais pas laquelle des deux, et P3 le mesure avant tout le reste.**

Critères, écrits avant la mesure :
1. La grandeur à restituer est **définie et défendue** avant d'être mesurée, et le **net signé**
   n'est jamais confondu avec la **quantité absolue**.
2. Le cas de T3 est **contrôlé** : perturbation sortante identifiable, grandeur de référence non
   nulle, erreur normalisée par elle et déclarée d'avance.
3. Ce que W **ne peut pas** représenter est nommé, et son devenir n'est pas compté comme une
   restitution réussie.
4. La réflexion est mesurée **en 3D**, directement, pas transportée de S269.
5. Les scènes antérieures restent **au bit** ; aucune revendication d'énergie.

### Plan

- [x] **P1** — amorce, jeton, plan seul.
- [x] **P2** — [ADR-179](../docs/adr/ADR-179-tolerances-de-conservation-et-grandeur-restituee.md) :
  huit décisions. **D3 sépare trois grandeurs que S310 mesurait ensemble** ; D2 dit que T2 n'est
  **pas** encore tenue — 5 s mesurées pour 10 s demandées.
- [x] **P3** — **réponse : non.** La vitesse normale aux faces extérieures vaut **0 exactement**,
  quand le champ atteint 0,60 m/s à l'intérieur. Le domaine est une **boîte fermée** ; le transport
  ne jette rien, il n'y a rien à jeter. **Le flux sortant est à construire, pas à récupérer.**
- [x] **P4** — `examples/sortie_canal.rs` : une onde longue traverse une **ligne de contrôle
  intérieure**. À `λ/h₀ = 24` : erreur de restitution **0,148 %**, retour **0,215 %**. Deux erreurs
  de montage trouvées par la mesure, pas par la relecture.
- [x] **P5** — W n'a que **deux** primitives, impact radial et sillage ; **l'impact porte de
  l'énergie, pas du volume** (−3,6·10⁻⁷ m³ mesurés pour 0,01 J). Le volume net n'a pas de receveur.
- [x] **P6** — la décomposition se lit sur la **ligne de contrôle** : ce qui la traverse vers la
  droite est l'onde (99,85 % du volume au cas de réception), le reste est la traîne dispersive.
- [x] **P7** — jauge sur la ligne, fenêtres séparées par la géométrie (7,66 s contre 15,33 s) :
  **réflexion en énergie 1,48·10⁻⁶**, quatre ordres sous le seuil de 1 %, mesurée **en 3D**.
- [x] **P8** — [SORTIE-DELTA-S311](../docs/validation/SORTIE-DELTA-S311.md). **Une question à
  trancher y est posée** : le volume net sortant n'a pas de receveur dans W — le perdre, étendre W,
  ou choisir une surface où il est nul.
- [x] **P9** — rituel REPRISE §6.

### Notes de reprise

**P3 a renversé la moitié du plan, et il valait mieux le mesurer que le supposer.**

Mesuré sur la scène couplée à fond spectral réel : `vitesse_normale_de_bord_max = 0` **exactement**,
`vitesse_max_du_champ = 0,605 m/s`. Le flux sortant vaut donc `0` non parce que la garde du
transport le jette, mais parce qu'**il n'existe pas** : `close_walls` met les faces normales
extérieures à zéro, et rien dans le pas ne les rouvre — la projection à Neumann préserve ce zéro.

**Conséquence, et elle est structurelle.** Un domaine δ est une **boîte fermée**. L'éponge n'est
pas une frontière absorbante au sens des ondes : c'est une **région d'amortissement à l'intérieur
d'une boîte**. Rien ne traverse la frontière ; la perturbation est éteinte avant de l'atteindre.
« δ ne ressort pas vers W » est donc plus fort que ce que S310 disait : ce n'est pas un chemin
manquant, c'est une **paroi**.

**Ce que cela change pour le lot 2.** Deux voies, et la seconde est la bonne pour un premier cas :
1. **ouvrir la frontière** — condition de radiation sur les caractéristiques. Gros lot, et il
   change le schéma ;
2. **mesurer la perturbation sortante sur une surface de contrôle intérieure** — la **ligne
   intérieure de la bande d'éponge**, où l'onde est encore intacte. Cette ligne est une face
   **intérieure** : `transport_coupled3` calcule déjà son flux, et il n'est pas nul. On lit ce qui
   la traverse vers l'extérieur, et c'est cela qui part vers W.

La voie 2 ne touche pas au schéma, se mesure avec le compteur existant, et respecte ADR-179 D3 :
ce qui traverse la ligne **sort**, alors que l'activité de l'éponge mélange trois choses. Elle
demande en revanche de séparer sortant et entrant sur cette ligne — ce qu'un **cas contrôlé**, où
rien n'entre, rend trivial. C'est exactement pourquoi l'utilisateur en demande un.

**P4 : le cas contrôlé marche, après deux erreurs de montage que seule la mesure a dites.**

1. **`λ/h₀ = 4` n'est pas une onde longue.** La condition initiale est la solution d'onde simple
   des équations en **eau peu profonde** ; le solveur est complet et **dispersif**. À `λ/h₀ = 4`,
   la bosse se sépare en une onde progressive et une **traîne dispersive** qui traverse la ligne
   dans les deux sens : retour de 36 %, **insensible au taux de l'éponge** — c'est ce qui a
   disculpé l'éponge et accusé la dispersion.
2. **L'éponge est symétrique.** `width_x` s'applique aux **deux** bords `x` ; la bosse, placée à
   `3σ`, démarrait *dedans*, et l'éponge de gauche en effaçait **14,7 %** avant le premier pas.
   Signature : une erreur de restitution constante à 14,7 % **quel que soit** `λ/h₀`.

| `λ/h₀` | retour relatif | erreur de restitution |
|---:|---:|---:|
| 8 | 7,30 % | 1,28 % |
| 16 | 0,646 % | 0,328 % |
| **24** | **0,215 %** | **0,148 %** |

Le retour s'effondre comme la dispersion diminue : **c'était bien elle**, démontré et non supposé.
Résidu du bilan ≤ 3·10⁻¹² m³ partout, `outgoing` nul partout — la paroi tient.

**P5 : ce que W peut recevoir, et le point dur.** W n'a que deux primitives de production —
l'**impact** radial (`wave_event.rs` : énergie, longueur d'onde, direction, anisotropie, TTL) et le
**sillage** le long d'une trajectoire. Aucune ne représente un front quelconque sortant d'un bord.
Et surtout : **l'impact de W porte de l'énergie, pas du volume**. Mesuré — l'essai
`initial_energy_in_disk_and_volume_residual` du dépôt imprimait déjà le chiffre sans jamais
l'affirmer : `disk_volume = −3,6·10⁻⁷ m³` pour 0,01 J, c'est-à-dire **zéro à la quadrature près**.

**Conséquence, et elle est plus intéressante que prévu.** La grandeur de référence que le cas
contrôlé mesure — un **volume net** sortant — est précisément la composante qu'**aucune primitive
de W ne peut porter**. Le cas prouve donc deux choses d'un coup : que la perturbation sortante est
correctement identifiée (point 1 d'ADR-179 D8), et que ce qu'il identifie **ne peut pas être remis
à W en l'état**. C'est exactement l'identification qu'ADR-179 D5 exige.

## Archive — notes de S308 (lot du rendu, clos par ADR-178)

## P2 + P3 — la photographie est chiffrée, et elle renverse l'ordre des travaux

`outils/cible_image.py` détecte l'horizon par la chute de luma la plus franche, sépare ciel et mer,
et publie séparément ce qui est **comparable** (tout ce qui est normalisé par la scène elle-même)
et ce qui ne l'est pas (les luma absolues — exposition et tone mapping inconnus).

Horizon de la photo : `y` = 356 sur 712, **exactement à mi-cadre**.

| grandeur comparable | **photo** | `eau_g2` (choisie par R14) | `tout` |
|---|---:|---:|---:|
| `mer_p05/p50` (densité des creux) | **0,193** | 0,277 | 0,415 |
| `mer_p95/p50` | 4,56 | 4,06 | 3,46 |
| **dynamique `p95/p05`** | **23,7** | **14,6** | 8,3 |
| **contraste local / p50** | **0,455** | **0,266** | 0,217 |
| fraction claire (> 4 × médiane) | 0,074 | 0,055 | 0,015 |
| creux `B/G` | **5,54** | **5,63** | 2,59 |
| creux `B/R` | 30,0 | 14,1 | 11,6 |
| crêtes `B/G` | **2,89** | **1,37** | 1,32 |
| crêtes `B/R` | 8,95 | 1,73 | 1,72 |
| ciel `B/G` | **2,03** | **1,41** | 1,41 |
| ciel `B/R` | 5,14 | 1,82 | 1,82 |
| **ciel haut / horizon** | **0,356** | **0,809** | 0,809 |

**Trois lectures, et la troisième commande la suite.**

1. **La couleur des creux est juste.** `eau_g2` donne 5,63 contre 5,54 mesurés. La couleur dérivée
   d'ADR-177 au gain 2 **tombe sur la cible**, et le verdict de l'utilisateur la désignait déjà.
   Noter au passage que la photo (5,54) se situe **entre** notre ancienne constante (2,83) et
   l'eau pure théorique (10,85) : une mer réelle porte des particules, et le capteur a sa balance.
2. **Notre dynamique et notre contraste local valent 60 % de ceux de la photo** (14,6 contre 23,7 ;
   0,266 contre 0,455). C'est exactement ce que l'utilisateur décrit — « creux plus denses,
   contraste local plus marqué ».
3. **Et la cause est le ciel.** Notre ciel est **plat** — sommet à 0,809 de l'horizon, quand la
   photo est à **0,356** — et **pâle** : `B/R` 1,82 contre 5,14. Mesuré bande par bande :

   | | horizon | | | | | haut |
   |---|---:|---:|---:|---:|---:|---:|
   | **photo** R | 0,311 | 0,211 | 0,122 | 0,079 | 0,058 | **0,043** |
   | **photo** G | 0,554 | 0,486 | 0,365 | 0,275 | 0,221 | **0,181** |
   | **photo** B | 0,795 | 0,787 | 0,740 | 0,687 | 0,631 | **0,574** |
   | **nous** R | 0,533 | 0,450 | 0,446 | 0,421 | 0,374 | 0,407 |
   | **nous** G | 0,675 | 0,588 | 0,572 | 0,545 | 0,502 | 0,523 |
   | **nous** B | 0,851 | 0,810 | 0,801 | 0,787 | 0,767 | 0,775 |

   Dans la photo, **le bleu ne bouge presque pas** (×0,72 de l'horizon au haut) pendant que le
   rouge s'effondre (**×0,14**) et le vert chute (×0,33). C'est la signature de Rayleigh : à
   l'horizon la masse d'air diffuse toutes les longueurs d'onde et le ciel blanchit ; au zénith
   seul le bleu survit. **Notre ciel ne fait rien de tout cela** : les trois canaux bougent de
   moins de 25 %, et le profil n'est même pas monotone.

**Conséquence, et c'est elle qui renverse le plan.** La mer est un miroir : un ciel plat et pâle
donne des crêtes grises (`B/G` 1,37 contre 2,89), une dynamique écrasée et un contraste local
faible. **Le ciel passe donc avant l'exposition et avant le gain de couleur**, que je comptais
traiter d'abord. L'utilisateur l'avait formulé sans le mesurer : « les reflets clairs restent
portés par le **ciel** et la géométrie, pas par une couleur de base trop élevée ».

**Ce que la mesure ne peut pas donner** : l'élévation réelle que couvre le haut du cadre de la
photo — focale inconnue. Le profil est donc mesuré en **fraction de cadre**, pas en élévation, et
tout modèle calé dessus porte une hypothèse déclarée sur ce champ.

## P4 — le ciel calé sur la photographie, et ce que la mesure dit ensuite

`--ciel-mesure[=degrés]` remplace l'interpolation de deux couleurs par une **extinction
exponentielle par canal** en `sin(élévation)` :

```
ciel(s) = CIEL_HORIZON · exp(−K · s),   K = −ln(CIEL_F) / sin(élévation du haut du cadre)
CIEL_HORIZON = (0,311 ; 0,554 ; 0,795)      CIEL_F = (0,139 ; 0,327 ; 0,722)
```

Les deux constantes **sont** la photographie, mesurée. `CIEL_F` porte la signature de Rayleigh :
le bleu à peine atténué, le rouge effondré. L'élévation du haut du cadre de la photographie est
inconnue (focale inconnue) : elle est **exposée en paramètre**, défaut 25°, et se balaye — même
discipline que le gain d'ADR-177. La brume d'horizon suit désormais la couleur du ciel à
l'horizon : c'est le même air.

| grandeur comparable | **photo** | avant (`eau_g2`) | **après (`ciel25`)** |
|---|---:|---:|---:|
| crêtes `B/R` | **8,95** | 1,73 | **3,03** |
| crêtes `B/G` | **2,89** | 1,37 | **1,69** |
| ciel `B/R` | **5,14** | 1,82 | **2,96** |
| ciel `B/G` | **2,03** | 1,41 | **1,63** |
| creux `B/R` | 30,0 | 14,1 | **29,5** |
| creux `B/G` | 5,54 | 5,63 | 7,04 |
| dynamique `p95/p05` | **23,7** | 14,6 | **12,3** |
| contraste local / p50 | **0,455** | 0,266 | 0,268 |
| `mer_p05/p50` | **0,193** | 0,277 | 0,310 |

**La teinte se rapproche partout, la dynamique non.** Les crêtes passent de 1,73 à 3,03 de `B/R`
(cible 8,95), les creux tombent à 29,5 contre 30,0 mesurés — **juste**. Mais la dynamique
**recule** (14,6 → 12,3) et le contraste local ne bouge pas.

**Deux erreurs de ma part, l'une dans l'instrument, l'autre dans ce que j'attendais.**

1. **L'instrument déclarait comparable ce qui ne l'est pas.** `ciel_haut_sur_horizon` est mesuré
   en **fraction de cadre** : deux images de champs de vision différents ne couvrent pas la même
   plage d'élévation, et le même ciel y donne deux profils. Notre cadre montre le ciel jusqu'à
   ≈ 15° d'élévation, la photographie jusqu'à ≈ 25° supposés — d'où 0,85 contre 0,36 **sans que
   le modèle soit en cause**. Corrigé : la grandeur est publiée sous `CADRAGE`, pas sous
   `COMPARABLE`.
2. **La détection d'horizon se trompe quand le ciel a un fort gradient** : sur le rendu à ciel
   calé elle a trouvé 125 au lieu de 223, la chute du ciel l'emportant sur celle de l'horizon
   (chute 0,017 contre 0,083 sur la photographie). La hauteur de la chute est désormais publiée
   à côté de la position : une chute faible dit qu'il faut forcer la ligne.

**Ce que la dynamique manquante désigne, et ce n'est pas le ciel.** Il reste un facteur **2** sur
`p95/p05` et sur le contraste local. Deux causes candidates, toutes deux hors du ciel :

- **l'absence de courbe de tonalité.** Nous écrivons du linéaire vers sRGB sans exposition ni
  contraste ; l'appareil photo, lui, applique une courbe en S qui écrase les ombres. C'est
  exactement le poste « exposition » que le verdict R14 nomme, et c'est désormais le plus gros
  levier restant **mesuré** ;
- **l'état de mer.** La photographie montre une mer de vent courte et raide ; notre scène porte
  une houle de 12 s plus douce. Des faces plus raides réfléchissent le ciel **haut**, donc sombre,
  et creusent la dynamique. **Je ne touche pas à cela** : l'utilisateur a suspendu les lots de
  forme, et c'est une hypothèse mesurée, pas une conclusion.

## P5 — la courbe de tonalité, et une erreur que la mesure a attrapée en un passage

*Étape interrompue par une coupure de session, reprise et **revérifiée** avant commit : le code de
la copie de travail a été rebâti, les images refaites et remesurées ; tous les chiffres ci-dessous
se reproduisent au centième. La photographie remesurée redonne elle aussi ses valeurs exactes —
l'instrument est reproductible.*

`--tonalite=e,g,w` : `x = (l·e)^g` écrase les ombres, `l' = x(1 + x/w²)/(1 + x)` comprime les
hautes lumières. Appliquée à **toute l'image**, mer et ciel, sinon le raccord se voit.

**Premier essai, faux : la courbe par canal.** Elle a crevé la teinte — `B/R` des creux passait de
29,5 à **191** pour 30,0 mesurés sur la photographie, parce qu'une puissance > 1 écrase d'autant
plus un canal qu'il est petit, et le rouge est le plus petit. Corrigé : la courbe ne touche que la
**luminance**, la teinte est conservée par un rapport. La couleur vient d'ADR-177 et du ciel ; la
courbe ne doit toucher qu'à la dynamique.

| grandeur comparable | **photo** | sans courbe | `1 ; 1,4 ; 6` | `1,2 ; 1,5 ; 5` | `1,4 ; 1,6 ; 5` |
|---|---:|---:|---:|---:|---:|
| `mer_p05/p50` | **0,193** | 0,310 | **0,192** | 0,172 | 0,154 |
| dynamique `p95/p05` | **23,7** | 12,3 | 28,4 | 35,1 | 43,7 |
| contraste local / p50 | **0,455** | 0,268 | 0,382 | 0,418 | 0,459 |
| creux `B/R` | **30,0** | 29,5 | **31,3** | 31,2 | 30,7 |
| creux `B/G` | 5,54 | 7,04 | 7,06 | 7,06 | 7,04 |
| fraction claire | **0,074** | 0,037 | 0,175 | 0,206 | 0,238 |

**Retenu : `1 ; 1,4 ; 6`.** Il tombe **exactement** sur la densité des creux (0,1923 contre 0,1926
mesurés) et porte le contraste local à 84 % de la cible, sans pousser la dynamique au-delà du
raisonnable. La teinte ne bouge plus d'un chiffre entre les trois réglages : la séparation
couleur / dynamique tient.

## P5 — résultat négatif : la coupure spectrale est définitivement le mauvais levier

Il reste **un** poste franchement hors cible : la **fraction claire**, 0,175 contre **0,074**
mesurés — 2,4 fois trop de pixels très clairs. C'est le tapis de scintillement, et c'est le
dernier item de la liste du verdict R14 (« gestion des hautes fréquences dans le reflet »).

S306 proposait la coupure spectrale. Elle a maintenant un critère, et elle échoue :

| `--coupure` | 1 | 1,5 | 2 | 3 |
|---|---:|---:|---:|---:|
| fraction claire *(cible 0,074)* | 0,175 | 0,179 | 0,183 | **0,200** |
| contraste local *(cible 0,455)* | **0,382** | 0,351 | 0,306 | **0,230** |

**Élargir la coupure n'enlève pas les taches claires — elle en ajoute — et elle détruit le
contraste local.** Le verdict est net : la coupure retire la structure qui porte le contraste sans
retirer ce qui brille. S306 la proposait comme piste, S307 disait déjà qu'elle n'était pas le bon
levier ; **c'est maintenant mesuré contre une cible**, et le sujet est clos. *(Le point extrême,
`--coupure=3`, a été refait à la reprise : 0,19976 et 0,2304, aux mêmes chiffres.)*

**Ce que les taches claires sont, alors.** Hypothèse désignée par élimination et par le code, pas
encore vérifiée : le **miroitement du soleil** est une puissance dure d'un soleil ponctuel
(`pow(dot(reflet, soleil), 180)` et un terme large `pow(·, 32)` dans le ciel), et non une
intégrale sur le disque solaire **et** sur la distribution de pentes. Sur une mer devenue sombre,
chaque facette qui attrape la direction du soleil s'allume seule. C'est exactement ce que
`ReflectedSunRadiance(..., σ²)` de Bruneton intègre, et ce que notre `--reflets-filtres` fait
pour le **ciel** — le terme de miroitement, lui, est ajouté **après** l'intégration. C'est P6.

**Ce que l'œil voit déjà sur elle, à confirmer par la mesure** : mer très sombre, creux presque
noirs ; crêtes gris-bleu clair ; ciel à gradient **fort** (blanc-cyan à l'horizon, bleu profond en
haut) avec une bande claire mince juste au-dessus de l'horizon ; **très peu d'écume**, quelques
mouchetures ; **pas de chemin de miroitement**, le soleil n'est pas dans le champ ; mer de vent
courte et raide, pas de houle longue dominante.
## P6 — le miroitement du soleil n'est pas le coupable : hypothèse réfutée

P5 laissait une hypothèse nommée : les 2,4 × trop de pixels très clairs viendraient du reflet
spéculaire du soleil, `pow(dot(reflet, soleil), 180)`, trop dur pour être résolu. Elle est
**fausse**, et il a suffi d'un interrupteur pour le savoir.

`--miroitement=<facteur>` échelonne ce terme ; 1 est le rendu historique, et il l'est **au bit** —
empreintes `0x4a200520b14ac563` (proche) et `0xe2abde41dd05f6ba` (rasante), identiques avant et
après l'ajout du facteur.

| | fraction claire *(cible 0,074)* | contraste local | dynamique |
|---|---:|---:|---:|
| `--miroitement=1` *(historique)* | 0,1747 | 0,382 | 28,4 |
| `--miroitement=0` *(éteint)* | **0,1837** | 0,385 | 28,8 |

**Éteindre complètement le miroitement ne retire pas un pixel clair — il en ajoute.** La hausse
n'est pas un paradoxe : le seuil est *relatif* à la médiane de la mer, qui baisse quand on retire
de la lumière. Le miroitement ne pèse donc rien dans ce poste.

**Où est le coupable, alors : dans la courbe.** Le chiffre était déjà sous les yeux en P5 et
personne ne l'avait lu comme une cause — sans courbe, la fraction claire vaut **0,037** ; avec la
courbe retenue, **0,175**. C'est la courbe qui fabrique les pixels clairs, en écartant la
distribution. Et ce n'est pas un défaut de réglage mais une **contrainte de forme** : dans la
plage de luma de la mer, `x = (l·e)^g` est presque une pure loi de puissance, si bien que

    p95/p50 ≈ (l95/l50)^g   et   p05/p50 ≈ (l05/l50)^g

sont commandés par le **même** exposant. Caler la densité des creux sur la photographie
(`g = 1,4`) fixe donc la queue claire du même coup — 2,4 fois trop haute. **Un seul paramètre ne
peut pas tenir les deux bouts.**

Ce qui reste à essayer, et c'est P7 : le genou de Reinhard ne mord qu'au voisinage de 1, très
au-dessus de la mer (`p95` après courbe ≈ 0,19). **L'exposition le descend dans la plage utile** —
elle n'est pas un réglage de clarté ici, c'est le levier de compression des hautes lumières. Un
premier balayage hors GPU le confirme : à `g` constant, passer `e` de 1 à 2 fait tomber la
fraction claire de 0,174 à 0,103 sans toucher à la teinte.

**Deux vérifications d'instrument, faites en passant.** L'image rendue sans courbe ne sature
**aucun** pixel de mer (0 sur 632 320 à 255), et un modèle hors GPU qui applique la courbe à sa
luma redonne les chiffres du GPU à 2 % près — `0,1956 / 27,84 / 0,3830 / 0,17402` contre
`0,1923 / 28,40 / 0,3821 / 0,17470`. **La recherche peut donc se faire hors GPU**, à condition de
revérifier le gagnant sur la carte. C'est ce que fait P7.

## P7 — la courbe ne peut pas fermer l'écart, et ce qui reste n'est pas de la tonalité

`outils/courbe_tonalite.py` cherche `(e, g, w)` contre les **quatre** cibles à la fois, sur un
rendu fait *sans courbe* : la courbe ne touchant qu'à la luminance, les statistiques de luma se
recalculent sans refaire l'image. La courbe est croissante, donc les centiles se lisent dans un
tableau trié **une seule fois** — 6 300 candidats en quelques secondes. Le contraste local, lui,
demande la fenêtre 9 × 9 : il n'est calculé que sur la liste courte. Critère déclaré avant la
mesure : minimiser le **pire** écart relatif logarithmique sur les quatre grandeurs.

Le premier étage atteint **0,058** de pire écart sur trois grandeurs (creux, dynamique, fraction
claire) — la photographie est donc rattrapable sur son histogramme. Puis le second étage tombe,
et il tombe **de la même façon pour les huit meilleurs candidats** :

| | `p05/p50` | dynamique | **contraste local** | fraction claire | pire écart |
|---|---:|---:|---:|---:|---:|
| **photo** | 0,193 | 23,7 | **0,455** | 0,074 | — |
| `3,096 ; 1,55 ; 11,31` | 0,178 | 23,8 | **0,318** | 0,082 | 0,357 |
| `4,458 ; 1,60 ; 2,83` | 0,176 | 23,8 | **0,318** | 0,071 | 0,358 |
| `2,580 ; 1,45 ; 11,31` | 0,197 | 21,2 | **0,311** | 0,072 | 0,380 |
| … les huit | … | … | **0,311 – 0,318** | … | … |

**Les huit meilleurs réglages donnent le même contraste local, à 2 % près, et il manque 30 %.**
Aucun n'est limité par autre chose : sur les huit, `cible_la_plus_dure` vaut `contraste`.

**Ce que cela démontre.** Le contraste local est une propriété **spatiale** — l'écart-type de luma
dans une fenêtre de neuf pixels. Une courbe de tonalité est une fonction **point à point** : elle
ne peut que redistribuer l'histogramme, pas créer de la structure à l'échelle de la fenêtre. Le
réglage précédent (`1 ; 1,4 ; 6`) obtenait 0,382 — mais en payant une fraction claire de 0,175
pour 0,074 visés, c'est-à-dire en fabriquant du mouchetage clair que la mesure compte comme du
contraste. **Il n'y a donc pas de réglage à trouver : ce qui manque est dans l'image, pas dans la
courbe.** C'est la mer elle-même — détail de surface à l'échelle de quelques pixels — et
l'utilisateur avait suspendu les lots de forme.

**Vérifié sur la carte**, et c'est la règle : `3,096 ; 1,55 ; 11,31` rendu par le GPU donne
`0,1800 / 23,65 / 0,3186 / 0,0824` contre `0,1781 / 23,80 / 0,3182 / 0,0815` prévus — 1 %.

**Une leçon d'instrument, et c'est la deuxième fois.** La détection automatique d'horizon s'est
encore trompée sur un de ces rendus, et a donné `0,3652 / 15,11 / 0,4890 / 0,1046` là où la mer
seule donne `0,1766 / 22,81 / 0,3082 / 0,0528`. P4 l'avait déjà constaté et avait publié la
hauteur de chute pour le signaler. **Cela ne suffit pas** : comparer des rendus entre eux demande
`--horizon=<y>` **forcé**, toujours. À inscrire en leçon au rituel.

**Ce qui n'a pas été fait, et qui reste dû** *(ADR-127 : rien n'est retiré du périmètre)* :
diffusion sous la surface aux crêtes, écume de Monahan, et la structure fine de la mer que le
contraste local réclame. L'utilisateur a redirigé la session vers la physique ; ces trois postes
partent en file, non construits, avec leur cible chiffrée — c'est-à-dire prêts.

## Archive — notes de S304 (lot de la mer, en attente du verdict R12)

**Construit** : `tayfun()` et `lagged_eps()` dans `water.wgsl`, accumulations par système dans la
boucle existante de `band_cwm` (branches explicites — FXC refuse l'indexation dynamique en
écriture, L345) ; douzième `vec4` de l'uniforme `(split, k̄₁, k̄₂, retard)` ; mêmes deux termes dans
la référence CPU `cwm_reference` ; activation par `--vagues --modulation`, témoin `--sans-asym`,
balayage `--retard=`. Le fragment n'a pas changé : le sommet lui transmet la déformation **retardée**.

**Réception d'ADR-176 §3, cinq critères tenus** :

| critère | exigé | obtenu |
|---|---|---|
| 1. statistiques | `Sk` ≥ 0,06 ; `c₀₃` ∈ [−0,18 ; −0,13] ; `c₂₁` −0,058 ± 0,02 ; `c₄₀` ∈ [0,26 ; 0,45] ; `mss` ± 2 % | **0,0656** ; **−0,155** ; **−0,057** ; **0,340** ; +0,2 % |
| 2. GPU contre CPU | 3 mm ; 5·10⁻⁴ | 1,59·10⁻⁶ m ; 2,69·10⁻⁴, aucun repli |
| 3. scènes au bit | défaut, `--houle`, `--vagues`, `--sans-asym` | **identiques** au binaire de S301 |
| 4. écart au jeu | publié | 0,3651 → **0,3999 m** (+3,5 cm ; CWM en porte 36,5) |
| 5. coût | publié, secteur relevé | 1,0138 → **1,0163 ms** (+0,25 %) |

`k̄` par système : **0,1742** (mer de vent) et **0,0317** (houle) — l'instrument et l'hôte donnent
les mêmes, donc les deux implémentations portent bien le même modèle.

**Images R12** : `--revue-mer=<état>`, quatre poses de R11, trois états nommés dans le fichier
(`a_houle_seule`, `b_vagues_modulation`, `c_asymetries`). Regardées ici : l'écart `a` → `c` est
franc (la mer de R11 était lisse et striée) ; l'écart `b` → `c` est plus fin — crêtes plus marquées,
contraste crête/creux plus net. C'est à l'utilisateur de juger, R12 pose quatre questions.

---

## Archive — notes de S303 (pour le lot)

**Ce que la recherche donne** (sources dans la preuve) : l'asymétrie verticale d'une mer profonde
vient des **harmoniques liées du second ordre** (`Sk = 3k̄σ` en bande étroite, Longuet-Higgins 1963,
Tayfun 1980) ; les pentes se mesurent au miroitement depuis Cox & Munk 1954, révisées par IASI ; et
— le plus visible — les **rides ne sont pas uniformes** : elles se raccourcissent et se redressent
sur les crêtes des vagues longues, s'aplatissent dans les creux (JFM 2024 : pente modulée de 20 %
à `ε_L` = 0,1, doublée à 0,4), avec un **retard** qui place leur maximum en avant de la crête.

**Ce que la mesure donne** (instrument de S260 étendu, 10⁶ points, même réalisation que le rendu) :

| | `mss` | `c₂₁` | `c₀₃` | `c₄₀` | `Sk` |
|---|---:|---:|---:|---:|---:|
| observations (Cox–Munk 7,95 m/s ; 3k̄σ) | 0,0437 | −0,058 | −0,222 | 0,40 | 0,156 |
| `--houle` — **la scène montrée en R11** | 0,0198 | −0,001 | 0,002 | −0,026 | −0,0001 |
| `--vagues --modulation` (le meilleur construit) | 0,0496 | 0,001 | 0,001 | 0,390 | 0,0030 |
| **S303 retenu** (+ Tayfun + retard −0,20) | 0,0497 | **−0,057** | **−0,155** | 0,340 | **0,066** |

**Trois choses apprises, dans l'ordre d'importance.**
1. **Faute de protocole de ma part** : la scène soumise à R11 était `--houle` **seule**, c'est-à-dire
   sans la queue d'équilibre ni les vagues pointues construites en S260–S261. C'est la
   configuration que S260 avait déjà mesurée « pentes quasi gaussiennes ». L'utilisateur a jugé la
   plus pauvre des trois mers du dépôt.
2. **Même la meilleure n'a aucune asymétrie** : `Sk` 0,003 contre 0,156 ; crêtes et creux aussi
   arrondis les uns que les autres — le « concave plutôt que convexe » du verdict, chiffré.
3. **Le second ordre par composante ne peut pas la produire** (mesuré : 0,0022) : ce sont les
   termes **croisés** qui la portent. D'où Tayfun, qui les contient tous pour une somme de plus.

**Le retard est le seul paramètre libre**, calé sur `c₀₃` ; son **signe** ne l'est pas — seul un
retard négatif (rides en avant de la crête) donne l'asymétrie du signe observé, et `c₂₁` tombe sur
Cox–Munk sans avoir été visé. Reste ouvert : le noyau exact du second ordre pour deux systèmes
(la vérité est entre 0,063 et 0,150), les capillaires parasites, l'asymétrie horizontale.

---

## Archive — notes de S302 (pour le lot)

**P2 — la scène tient** (`--delta3d-scene-mesure`, mer `--houle` à 64 composantes, domaine
96×128×28 à 25 cm = 344 064 mailles, repos 3,5 m, éponge 3 m, 32 cycles, 60 Hz, 12 s) :
- **aucune colonne hors bornes**, jamais, avec et sans paquet ;
- le paquet garde ses 25 cm et traverse de `y = 26` à `y ≈ 7` en 11 s, soit **1,7 m/s** — la
  vitesse de groupe théorique d'une onde de 8 m vaut 1,77 m/s. L'onde isolée (paquet − témoin)
  reste à 0,18–0,26 m ;
- δ **sans paquet** porte déjà 9 à 19 cm : la correction couplée de B à Hs 2,5 m (R10 : 10,6 cm
  mesurés en 2D à Hs 2 m). À dire dans la revue : une partie de ce que l'utilisateur verra n'est
  pas l'onde mais cette correction ;
- coût **4,23 ms** par pas (médiane, 177 dispatchs) : au-delà des 2 ms d'ADR-174 D3, ce qui est un
  point de la porte C, pas de la revue ; tenable à 60 Hz avec le rendu (≈ 2 ms) ;
- **tous les pas reçus sont déclarés dégradés** au sens d'ADR-144 (divergence franche 2·10⁻²).
  Les diagnostics ne reviennent que 60 fois sur 720 : l'anneau est plein tant que la carte
  travaille — perdus, jamais retardants, comme D3 le prévoit.

`Step3::on_device` : le pas se construit sur un device **fourni**, celui du rendu ; la surface
publiée devient un tampon que le rendu lie (D7), sans passage par le CPU. `published_buffer()` est
le seul tampon exposé.

**P3 — les à-coups d'A297 se voient dans les chiffres** (`--delta3d-scene-acoups`, dérivée seconde
temporelle par colonne, 2,2 à 4,4 millions d'échantillons) :

| | d2 RMS | q99,99 | max | localité | rugosité de maille RMS | max |
|---|---:|---:|---:|---:|---:|---:|
| témoin, 32 cycles | 9,2·10⁻⁵ m | 1,8·10⁻³ | 7,2·10⁻³ | **7,4** | 2,17·10⁻³ m | 5,9·10⁻² |
| témoin, 128 | 9,2·10⁻⁵ | 1,8·10⁻³ | 4,8·10⁻³ | 7,3 | 1,99·10⁻³ | 2,1·10⁻² |
| témoin, 512 | 9,2·10⁻⁵ | 1,7·10⁻³ | 4,8·10⁻³ | 7,3 | 2,03·10⁻³ | 2,2·10⁻² |
| paquet, 32 | 1,6·10⁻⁴ | 2,0·10⁻³ | 6,7·10⁻³ | 7,3 | 2,68·10⁻³ | 4,7·10⁻² |

« Localité 7,4 » : au pire à-coup, la colonne fautive vaut 7,4 fois la moyenne de ses huit
voisines — la signature d'une bascule, pas d'une onde. **Le balayage de cycles tranche** : de 32 à
512 cycles la rugosité ne bouge pas (2,17 → 1,99 → 2,03 mm) ; ce n'est **pas** une pression
sous-convergée, c'est le schéma. À comparer à la signature du paquet lui-même à l'échelle de la
maille, `a·k²·dx²` = 9,6·10⁻³ m : le bruit vaut environ **un cinquième** du signal utile en
hauteur, et jusqu'à 5 fois au pire point. Conséquence pour la revue : le bruit de maille est à
montrer et à nommer, pas à cacher ; 32 cycles suffisent visuellement (le maximum seul gagne à
128), donc le choix de cycles est une question de coût.

**P4 et P5 — la scène est rendue et soumise.** `Step3::on_device` fait naître le domaine sur le
device du rendu ; le nuanceur lie **la seule surface publiée** (groupe 3), l'interpole en
Catmull-Rom bicubique et l'ajoute à la somme des couches avec un fondu de 3 m. Témoin : les sept
images de la revue R9 gardent leurs empreintes **au bit** (0x8ae42dfb1fe7d1b3…) — le rendu existant
n'a pas bougé. Fenêtre : `--houle --delta3d`, **D** bascule, **R** relance l'onde ; un pas fixe de
16,667 ms par image, donc le fond que δ consomme et celui que l'image montre sont au même instant.
Cadence **197 Hz** (5,06 ms), essai de 120 images passé.

**Dimensionnement, deux allers-retours mesurés** : d'abord 24×32 m avec une onde de 25 cm à 8 m —
l'onde se noyait dans une mer de 2,5 m (les deux images se ressemblaient). Puis 32×32 m : **refusé
par le device**, le tampon des faces (26 flottants par face, S300) franchissant les 128 Mio d'une
liaison de stockage. Retenu : **30 × 28 m** (120×112×28) et un **front** de 65 cm sur 16 m, crête
longue de 12 m — visible sans être hors du régime perturbatif (`ak` 0,26 ; le refus non diagnostiqué
de 2D est à 0,335). Une vue plongeante a été essayée et écartée : rien ne s'y lit (déjà mesuré S275).

Captures : quatre poses × avec/sans × quatre instants, plus deux images de différence ×6.
La couche change **13,5 à 15,7 %** des octets à la pose de référence (8,5 à 11,2 % de plus de
quatre niveaux). Preuve : [SCENE-DELTA3D-S302](../docs/validation/SCENE-DELTA3D-S302.md) ; demande
de revue : REVUE-VISUELLE §16 (R11), avec **quatre questions explicites** — R10 avait échoué faute
de dire ce qu'on attendait.

*Tenue du plan* : le battement du commit P2+P3 a été écrit **sans lire l'horloge** (23:18 au lieu
de 23:06, L237) ; corrigé au commit suivant. Une fenêtre interactive s'est ouverte une fois parce
que l'ancien binaire ignorait un drapeau neuf après un échec de compilation — vérifier que la
compilation a réussi avant de lancer.

---

## Archive — notes de S301 (pour le lot)

**Architecture retenue (P2).** `Step3` crée **un** device et y compile les trois sources telles
quelles — `delta3d_background.wgsl` (S300), `delta3d_cg.wgsl` (S299), `delta3d_step.wgsl`
(nouveau) — chacune avec sa propre disposition de liaisons, sur des tampons communs. Rien de
S299/S300 n'est réécrit (L137). Rangements : `vel = [u|v|w courants | u|v|w prédits]`, l'indice
d'une face étant aussi celui de son échantillon de fond ; `cells_in = [eta | divergence |
eta_roundoff]` — **eta vit là**, c'est l'entrée de `couple_columns` ; `cells_out` = celui de S300 ;
`work = [flux_x | bande_x | flux_y | bande_y | surface publiée]`. Les passages vers la projection
(surface totale → `heights`, second membre → tranche B, préconditionneur → tranche M) seront des
copies de tampon **dans l'encodeur**, sans retour CPU.

**P2 reçu** (`--delta3d-prediction`, 15×11×14, 7 459 faces, fond S300 à 64 composantes, vitesses
d'ordre 0,2 m/s, éponge (1 ; 0,75 ; 2 s⁻¹), dt 5 ms, trois instants) : pire écart **6,0·10⁻⁸ m/s**,
soit **≤ 9,0·10⁻⁶ de l'incrément** du pas, sur les trois familles ; 70 à 85 % des faces au bit ;
zéro face fautive. Refus : durée nulle, éponge trop large, `dt²g/dx > 1`, longueur.

**Défaut trouvé par le banc, corrigé avant commit** : le noyau couplait les faces `i = nx` (u) et
`j = ny` (v), que le cœur saute — borne comparée à `n + 1` au lieu de `n`. Écart de **60 % de
l'incrément**, sur ces seules faces. Le compteur `faces_fautives` (> 5 % de l'incrément) reste
dans le banc : c'est lui qui voit une règle de bord portée autrement, l'arrondi ne le peut pas.
Rapporter l'écart à l'incrément et non à la vitesse était nécessaire : rapporté à la vitesse, le
même défaut ne pesait que 1 %.

**Incident d'outillage, corrigé par un commit séparé** : le battement de P2 a été écrit par
`Get-Content -Raw | Set-Content -Encoding utf8` de Windows PowerShell 5.1, qui **lit en ANSI** :
`REPRISE.md` est parti ré-encodé (mojibake) dans `fff03d5`. Restauré depuis `7ebeeb5`, battement
réécrit à l'outil d'édition. **Ne jamais réécrire un fichier du dépôt par `Get-Content` /
`Set-Content`** ; `[IO.File]::ReadAllText/WriteAllText` (UTF-8 par défaut) ou l'outil d'édition.

**P3a reçu** (`--delta3d-pression`, même fixture, 1 510 mailles mouillées sur 2 310, départ
`p = 0` des deux côtés). Le cœur converge en 65 et 68 itérations, sans affinage. La carte :

| cycles | dispatchs | écart p (t=0) | relatif | écart p (t=1,23 s) | relatif |
|---|---|---|---|---|---|
| 8 | 51 | 2 757 Pa | 0,17 | 2 992 Pa | 0,18 |
| 32 | 171 | 218 Pa | 1,3·10⁻² | 196 Pa | 1,2·10⁻² |
| 64 | 331 | **0,27 Pa** | 1,6·10⁻⁵ | **0,20 Pa** | 1,2·10⁻⁵ |
| 128 | 651 | 0,38 Pa | 2,3·10⁻⁵ | 0,077 Pa | 4,7·10⁻⁶ |

Échelle 16 400 Pa. Plateau dès 128 cycles : plancher f32. En usage : 0,27 Pa ≈ **0,03 mm d'eau**.
Aucune maille sèche non nulle. Résidu vrai de la carte 3,6·10⁻⁷ à 64 cycles, celui du cœur
2,3·10⁻⁷. À 8 cycles l'écart vaut 27 cm d'eau **depuis p = 0** : en trajectoire le départ chaud
part de la pression du pas précédent, et c'est P5 qui dira combien de cycles il faut alors.
`couple_rhs` lit désormais `eta_roundoff` (`cells_in` après la divergence) ; banc S300 rejoué,
chiffres identiques (1,06·10⁻⁶ ; 6,0·10⁻⁸ ; 3,9·10⁻⁷). `init_warm` ajouté à `delta3d_cg.wgsl`.

**P3b reçu** (`--delta3d-correction`, même fixture, vitesses de fin de pas contre celles du cœur
après `step_perturbation_mobile`, extrapolées comprises) : incrément du pas ≈ 0,5 à 0,76 m/s (le
champ de départ est fortement divergent, la projection le redresse). Écart **≤ 1,4·10⁻⁵ m/s**,
**≤ 2,6·10⁻⁵ de l'incrément** à 128 cycles, ≤ 1,7·10⁻⁵ à 64 ; **zéro face fautive** sur les trois
familles et trois instants. Fantômes latéraux avec le fond de la face (`p_dyn`, `grad_p_dyn`),
fantôme du haut avec `eta_roundoff` et `ghost_up` de S300 : aucune règle de bord divergente.

**P4 reçu** (`--delta3d-pas`, pas complet à 128 cycles, **655 dispatchs**, trois instants) :
- hauteur : incrément du pas 4,2 à 5,1 mm ; écart **2,4·10⁻⁷ m** (un ulp de η à 2,25 m),
  159 à 162 colonnes sur 165 au bit ; surface publiée au même écart ;
- **hauteur vraie** `η − reste` : écart **3 à 4·10⁻⁸ m**, six fois sous l'ulp — la compensation
  est portée ;
- vitesses ≤ 1,4·10⁻⁵ m/s (2·10⁻⁵ de l'incrément), pression ≤ 0,38 Pa (2,3·10⁻⁵).
Surface publiée = `(η − repos) − reste` par colonne, **tampon à part** (liaison 7) : le rendu ne
liera jamais `cells_in` ni `vel` (D7, I-13). Accès d'essai `surface_roundoff_for_trials` ajouté au
cœur.

**Défaut trouvé, corrigé avant commit — la somme compensée détruite par le compilateur.** Premier
passage : reste nul sur les 165 colonnes. Expérience : sur la carte `(η + inc) − η` rendait `inc`
**au bit** pour 165/165 colonnes, quand la même addition est inexacte sur CPU pour 164 à 165/165.
Le compilateur de la carte (DX12) simplifie `(a + b) − a → b` ; la compensation de S233 disparaît
sans bruit — hauteur au bit près dans 130/165 colonnes seulement, et une perte systématique de
l'ordre de l'ulp de η par pas. Remède : `exact_difference(s, a)`, soustraction **en entiers sur
les bits IEEE**, exacte par Sterbenz pour deux hauteurs à moins d'un facteur deux. Même famille
que L345 (fraction de phase) : **une identité flottante du source n'est pas une identité du
binaire compilé** — généralisation à écrire en leçon. *(Écrite : L346.)*

**P5 — trajectoire S298, premier passage** (`--delta3d-trajectoire`, 32×24×36, 64 composantes,
1 200 pas, cycles 8/16/32/64 en parallèle, 4 min 39) : écart de hauteur **≤ 5·10⁻⁶ m jusqu'à
t = 1 s** pour 32 et 64 cycles (1·10⁻⁴ à 8 cycles, 5·10⁻⁵ à 16), puis saut au millimètre vers
t ≈ 1,1 s et 1 à 3,5 cm ensuite, **quel que soit le nombre de cycles**. Donc pas la projection.
Localisation (`PAS`, `CYCLES`, `SONDE_PAS` en variables d'environnement) : le saut naît à
**n = 215** dans la maille **(19,10,32)** — **sèche pour le cœur (p = 0), mouillée pour la
carte (p = 37 Pa)** : une surface passée à moins de 10⁻⁶ m d'un centre de maille, classée des deux
côtés opposés. Une seule bascule donne 0,25 m/s d'écart sur une face `v`, 0,08 sur `u`, puis des
centimètres de hauteur. Avant la bascule, les faces adjacentes aux mailles de surface à petit θ
diffèrent déjà de 2·10⁻³ m/s (coefficient 1/θ jusqu'à 1 000) sans effet visible sur η.
**Fausses pistes écartées** : B n'est pas en cause (élévation carte/cœur à 9·10⁻⁸ m sur ce fond,
`--delta3d-fond-s298`) ; l'éponge non plus (colonne intérieure). Une première sonde décalée d'un
pas (lue après le pas du cœur) avait fait croire à un écart de B — artefact de l'instrument.
**Question ouverte à trancher par la mesure** : sensibilité **propre** du schéma de référence
(deux cœurs à ±10⁻⁶ m au départ, `--delta3d-sensibilite`) — si deux cœurs se séparent pareil,
l'écart est une propriété de la référence et le critère ponctuel de §4.2 n'est tenable que sur
une durée déclarée avant la première bascule.
Artefact de banc corrigé : la surface publiée n'était écrite qu'au premier pas (écart 0,16 m à
t = 0) ; `set_state` la publie désormais.

**P5 — tranché par la sensibilité de la référence** (`--delta3d-sensibilite`, deux cœurs, le
second à ±10⁻⁶ m au départ, motif haché, 1 200 pas) : **ils se séparent exactement comme la carte
et le cœur** — 4,6 à 7·10⁻⁵ m jusqu'à t = 1 s, premier dépassement du millimètre au **pas 260**
(1,3 s), 1 à 4,5 cm ensuite, écart quadratique 2,3 à 3,1 mm de t = 3 à 6 s. Carte contre cœur à
64 cycles : quadratique 1,9 à 2,4 mm sur la même période, premier millimètre vers le pas 216.
**L'écart de P5 est une propriété du schéma de référence, pas de la production.** Avant son
horizon, la carte suit le cœur **mieux** (≤ 5,5·10⁻⁶ m à 32–64 cycles) qu'un cœur perturbé d'un
ulp ne suit le cœur (≤ 7·10⁻⁵ m).
**Mécanisme** (sonde) : le transport de la hauteur lit la vitesse de la couche **partiellement
mouillée** (fraction `(surface − k·dx)/dx`). Quand la surface passe le centre de cette couche, sa
face bascule de « projetée » (correction à 1/θ, θ petit) à « extrapolée » depuis la couche
inférieure : 0,25 m/s d'écart sur une face, soit ~6·10⁻⁴ m de hauteur par pas
(`dt/dx · Δu · dx · ½`), observé 5,8·10⁻⁴ puis 2,8·10⁻⁴ m par pas. La hauteur est donc
**discontinue** en la position de la surface par rapport aux centres de maille : deux états
distants de 10⁻⁶ m divergent de centimètres en une demi-seconde. Nouvel angle mort (A297).
**Lecture du critère 2** (ADR-175 §4.2, « sur la durée déclarée ») : la durée ponctuelle ne
peut excéder l'horizon de prévisibilité **de la référence elle-même**, mesuré ; au-delà, la
production se compare à l'enveloppe que la référence a contre elle-même. Aucun seuil relevé.

**P6 mesuré** (`--delta3d-cout-pas`, pas entier horodaté de la première passe à la dernière,
copies comprises, fond S298 à 64 composantes, 30 passages, premier écarté ; secteur aux deux
bornes, BatteryStatus = 2, 98 %) — médianes de banc :

| domaine | mailles | 8 cycles | 16 | 32 | 64 |
|---|---:|---:|---:|---:|---:|
| 32×24×36 (cas S298) | 27 648 | 0,305 ms | 0,382 | 0,532 | 0,838 |
| 32³ | 32 768 | 0,329 | 0,410 | 0,561 | 0,876 |
| 64×64×32 | 131 072 | 1,106 | 1,299 | 1,697 | 2,504 |

Dispatchs : 57, 97, 177, 337. Un maximum isolé à 7,34 ms (64×64×32, 8 cycles, premier passage
8,39 ms) : pic d'amorçage de la famille A294, non attribué. **Ce n'est pas la porte C** (pas de
scène, pas de 99ᵉ centile) : 64×64×32 tient sous 2 ms jusqu'à 32 cycles en médiane de banc.

**P5, horizon mesuré** (trajectoire relancée avec la métrique) : premier millimètre au pas
260 (8 et 16 cycles) et 220 (32 et 64), contre 260 cœur contre cœur ; pire écart avant l'horizon
9,1·10⁻⁴ (8), 5,8·10⁻⁵ (16), 1,0·10⁻⁴ (32), 2,4·10⁻⁵ m (64) ; pire sur 6 s 3,4 à 4,3 cm, contre
4,5 cm pour la référence contre elle-même.

*Tenue du plan* — **fusion déclarée P5+P6+P7 en un commit** : leur code a été écrit pendant les
calculs de P5 (4 à 8 min chacun) dans le même fichier, et leurs mesures sont consignées ici.
Le banc de localisation `--delta3d-fond-s298` est gardé : il a écarté B.

**P7 reçu** (`--delta3d-diagnostics`) — justesse, depuis l'état de P4 : divergence des lignes
franches carte **1,1 à 1,8·10⁻⁶** à 128 cycles contre **2,7 à 7,3·10⁻⁶** pour le cœur (sa
projection s'arrête à ses critères, la carte fait 128 cycles) ; toutes lignes 2,5·10⁻⁶ à
8,4·10⁻⁵. À 16 cycles depuis p = 0 : 1,6 à 2,7·10⁻² → **déclaré dégradé**. Différé, 100 pas du
cas S298 à 16 cycles par le seul `step` : 97 diagnostics rendus, **âge 1 pas** pour 96 d'entre
eux, 3 appels sans retour (premiers pas), aucune attente, 68 ms au mur pour les 100 pas.
**84 pas sur 100 déclarés dégradés à 16 cycles** (franche jusqu'à 1,2·10⁻³) alors que la
trajectoire à 16 cycles suit le cœur à 5·10⁻⁵ m jusqu'à t = 1 s : la tolérance d'ADR-144 est
bien plus stricte que l'usage en hauteur — à documenter, sans la relever.
Anneau de 3 emplacements ; si aucun n'est libre, le diagnostic du pas est perdu, jamais le pas
retardé.

---
