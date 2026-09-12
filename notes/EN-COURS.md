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

Session : S194 — terminée
Agent : Claude Code (Opus 5 ; fichiers, git et cargo 1.97.0 disponibles)
Objectif : S193-1, mesurer le **couplage de deux trains** sur le véhicule non linéaire
dispersif de S193 — écart entre la **somme des évolutions** et l'**évolution de la
somme** — sous le critère d'ADR-120. C'est la mesure qu'ADR-112 attend et qu'A217
nomme ; elle n'était pas possible avant S193. Aucun choix δ, aucun seuil de bascule.

### Plan

- [x] **P1** — reprise, état réel (quatre copies au même commit), jeton et plan seuls.
- [x] **P2** — dérivation et protocole **avant tout code** : décomposition des termes
  croisés, deux régimes prédits (harmoniques liées croisées, non cumulatives, pente 1 ;
  modulation croisée de fréquence, cumulative en temps, pente 2), exigence de bande
  pour contenir k₁±k₂ et leurs harmoniques, couples résonants et non résonants,
  normalisation, réceptions chiffrées et contre-épreuves déclarées.
- [x] **P3a** — banc de couplage `nl_coupling_2d.rs` réutilisant `support/nl_surface.rs`
  sans le modifier ; tests propres dont **train unique** et **M=1** à écart nul, et
  couple à modes disjoints exact au bit.
- [x] **P3b** — campagne : échelle d'amplitude, partage d'amplitude, croissance en
  temps, échelle M, couples résonant/non résonant, deux profondeurs ; ajustement
  `écart = α·s + β·s²·N` ; frontière des 2 % en (cambrure × durée) ; reproductibilité.
- [x] **P4** — documenter, propager A217/A216/A50/B4 et la file ; ADR seulement si une
  décision de projet est prise ; ne **pas** dériver de seuil de bascule W/δ.
- [x] **P5** — rituel de fin (§6) : journal, angles, leçons, index/README/décomptes,
  jeton `libre`, copies avancées sans suppression non prouvée.

### Notes de reprise

Hérité de S193 : véhicule `NlSurface` (bande spectrale Q, convolution tronquée,
relèvement de S192 à symbole horizontal exact, ordres M=1/2/3, RK4), reçu contre
Stokes — `b₂` à 0,4555 %, décalage de fréquence à 1,6454 %, empreinte
`0x41fc3b13793bee10`. ADR-122 retient **M=3**. Seuil 2 % d'ADR-120 fixé, jamais
redemandé. ADR-112 : une superposition indépendante ne reçoit pas le couplage.

Thèse déclarée avant mesure : l'écart de superposition n'est **pas un nombre** mais un
domaine en (cambrure × durée), parce qu'il a deux parts de natures différentes — une
part instantanée non cumulative d'ordre un en cambrure, et une part de phase cumulative
d'ordre deux multipliée par le nombre de périodes observées. Le détail est écrit en P2
avant tout code.

Leçons de S193 à appliquer ici : **L271** — déclarer au moins une configuration où
l'effet est nul par construction (ici train unique, et M=1) ; **L272** — toute ligne de
base doit être la grandeur que le véhicule porte, pas celle du continu ; **L273** — une
prédiction d'ordre se mesure avant d'être commentée.

P2 : protocole dans COUPLAGE-DEUX-TRAINS-S194. Derivation : F(a+b)=F(a)+F(b)+2Q(a,b)
+3C(a,a,b)+3C(a,b,b), donc l'ecart est la reponse a un forcage croise explicite.
Deux parts de natures differentes, sur des modes differents, donc mesurables
separement sans ajustement : quadratique liee sur k1+-k2 (pente 1, non cumulative
car aucune triade resonante en eau profonde, redemontre ici) et cubique seculaire
sur k1 et k2 (pente 2, lineaire en N). These : la validite de la superposition est
un **domaine en cambrure x duree**, pas un seuil.
Prediction de profondeur : desaccord de triade Delta calcule d'avance (2,526 / 2,519
/ 1,464 / 0,509 pour h=8/2/0,5/0,25), alpha doit croitre quand Delta decroit ; le
peu profond est mesurable ici car la comparaison n'a pas besoin d'oracle (A234).
Une affirmation fausse corrigee avant commit : j'avais ecrit que la 2D cree des
triades resonantes en eau profonde ; l'argument de non-resonance est vectoriel et
survit a l'obliquite. Ce que la 2D change est le desaccord et les quatuors.
Bande portee a Q=16 (Q=8 n'aurait pas contenu les produits cubiques jusqu'a q=9).

