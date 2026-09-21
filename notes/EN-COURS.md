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

Session : S320 — **en cours**. Le **lot 5** sur le candidat retenu : **B10 sur APIC**
([ADR-186](../docs/adr/ADR-186-apic-seconde-representation.md) §3) — un objet **cinématique**
entre dans l'eau : couronne, cavité, pincement, jet.
Agent : Claude Opus 5, application desktop ; fichiers, git, cargo, carte réelle, accès web.
Entrée : *« Continue »*, après S319 ; suite déclarée : B10 sur APIC, puis l'essai sur A289.

**Ce que B10 doit rendre possible.** Le premier cas que la fonction hauteur **ne peut pas** porter —
une cavité d'air sous la surface, derrière un corps — porté par la seconde représentation, avec son
volume rendu. Consommateur : la porte D (un objet qui entre dans l'eau), C20, et le raccord futur
particules ↔ colonnes.

**Le banc.** Celui de S318, en deux dimensions : un **cylindre** de diamètre `D` descend à vitesse
**imposée** `U` (objet cinématique — aucun corps rigide n'existe, ADR-184 D3), depuis juste au-dessus
de la surface. Nombre de Froude d'entrée `Fr = U/√(g·D)`.

**Les références, et pourquoi pas une table.** Les travaux accessibles portent sur des **sphères** et
des **disques** (axisymétrie) ; ce banc est **plan**. Aucune valeur ne se transpose (I-14). Le cas se
juge donc sur trois épreuves **sans donnée extérieure** :

1. **la masse** — exacte par construction : le compteur doit la retrouver au bit ;
2. **la similitude de Froude** — sans viscosité ni tension de surface, deux entrées de même `Fr` et de
   même `D/dx`, à deux échelles, donnent les **mêmes** grandeurs sans dimension (temps en `√(D/g)`,
   longueurs en `D`) : tout écart est une faute du banc ;
3. **la convergence** — même `Fr`, `D/dx` doublé.

Grandeurs publiées : temps et profondeur de **pincement** (air enfermé sous la surface, détecté par
remplissage depuis l'atmosphère), hauteur de la **couronne** avant, du **jet** après, en `D` et en
`√(D/g)` ; masse ; coût.

Critères, écrits avant le code :
1. Le corps **au repos** dans l'eau ne crée pas d'écoulement ; **déplacé lentement**, il élève le niveau
   de son volume immergé exactement — deux essais du corps avant B10.
2. Similitude : écart sans dimension **< 5 %** sur le pincement, entre les deux échelles.
3. Trois `Fr` au moins ; le régime (pincement profond, ou pas de pincement) se dit, il ne se suppose pas.
4. Rien dans le cœur ; aucune réception antérieure touchée.

### Plan

- [x] **P1** — amorce, jeton, plan seul.
- [ ] **P2** — le corps cinématique dans la grille : cellules solides, faces à la vitesse du corps,
  particules repoussées.
- [ ] **P3** — le corps au repos, puis déplacé lentement : écoulement nul, niveau élevé de son volume.
- [ ] **P4** — la détection : air enfermé, pincement, couronne, jet.
- [ ] **P5** — B10 : trois `Fr`, similitude à deux échelles, convergence.
- [ ] **P6** — preuve publiée.
- [ ] **P7** — rituel REPRISE §6.

### Notes de reprise

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

**P4b — premier discriminant de Stokes : contredit à 25 cm.** Demi-amplitude (1 cm) contre 2 cm :
`D(10 λ)` passe de −0,9142 à −0,9160 tour, soit **−0,0018** pour **−0,030 prédit** ; `ω₀` **monte**
de 0,057 % (5,31913 → 5,32218) au lieu de descendre de 0,15 %. **À 25 cm, δ ne porte pas la
correction de Stokes.** Deux lectures, et 12,5 cm les sépare : δ est linéaire à ce degré ; ou sa
non-linéarité n'est pas résolue quand l'amplitude vaut 8 % d'une maille. Tout le reste est
indépendant de l'amplitude à mieux que 1 % (retard −7,37/−7,40 s, `c_g` −25,3/−25,4 %, réflexion
~1·10⁻⁷). Pas de remaniement de la prédiction : elle est écrite, elle est confrontée.

**P4b′ — 12,5 cm reçu, et la prédiction de 6,25 cm révisée avant son résultat (08:34).**

| | 25 cm, 2 cm | 25 cm, 1 cm | 12,5 cm, 2 cm | 12,5 cm, 1 cm |
|---|---:|---:|---:|---:|
| `D(10 λ)` déroulé, raccord S316 | **−0,9142** | −0,9160 | **−0,2372** | −0,2422 |
| prédiction S315 §2 | −0,853 | | −0,165 | |
| `k_W(ω₀) − k_δ(ω₀)`, rad/m | −0,2894 | −0,2907 | −0,0765 | −0,0781 |
| retard de groupe à 10 λ | −7,37 s | −7,40 s | **−1,75 s** | −1,77 s |
| `c_g` de δ contre l'onde émise | −25,3 % | −25,4 % | −7,2 % | −7,3 % |
| `|Z_δ(ω₀)|` : x₂ / x₁ | 1,0000 | 1,0000 | 1,0065 | 1,0017 |

1. **La non-linéarité de δ converge avec la maille**, elle n'en est pas indépendante : part non
   linéaire à 2 cm, extrapolée en `a²`, **+0,0024** tour à 25 cm, **+0,0067** à 12,5 cm, pour
   **+0,040** de Stokes plein. δ n'en porte que 6 %, puis 17 %. P4a′ avait tort sur ce point.
2. **Les prédictions de S315 manquent de 0,06 et 0,07 tour, et la cause est physique, pas
   numérique** : `k_δ(ω₀)` ≠ `k₀` (+1,0 % ; +0,64 %). Le raccord lit le maximum du spectre
   **temporel** ; la condition initiale pose celui du spectre **spatial**. Les deux diffèrent du
   jacobien `dk/dω = 1/c_g`, qui croît avec `k` : décalage `σ_k²/(2k)` = +0,56 % à σ = 3 m, contre
   +0,64 % mesuré. S315 supposait `k_δ(ω₀) = k₀`.
3. **Le retard de groupe tranche entre les deux prédictions de P2** : −1,75 s, contre 1,66 s lu par
   S315 sur les enveloppes et 0,93 s sous une loi en `(k·dx)²` — **la seconde est réfutée**.
4. **Les « 6,3 % de dissipation » de S315 n'en étaient pas** : à fréquence fixe, `|Z_δ|` ne perd
   rien entre les deux lignes (1,0000 à 25 cm) ; il **gagne** même 0,65 % à 12,5 cm sous 2 cm et
   0,17 % sous 1 cm — un transfert non linéaire vers la fréquence centrale, en `a²`. Le rapport
   d'amplitudes de S315 mesurait la **différence d'étalement** de deux paquets de dispersions
   différentes, pas une perte.

**Prédiction pour 6,25 cm, depuis 25 et 12,5 seuls.** Partie linéaire (extrapolée à `a → 0`) :
−0,9166 puis −0,2438, rapport 3,72 (ordre 1,90) ⇒ **−0,066 tour** à 6,25 cm. Partie non linéaire
sous 2 cm : entre **+0,010 et +0,034** (de la tendance à 85 % de Stokes, A289). D'où **D(10 λ) entre
−0,056 et −0,032 tour à 6,25 cm, 2 cm** — et donc **la prédiction de S315 (−0,0072) manquée**.
Sous 1 cm : entre −0,063 et −0,057. Et le pas moitié : écart < 0,005 tour.

**Échange avec l'utilisateur en cours de session (2026-09-21, ~08:50) — à porter au journal et à
la file.** L'utilisateur : *« dans la zone δ il s'agit comme une simulation de bille ou autres types
qui permettent de pouvoir plusieurs particules d'eau sur la même ordonnée »* — il craignait un
malentendu. Réponse : l'exigence est dans les sources (architecture §12, guide de topologie « un
heightfield ne peut pas avoir deux hauteurs pour une même paire (x,y) », zones ouvertes) et dans
ADR-001 ; **ADR-175 D5 (S294) a choisi une surface à une hauteur par colonne pour démarrer**, et
ADR-178 a placé la seconde représentation au **lot 5, après la v1**. Deux questions posées
(représentation, priorité) ; réponses : *« Est-ce que c'était indiqué dans le projet ? »* et
**« Attends »** sur la priorité. **Aucun ordre de lot n'est changé.** Le point reste ouvert, et il
est de l'utilisateur : représentation (particules sur grille, SPH, surface implicite) et moment.
**Suite (même matinée)** : *« je ne savais pas si cela était indiqué depuis le début, ne fait rien de
particulier, on laisse le plan initial en route »*. **Ordre d'ADR-178 confirmé par l'utilisateur** :
la seconde représentation reste au lot 5 ; son choix reste à lui proposer, chiffré, en son temps.
**Puis** : *« peut-être que le développement de ce système peut se faire en parallèle — réfléchis à
savoir si tu as besoin d'éléments qui ne peuvent pas être décidés maintenant »*. Réflexion rendue en
réponse (à reporter dans la ligne « Lot 5 » de la file au rituel) : parallélisable par **sessions
alternées** (jamais deux sessions simultanées — jeton, L137) ; ne dépend ni du lot 3 ni du lot 4 si
l'objet qui entre dans l'eau est **cinématique** ; réutilise grille MAC, multigrille et compteur du
lot 1 ; **le vrai chantier interne** est la condition de surface libre, aujourd'hui construite
colonne par colonne (`ghost_up3`, `ghost_side3` dans `delta3d_mobile.rs`) et à généraliser à une
frontière liquide/air dans toutes les directions ; références de B10 déjà écrites (couronne,
cavité, pincement, jet de Worthington). **Hors de portée d'une session** : le choix de
représentation (à proposer chiffré), les **faits de jeu** (quels impacts : tailles, vitesses), la
frontière entre eau physique et effet visuel (embruns), le critère de bascule (mesure, ADR-112), les
références visuelles (plus tard), et la décision d'ordre elle-même (note à ADR-178).
**Décision (08:53)** : *« je suis ta recommandation »* — **ADR-184** : lot 5 en parallèle du lot 2,
par sessions alternées ; il commence par la comparaison chiffrée (particules sur grille, SPH,
surface implicite) sur B10 et une rupture de barrage ; objet cinématique ; le choix reste à
l'utilisateur. À porter au rituel : bloc des décisions de la file, ligne « Lot 5 », feuille de
route §3 bis, index, mémoire.

**P4b — la phase à dix longueurs d'onde, déroulée, aux trois mailles (10:08).** Raccord S316.

| maille | `D(10 λ)`, 2 cm | `D(10 λ)`, 1 cm | part linéaire (`a → 0`) | part non linéaire, 2 cm | fraction de Stokes |
|---:|---:|---:|---:|---:|---:|
| 25 cm | −0,9142 | −0,9160 | −0,9166 | +0,0024 | 6 % |
| 12,5 cm | −0,2372 | −0,2422 | −0,2438 | +0,0067 | 17 % |
| 6,25 cm | **−0,0452** | −0,0524 | −0,0548 | +0,0096 | 24 % |

- **La part linéaire converge à l'ordre deux** — rapports 3,76 puis 4,45, ordres 1,91 et 2,15 :
  c'est l'erreur de dispersion de δ, et elle s'efface.
- **La part non linéaire croît avec la maille** : δ résout de mieux en mieux la correction de
  Stokes, que W ne porte pas. Fraction encore loin de 85 % (A289, 2D) : le paquet s'étale de 25 %
  sur le trajet et son amplitude effective baisse, donc « Stokes plein » (0,040) majore.
- **Confrontation des prédictions écrites avant** : S315 (−0,853 ; −0,165 ; −0,0072) **manquées**
  de 0,06, 0,07 et 0,04 tour — jacobien, voir P4b′ ; P4b′ pour 6,25 cm : −0,056 à −0,032 sous 2 cm,
  **tenue** (−0,0452) ; −0,063 à −0,057 sous 1 cm, **manquée** de 0,005 (la part linéaire a
  convergé plus vite que la loi tirée de deux points — L361).
- **Retard de groupe à 10 λ** : −7,37 ; −1,75 ; −0,38 s, rapports 4,2 et 4,6. `c_g` de δ : −25,3 ;
  −7,2 ; −1,65 %.
- **En usage** (METHODE, précision rapportée à l'usage) : à 6,25 cm, −0,045 tour sur 20 m, c'est
  **9 cm** de décalage de crête (0,45 %) et, pour une vague de 2 cm, **5,7 mm** d'écart de hauteur
  crête à crête au bout de dix longueurs d'onde — au-dessus des 3 mm d'image. Mais l'écart est
  **celui de δ contre la physique exacte**, et le raccord le **coupe** : au-delà de la ligne, W porte
  la dispersion exacte. Ce que la mesure dit au transfert : **le train émis a la bonne phase** ; c'est
  δ qui la perd sur sa propre longueur, au rythme de sa maille.
- **Volume net, par mètre de crête** (la largeur du domaine vaut `2·dx` et change avec la maille) :
  2,57 ; 1,96 ; 1,68·10⁻⁴ m³/m sous 2 cm, **×4,0 exactement** entre 1 et 2 cm aux trois mailles —
  une grandeur d'**ordre deux**, limite extrapolée ≈ 1,5·10⁻⁴ m³/m. W n'a pas de moyenne : son
  receveur est B ou V (ADR-181 D1), et c'est l'ordre D.

**P5 — les six propriétés du même transfert, attribuées (raccord S316, 2 cm).** Règle de P1 :
converge → δ ; présent dès l'émission et ne converge pas → raccord ; existe sans δ → primitive.

| propriété | 25 cm | 12,5 cm | 6,25 cm | lecture | **attributaire** |
|---|---:|---:|---:|---|---|
| amplitude à l'émission, `|Z_W|/|Z_δ|` à `ω₀` | 1,0276 | 1,0102 | 1,0057 | converge vers ≈ 1,003 | δ ; **raccord ≤ 0,3 %** |
| amplitude du train le long du trajet | 1,000004 | 1,000002 | 0,999985 | exacte | **primitive : rien** |
| `|Z_δ|` de x₁ à x₂ | 1,0000 | 1,0065 | 1,0109 | en `a²` ; part linéaire ≈ 0 | **physique que W n'a pas** |
| spectre, écart | 7,3 % | 4,0 % | 2,8 % | limite ≈ 2 % | δ ; **raccord ≈ 2 %** |
| spectre, centroïde | +0,29 % | +0,24 % | +0,19 % | lent | **raccord** |
| spectre, largeur | −7,5 % | −2,6 % | −1,5 % | limite ≈ −1,2 % | δ ; raccord |
| phase à l'émission | 0,0078 | 0,0032 | 0,0027 tour | limite ≈ 0,0025 tour (0,9°) | **raccord** |
| phase à 10 λ | −0,914 | −0,237 | −0,045 tour | linéaire d'ordre 2 ; non linéaire croissante | **δ** ; physique que W n'a pas |
| retard à l'émission | −0,062 | −0,034 | −0,031 s | limite ≈ −0,03 s | **raccord** |
| vitesse de groupe de δ | −25,3 % | −7,2 % | −1,65 % | ordre ≈ 2,1 | **δ** |
| réflexion, par sens | 1,1·10⁻⁷ | 8,2·10⁻⁷ | 1,5·10⁻⁶ | indépendante de `a`, ≪ 1 % | **éponge de δ** |
| volume net | 100 % attente | 100 % | 100 % | `a²`, ≈ 1,5·10⁻⁴ m³/m | **architecture — ordre D** |
| direction | *(P7b)* | | | | |

**Ce qui revient au raccord, et c'est peu** : 0,9° de phase, 0,03 s de retard, ≤ 0,3 % d'amplitude,
≈ 2 % d'écart de spectre avec un centroïde décalé de +0,2 %. Nature : une porteuse sous enveloppe
gaussienne, identifiée par les moments de `η²`, ne décrit ni le **chirp** qu'un paquet acquiert en
se dispersant avant la ligne, ni l'asymétrie du jacobien — c'est la « limite de modèle » que P2
prévoyait (4,9 % extrapolés sur la forme ; ≈ 2 % mesurés sur le spectre).

**Les deux fuites** : la demi-maille est un décalage pur (§ P3) ; la vitesse posée **aidait par
accident** la largeur de spectre aux mailles grossières (−3,5 % contre −7,5 % à 25 cm) en
compensant l'erreur de δ par une information qu'une scène n'a pas — et ne change plus rien à
6,25 cm (−1,47 contre −1,51 %), où les deux vitesses coïncident.

**Et une cinquième colonne que la règle ne prévoyait pas** : la non-linéarité. δ porte une part
croissante de Stokes (phase) et un transfert d'énergie vers la fréquence centrale (amplitude), les
deux en `a²` ; **W, linéaire par construction, ne les porte pas**. Ce n'est ni un défaut du raccord
ni une erreur de δ : c'est la frontière du modèle de W, et elle se voit dès dix longueurs d'onde.

**Suite complète** (`cargo test --workspace --release`, 10:13) : **571 réussis, 18 ignorés, 0 échec**
— cœur 450, harnais 98 dont les trois `a306_*`, le reste inchangé.

**P7b — 20° à 12,5 cm (10:38).** Angle lu **21,03°** (écart **1,03°**, contre 2,57° à 25 cm) —
**dans la fourchette prédite** (20,9–21,3°). `ω` lue 5,5584 (+0,12 % sur la posée), `k_y` **+5,2 %**
(+5,3 % à 25 cm), miroir transverse **1,2·10⁻⁷**. Lecture : la part de `ω` converge (δ) ; celle de
`k_y` **ne converge pas du tout** — elle revient à la **lecture de la direction** (maximum du
périodogramme en `k_y` pour un paquet court, σ = 1 λ), donc au raccord qui s'en sert. Ma
décomposition (+2,5 % d'instrument) en sous-estimait la moitié ; la limite est ≈ **+1,1° à 20°**.
S315 attribuait tout l'écart à la maille de δ : juste pour moitié.

**P7b — 40° à 12,5 cm (11:08), et le bilan de l'oblique.**

| `θ` posé | lu à 25 cm | lu à 12,5 cm | `k_y` à 25 / 12,5 cm | miroir à 25 / 12,5 cm |
|---:|---:|---:|---:|---:|
| 20° | 22,57° | **21,03°** | +5,3 % / **+5,2 %** | 1,3·10⁻⁶ / **1,2·10⁻⁷** |
| 40° | 45,10° | **41,85°** | +4,0 % / **+4,3 %** | 2,1·10⁻⁷ / **2,2·10⁻⁸** |

Prédiction : 20,9–21,3° **tenue** ; 42,0–43,1° **manquée de 0,15°** — j'avais pris l'erreur de `ω`
du cas 1D (−0,83 %), alors qu'avec ce paquet plus court (σ = 1 λ) `ω` lue passe **au-dessus** de la
posée (+0,12 % et +0,25 %) : le décalage du jacobien (P4b′) vaut ici `σ_k²/(2k)` ≈ 4 % sur `k`.
**Attribution** : l'écart d'angle converge pour sa part `ω` (δ, et le jacobien) ; sa part `k_y`,
+4 à +5 %, **ne bouge pas avec la maille** — c'est la **lecture de la direction** par le maximum du
périodogramme, donc le raccord qui la consomme. Le train est construit sur la direction lue (41,855°
porté) : **le raccord transmet fidèlement une direction qu'il lit biaisée d'environ +1 à +2°**. La
composante transverse parasite **décroît** avec la maille (×0,1) : aucune.

**P4c — le pas de temps moitié (11:21).** 6,25 cm, 2 cm, 5 ms contre 10 ms : `D(10 λ)` −0,04443
contre −0,04517 (**7·10⁻⁴**, prédit < 0,005) ; retard −0,388 contre −0,381 s ; `c_g` de δ −1,68 %
contre −1,65 % ; volume −1,4 % ; `ω₀` lue +0,11 % — le maximum du spectre bouge plus que la phase
à fréquence fixe, qui est la mesure robuste. **Le pas de temps ne contribue pas.** L'écart de
spectre du raccord varie de 2,8 à 4,4 % entre les deux pas : la fourchette « 2 à 4 % » est la
bonne lecture, pas un chiffre à la décimale.
**P3 — prédiction, écrite avant le banc (20:27).** ADR-179 D3 a fixé la grandeur : le flux sortant
à la surface de contrôle. Ce qui reste à montrer est que ce flux **ferme le bilan de l'intérieur** —
le volume de δ entre une ligne gauche, au bord de l'éponge, et la ligne droite de l'ordre C :
`ΔV_intérieur + Q_sortant(droite) + Q_sortant(gauche) = résidu`, au **plancher d'arrondi** puisque
le transport télescope exactement (S310, S313 : 10⁻¹³ m³ par pas). Prédit : résidu cumulé ≤ 10⁻¹⁰
m³ sur tout le trajet, pour un sortant net d'ordre 10⁻⁵ m³ (4,9·10⁻⁵ à 12,5 cm sous 2 cm, S316).
Et le volume que les éponges effacent ensuite, cumulé, **rejoint** ce sortant une fois le paquet
entré dans l'éponge — la même eau comptée plus tard, que la région ne doit pas recevoir deux fois.

**P4 + P5 — le receveur (20:35).** `regional_level.rs` : une région adossée à un segment de ligne
de contrôle, du côté sortant, profondeur déclarée ; niveau = volume reçu / aire ; `boundary_out`
= 0, choix déclaré (ADR-185 D8) ; refus `Line`, `Outward`, `Depth`, `Domain`, `NonFinite` — une
région qui déborderait de sa cellule est refusée, l'océan ne s'obtient pas en élargissant.
`Receipt` : ne se construit que par `receive`, ne se copie pas, se consomme au registre.
`Ledger3` : `restituted` ne croît que par `account_restitution(reçu)` ; `pending` en retranche ;
**`global_conservation_claimable` devient faux dès qu'une région locale a reçu** — la
représentation se ferme (`representation_closed`), pas le monde (ADR-185 §3). Sept essais neufs,
**verts au premier passage** ; les huit du registre inchangés.

**P3 — le flux à la ligne ferme l'intérieur (20:40).** Volume de contrôle entre la première face
hors de l'éponge gauche et la ligne de l'ordre C ; aucune éponge n'y agit.

| | 25 cm | 12,5 cm |
|---|---:|---:|
| résidu final `ΔV + ΣQ_sortant` | −1,6·10⁻¹⁰ m³ | −2,5·10⁻¹⁰ m³ |
| résidu maximal sur le trajet | 5,4·10⁻¹⁰ | 3,0·10⁻¹⁰ |
| flux absolu cumulé | 0,130 m³ | 0,053 m³ |
| sortant à droite | +1,17·10⁻⁴ | +5,29·10⁻⁵ |
| sortant à gauche | **−1,37·10⁻⁴** | **−5,37·10⁻⁵** |
| variation de l'intérieur | +2,0·10⁻⁵ | +8,2·10⁻⁷ |
| éponges : volume retiré | **−5,9·10⁻⁵** | **−8,6·10⁻⁶** |

Prédiction : résidu ≤ 10⁻¹⁰ m³ — **manquée d'un facteur 3 à 5**, au plancher d'arrondi pourtant
(2 à 6·10⁻⁹ du flux cumulé) ; ma borne était trop serrée, pas le bilan faux. Trois lectures :

1. **Le flux à la ligne ferme l'intérieur** : c'est bien la grandeur à restituer (ADR-179 D3).
2. **L'eau entre aussi par l'arrière.** Ce qui sort devant, une onde longue du second ordre le
   ramène par derrière : à 12,5 cm, sortant et entrant s'équilibrent à 2 % près. Le paquet
   **déplace** de l'eau de l'arrière vers l'avant. Un receveur n'a de sens qu'**à chaque ligne**.
3. **Les éponges ajoutent de l'eau** (volume retiré négatif) : elles ramènent au repos un niveau
   abaissé. Compter leur action en plus des lignes compterait deux fois — et à contresens.

**P6 — la restitution sur le banc (20:46).** Une région par ligne, adossée du côté sortant, jusqu'au
bord du domaine ; flux signé de la ligne → registre → région → reçu → registre, à chaque pas.

| | 25 cm | 12,5 cm |
|---|---:|---:|
| reçu à gauche (aire, profondeur) | −1,37·10⁻⁴ m³ (9 m², 18 m) | −5,37·10⁻⁵ m³ (4,5 m², 18 m) |
| reçu à droite | +1,17·10⁻⁴ m³ (24 m², 48 m) | +5,29·10⁻⁵ m³ (12 m², 48 m) |
| niveau gauche / droite | −15,2 / +4,9 µm | −11,9 / +4,4 µm |
| attente des deux registres | **0 exactement** | **0 exactement** |
| eau créée | 0 | 0 |
| représentation fermée / monde revendicable | oui / **non** | oui / **non** |
| bilan δ intérieur + régions, final (max) | −1,6·10⁻¹⁰ (5,4·10⁻¹⁰) m³ | −2,5·10⁻¹⁰ (3,0·10⁻¹⁰) m³ |
| reçus | 19 014 | 19 014 |

Critères de P1 : région identifiable (1), rien sans reçu (2), bilan fermé au résidu et attente au
résidu — **exactement zéro** ici, puisque la région reçoit la même suite d'additions (3), frontière
publiée à zéro par choix (4), réceptions antérieures au bit : aucune scène ni banc antérieur
touché, suite complète en cours (5). **En usage** : quelques micromètres de niveau, invisibles —
mais **comptés**, et c'est ce que l'utilisateur demandait (« même lorsque son effet local sur le
niveau moyen est négligeable »).

**P2 — le protocole, écrit avant le code (21:19).**

*Échelle.* Celle du scénario 2 de B3 : `dx` = 0,05 m, et 0,025 m pour la seconde résolution. En
deux dimensions, une rangée d'épaisseur unité ; volumes en m² par mètre de largeur.

*Mêmes degrés de liberté pour tous.* FLIP/APIC : quatre particules par cellule d'eau, grille `dx`.
SPH : espacement initial `dx/2` — quatre particules par cellule aussi —, rayon de lissage `1,3·dx/2`.
Ensemble de niveaux : la grille `dx`, sans particule. Coût publié **par pas**, **par seconde
simulée** et **par degré de liberté** — CPU, un fil, même machine ; la carte n'est pas mesurée.

*Cas 1 — repos.* Bassin 2 m × 1 m, eau sur 0,5 m, 10 s. Publié : vitesse maximale (courants
parasites), dérive de volume.

*Cas 2 — ballottement.* Même bassin, premier mode, `η = A·cos(k·x)`, `k = π/L`, `A` = 1 cm
(`a·k` = 0,016, linéaire). Référence exacte : `ω² = g·k·tanh(k·h)` → **ω = 3,1789 rad/s, T =
1,9765 s**. Publié : période par **deux estimateurs** (passages par zéro et périodogramme, L360),
écart à la référence, amortissement par période.

*Cas 3 — rupture de barrage.* Colonne `a × 2a`, `a` = 0,8 m, contre la paroi gauche d'une boîte de
4a × 2,5a ; 2 s. Publié : dérive de volume ; énergie `E_c + E_p` contre `E₀` = 10 045 J/m (elle ne
doit pas croître) ; front `x_f(t)` contre le **plafond de Ritter** `a + 2√(g·2a)·t`, soit
7,92 m/s — un front au-dessus serait une faute ; temps où la lame frappe la paroi opposée ; et la
**capacité multicouche** — le nombre de segments d'eau le long d'une verticale, dont le premier
passage à deux dit que la lame s'est retournée. Accord **entre** candidats à deux résolutions :
publié, jamais appelé validation.

*Mesure du volume, par candidat — les trois ne se mesurent pas pareil, et c'est une des choses que
la comparaison doit montrer* : FLIP, la masse est exacte (particules comptées) et le volume
**occupé** se lit `Σ min(1, n/4)·dx²` ; SPH, la masse est exacte et le volume vaut `Σ m/ρᵢ` — son
écart dit la compressibilité ; ensemble de niveaux, le volume est l'aire de `φ < 0`, lissée sur une
maille, et rien ne le conserve par construction.

*Hypothèses déclarées (ADR-184 D5)* : aucun impact de jeu n'entre dans cette session ; les
échelles sont celles de B3, pas d'un objet du jeu. *Références et provenance* : la fréquence de
ballottement est la relation de dispersion linéaire (SPEC-001 §1) ; le plafond du front est la
solution de Ritter pour un fond sec ; la géométrie `a × 2a` est celle de Martin et Moyce, dont les
données — derrière un péage — ne sont **pas** utilisées.

**P4b — APIC au repos et en ballottement (21:27), et deux corrections du protocole.**

*Amendements déclarés.* (1) **`A` = 2 cm et non 1 cm** : à `dx` = 5 cm, 1 cm est sous l'espacement
des particules (2,5 cm), aucune n'est placée au-dessus du repos, et le mode **n'existe pas** — ce qui
est déjà un résultat : une représentation particulaire ne porte pas une onde plus petite que son
espacement. (2) **La jauge devient la hauteur moyenne sur le premier quart du bassin**, déduite du
volume représenté, avec un poids de bord : la particule la plus haute au bord était quantifiée, et
la bande sans poids sautait d'une colonne entière de particules (41 d'un coup).

*La faute du premier passage.* `p = 0` au centre des cellules d'air (`θ = 1`) : la surface n'est
connue qu'à la maille près, et le ballottement s'éteignait en trois secondes, période fausse de 23 à
34 %. Corrigé comme le fait toute production — surface **reconstruite** des particules (Zhu et
Bridson 2005), `φ = |x − x̄| − r`, `r` calculé pour qu'une nappe au repos ait son iso-zéro à sa
hauteur vraie — et le même fluide fantôme que l'ensemble de niveaux. Leçon candidate : **la
condition de surface, pas la représentation, décidait du résultat** ; comparer deux représentations
exige la même qualité de frontière.

| APIC | repos, 5 cm | ballottement, 5 cm | ballottement, 2,5 cm |
|---|---:|---:|---:|
| degrés de liberté | 1 600 | 1 600 | 6 400 |
| période (zéros ; périodogramme) | | +5,9 % ; +6,4 % | **−0,15 % ; −0,01 %** |
| amortissement par période | | 1·10⁻⁴ | 2,1 % |
| volume (masse / ρ) | exact | exact | exact |
| occupation, dérive maximale | 0 | 5,6 % | 4,8 % |
| énergie : maximum − initiale | +0,001 J | +0,37 J | +0,06 J |
| vitesse parasite au repos | 4,4 mm/s | | |
| coût | 1,3 ms/pas | 1,6 s pour 10 s | 11,6 s pour 10 s |

L'amplitude **semée** n'est pas 2 cm : 2,66 cm à 5 cm, 1,76 cm à 2,5 cm — les particules posent un
profil en marches ; la période se lit quand même, et c'est elle qui se compare.

**P5a + P6a — les deux autres candidats (21:34), et trois fautes attrapées avant de conclure.**
SPH : Wendland C2, Tait (γ = 7, `c₀` = 10·√(2g·h)), δ-SPH (δ = 0,1), viscosité de Monaghan
(α = 0,02), parois en particules dynamiques (trois rangées), densité initiale hydrostatique,
point milieu. Ensemble de niveaux : MacCormack limité pour `φ` et les vitesses, réinitialisation
tous les cinq pas par balayage rapide, fluide fantôme `θ` lu sur `φ`.
*Fautes* : (1) le terme δ-SPH écrit avec `x_a − x_b` **concentrait** la densité — relu avant tout
lancement, corrigé (`x_b − x_a`, Molteni et Colagrossi) ; (2) les faces d'air hors de la bande
d'extrapolation **accumulaient la gravité** — 6 m/s « maximum » au repos pour l'ensemble de
niveaux, pas de temps divisé par trois ; remises à zéro ; (3) le premier correctif de (2)
déclarait **valides** les faces alimentées par des particules, et APIC **créait** de l'énergie
(+11 % en 10 s) — les particules d'air au-dessus de la surface reconstruite gardaient une vitesse
balistique ; ordre rétabli : on extrapole d'abord depuis l'eau, et seules les faces que
l'extrapolation n'atteint pas gardent leur valeur. APIC retrouve ses chiffres de P4b **au bit**.

**P6b — l'ensemble de niveaux au repos et en ballottement (21:34).**

| niveaux | repos, 5 cm | ballottement, 5 cm | ballottement, 2,5 cm |
|---|---:|---:|---:|
| degrés de liberté (cellules) | 800 | 800 | 3 200 |
| période (zéros ; périodogramme) | | +1,6 % ; +0,9 % | +1,0 % ; **+0,19 %** |
| amortissement par période | | ≈ 0 (−5·10⁻⁴) | 0,76 % |
| dérive de volume (finale ; max) | 10⁻¹² | 5·10⁻⁵ ; 2,8·10⁻⁴ | **−0,17 %** ; 0,17 % |
| énergie : max − initiale ; finale − initiale | 0 | +0,58 J ; +0,20 J | 0 ; −8,1 J |
| vitesse parasite au repos | 0 | | |
| coût | 1,6 ms/pas | 1,8 s pour 10 s | 12,8 s pour 10 s |

Amplitude représentée 1,85–1,89 cm pour 2 cm posés : la distance signée porte l'onde **à une
fraction de maille** — ce que les particules, en marches, ne font pas (2,66 et 1,76 cm).

**P7 — la rupture de barrage, trois candidats, deux mailles (21:49).** Colonne 0,8 × 1,6 m, 2 s.

| | APIC 5 cm | APIC 2,5 cm | niveaux 5 cm | niveaux 2,5 cm | SPH 5 cm | SPH 2,5 cm |
|---|---:|---:|---:|---:|---:|---:|
| degrés de liberté | 2 048 | 8 192 | 2 560 | 10 240 | 2 048 | 8 192 |
| volume, dérive max | **0** (masse) | **0** | **7,8 %** | **3,7 %** | 0,36 % | 0,37 % |
| énergie max − initiale | 0 | 0 | **+1,5 %** | **+8,0 %** | +0,1 % | +0,1 % |
| énergie finale / initiale | 0,49 | 0,78 | 0,62 | 0,84 | 0,68 | 0,73 |
| front à 0,3 s (m) | 1,627 | 1,632 | 1,580 | 1,606 | 1,608 | 1,611 |
| front à 0,5 s (m) | 2,611 | 2,626 | 2,575 | 2,617 | 2,588 | 2,619 |
| impact sur la paroi | 0,60 s | 0,60 s | 0,60 s | 0,60 s | 0,60 s | 0,60 s |
| couches max sur une verticale | 5 | 7 | 3 | 6 | 5 | 9 |
| premier retournement | 0,88 s | 0,46 s | 0,72 s | 0,71 s | 0,67 s | 0,63 s |
| vitesse maximale | 9,5 | 16,0 | 13,4 | **23,7** | 12,1 | 11,6 m/s |
| s de calcul par s simulée | **0,94** | **10,4** | 2,2 | 26,6 | 49 | **405** |

- **Les trois fronts s'accordent à 1,6 % à 2,5 cm**, tous sous le plafond de Ritter (7,9 m/s) :
  accord entre trois méthodes indépendantes, **pas** une validation (I-14 ; Martin et Moyce
  inaccessibles).
- **L'ensemble de niveaux perd puis regagne du volume** (−7,6 % puis +1,6 %) et **crée de
  l'énergie** (+8 % à 2,5 cm), avec des vitesses parasites de 24 m/s sur les lames minces : la
  faiblesse connue de la famille dans un écoulement violent, **sans** particules de correction.
- **APIC conserve la masse exactement et n'a jamais créé d'énergie** ; SPH respire de 0,37 % et
  échange 0,1 % avec son énergie élastique.
- **Le coût sépare les familles** : SPH paie son pas acoustique — **40 fois** APIC par seconde
  simulée à la même maille.

**P5b — SPH au repos et en ballottement (22:01), et un défaut que je n'ai pas su isoler.**

| SPH | repos, 5 cm | ballottement, 5 cm | ballottement, 2,5 cm |
|---|---:|---:|---:|
| particules d'eau ; pas | 1 600 ; 39 000 | 1 600 ; 40 000 | 6 400 ; 79 019 |
| période (zéros ; périodogramme) | | −6,2 % ; −7,1 % | **−6,2 % ; −6,0 %** |
| amortissement par période | | 29 % | **27 %** |
| volume `Σ m/ρ`, écart initial ; dérive max | −0,25 % ; 5·10⁻⁴ | 4,6·10⁻⁴ | 3,1·10⁻⁴ |
| énergie perdue en 10 s | 81 J (tassement) | 83 J | 48 J |
| vitesse parasite | **0,12 m/s** | 0,19 | 0,17 |
| coût | 142 s pour 10 s | 178 s | **1 324 s** |

**L'erreur de période et l'amortissement ne convergent pas** — identiques à deux mailles : ce n'est
pas la résolution. Quatre diagnostics à 5 cm, pour l'isoler :

| variante | période | amortissement | énergie perdue |
|---|---:|---:|---:|
| de référence (δ 0,1 ; α 0,02 ; parois adhérentes) | −6,2 % | 29 % | 83 J |
| sans diffusion δ | −8,7 % | 26 % | **8 J** |
| α = 0,005 | −6,1 % | 22 % | 83 J |
| parois glissantes | −6,7 % | 23 % | 83 J |

Ce qu'ils établissent : **le tassement au repos vient de la diffusion δ** (sa forme simple, incohérente
à la surface libre — Antuono l'a corrigée) ; **ni δ, ni la viscosité, ni l'adhérence aux parois
n'expliquent l'erreur de période ni l'amortissement**. Suspect restant, non éprouvé : les **parois
en particules dynamiques** elles-mêmes, ou l'intégrateur. **Le défaut n'est pas attribué à la famille
SPH** : ce banc ne met pas SPH au même niveau que les deux autres, et la comparaison ne retiendra
contre lui que ce qui ne dépend pas de ce défaut — la conservation (masse exacte, volume à 0,4 %) et le
**coût du pas acoustique**, propre au SPH **faiblement compressible** (les variantes incompressibles,
IISPH ou DFSPH, changent ce compte).

**P3 + P4 — E1, la mer seule : la restitution de S317 ne tient pas en mer réelle (22:16).**
Houle B d'une composante, λ = 4 m, δ nul au départ, 20 s ; restitution par reçus aux deux lignes.

| | 25 cm, a = 5 cm | 25 cm, a = 2,5 cm | 12,5 cm, a = 5 cm |
|---|---:|---:|---:|
| δ parasite maximal | **2,83 cm** | 0,28 cm | **2,46 cm** |
| reçu à droite | 3,41·10⁻² m³ | 1,87·10⁻³ | 9,49·10⁻³ (largeur moitié) |
| reçu à droite / paquet de 2 cm (S317) | **292** | 16 | **179** |
| bilan de l'intérieur (final) | 6,8·10⁻⁴ m³ | 7,2·10⁻⁵ | 8,7·10⁻⁵ |
| transport de Stokes de la houle, même largeur, 20 s | 4,9·10⁻² | 1,2·10⁻² | 2,5·10⁻² |

**Le critère (< 10 % d'un paquet) est manqué d'un facteur mille.** Trois lectures :

1. **δ n'est pas petit sous une vraie mer** : 2,8 cm pour une houle de 5 cm, croissant le long du
   sens de propagation (3 mm à l'entrée, 2,7 cm à 30 m) — une onde de **désaccord** qui s'accumule.
2. **Ce n'est pas la maille** — 2,83 puis 2,46 cm à maille moitié — **c'est la non-linéarité** : à
   demi-amplitude, δ tombe dix fois (≈ `a³`) et le flux dix-huit fois (≈ `a⁴`). δ porte la
   correction d'amplitude de la vitesse de phase que B, linéaire, n'a pas : **A289 se matérialise
   dans le bilan de volume** (S274 l'avait vue en hauteur, `η'` croissant comme `a·ω₂·t`).
3. **Le flux restitué est le transport croisé** `a_B·a_δ·ω/2` — environ 2,8·10⁻² m³ en 20 s à 25 cm,
   3,4·10⁻² lus —, que ce δ **calé en phase** sur B fait passer à la ligne. La restitution de S317 le
   prend pour de l'eau sortie de δ. Et le bilan de l'intérieur ne ferme plus : le couplage à B
   apporte du volume **dans** l'intérieur, pas seulement à travers ses bords.

**Conséquence pour l'ordre D** : ADR-185 D6 renvoyait le cas couplé ici, et il ne se règle pas par
un raffinement. Remède à éprouver en E2 : un **δ témoin** — même domaine, même mer, sans le paquet —
et la restitution de la **différence** des flux ; le banc `delta3d_preview` isole déjà la perturbation
ainsi pour l'affichage. Il coûte un second domaine.

**P5 — prédiction écrite avant E2 (22:18).** Paquet de 2 cm dans la houle de 5 cm, et un δ témoin
sans le paquet, au même pas. La restitution **de la différence** des flux doit retrouver le paquet
seul de S317 — 1,17·10⁻⁴ m³ à droite, −1,37·10⁻⁴ à gauche à 25 cm — **à 30 % près** : l'interaction
paquet–houle, à des fréquences qui ne se calent pas l'une sur l'autre (5,3 et 3,9 rad/s), doit se
moyenner sur le passage. La restitution **brute** doit valoir celle d'E1 plus celle du paquet.

**P5 — E2, et ce qu'il a trouvé en chemin : sous une vraie mer, δ ne reste pas petit (22:34).**

*Le premier lancement était l'ancien binaire* — mon script d'édition avait échoué sans que je le
voie avant la compilation : paquet et houle sans témoin, δ monté à **20 cm**. Relancé avec le
témoin et une trace de δ dans le temps.

*E2, paquet de 2 cm dans la houle de 5 cm, 25 cm* : la restitution **brute** reçoit 0,44 m³ à droite
(3 700 fois le paquet) ; celle **de la différence** avec le témoin, 2,2·10⁻³ m³ (**18 fois** le paquet,
prédit à 30 % près : **manqué**) et un signe faux à gauche. Le remède du témoin ne tient pas, et la
trace dit pourquoi : **le témoin lui-même** — la mer seule, sans paquet — **croît jusqu'à 17 cm**.

*La dérive longue, δ sous la seule houle* (E1 prolongé à 120 s, 25 cm) :

| t | houle 2,5 cm (`ak` = 0,039) | houle 5 cm (`ak` = 0,079) |
|---:|---:|---:|
| 20 s | 2,3 mm | 2,8 cm |
| 40 s | 6,2 mm | **16,5 cm** |
| 60 s | 1,3 cm | 13,4 cm (saturé) |
| 95 s | 4,5 cm | 12,0 cm |
| 115 s | **7,7 cm** | 16,0 cm |
| taux de croissance | ≈ 0,04 s⁻¹ | ≈ 0,10 s⁻¹ |

**La croissance est exponentielle**, pas linéaire ; son taux croît avec l'amplitude ; δ finit **plus
grand que la mer** qui le porte. A289 prévoyait une dérive **linéaire** et lente (« ≈ 7 cm/min sous
`ak` = 0,06 », par formule, en 2D) : ceci est plus rapide et d'une autre nature. **Aucune session n'avait
fait vivre un domaine δ plus de quelques secondes sous une houle** (S302 : 6 s). Numérique ou physique
— instabilité paramétrique du pas couplé, ou modulation de l'onde totale — le raffinement le dira :
12,5 cm lancé, 120 s.

**Conséquence pour l'ordre E** : tant que δ n'est pas stable sous B, aucun bilan du couplage complet
n'a de sens en mer ; E3 (contour fermé, sans houle) ne dépend pas de ce défaut mais n'apprendrait rien
qui débloque. **Découpage déclaré : E3 est reporté**, daté dans la file ; la session publie ce qu'elle a
trouvé.

**12,5 cm, houle 2,5 cm, 120 s (22:57)** : δ 3,0 mm à 20 s, 11,8 mm à 60 s, **3,95 cm à 115 s** —
taux ≈ 0,034 s⁻¹ contre 0,043 à 25 cm. **Un peu plus lent à maille fine** : l'advection centrée, qui
irait quatre fois plus vite, n'est pas la cause dominante. Reçu à droite : 265 fois un paquet.

