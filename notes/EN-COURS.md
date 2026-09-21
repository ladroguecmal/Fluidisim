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

Session : S316 — **en cours**. L'**ordre C** d'ADR-181 D10 : les six propriétés du transfert
δ → W mesurées **ensemble**, chacune **attribuée** à la primitive, au raccord ou à δ ; et la phase
à dix longueurs d'onde, qu'S315 a laissée **indéterminée**, **déroulée** (A307).
Agent : Claude Opus 5, application desktop ; fichiers, git, cargo, carte réelle, accès web.
Entrée : la décision de l'utilisateur du 2026-09-20 ([ADR-183](../docs/adr/ADR-183-essai-oblique-phase-a-distance-et-ordre-c.md)),
D5 — *« évaluez conjointement l'amplitude, la direction, le spectre, la phase, la vitesse de
propagation et la réflexion artificielle. Distinguez les résultats propres à la primitive W, ceux
du raccord et ceux de la simulation δ »* ; et §7 de [S315](../docs/validation/ORACLE-ET-OBLIQUE-S315.md),
qui en est l'entrée.

**Contrôle de la règle des trois sessions (REPRISE §6.7).** Le lot 2 porte S310 à S315. Ce qui
autorise celle-ci n'est pas la suite déclarée par S315 : c'est la **décision de l'utilisateur**,
qui nomme l'ordre C comme étape suivante et la condition de sortie du qualificatif « partiel »
(ADR-183 D6). La demande de l'utilisateur prime sur la règle automatique.

Capacité visée : **le transfert δ → W est qualifié propriété par propriété** — pour chacune, sa
valeur à trois mailles, sa limite quand la maille s'affine, et son **attributaire** ; la phase à
dix longueurs d'onde n'est plus indéterminée. Consommateur : la décision de l'utilisateur sur le
passage à l'ordre D (restitution du volume net), qu'ADR-183 D6 conditionne à cette qualification.

**La question de fond, posée avant de mesurer : comment un écart s'attribue-t-il ?** Trois
sources, et un critère pour chacune, écrit avant le premier chiffre :

- **δ** — l'écart **converge** quand la maille s'affine (25 → 12,5 → 6,25 cm), vers zéro ;
- **raccord** — l'écart est présent **dès la ligne d'émission** et **ne converge pas** avec la
  maille : il vient du modèle d'identification (une porteuse, une enveloppe gaussienne) ;
- **primitive** — l'écart existe **sans δ ni ligne** : le train comparé, à sa naissance, à la
  forme qu'on lui a demandée ;
- et une quatrième, que S315 a dû ajouter : **l'instrument** — un écart qui vient de la façon de
  mesurer, et qui se démontre en donnant à l'instrument un défaut **connu** à trouver.

Trois mailles ne font pas une loi : si les trois points ne suivent pas une puissance de la maille,
**je le dis** et je n'attribue pas.

**L'instrument de phase (A307), déclaré avant d'être écrit.** Transfert à **fréquence fixe** :
à chaque colonne entre les deux lignes, la projection de `η(t)` sur `e^{iωt}` sur une **fenêtre
commune** aux deux signaux ; l'écart `D(x) = arg(Z_W · conj Z_δ)` se **déroule le long de `x`**, où
il varie lentement. Aucune pulsation ajustée, aucun terme de temps, aucun décalage (ADR-183 D3). À
fréquence fixe, le retard de groupe n'est plus une erreur de phase : il devient la **pente de `D`
en `ω`**, et c'est lui qui mesure la vitesse.

Critères, écrits avant la mesure :
1. L'instrument retrouve un écart de phase **connu et supérieur à un demi-tour** avant d'être
   appliqué à δ.
2. Les prédictions de S315 §2 (−0,853 ; −0,165 ; −0,0072 tour à 10 λ) sont confrontées telles
   qu'écrites, **sans être recalculées** après la mesure.
3. Chaque propriété est publiée à trois mailles, avec sa limite et son attributaire.
4. Aucun seuil n'est posé qui n'ait été écrit avant ; ADR-183 §3 laisse le seuil de phase ouvert.
5. Scènes et réceptions antérieures **au bit** ; le banc ajoute, il ne modifie pas.

**Ce que je ne fais pas** : l'ordre D (le volume net reste **intégralement** dans `pending`) ; A305
(bande puis éponge) ; la profondeur finie ; le rendu ; toute correction de phase par amplitude ou
décalage.

### Plan

- [x] **P1** — amorce, jeton, plan seul.
- [x] **P2** — la règle d'attribution et les **prédictions chiffrées**, écrites avant toute mesure
  (notes de reprise).
- [x] **P3** — l'instrument de phase déroulée, et son **essai sur un défaut connu** : deux trains
  de nombres d'onde différents, écart attendu > ½ tour à 10 λ.
- [x] **P4a** — *découpage déclaré en cours de route* : le banc de l'ordre C écrit et **lancé**
  aux trois mailles, plus 6,25 cm à 5 ms (le discriminant du pas de temps, P2). Quatre calculs
  parallèles, de 3 min à ~1 h 20 ; **P6 et P8 s'exécutent pendant ce temps**, puisqu'ils n'en
  dépendent pas.
- [x] **P4a′** — *second découpage* : le premier lancement a échoué à 25 cm (six stations au relevé
  complet) et révélé une fenêtre de retour calée sur la vitesse posée. Banc prolongé de 10 m, durée
  décidée sur la vitesse **mesurée** de δ, amplitude en paramètre ; **l'hypothèse de Stokes écrite
  avant les résultats à 12,5 et 6,25 cm** (notes).
- [ ] **P4b** — la phase à 10 λ **déroulée**, à trois mailles, contre les prédictions de S315.
- [ ] **P5** — les six propriétés **sur le même transfert**, à trois mailles : amplitude, spectre,
  phase, vitesse de groupe, réflexion, volume.
