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

Session : S157 — en cours
Agent : Claude Code (Opus 5 ; fichiers, git et cargo disponibles)
Objectif : S156-1, A214. Remplacer **deux encadrements par une loi**. S156 a montré que le pas
radial borne la durée d'un sillage par périodicité, mais n'a mesuré que deux seuils — radial 128
décroche entre 15 et 20 s, radial 256 entre 45 et 50 s — et la formule candidate se trompe d'un
facteur 2,5 sur le second. Sans loi, pas de garde-fou : c'est exactement ce que dit A214.

### Plan

- [x] **P1** — état réel, jeton, plan déclaré et committé seul.
- [x] **P2** — un critère **lisse** et sa vérification de monotonie. Le critère de S156 — premier
      rayon qui dépasse le seuil — n'était pas monotone en temps ; une dichotomie posée dessus
      donnerait un chiffre faux avec l'apparence d'un chiffre précis.
- [x] **P3** — trois seuils par dichotomie, radial 64 / 128 / 256, angulaire fixé à 512.
      Trois points suffisent à trancher entre les deux exposants candidats.
- [x] **P4** — dépendance à `sigma` : mêmes seuils à `sigma` 4 m, `cutoff` 1,5 pour garder le
      **même produit réduit** `sigma*cutoff = 6` et donc la même forme spectrale à échelle près.
- [x] **P5** — décider : loi écrite si elle tient sur les deux familles, refus argumenté sinon.
      Garde-fou reçu seulement si la loi le mérite — A214 dit pourquoi un garde faux est pire.
- [ ] **P6** — livrable, rituel de fin, fusion `--ff-only`.

### Notes de reprise

Départ dc78f2e = master, trois copies coïncidentes, rien en attente.
298 tests/cinq ignorés, 107 ADR, 214 angles, 234 leçons, 18 invariants.

Ce que S156 laisse et qu'il ne faut pas refaire :
- `examples/wake_reach.rs` porte `profil(radial, angular, us)` et `rayon_honnete` ;
- mécanisme établi : période spatiale `2*pi*radial/cutoff`, soit 67 / 134 / 268 / 536 m ;
- le rayon est borné indépendamment par `angular`, proportionnellement — fixer `angular` à 512
  met cette seconde borne hors du chemin, et c'est pour cela qu'on la fixe ;
- plafond de la grammaire : `radial` et `angular` au plus 512. Donc `t_max(512)` restera
  **extrapolé**, jamais mesuré, et cela devra être écrit comme tel.

**Prédiction écrite avant la mesure.** Deux exposants sont en lice. Si la vitesse pertinente est
celle du plus petit nœud du maillage, `t_max` croît comme `sqrt(radial)` — c'est la formule de
S156, qui se trompe d'un facteur 2,5 sur 256. Si elle est fixée par l'échelle où vit l'énergie,
donc indépendante du maillage, `t_max` croît comme `radial`. Les deux seuils connus donnent un
rapport de 2,7 pour un doublement : **plus proche de la seconde**, et même au-delà. Le troisième
point tranchera, et s'il donne un rapport franchement différent de 2,7 c'est qu'aucune loi de
puissance simple ne décrit le phénomène — ce serait le résultat le plus utile, parce qu'il
interdirait le garde-fou pour de bon.

Piège de méthode déjà payé en S156 : un critère qui compare `radial` à `2*radial` mesure la
défaillance **de la plus grossière des deux**, à condition que la plus fine soit encore dans son
domaine. Pour radial 256 contre 512 à 50 s, cette condition n'est pas vérifiable. Le dire.

