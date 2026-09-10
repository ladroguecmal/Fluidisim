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

Session : S156 — terminée
Agent : Claude Code (Opus 5 ; fichiers, git et cargo disponibles)
Objectif : S155-1. La première branche que S154 proposait, que le refus du noyau rendait
inaccessible et que ADR-106 vient d'ouvrir : **bilan énergétique d'un sillage prolongé et domaine
de collecte requis**, comme S153 et S154 l'ont fait pour les impacts.

### Plan

- [x] **P1** — état réel, jeton, plan déclaré et committé seul.
- [x] **P2** — énergie d'un sillage prolongé jusqu'à 60 s : puissance pendant le forçage,
      conservation après extinction. C'est exactement ce que S155 vient de rendre mesurable, et
      personne ne l'a encore regardé au-delà de 8 s.
- [x] **P3** — **où** est cette énergie dans l'espace à 60 s : balayage radial du champ contre
      l'oracle f64 à quadrature doublée, et fraction contenue dans un rayon donné.
- [x] **P4** — décider d'après les chiffres : la borne utile est-elle en temps, en domaine, ou
      en **résolution spectrale** ? Les trois ne se corrigent pas au même endroit.
- [x] **P5** — recevoir ce qui doit l'être, avec un test témoin ; ne rien construire dont le
      prix dépasse le bénéfice.
- [x] **P6** — livrable, rituel de fin, fusion `--ff-only`.

### Notes de reprise

Départ d7db1d2 = master, trois copies coïncidentes, rien en attente.
297 tests/cinq ignorés, 106 ADR, 213 angles, 232 leçons, 18 invariants.

Outillage déjà en place, à ne pas réécrire :
- `examples/wake_motion.rs` (S150) monte la chaîne complète — `Wake::build`, WPRS, journal,
  `Prepared::from_journal`, oracle f64 `GaussianPressure` à quadrature doublée ;
- `Prepared::energy_j()` et `power_w()` donnent le bilan spectral en joules ; l'énergie est une
  somme de Kahan sur les nœuds, `rho/2 * poids * (g|eta|^2 + |v|^2/k)` ;
- `wake_emitter` découpe le mouvement en tronçons, chacun source distincte (ADR-104).

**Prédiction écrite avant la mesure**, pour qu'elle puisse être démentie — c'est ce qui a le
mieux rapporté en S155. Le paquet s'étale à la vitesse de groupe `c_g = ½ sqrt(g/k)` : pour
sigma 1 m et cutoff 6, les modes rapides sont les **petits** k, et à 60 s ils sont déjà à des
centaines de mètres. Mais la quadrature est **discrète** : un pas radial `dk ≈ cutoff/radial`
rend le champ périodique de période `2*pi/dk`, soit ~134 m pour radial 128. Si le paquet dépasse
cette période, il **revient** par l'autre bord au lieu de partir. La borne utile serait alors ni
le temps ni le domaine, mais la **résolution spectrale**, et le rayon honnête décroîtrait
relativement à l'étalement au lieu de croître.

Si c'est vrai, ADR-106 est nécessaire et **non suffisant** : porter l'horizon à 64 s sans porter
la résolution ne donne qu'un champ replié. Si c'est faux, il faut le dire aussi.

Piège identifié d'avance : l'oracle `GaussianPressure` porte **la même** discrétisation, en plus
fin. Un accord candidat/oracle ne prouverait donc rien sur le repliement — les deux replient. Le
seul juge est la comparaison entre **deux résolutions** et, si possible, une quantité physique
indépendante de la quadrature : l'énergie totale, qui doit être conservée après extinction.

P2 — l'energie d'un sillage prolonge se conserve, et c'est un resultat vide.
Forcage 16 s en huit troncons de 2 s a 2 m/s, 100 N, sigma 1 m, cutoff 6 ; observation a 60 s,
possible depuis ADR-106. Puissance exactement nulle des l'extinction ; energie **identique au bit
pres** a 16, 20, 30, 45 et 60 s : 2,882884145e-1 J en 128x128, 2,884232700e-1 en 256x256.
Mais cette conservation est **structurelle** : apres extinction chaque mode tourne, et la rotation
laisse g|eta|^2 + |v|^2/k invariant. Le bilan spectral ne pouvait pas ne pas se conserver. Il
confirme l'implementation, il ne dit rien de la validite spatiale du champ. Le noter comme
resultat aurait ete une mesure vide.
Ecart de quadrature sur l'energie totale entre 128x128 et 256x256 : 4,7e-4 relatif. Petit.

P3 — deux bornes, deux lois, et la prediction n'etait juste qu'a moitie.

Ce qui est confirme : la quadrature discrete rend le champ **periodique**, de periode
`2*pi*radial/cutoff` — 67 / 134 / 268 / 536 m pour radial 64 / 128 / 256 / 512. A 4 s, deux
resolutions voisines cessent de s'accorder a 45 m (64 contre 128) et 100 m (128 contre 256),
soit les **deux tiers** de la periode de la plus grossiere dans les deux cas. Le paquet ne part
pas : il **revient par l'autre bord**.