- [x] **P6** — la part de la **primitive seule** : le train à sa naissance contre la forme demandée.
- [x] **P7a** — *découpage déclaré* : prédiction écrite (notes), oblique à 12,5 cm **lancé** à 20° et
  40° le 2026-09-21 à 08:01.
- [ ] **P7b** — l'oblique à **maille plus fine** : l'écart d'angle se partage-t-il entre δ et
  l'instrument comme S315 l'a prévu ? *(si le coût mesuré le permet ; sinon, le coût est publié et
  l'essai daté dans la file)*.
- [x] **P8** — A306 : les **deux emplois** de l'estimateur par passages par zéro dans le harnais,
  et les réceptions antérieures qui en dépendent (ADR-183 D8).
- [ ] **P9** — preuve publiée : le tableau d'attribution de l'ordre C.
- [ ] **P10** — rituel REPRISE §6.

### Notes de reprise

**P2 — les prédictions, écrites avant la première mesure de S316.**

*Phase à 10 λ*, écart `D(x₂) − D(x₁)` à la pulsation d'émission, reprises **telles qu'écrites** en
S315 §2 : **−0,853 ; −0,165 ; −0,0072 tour** à 25 ; 12,5 ; 6,25 cm. Elles reposent sur deux faits :
le raccord conserve `ω` — la grandeur invariante à une frontière immobile, comme `k_y` l'est à une
frontière plane —, et dans un milieu homogène δ garde son `k` initial. `D = 20·ε` tours, où `ε` est
l'écart relatif de `ω` lu en S314 (−4,36 % ; −0,83 % ; −0,036 %).

*Le point à 6,25 cm n'est pas sur la loi des deux autres.* Les rapports d'écart sont 5,3 puis **23** :
une loi en puissance passant par les deux premiers (p ≈ 2,4) donnerait −0,16 % à 6,25 cm, et on lit
−0,036 %. Un second terme, de signe opposé et d'ordre +0,1 %, entre en jeu. Suspect nommé : le
**pas de temps**, fixé à 10 ms aux trois mailles (`ω·dt` = 0,056), donc une part de l'erreur de δ
**ne converge pas avec la maille**. Discriminant : 6,25 cm à 5 ms, si le coût le permet.

*Vitesse de groupe à 10 λ, 12,5 cm* : **deux prédictions concurrentes**, et l'instrument tranche.
S315 a lu un retard d'arrivée de **1,66 s** sur les enveloppes. Une erreur de dispersion en
`(k·dx)²` donnerait `δc_g/c_g = 5ε` = 4,1 %, soit **0,93 s**. L'instrument lit ce retard comme la
pente `∂D/∂ω`, sans jamais ajuster d'enveloppe.

*Part du raccord, extrapolée des trois mailles de S314* (Richardson, `e(h) = e∞ + C·h^p`) : erreur de
forme 13,3 → 7,5 → 5,7 %, p ≈ 1,7, **limite ≈ 4,9 %** ; écart de bande 19,4 → 7,6 → 3,5 %, **limite
≈ 1,3 %** ; amplitude 3,2 → 1,7 → 1,1 %, **limite ≈ 0,7 %**. Une limite non nulle est ce que le
raccord ne sait pas représenter — une porteuse sous enveloppe gaussienne ne décrit pas un paquet
que la dispersion a déjà déformé. Trois points par limite : la prédiction est fragile, et elle est
écrite pour être contredite.

*Part de la primitive* : bande tronquée à ±4 écarts-types, 128 modes, réplique hors du disque —
**erreur de forme à la naissance < 10⁻³**. Si P6 lit davantage, la primitive porte une part.

*Dissipation de δ* : 6,3 % d'amplitude perdue sur 10 λ à 12,5 cm (S315). Prédit : elle **converge**.
Aucune valeur écrite pour les deux autres mailles — pas de loi disponible.

**P3 — l'instrument, sur un défaut connu.** Deux trains exacts, gravités `g` et `g(1+ε)²`,
ε = −4,36 % (l'écart de 25 cm) : **−0,85299 tour lu pour −0,85299 attendu**, saut maximal entre
stations 0,0067 tour ; l'extrémité seule lit **+0,14701** — l'ambiguïté d'A307, reproduite et levée.
Retard de groupe **−2,0189 s pour −2,0189 s**, décalage d'émission 0,0000 s.

*L'impasse du premier passage, qui vaut pour la suite* : `∂D/∂ω` pris **entre les deux lignes**
rendait −1,797 s (11 % d'erreur). Les relevés commencent à la naissance, donc coupés au milieu de
l'enveloppe à la première ligne, et une gaussienne coupée a son propre retard `σ_t·√(2/π)` —
différent pour deux trains de `c_g` différents. Remède : le retard se lit comme une **pente en
`x`** sur les stations au relevé complet (au-delà de 4,5 σ), l'ordonnée ramenée à la ligne donnant
le décalage d'émission. La **phase** à `ω₀`, elle, n'est pas biaisée par la coupure (désaccord nul).

*Et la même cause a failli bloquer P3 autrement* : mon banc de référence S315, lancé pour chiffrer
le coût, **verrouillait l'exécutable** — `link.exe` 1104, exactement L362. Arrêté (son premier
point reproduisait S315 au chiffre près : −0,0581), et **chaque mesure part désormais d'une copie
de l'exécutable** dans le répertoire de travail temporaire. Coût mesuré : 1 λ à 12,5 cm ≈ 4 min.

*Deux défauts du raccord de S314/S315, lus dans le code avant de mesurer* : (1) la largeur
temporelle devient spatiale par la vitesse de groupe de l'onde **posée** — une information qu'une
scène réelle ne donne pas ; (2) le train naît sur la **face** `x_ligne` alors que la jauge lit le
**centre** de la colonne, une demi-maille plus loin : `k·dx/2` = 0,031 tour à 12,5 cm, 0,063 à 25 cm.
L'ordre C mesure le raccord tel qu'il était **et** corrigé, sur le même relevé de δ.

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

**P4 — le registre se défend par sa forme, pas par sa documentation.**

`delta3d_transfer.rs`, `Ledger3`. Le danger n'était pas de mal calculer mais d'écrire un compteur
qui se laisse satisfaire. Trois traits, et ce sont eux la décision :

- **`pending` n'est pas un champ, c'est une différence** — rien ne peut l'écrire, donc rien ne
  peut l'effacer ; la seule façon de le faire baisser est de transférer pour de bon ;
- **il est signé**, et négatif il dit qu'on a **créé de l'eau** (`created()`), pas que le compte
  est bon ;
- **aucune méthode ne s'appelle « restituer »**, et `global_conservation_claimable()` *calcule*
  ce qu'ADR-180 D1 interdit d'affirmer en prose.

Le registre ne sait pas que W porte zéro volume, et c'est voulu : écrire ce zéro en dur le rendrait
faux le jour où un receveur existe. L'unité n'est pas dans le type — un registre par grandeur,
volume en m³ et grandeur propagative en joules de jauge. Six essais.

**P5 — la réponse est non, et elle disqualifie le cas de S311 pour le transfert.**

Les deux champs d'impact **refusent** l'onde du canal — `Regime` et `Medium`. Seuils balayés au
centimètre : `RadialImpact` exige `h ≥ λ`, `ImpactField` `h ≥ 2λ`, aux trois longueurs d'onde
essayées. Et `ModalPressure` n'a **pas de paramètre de profondeur** : `ω = √(g|k|)` est écrit dans
le constructeur. **Toute la couche W est en eau profonde.**

Ce que coûterait de passer outre, chiffré : W donnerait à l'onde de S311 **44,3 % de célérité en
trop**. Deux repères qui remettent la chose à sa place : la scène δ 3D **réelle** de S302
(λ = 8 m, h = 3,5 m) n'est qu'à **0,41 %** — l'eau y est déjà profonde ; c'est le **canal** de
S311 qui était peu profond. Et le candidat d'eau profonde est exact à 10⁻⁴ près.

**Conséquence pour le plan** : le transfert demande un **second cas contrôlé**, en eau profonde.
Ce n'est pas un contournement, c'est ce que la mesure impose. Le premier cas garde sa valeur : il
reste celui du **volume net**, que le registre porte.

*Tenue du plan* : battement du commit P4 écrit à 16:55 alors que l'horloge disait 16:48 (L237) ;
corrigé au commit suivant, et dit ici plutôt qu'effacé.

**P6+P7+P8+P9 — fusion déclarée.** Le second cas contrôlé, le transfert et sa vérification vivent
dans **un seul exemple** (`transfert_paquet.rs`) : le transfert consomme ce que le cas mesure, et
la vérification consomme le transfert. Les séparer aurait donné trois commits dont deux ne
compilent pas utilement. La preuve est écrite en même temps parce que trois chiffres sur quatre ont
changé en cours de route.

**Le paquet, et pourquoi il répond au point 2 mieux qu'un découpage.** `∫η dx = a·σ√(2π)·exp(−k²σ²/2)` :
le volume net d'un paquet vaut `exp(−k²σ²/2)` fois l'échelle de son volume absolu. À `kσ` = 0 c'est
la bosse de S311 ; à **9,42** c'est ce paquet, net/absolu **2,47·10⁻⁶** à l'état initial et
**9,49·10⁻⁴** au passage de la ligne. Les deux transportent autant d'eau d'avant en arrière. **Le
flux sortant ne se découpe pas** en « part nette » et « reste » : son intégrale et sa forme sont
deux fonctionnelles, et l'utilisateur avait raison de l'interdire.

**Le transfert passe T3, et les trois pertes sont chiffrées.**

| | mesure | seuil |
|---|---:|---|
| **amplitude** — énergie que le champ porte | **1,24·10⁻⁵** | 5 % ✓ |
| **réflexion artificielle, en 3D** | **2,84·10⁻⁷** | 1 % ✓ |
| direction — part avant | **0,5000** | *la moitié repart à contresens* |
| spectre — `λ` à 10 s contre demandée | **27,3 %** | *bande de deux octaves contre 11 %* |
| propagation — crête contre `cg` | **12,4 %** | *conséquence du spectre* |
| phase | `∂η/∂t` = **0** à la naissance | *impossible par construction* |

**Double comptage** : `band_in` et `perturbation_in` valent **0 exactement** sur 5 432 pas.

**Trois biais d'instrument, trouvés par la mesure et non par relecture.** (1) `sample` attend des
coordonnées **absolues** : tout le disque était refusé, `Error::Domain`. (2) La longueur d'onde était
mesurée sur tout le disque, queue de bruit `f32` comprise — 1,04 m qui ne mesurait rien ; fenêtre
ramenée à `|η| ≥ 20 %` du pic. (3) La grille du partage avant/arrière avait un **compte impair** :
la colonne `x = 0`, la plus haute, tombait entière du côté avant et donnait 0,579 au lieu de 0,5.

**Et une trouvaille qui n'était pas cherchée : T1 n'est pas mesurable sur un cas de moyenne nulle.**
Le rapport dépend entièrement du plancher d'activité — 1,95 sans plancher, **1,79·10⁻⁴ sur les 318
pas les plus actifs**, toujours 180 fois au-dessus du seuil. Ce n'est pas une division par un pas
mort. La cause est la **normalisation** : l'échelle du pas rétrécit avec `dt` (1,15·10⁻⁷ m³ au
maximum du banc) quand le résidu reste au plancher `f32` (2,0·10⁻¹¹ m³) ; et normaliser par
`Balance3::volume` ne sauve rien, c'est une somme **signée**, nulle par construction pour un paquet
(2,78·10⁻⁴). Rapporté à une grandeur **absolue** : 8,3·10⁻¹⁰. **À proposer à l'utilisateur** comme
précision à ADR-179 D1, dont le statut provisoire le prévoit. Aucun banc de la session ne
revendique T1.

**Suite de tests** : 455 essais, 0 échec.

**P4 — le module publie, il ne décide pas.**

`delta3d_closure.rs`, `Closure3`. Les quatre grandeurs de l'utilisateur, chacune avec son unité,
et **aucun seuil** : ADR-181 D5 interdit un chiffre avant la démonstration complète, qui comprend
la sensibilité de P7. La grandeur à lire est `over_floor` — le rapport du résidu au plancher
**dérivé**, pas à une échelle inventée.

Deux choix que P3 a rendus obligatoires :

- **l'échelle pertinente est le volume absolu de perturbation**, parce que c'est le seul
  dénominateur dont le rapport reste constant sur quatre décades ;
- **un pas sans activité ne fabrique pas de rapport** : son plancher vaut zéro, et `résidu / 0`
  ne mesurerait que la division — c'est exactement la faute d'A304, et elle ne se répète pas.

`random_walk_ratio` est le seul des quatre qui sépare un **bruit** d'une **fuite lente** : ≈ 1
pour des signes alternés, `√pas` pour un seul signe. Six essais, dont les deux qui l'encadrent.

**P7 — l'erreur volontaire, et elle apprend deux choses dont une n'était pas cherchée.**

Deux niveaux d'injection, parce qu'ils ne posent pas la même question.

**Fuite de bilan** — un écart d'un seul signe ajouté au résidu, sans toucher au champ : c'est ce
qu'un défaut de solveur ferait s'il retirait de l'eau sans la déclarer.

| fuite / pas | rapport au plancher | forme du cumulé |
|---:|---:|---:|
| 0 *(témoin)* | 3,00 | 0,741 |
| 10⁻¹⁵ | 3,00 | 0,775 |
| 10⁻¹⁴ | 3,01 | **1,08** |
| **10⁻¹³** | **20,6** | **4,03** |
| 10⁻¹² | 202 | 13,7 |
| 10⁻¹¹ | 2 021 | **14,14 = √200** |

**Sensibilité : 10⁻¹³ m³ par pas**, sans ambiguïté sur les deux indicateurs — soit **0,11 fois le
plancher** et **3·10⁻¹²** du volume absolu. Le cumulé bouge dès 10⁻¹⁴ mais une seule réalisation ne
suffit pas à l'affirmer. Et la forme du cumulé **sature à `√pas`** quand la fuite domine, comme la
dérivation le prédisait.

*Le rapport au plancher est plus sensible que le plancher du pire pas ne le laisse croire* : il
prend le pire **rapport**, donc il attrape une fuite dès qu'elle dépasse le plancher du pas le
plus **calme**, pas du plus actif.

**Fuite d'état** — du volume réellement retiré au champ, et **c'est un résultat négatif** :

| fuite / pas | résidu | rapport au plancher | **dérive / volume absolu** |
|---:|---:|---:|---:|
| 0 | 1,82·10⁻¹² | 3,67 | 1,86·10⁻⁵ |
| 10⁻⁹ à 10⁻⁷ | **identique au bit** | 3,67 | 1,86·10⁻⁵ |
| 10⁻⁶ | 2,73·10⁻¹² | 3,78 | **4,6·10⁻³** |
| 10⁻⁴ | 2,44·10⁻¹² | 2,47 | **0,518** |

**T1 est aveugle à cette fuite à toutes les tailles** — le résidu ne quitte jamais son plancher
alors que le domaine perd **52 %** de son volume de perturbation. T2, la dérive, la voit
immédiatement et proportionnellement.

Ce n'est pas un défaut de l'instrument, c'est **sa portée** : le résidu ferme **un pas**, et une
fuite qui a lieu **entre** deux pas est hors de son champ. **T1 et T2 ne sont donc pas
redondantes** — chacune attrape exactement ce que l'autre laisse passer, et aucune seule ne suffit
à parler de conservation.

*Second fait, mesuré en passant* : de 10⁻⁹ à 10⁻⁷, les sorties sont **identiques au bit**. L'offset
demandé (fuite/3 m) est sous `ulp(2,0)` = 2,4·10⁻⁷ m : **la fuite n'a pas lieu**, elle n'est pas
représentable. C'est l'autre face de la somme compensée — elle sauve les incréments du pas, elle ne
peut rien pour une écriture extérieure.

**P8 — T2 est tenue sur les 10 s demandées, et sa croissance est une marche aléatoire.**

| durée | dérive / amplitude, `dx` = 0,25 | `dx` = 0,125 |
|---:|---:|---:|
| 1 s | 6,91·10⁻¹⁰ | 3,38·10⁻¹⁰ |
| 5 s | 1,54·10⁻⁹ | 5,77·10⁻¹⁰ |
| **10 s** | **2,99·10⁻⁹** | **5,77·10⁻¹⁰** |
| 20 s | 3,52·10⁻⁹ | 7,92·10⁻¹⁰ |

**Seuil 10⁻⁶ : tenu avec 340 fois de marge** à la maille grossière, 1 700 à la fine. De 1 s à
20 s la dérive croît de **5,1 fois** pour `√20` = 4,5 attendus : c'est une **marche aléatoire**,
pas une fuite. Et elle **diminue quand la maille se raffine** — conforme à la loi en `A/√N` de P3,
ce qui est un troisième contrôle de la borne sur un banc qui ne l'avait pas servie à l'écrire.

ADR-179 D2 est donc rendue, et la note d'honnêteté de S311 (« T2 n'est pas tenue, 5 s pour 10 »)
est levée **par la mesure**, pas par un changement de seuil.