P3a : nl_coupling_2d.rs, support S193 **non modifie** (idiome du depot :
#[allow(dead_code)] sur le mod partage). Huit tests passent debug et release
(quatre herites du support, quatre nouveaux). Le test M=1 a corrige une
affirmation du protocole : la superposition est exacte **au bit sur l'etat** mais
pas sur le **champ reconstruit** (2,6e-16, sommation flottante non associative).
Le banc releve desormais les deux ecarts ; 2,6e-16 est le plancher de mesure.
Controle de vie a s=0,05, couple (2,3), h=8, 20 periodes :
 M=1 ecart 2,6e-16, croise et train **exactement nuls** ;
 M=2 ecart 15,36 % ; M=3 ecart 36,12 %, 8,65 % des la premiere periode.
Rapports dernier/premier quart : croise 1,023 (stationnaire, predit),
train 4,197 (croissant, predit > 2). Les deux mecanismes se separent bien.
Ordre de grandeur a retenir pour P3b : alpha ~ 1,6, donc la frontiere des 2 %
tombe vers s ~ 0,01 et la duree ne l'achete pas — a mesurer proprement.

Incident de procedure, P3a : le commit d'etape est parti **sans** la case cochee,
un `cd` ayant deplace le repertoire avant l'ecriture du plan. Repare par amende
du commit local non pousse, la regle du depot exigeant le travail et la case dans
le meme commit. A retenir : ecrire le plan **avant** `git add`, pas apres.

P3b : campagne executee, reception=true, empreinte 0x4bc0934d630c2c50, deux
executions release identiques, huit tests debug/release.
Les quatre predictions de mecanisme tiennent : part croisee pente 1,0041/1,0365 et
**stationnaire** (rapport 1,002-1,023) ; part de train pente 2,0178/2,0627 et
**croissante** (rapport 4,088-4,197). Ajustement alpha=1,3026 beta=5,8987,
residu 1,82 % de l'ecart maximal.
LIVRABLE : frontiere des 2 %. s<=0,008 tient au moins 20 periodes ; 0,009 -> 19,8 ;
0,010 -> 11,7 ; 0,0125 -> 5,4 ; s>=0,014 -> moins d'une periode. Le domaine existe
mais la duree ne s'achete pas : tout le levier est dans 0,009-0,014.
Contraste a retenir : a s=0,0125 S193 recevait UN train a 0,4555 % ; DEUX trains
franchissent 2 % en 5,4 periodes. Facteur 40 a cambrure egale.
Profondeur : alpha 1,383/1,416/4,083/11,855 pour h=8/2/0,5/0,25, desaccord
2,470/2,515/1,465/0,508 -> prediction tenue (x8,6 pour /4,9), **exception** entre
h=8 et h=2 ou les deux varient dans le meme sens (1,8 % et 2,4 %). Reception 9
partielle, dit comme tel.
Couple : ecart varie de 13 % seulement sur quatre geometries, contra-propagation
comprise. La cambrure decide, pas la geometrie.
M=2 sous-estime beta d'un facteur 3,4 : argument independant pour ADR-122.
s=0,1 a M=3 hors domaine (energie 4,64e-3), publie et exclu par la regle declaree.
DEUX REFUTATIONS publiees : (1) le controle de non-artefact du protocole confond
convergence et artefact — le rapport des deplacements vaut 3,81 donc ordre 2,
ordre mesure 1,93, residu 0,47 % a K=64 ; (2) l'hypothese « la condition initiale
en b2 du continu cause la sensibilite en K » est FAUSSE (0,4698 % vs 0,5026 % sans
le terme). Et un resultat de methode : le **maximum sur fenetre ne converge pas**
(ordres -0,79/-0,37/+0,61) parce que c'est une statistique d'ordre sur un residu.
Candidats P4 : ADR-123 (domaine de validite de la superposition), angles et lecons.

P4 : ADR-123 acte le domaine de validite de la superposition et donne a ADR-112 sa
grandeur. **A217 close** : la variable est la cambrure, plus deux variables qu'A217
ignorait (duree, desaccord de triade). SPEC-001 gagne un 1 quinquies : non-resonance
des triades en eau profonde, derivee et vectorielle, resonance generale en faible
profondeur — c'est la provenance du mecanisme. SPEC-004, PLAN-BENCHMARK B3/B4 et
BILAN-B4 actualises. File active renommee S194, ancres repointees, ligne neuve
`n` sources / A240, ligne A216/A217 scindee (A217 close, A216 seule reste).
Angles et lecons en P5 : A238 (le maximum d'un residu ne converge pas), A239
(controle de non-artefact mal forme), A240 (`n` sources non extrapolables),
L274 (juger l'ordre et non la taille du deplacement), L275 (une hypothese de cause
numerique se teste en retirant le terme soupconne).

P5 : rituel execute. Journal, trois angles A238-A240, deux lecons L274-L275,
suivis A217 (**close**), A218, A50/B4, A234, A236, A211. A217 marquee close dans
son en-tete, par ADR-123. Bloc d'etat S194 dans INDEX, README, REPRISE ; titre de
la file active porte a S194 dans les deux fichiers. Decomptes **comptes** :
123 ADR, 240 angles A1-A240, 275 lecons, 18 invariants, 6 SPEC, 23 cas.
Workspace rejoue en debug : 331 reussis / cinq ignores, identiques au recu ;
les quatre textes qui annoncaient « non rejoue » ont ete corriges.
Jeton libre. Suite S195 laissee **a instruire** entre deux options — `n` sources
(A240) ou la correction croisee quadratique — plutot qu'imposee par proximite,
ce qui est exactement ce qu'A211 reproche au chainage « suite Sxxx ».
Trois copies a avancer sur master apres ce commit, aucune suppression autorisee.