P2 — la dichotomie est exclue, et le premier critere lisse etait mauvais pour une autre raison.
Ecart global L2 entre profils, reference **fixee a radial 512** (comparer chaque resolution a sa
voisine attribue la panne a la mauvaise quand c'est la voisine qui defaille) : 4 / 5 / 3 reculs
sur 15 intervalles pour radial 64 / 128 / 256. **Non monotone : pas de dichotomie.**
Pire, le critere a un genou simultane a 20 s pour radial 64 **et** 128, alors que leurs periodes
spatiales different d'un facteur deux. Deux resolutions ne peuvent pas defaillir en meme temps
pour un mecanisme proportionnel a la periode : le genou vient de ma **fenetre d'echantillonnage**
— les 200 m sondes sont quittes vers 20 s par le front rapide — et pas du champ. Encore un
artefact de sonde qui ressemble a un resultat, le troisieme en trois sessions.
Critere retenu, propre au mecanisme : **exces d'elevation en champ proche**, cinq premiers cercles,
en **rapport** a la reference. Il vaut 1 tant que rien n'est revenu, et ne bouge que si du signal
apparait la ou il ne devrait plus y en avoir. Quasi monotone, et il separe les trois resolutions :
radial 64 quitte 1,10 entre 16 et 20 s ; radial 128 entre 32 et 36 s ; radial 256 entre 52 et 56 s.
A seuil 2,0 : ~30 s, ~47 s, au-dela de 64 s.
Rapport par doublement de `radial` : **~1,5**, soit un exposant 0,58. La racine (1,41) est proche,
le lineaire (2,0) est exclu. La formule de S156 avait donc le bon **exposant** et une mauvaise
constante — l'inverse de ce que j'avais conclu.

Correctif immediat : le battement de P2 a ete ecrit 16:40 alors que `date` disait 16:50 — la
valeur etait dans le script **avant** que l'horloge soit lue. Corrige. Cause commune avec le
battement en avance de S156 P5 : j'ecris la valeur puis je la verifie, au lieu de lire puis
d'ecrire. La regle du depot dit "le relever, jamais l'ecrire de memoire" ; elle vise ce
geste-la exactement. A generaliser en fin de session.

P3 — trois seuils obtenus, et **aucune loi de puissance ne les decrit**.
Franchissement lu sur la courbe lissee — mediane glissante a trois points puis maximum courant,
parce que la recurrence est un battement et qu'une pointe isolee n'est pas un franchissement.

| seuil | radial 64 | radial 128 | radial 256 | rapport 64→128 | 128→256 | exposant |
|---|---|---|---|---|---|---|
| 1,10 | 16 s | 36 s | 56 s | 2,25 | 1,56 | 0,90 |
| 1,25 | 16 s | 40 s | 58 s | 2,50 | 1,45 | 0,93 |
| 1,50 | 26 s | 42 s | 62 s | 1,62 | 1,48 | 0,63 |
| 2,00 | 32 s | 46 s | au-dela de 64 s | 1,44 | — | — |

L'exposant vaut 0,63 a 0,93 selon le seuil, et **les deux rapports d'un meme seuil different d'un
facteur 1,5** — ce qu'une loi de puissance interdit. Avant lissage c'etait pire : 0,48 a 1,00.
La degradation est **graduelle**, pas un seuil : c'est pourquoi l'instant de franchissement herite
du seuil qu'on choisit. Chercher un t_max unique etait mal pose.

Le meilleur regroupement essaye : `t * dk` avec `dk = cutoff/radial`. Au seuil 1,10 il vaut
1,50 / 1,69 / 1,31 pour radial 64 / 128 / 256 — constant a ±15 %. Aux seuils plus hauts il derive
(2,44 / 1,97 / 1,45 a 1,50). Donc `t_max` proportionnel a `radial`, c'est-a-dire a la periode
spatiale, **approximativement et au seuil le plus bas seulement**.
`t * dk` n'est pas sans dimension : il manque une vitesse. C'est exactement ce que le balayage en
sigma de P4 doit reveler — si le regroupement tient a sigma 4 m, la vitesse manquante est fixee
par la source ; s'il ne tient pas, il n'y a rien a ecrire.
Censure a connaitre : l'horizon de 64 s (ADR-106) coupe la mesure pour radial 256 des le seuil 2,0,
et rend radial 512 inobservable. Le plafond de la grammaire et l'horizon se conjuguent.

P4 — l'epreuve du sigma refute le regroupement, et aucun autre ne tient.
Trois sources a produit reduit `sigma*cutoff = 6` constant, donc meme forme spectrale a l'echelle
pres. Grille adaptee a l'echelle : 1 s pour sigma 0,25 m, 4 s pour les autres — une grille de 4 s
quantifiait la source etroite au point de la rendre illisible.

| sigma | cutoff | radial 64 | radial 128 | radial 256 |
|---|---|---|---|---|
| 0,25 m | 24 | 4 s | 13 s | 21 s |
| 1 m | 6 | 20 s | 36 s | 56 s |
| 4 m | 1,5 | 32 s | au-dela de 64 s | au-dela de 64 s |

- `t*dk` : 1,50 / 2,44 / 1,97 puis 1,88 / 1,69 / 1,31 puis 0,75. **Facteur 4. Refute.**
- `t*0,5*sqrt(g*sigma)*dk`, le groupe physiquement motive : 1,17 a 2,94. **Facteur 2,5.** Mieux,
  pas constant.
- Ajustement libre `t = C*sigma^p*dk^q` : dispersion 1,38, `q = -1,04` — proportionnel a la periode
  spatiale. Mais **le plan d'experience est degenere** : `dk = 6/(sigma*radial)` par construction,
  donc `sigma` et `dk` ne sont pas independants et les deux exposants ne sont pas identifiables.
  Trois parametres pour sept points lies : ce 1,38 n'est pas une loi, c'est un ajustement.
- En variables reellement independantes `sigma` et `radial` : l'exposant en `radial` vaut 0,74 a
  sigma 1 m et 1,20 a sigma 0,25 m. **Il depend de sigma : pas de loi de puissance.**

Conclusion : la degradation est graduelle, l'instant de franchissement herite du seuil choisi
(exposant 0,63 a 0,93 selon le seuil a sigma 1 m), et aucun groupe ne rassemble les mesures a
mieux qu'un facteur 2,5. **Il n'y a pas de loi a ecrire.**

Trois choses manquent, et deux sont fermees par des decisions deja prises :
1. une **specification de l'erreur acceptable** — le seuil de 10 % d'exces est ma convention, et
   c'est elle qui commande l'instant ; personne n'a dit ce qu'un consommateur tolere ;
2. une fenetre d'observation **au-dela de 64 s** — ADR-106 la borne, et la censure frappe sigma 4 ;
3. des resolutions **au-dela de 512** pour allonger le bras de levier — ADR-097 la borne.

Sonde temporaire `wake_law_sigma.rs` supprimee : un duplicat d'exemple diverge, c'est le
mecanisme des trois forks a l'echelle d'un fichier. Le balayage vit dans `wake_law`.

P5 — ADR-108 : pas de garde-fou sans tolerance declaree. Refus argumente, pas un report.
Encoder un seuil gelerait dans l'API une tolerance que personne n'a specifiee, et le tableau des
seuils montre que l'instant limite change du simple au double entre 10 % et 100 % d'exces.
Publie a la place : une **estimation conservatrice**, dans la documentation et non dans le code —
duree sure ~ 1,17 / (0,5*sqrt(g*sigma)*cutoff/radial) — accompagnee de sa dispersion, facteur 2,5.
A214 reste ouverte mais **change de nature** : il ne manque plus une mesure, il manque une
specification, et deux bornes de grammaire (64 s par ADR-106, 512 par ADR-097) limitent ce qu'on
peut mesurer. C'est ce qu'une session de mesure a rapporte.
Aucun code modifie, donc aucun test nouveau : un test ne peut pas temoigner d'un refus de
construire. Les 298 tests existants restent la seule verification, et ils passent.