**P9 — trois seuils proposés, et le premier service du critère est de déclarer quelque chose
non conforme.**

| | proposition | ce qui la justifie |
|---|---|---|
| **C1** | rapport au plancher ≤ **10** | témoin 3,0 à 5,7 ; plus petite fuite détectée 20,6 |
| **C2** | forme du cumulé ≤ **5**, sur ≥ 200 pas | témoin 0,06 à 2,93 sur 22 passages ; fuite à 10⁻¹² → 13,7 |
| **C3** | T2 **inchangée** | mesurée 2,99·10⁻⁹ et 5,77·10⁻¹⁰ pour 10⁻⁶ |

Sensibilité combinée **10⁻¹³ m³ par pas**, portée par C1 — rien de plus fin n'aurait de sens.
Et une exigence qui n'est pas un chiffre : **les trois sont requises ensemble**, parce que §4.2
montre qu'une fuite passe C1 et échoue C3.

**Le cas ouvert échoue C2** (13,67 pour √200 = 14,14) : son résidu est d'un seul signe. Minuscule
— 5·10⁻⁸ de la dérive physique — mais **systématique**. Suspect nommé, non démontré : la bande ou
l'éponge, qui n'existent que là. Enregistré en A305.

**P4 — `wave_train.rs`, et ce sont les refus qui portent la conception.**