Ce qui n'etait pas prevu : la borne **angulaire** est independante et suit une autre loi. A 8 s,
radial fixe a 512, le rayon honnete vaut 20 / 45 / plus de 200 m pour angular 64 / 128 / 256 —
proportionnel a `angular`, comme l'alias de `exp(i k.x)` sur le cercle le veut. Les deux bornes
ne se corrigent donc pas au meme endroit, et **laquelle mord depend de l'instant** : a 8 s c'est
l'angulaire (512x128 honnete a 45 m, 128x512 honnete au-dela de 200 m) ; a 60 s c'est la radiale,
exactement l'inverse (512x128 reproduit 512x512 au bit pres en champ proche, 128x512 se trompe
d'un facteur 75).

Duree honnete mesuree, angulaire fixe a 512 : radial 128 cesse de s'accorder **entre 15 et 20 s**,
radial 256 **entre 45 et 50 s**. La recurrence predite `2*pi/(c_g,max * dk)` donne 13,1 et 18,5 s :
juste a 30 % pour 128, trop pessimiste d'un facteur 2,5 pour 256. La formule prend `c_g` au plus
petit noeud du maillage, alors que les tres petits k ne portent presque pas d'energie. **La loi
n'est pas verifiee ; seuls les deux encadrements le sont**, et c'est ce qui sera publie.

Point de logique a ne pas rater : quand 256 et 512 divergent a 50 s, cela accuse **256**, pas 512
— la coherence en dessous de 45 s et l'echec plus precoce de 128 le montrent. Mais `radial` et
`angular` plafonnent a **512** dans la grammaire de recette (`validate_recipe`), donc
**t_max(512) n'est pas mesurable** : il n'existe aucune reference plus fine. Extrapoler donnerait
plus de 100 s ; ce serait une extrapolation, pas une mesure, et il faut le dire.

Consequence pour B2 : la recette de la reception de sillage S150 est 128x128 — honnete jusqu'a
~17 s et ~45 m. Un bilan a 60 s demande radial >= 512 et angular >= 256, soit 131072 noeuds
contre 16384 : **huit fois plus**.

P4 — ADR-107 : le domaine d'un sillage se deduit de sa recette. Prix mesure, mediane sur cent
repetitions apres chauffe separee : 25,6 / 102,0 / 205,5 / 402,9 ms de preparation et 19,5 / 75,0
/ 150,4 / 298,3 ms par lot de 64 points pour 128x128 / 256x256 / 512x256 / 512x512. Cout lineaire
en nombre de noeuds. Le plafond de 512 n'est pas releve : aucune reference plus fine n'existe pour
verifier, et le prix de ce qu'il faudrait est deja hors de portee.
Verdict B2 volet sillage : **partiel et negatif a 60 s**, fonde sur une mesure et non sur un
manque de mesure. Aux durees ou il a ete recu (8 s, S150/S151), le candidat est dans son domaine.
Non fait et nomme (A214) : le garde-fou d'admissibilite. La loi en duree n'est encadree qu'en deux
points ; un garde bati dessus refuserait du valide ou admettrait de l'invalide.

P5 — un seul test recu, et rien d'autre construit. `recurrence_radiale_hors_domaine_s156` epingle
le fait, pas la loi : a 8 s radial 128 et 512 s'accordent a 2,4 % pres du centre ; a 60 s la
grossiere y montre 6,406e-5 m contre 1,322e-6 pour la fine, **quarante-huit fois plus**, energie
revenue par periodicite. Angulaire fixe a 64, points a moins de 5 m — dans le domaine angulaire,
pour que seul le pas radial soit juge.
Temoin verifie : les deux mesures prises a radial 512, le test echoue (ligne 218). Il ne peut donc
pas passer par construction.
Cout 0,66 s en debug ; 298 tests/cinq ignores (205+93), debug et release, zero echec.

P6 — SILLAGE-DOMAINE-S156, ADR-107, A214, L233, L234, journal, index, README, REPRISE, jeton
rendu, ff-only. 298 tests/cinq ignorés en debug et en release.

Pour S157 sans relire : S156-1 remplace deux encadrements par une loi. La sonde `wake_reach`
fait déjà tout le travail — `rayon_honnete` compare deux profils, `profil(radial, angular, us)`
construit le champ. Ce qui manque est une dichotomie sur l'instant de décrochage à radial 64, 128
et 256 (angulaire fixé à 512), soit trois seuils au lieu de deux encadrements, plus la même chose
à sigma 4 m pour voir si la loi dépend de la largeur de la source. Attention : le décrochage n'est
pas monotone point par point — à 45 s, 128 contre 256 donnait « 2 m » entre deux « aucun ». La
dichotomie doit porter sur une quantité lissée, pas sur le premier rayon qui dépasse le seuil.
Ne pas oublier ce que la session a coûté à apprendre : le plafond 512 borne ce qui est
vérifiable, donc la loi devra être extrapolée pour 512 et cela devra être écrit comme tel.
