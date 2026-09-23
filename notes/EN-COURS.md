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
- **Ce fichier ne porte que la session en cours** ([ADR-187](../docs/adr/ADR-187-methode-refondue-s321.md)
  D3). À la clôture, ce qui doit survivre des notes va à la preuve ou au journal ; la session
  suivante remplace ensuite toute la section. Aucune section d'archive, 300 lignes au plus :
  `outils/etat_projet.py --check` le vérifie. Notes de S301 à S320 : `git show 78622a19:notes/EN-COURS.md`.

---

## Session en cours

Session : S327 — **en cours**. **Lot 5 : recevoir le raccord dynamique** (A316), alternance
d'[ADR-188](../docs/adr/ADR-188-lot-3-a-la-place-du-lot-2-bloque.md).
Agent : Claude Opus 5.5, application desktop ; fichiers, git, cargo, carte réelle, accès web.
Entrée : *« Continue »*, après S326 ; suite déclarée : recevoir le raccord dynamique. **Maillons à 2** :
la session doit viser une capacité, pas un diagnostic.

**Ce que la session doit rendre possible.** Une eau qui vit à la fois en colonnes et en particules
**sans que la frontière se voie** : le geste qui fera consommer APIC par δ. Point 4.12 (« le raccord aux
colonnes » y manque). Consommateur : le raccord en 3D, puis C20 et la porte D.

**L'état laissé par S325** ([preuve](../docs/validation/B10-APIC-S320.md) §11). Masse exacte, échanges
dans les deux sens ; mais au repos 1,4 cm/s de vitesse parasite, et en ballottement un écart de surface
de 1,8 à 2,8 mailles à la frontière — **3,5 à 4,5 fois l'amplitude de l'onde** (2 cm) —, un amortissement
de 16 % par période à 5 cm. Trois suspects nommés ; un quatrième se lit dans le code : l'échange est
**asymétrique** — la sortie est un flux eulérien de la grille, l'entrée une traversée de particules —,
ce qui redresse toute oscillation.

Critères, écrits avant le code — ceux de S325, plus l'amortissement, qu'ils ne bornaient pas :
1. **Masse** : particules libres + colonnes + attente conservées à 10⁻¹⁰ près en relatif.
2. **Repos**, 5 s : vitesse maximale sous 1 cm/s ; écart de surface à la frontière sous 0,2 maille.
3. **Ballottement**, 10 s, mailles de 5 et 2,5 cm : période à 1 % de celle d'APIC seul ; écart à la
   frontière sous 0,5 maille ; **amortissement par période à 1 point d'APIC seul** ; aucune divergence.
4. APIC seul inchangé au bit ; rien dans le cœur.

Tous tenus : le raccord est **reçu** sur le banc 2D, et 4.12 avance. Sinon, chaque suspect éprouvé
seul — son effet mesuré contre le montage de S325 — reste une attribution publiée.

### Plan

- [x] **P1** — jeton, plan seul.
- [x] **P2** — instrument : série à la frontière — hauteurs géométriques des deux côtés, hauteurs de masse
  des colonnes, flux entrant et sortant, APIC seul au même endroit. Quel côté s'écarte, et quand.
- [x] **P3** — suspect choisi par P2 ; commutable, mesuré seul.
- [ ] **P4** — suspect suivant ; commutable, mesuré seul.
- [ ] **P5** — suspect suivant ; commutable, mesuré seul.
- [ ] **P6** — le raccord corrigé, contre les critères, deux mailles.
- [ ] **P7** — preuve : section datée de [B10-APIC-S320](../docs/validation/B10-APIC-S320.md), avec
  « Reproduire » ; file, liste.
- [ ] **P8** — rituel.

Les quatre suspects : (a) la surface des colonnes vue par la pression, arrondie au quart de maille par
`round(4h/dx)` — sur une onde de 0,4 maille à 5 cm ; (b) l'insertion des particules sortantes, toutes à
`x_b − dx/4` et à la même hauteur dans la maille, l'alternance repartant à zéro à chaque pas ; (c)
l'échange asymétrique ; (d) les colonnes sans mémoire de vitesse propre.

### Notes de reprise
**P2 (21:58).** `RACCORD_SERIE=1 … raccord_dyn ballottement 0.05` : une ligne `SERIE_S327` par dixième de
seconde. La frontière est au nœud du premier mode : APIC seul y reste à 0,48–0,50 m. **Séquence de
l'hybride** : de 0 à 0,2 s, la première colonne se vide — `h[0]` 0,500 → 0,473 m — par le flux eulérien
vers la colonne suivante, alors qu'**aucune particule n'a encore franchi** la frontière (entré = 0 ; la
plus proche est à un quart de maille). Puis les particules libres **s'entassent** dans la dernière colonne
libre — hauteur de masse 0,675 m pour 0,494 m géométrique à 0,4 s, 35 % de particules en trop — et
entrent **en rafale** : `h[0]` 0,604 m à 0,5 s, 0,014 m² entrés en un dixième de seconde. D'où l'écart
d'environ deux mailles. **Suspect (c) désigné** : l'entrée lagrangienne est en retard puis en rafale sur
une sortie eulérienne immédiate. P3 l'éprouve : échange eulérien dans les deux sens.

**P3 (22:02) — (c), l'échange eulérien, seul** (`RACCORD_ECHANGE=eulerien` ; défaut inchangé au bit : la
ligne de S325 se retrouve). Le flux de la grille porte l'échange dans les deux sens ; l'entrée est
créditée aussitôt, les particules libres la doivent — payée par celles qui franchissent, ou en
retirant la plus proche de la frontière ; attente et dette se compensent. Masse à l'arrondi.

| | S325 | (c) seul |
|---|---:|---:|
| écart à la frontière, 5 / 2,5 cm, mailles | 1,81 / 2,85 | **0,71 / 0,79** |
| amortissement par période, 5 / 2,5 cm | 16 % / 5,4 % | 14 % / 5,9 % |
| période (zéros), 5 / 2,5 cm — APIC +5,6 / −0,18 % | +7,6 / +2,4 % | +7,1 / +1,5 % |
| vitesse maximale, repos 5 cm / ballottement 2,5 cm | 1,4 cm/s / 0,87 m/s | **4,7 cm/s / 1,02 m/s** |

L'écart est divisé par 2,5 à 3,6 ; l'amortissement ne bouge pas, et le repos empire. **(c) explique le
saut, pas la dissipation.** Suivant : (a), la surface des colonnes arrondie au quart de maille — à
5 cm, chaque particule qui apparaît ou disparaît la fait sauter de 1,25 cm, pour une onde de 2 cm ; et
(c) la fait bouger plus souvent, ce qui expliquerait le repos dégradé.