`WaveTrain<N>` : une somme d'ondes planes bornée en **bande** (`Δk = 1/σ`) et en **secteur**.
Chaque mode est une solution exacte de la houle linéaire en eau profonde, donc **direction,
spectre et phase ne sont pas approchés, ils sont portés** ; ce qui est approché est
l'échantillonnage, et il a son refus.

Onze essais, un par propriété, aucune déduite d'une autre :

- l'amplitude demandée **est** la crête à la naissance (10⁻⁷) ;
- le **volume net est nul** à 10⁻⁵ de l'absolu — W ne porte pas de moyenne (ADR-182 D9) ;
- le champ est invariant **au bit** en travers quand le secteur est nul, et son énergie part
  **dix fois** plus vers l'avant que vers l'arrière ;
- un quart de tour de phase échange **exactement** crête et zéro — la propriété qu'aucun champ
  d'impact ne peut offrir ;
- la vitesse de groupe mesurée sur le centre d'énergie tombe à **1 %** de `½√(g/k)` ;
- la **borne de pente est atteinte** (rapport 0,9 à 1,0), contrairement aux champs d'impact qui
  majorent d'un facteur constant : pas de constante de conversion à calibrer.

**Trois refus que la dispersion impose**, et qui n'existent dans aucune autre production de W :
`Horizon` (au-delà de l'élargissement admis), `Radius` (le disque ne contient plus le paquet),
`Resolution` (la réplique du spectre discret entre dans le disque). Le dernier a rendu un service
immédiat : **ouvrir le secteur coûte des modes de bande**, et à `N` = 64 un secteur de 5 directions
est refusé — il faut `N` = 256. Le compromis n'est pas commenté, il est **appliqué**.

Deux erreurs de ma part, corrigées : une condition de réplique posée comme un facteur arbitraire
(4 × rayon) au lieu de la vraie — « la réplique reste hors du disque à l'âge où elle s'en approche
le plus » ; et un essai de direction qui comparait `η` en deux points, donc mesurait la **phase de
la porteuse** et non la position du paquet. Corrigé en comparant l'**énergie** de deux fenêtres.

**P5+P6+P7 — fusion déclarée.** L'extracteur, l'essai 1 et l'essai 2 sont un seul banc :
l'identification n'existe que pour alimenter le train, et les deux essais ne diffèrent que par la
largeur de l'enveloppe.

**Le transfert, en trois gestes** : lire `η(t)` sur la ligne ; l'identifier par **cinq nombres**
— arrivée, largeur, amplitude, pulsation, **phase** — tous issus du même signal ; émettre un
`WaveTrain` qui les porte. Rien n'est ajusté après coup, et surtout pas l'amplitude (ADR-182 D8).

**Essai 2, le paquet — les trois limitations de S312 sont corrigées.**

| propriété | S312, impact | **S314, train** |
|---|---:|---:|
| **direction** — part vers l'avant | 0,500 | **1,00000** *(recul 3,96 σ)* |
| **spectre** — bande relative | deux octaves pour 11 % demandés | **0,0973 contre 0,1061**, écart **8,3 %** |
| **phase** — erreur de forme RMS | impossible, le champ naît au repos | **9,4 %**, corrélation **0,9907** |
| amplitude | calibrée en énergie | écart **2,1 %** |
| vitesse de groupe | 12,4 % | **0,46 %** |
| réflexion *(mesurée à part)* | 2,84·10⁻⁷ | 2,84·10⁻⁷ |
| volume net | 100 % en attente | 100 % en attente *(ADR-182 D9)* |

**Deux biais d'instrument, encore, et encore trouvés par une réponse connue d'avance** (L357).
(1) L'amplitude lue valait **0,7071** fois l'amplitude émise — exactement `1/√2`, la signature
d'une projection normalisée par `∫w dt` au lieu de `∫env·w dt`. Le facteur ne se règle pas, il se
**calcule**. (2) La direction était mesurée sur deux fenêtres de deux longueurs d'onde qui
**recouvraient le paquet** : 0,898 au lieu de 1,000. Remplacée par un partage en **demi-plans** à
un âge où le paquet a reculé de plus de trois écarts-types — condition publiée, pas supposée.

**P8 — l'oblique, la moitié que la primitive peut porter seule.**

Un train à 30° : le centre d'énergie se déplace de `cg·Δt` **en norme** (2 %) et dans la bonne
direction **composante par composante** (2 %) — un axe inversé ou permuté se verrait sur la
seconde et pas sur la première, et c'est pour cela que les deux sont vérifiées.

**Ce que cela ne fait pas** : l'essai 3 demande une propagation oblique **à la frontière**, donc un
front qui sort d'un domaine δ large en `y` et se lit sur sa ligne de contrôle. La primitive le
**supporte** ; supporter n'est pas éprouver, et l'essai reste dû.

**P9 — essai 5, et il a fallu deux passages : le premier accusait le mauvais coupable.**

Au premier passage, **le maillage le plus fin était le pire** : `ω` lue passait de −4,1 % à
−0,45 % puis **+4,7 %**, changement de signe, erreur de forme à 60 %, régression à 0,66. Deux
points suggéraient une convergence nette ; le troisième la détruisait.

La cause n'était pas le schéma mais l'**estimateur de fréquence**. Les passages par zéro comptent
*toutes* les traversées — traîne courte, ride résiduelle — et rendent une période trop brève. Or
**une maille fine amortit moins les courtes** : l'estimateur se dégrade *exactement quand le
domaine s'améliore*. Les deux estimateurs, mesurés côte à côte sur le même signal :

| maille | passages par zéro | **périodogramme** | écart |
|---:|---:|---:|---:|
| 25 cm | 5,3227 | 5,3094 | 0,25 % |
| 12,5 cm | 5,5263 | 5,5056 | 0,38 % |
| **6,25 cm** | **5,8110** *(+4,7 %)* | **5,5495** *(−0,036 %)* | **4,7 %** |

Avec le périodogramme, **les six grandeurs convergent de façon monotone** : écart de célérité
4,56 % → 0,83 % → **0,035 %** ; bande 19,4 % → 7,6 % → **3,5 %** ; amplitude 3,2 % → 1,7 % →
**1,1 %** ; forme 13,3 % → 7,5 % → **5,7 %**. Ce que le transfert reproduit est **ce que le domaine
a réellement produit**, et l'écart à l'onde *posée* est la dispersion numérique de δ.

**Portée** : `transfert_paquet.rs` (S312) emploie l'estimateur par zéros. À sa maille l'écart vaut
0,38 %, donc ses conclusions tiennent — mais son `λ_mesure` se lit 2,034 m au périodogramme.

**Essai 4** — la cohérence de phase est mesurée **sur la ligne d'émission**, régression 0,9938
(paquet) et 0,9734 (onde) : le train **part** avec la phase du signal sortant. Qu'il la conserve à
dix longueurs d'onde demande un oracle que cette session n'a pas construit.

**P11 — et le garde-fou du dépôt a attrapé ma propre construction.**

`contrat_pente::no_undeclared_comparison_to_max_slope_s143` a **échoué** au premier passage de la
suite : `wave_train.rs` comparait quelque chose à `max_slope` sans l'avoir déclaré. C'est
exactement ce que cette garde existe pour faire — « une implémentation de plus qui ignore le
contrat » —, et elle l'a fait sur la mienne. Le site est déclaré avec sa justification : le train
**ne convertit pas** sa borne L1, parce qu'à bande étroite elle **est** la pente réelle (0,9 à 1,0,
vérifié par `slope_bound_is_tight`). La déclaration et l'essai se tiennent l'un l'autre.

**449 essais, 0 échec.**

**P4 — le montage à deux lignes, et deux fautes de méthode avant le premier chiffre utile.**

**Le contrat de la primitive a refusé la distance demandée, et il avait raison.** Garder le train
jusqu'à la seconde ligne demande une cinquantaine de secondes, où l'enveloppe s'élargit de 11 % —
au-delà des 10 % par défaut. Deux issues : relever la constante, ou **déclarer** ce qu'on accepte.
La première l'aurait fait en silence **pour tous les appelants** ; la seconde le dit pour celui-ci.
`TrainSpec::spread_limit` est donc devenu un champ, `SPREAD_LIMIT` reste le défaut, et le banc
**publie l'élargissement obtenu** à côté de celui qu'il a déclaré. Le module l'avait prévu en
toutes lettres ; il a suffi de le rendre exécutable.

*Au passage, un essai nouveau* : un rayon **trop large** est un refus `Resolution`, pas une
précaution — la réplique du spectre discret entre dans le disque. C'est contre-intuitif et
maintenant vérifié.

**Faute 1 — comparer deux phases référencées à deux instants différents.** `identifie` rend une
phase relative à **l'arrivée de son propre signal**. Or la prédiction arrive **1,66 s avant** δ :
les deux origines sont distantes de 1,45 tour. La soustraction directe rendait 0,38 tour d'écart
« inexpliqué » qui ne mesurait que ce décalage d'origine. Corrigé : les deux phases se lisent à un
**instant absolu commun**, `ω·(t−t_c)/2π − φ`.

**Faute 2 — un binaire périmé qui rend des chiffres plausibles.** Deux exécutions précédentes
tournaient encore et **verrouillaient l'exécutable** ; `cargo build` échouait, son erreur était
avalée par le filtre de la ligne de commande, et l'ancien binaire s'exécutait. J'ai failli publier
une mesure de phase calculée par la version d'avant. **Un échec de compilation filtré est pire
qu'une erreur : il rend la sortie d'un autre programme.**

**P4 — la mesure, et un résidu que la prédiction n'explique pas.**

| | 25 cm | 12,5 cm |
|---|---:|---:|
| `k_δ` **mesuré** *(posé 3,1416)* | 3,1081 | **3,1395** |
| `k_train = ω²/g` | 2,8735 | 3,0875 |
| `ω` ligne 1 → ligne 2 | 5,3094 → 5,0232 | 5,5035 → **5,4985** |
| `Δφ` espace | −0,7466 | **−0,1655** |
| `Δφ` temps | +1,2601 | +0,0559 |
| **`Δφ` attendu** | −0,4864 | **−0,1096** |
| **`Δφ` mesuré** | −0,0609 | **+0,2463** |
| **résidu** | **0,4256** | **0,3559** |
| rapport d'amplitude | 1,4197 | 1,0629 |
| écart d'arrivée | −5,50 s | −1,66 s |

**Ce qui marche.** Le périodogramme spatial retrouve `k_δ` à **0,07 %** du posé à 12,5 cm, sans
rien supposer de la condition initiale — l'oracle sait mesurer ce qu'il doit mesurer. Et le terme
d'espace tombe **exactement** sur la prédiction de P3 : −0,1655 contre −0,165 annoncés.

**Ce qui ne marche pas.** Il reste **0,36 tour** de désaccord à 12,5 cm et 0,43 à 25 cm, que le
modèle à deux termes n'explique pas. Les deux valeurs sont **proches** alors que la physique
diffère d'un facteur 4 sur le terme d'espace : cela ressemble à un **décalage de comparaison**
plutôt qu'à un effet accumulé.

**Le discriminant est la distance elle-même**, et il est bon marché : un résidu indépendant de la
séparation est un décalage d'instrument ; un résidu qui croît avec elle est un écart de nombre
d'onde. Balayage 1 λ / 3 λ / 10 λ lancé, tout le reste fixé.

**P4 conclu — le balayage tranche, et la réponse est « indéterminé », pas « faux ».**

| séparation | `Δφ` espace | `Δφ` attendu | `Δφ` mesuré | résidu | rapport d'amplitude |
|---:|---:|---:|---:|---:|---:|
| 1 λ | +0,0729 | +0,0807 | +0,0227 | −0,058 | 0,985 |
| 3 λ | −0,0057 | +0,0109 | +0,0746 | +0,064 | 1,021 |
| 10 λ | −0,1655 | −0,1096 | +0,2463 | **+0,356** | 1,063 |

Le résidu **croît avec la distance**, donc ce n'est pas un décalage fixe — le balayage servait
exactement à ça. Mais le modèle à deux termes ne le prédit pas, et la raison est structurelle :
**une phase n'est connue que modulo un tour**. Le train et δ n'arrivent plus ensemble (1,66 s, soit
**1,46 tour** de porteuse), et comparer deux phases enroulées aux deux extrémités devient ambigu.

**La phase à dix longueurs d'onde est donc indéterminée par ce montage** — ni validée, ni
invalidée. Le remède est nommé : **dérouler la phase le long du trajet** au lieu de la comparer
aux extrémités. Ce que je n'ai **pas** fait, et qui aurait « marché » : décaler le train de 1,66 s
pour superposer les deux signaux. ADR-183 D3 l'interdit, et cela n'aurait mesuré rien.

**Ce que l'oracle établit quand même** : `k_δ` mesuré à **0,07 %** du posé, sans rien supposer de
la condition initiale ; le terme d'espace **exactement** sur la prédiction de P3 (−0,1655 contre
−0,165) ; et deux propriétés de δ que personne n'avait chiffrées sur ce trajet — **6,3 % de
dissipation** sur dix longueurs d'onde, et une vitesse de groupe **7,5 % sous** celle du train.

**P5+P6+P7+P8+P9 — fusion déclarée.** Le montage oblique, l'extraction de direction et les trois
mesures sont un seul banc ; les séparer aurait donné quatre commits dont trois sans résultat.

**L'essai 3 — et le résultat le plus net n'est pas celui que j'attendais.**

| `θ` posé | `θ` lu | écart | `k_y` lu / posé | **miroir transverse** |
|---:|---:|---:|---:|---:|
| 0° | **0,00°** | **0,000°** | — | *(sans objet)* |
| 20° | 22,57° | 2,57° | +5,3 % | **1,28·10⁻⁶** |
| 40° | 45,10° | 5,11° | +4,0 % | **2,09·10⁻⁷** |

**Le raccord ne crée aucune composante transverse artificielle** — le miroir `−k_y` vaut 10⁻⁶ à
10⁻⁷ de la composante utile. C'est le troisième point de la décision, et il est tenu net.

**La direction est lue, pas supposée** : à 0° la lecture rend exactement zéro. Et l'écart aux deux
autres angles **s'attribue** : `k` vient de `ω²/g` avec `ω` lue à −3,3 % (dispersion de δ à huit
mailles par longueur d'onde, la même qui converge en S314 §7), et `k_y` mesuré rend +4 à 5 % (la
largeur spectrale d'un paquet de deux longueurs d'onde). Les deux s'additionnent dans le même sens
puisque `θ = asin(k_y/k)`. **Prévu, pas mesuré** : que l'écart suive la même convergence — le cas
oblique à 12,5 cm coûte huit fois celui-ci.

**Le contrat a encore attrapé quelque chose.** Le banc n'a **pas pu construire** son train :
`Error::Envelope`, parce que son enveloppe valait une longueur d'onde **posée** (2,00 m) pour une
longueur d'onde **lue** de 2,14 m. La dispersion de δ allonge l'onde, et le paquet cesse d'être
plus large qu'une longueur d'onde. La chaîne est fermée par un essai unitaire bâti sur les
`(k_x, k_y)` **mesurés** : la direction portée est la direction lue à **0,05°** près aux trois
angles. Le banc est corrigé pour dimensionner son enveloppe sur la longueur d'onde **lue** ; la
correction n'a pas été rejouée sur δ.

**P6 — la primitive seule, contre l'évolution exacte de sa propre demande.** Quadrature continue
en `f64` sur ±8 écarts-types — une autre écriture, pas le train relu. Demande du raccord S316 à
12,5 cm (`k = ω₀²/g`, σ = 3 m, a = 20 mm). **Erreur de forme 1,02·10⁻⁴ à la naissance, 1,04·10⁻⁴
après 40 s** — dix longueurs d'onde de trajet ; écart maximal **1,8 µm**. C'est la troncature de
la bande à ±4 σ : `√erfc(4)` ≈ 1,2·10⁻⁴. Elle **ne croît pas** en se propageant. Prédiction de P2
(< 10⁻³) tenue. En usage : 1,8 µm contre 3 mm de tolérance d'image — la primitive ne porte **aucune**
part mesurable des écarts du transfert.

**P4a′ — le premier lancement, et ce qu'il a appris avant d'échouer.** À 25 cm (134 s de calcul) :
`c_g` de δ **0,688 m/s contre 0,922** pour l'onde émise, soit **−25,4 %** ; `k_δ(ω₀)` = 3,175 ;
saut maximal de la phase de δ entre colonnes **0,162 tour** (sous le demi-tour, déroulement sûr).
Deux défauts du banc, tous deux corrigés avant de relancer : le raccord S315 n'avait que six
stations au relevé complet (enveloppe 4,2 m) — d'où les 10 m de prolongement ; et la fenêtre de
retour, calée sur la vitesse **posée**, lisait la traîne du passage direct d'un δ 25 % plus lent —
**3,5·10⁻⁴** « réfléchis », contre 5,7·10⁻⁶ en S314. *Portée* : **la réflexion de S314 à 25 cm a été
mesurée avec la même fenêtre** calée sur la vitesse posée ; elle n'est pas sûre non plus.

**L'hypothèse de Stokes — écrite avant les résultats à 12,5 et 6,25 cm.** Le paquet a `a·k` =
0,063 : sa pulsation porte la correction non linéaire `(ak)²/2` = **+0,197 %**, que W — linéaire —
ne porte pas. Les `ω` de S314 **la contiennent**. Retirée, l'erreur numérique de δ vaut −4,56 ;
−1,03 ; −0,233 % aux trois mailles : **rapports 4,44 et 4,40, ordre 2,15 et 2,14** — une convergence
d'ordre deux nette, là où P2 lisait 5,3 puis 23. **Le « second terme » de P2 n'était pas le pas de
temps : c'était la non-linéarité.** Conséquences, chiffrées maintenant :

- à 10 λ, `D` se partage en **numérique** (−0,891 ; −0,204 ; **−0,047** tour) et **non linéaire**
  (**+0,040** tour, indépendant de la maille, en `a²`) ; à 6,25 cm les deux **se compensent
  presque** — la prédiction de S315 (−0,0072) est une différence de deux termes six fois plus grands ;
- **le témoin à demi-amplitude** (12,5 cm, `a` = 1 cm) doit déplacer `D` de **+0,030 tour** (les
  trois quarts de 0,040 retirés) — c'est le discriminant ;
- **le pas de temps moitié** (6,25 cm, 5 ms) doit laisser `D` presque inchangé : écart **< 0,005
  tour**, si le pas de temps n'est pas en cause ;
- et l'attribution change de nature : la part non linéaire ne revient **ni au raccord ni à δ** —
  δ a raison, W est linéaire. Elle revient au **modèle de la primitive**.

**P7 — la prédiction de l'oblique à 12,5 cm, écrite avant son lancement.** À 25 cm, S315 lisait
22,57° et 45,10° pour 20° et 40°, et attribuait l'écart à deux causes qui s'additionnent : `ω` lue à
−3,3 % (δ) et `k_y` à +4–5 %. La seconde n'est **pas** toute de δ : pour un paquet de largeur
angulaire `s = 1/(kσ)` = 0,159 rad, le maximum du périodogramme en `k_y` à `ω` fixée glisse vers les
grands angles de `s²·tan θ` — **+2,5 % sur `k_y` aux deux angles**, une propriété de l'**instrument**,
indépendante de la maille. À 12,5 cm, avec `ω` à −0,83 % : angle lu **20,9–21,3°** et **42,0–43,1°**
— l'écart tombe de moitié environ, **pas à zéro**. S'il tombait à zéro, l'instrument n'y serait pour
rien ; s'il ne bougeait pas, ce serait l'instrument seul. Coût : le pas de δ 3D sur CPU ne se
parallélise pas (réductions séquentielles), ≈ 1 h 30 par angle ; lancé en parallèle du reste.

**P8 — A306 : les emplois du harnais, relus par un second estimateur sur le même signal.** Critère
écrit avant : une réception a pu être affectée si l'écart entre estimateurs **dépasse sa marge**
(ce qui la sépare de sa tolérance). Trois essais permanents, `a306_*` :

| emploi | réception | écart entre estimateurs | marge |
|---|---|---:|---:|
| `physics_shallow.rs`, C03 sur `Shallow1D` | C03-T (mode), C03-T-mur | 0,038 %, 0,039 % | 1,00 % |
| `physics.rs`, `mesurer_seiche` sur `Delta1D` | C03, C03-mode | 0,035 %, 0,037 % | 1,00 % |
| `physics_dispersif.rs`, milieu vérifié | disp-λ32, λ16, λ8 | 0,232 %, 0,235 %, 0,236 % | 1,00 % |

**Aucune réception affectée.** Et le sens de l'écart est instructif : ici c'est le **périodogramme**
qui se trompe. Les passages par zéro tombent sur la référence à 5·10⁻⁶ près (milieu dispersif),
tandis que le périodogramme d'une sinusoïde **pure** de 8 périodes est biaisé de **−0,236 %** par la
fuite de sa fenêtre (−0,038 % à 20 périodes) — reproduit à part sur un signal synthétique, et selon
la phase. Le mécanisme d'A306 exige une composante **courte qui traverse zéro** sans porter
d'énergie : les signaux de **mode** du harnais n'en ont pas (au mur de C03, les harmoniques impaires
sont en phase et ne déplacent pas les zéros). Le cinquième emploi, C02 sur B, lit une composante
**unique** analytique : hors d'atteinte par construction. **A306 se clôt pour le harnais** ; le
remède de L360 reste vrai, avec son corollaire : **le second estimateur a son propre biais**.

