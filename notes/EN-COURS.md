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

Session : S334 — **en cours**. **A317** : une coque qui perce le couvercle de δ rayonne selon la position de sa
paroi dans la maille ; chemin de la porte D ([ADR-189](../docs/adr/ADR-189-la-v1-d-abord.md)).
Agent : Claude Opus 5.5, application desktop ; fichiers, git, cargo, carte réelle, accès web.
Entrée : *« Continue, pour la référence je n'ai pas trouvé »* (2026-09-24, 08:00) — aucune référence réelle,
aucun verdict formulé sur les images de S333 : la porte D reste en attente de son verdict (R15). Suite
déclarée par S333 sans verdict : A317.

**Ce que la session doit rendre possible.** Un rayonnement de δ qui ne dépende pas de la position de la paroi
sous la maille — sans quoi une coque qui se déplace dans la grille verra ses anneaux changer à chaque maille
franchie. **Diagnostic, écrit avant le code** : δ porte par colonne une hauteur de *remplissage*, l'excès
d'eau rapporté à la section entière ; sous une coque qui perce le couvercle, cet excès monte dans la seule
part libre `a` du couvercle, où la surface vaut `(η − z₀)/a`. δ y impose pourtant `ρg(η − z₀)` : la surface
d'une colonne en lamelle est `1/a` fois trop molle — 12,5 fois à 8 %. **Correctif proposé** : la pression du
couvercle d'une colonne en partie couverte est `ρg(η − z₀)/a`, ouverture bornée par `dt²·g/dx` pour la
stabilité ; ni l'état ni le transport ne changent, donc ni la conservation ni les colonnes pleines ou fermées.

Critères, écrits avant le code :
1. **Reproduction hors du jeu** : pilonnement imposé — 5 cm, 4,48 rad/s — de la coque 4 × 1,6 × 1 m dans un δ
   de 16 × 16 m, cinq placements sous la maille (φ = 5, 8, 30, 55, 80 % d'eau dans la maille de bord −y) ;
   amplitude rayonnée à 3 m de chaque flanc long : la dispersion et la dissymétrie d'A317, mesurées.
2. **Après correctif** : les dix amplitudes (cinq placements, deux flancs) à ± 5 % de leur moyenne, et chaque
   rapport de flancs à ± 5 % de 1.
3. **Rien d'autre ne change** : essais S324–S333 verts, valeurs imprimées inchangées — les cubes de S332 ont
   leurs parois sur des faces, leurs couvercles sont pleins ou fermés.
4. **Stabilité** : la scène de la porte D, 800 pas, reste stable ; volume au plancher du transport.
5. **La scène de la porte D rejouée** : placements 8/52 et 30/30, amplitudes des deux flancs à ± 5 % l'une de
   l'autre ; images refaites.

### Plan

- [x] **P1** — jeton, plan seul.
- [x] **P2** — banc `a317_lamelle` : pilonnement imposé, cinq placements ; critère 1 (avant correctif).
- [x] **P3** — correctif : la pression du couvercle en partie couvert, commutable pour la mesure ; critère 3.
- [ ] ~~**P4** — banc après correctif ; critère 2.~~ *Caduc : le correctif ne porte pas A317 (P3).*
- [x] **P4** — *amendé* : **convergence** sur une tranche quasi-2D — coque infiniment longue, `ny` = 2 —,
  mailles de 25 ; 12,5 ; 6,25 ; 3,125 cm, quatre placements chacune ; couvercle partiel éteint puis allumé.
- [x] **P5** — *amendé deux fois* : **l'eau poussée par une paroi qui glisse** — quand l'ouverture du couvercle
  d'une colonne se referme, l'eau de surface de la part recouverte passe aux colonnes voisines au couvercle
  ouvert ; puis **couvercle partiel par défaut** si les critères 9 à 11 tiennent.
- [x] **P5 bis** — la scène de la porte D rejouée avec le couvercle partiel : critères 3 à 5, images.
- [ ] **P6** — preuve (PORTE-D-S333 §6, S334) ; A317 note datée ; R15 au registre des revues ; file, feuille
  de route si un état change.
- [ ] **P7** — rituel.

**Amendement après P3, déclaré avant le code.** Le couvercle partiel ne retire pas la dépendance au placement.
Question suivante : A317 est-il une **erreur de résolution** près de la paroi — la colonne voisine de la coque
porte seule la surface qui rayonne, à 6,4 mailles de largeur de coque —, ou un **défaut de structure** ? Une
tranche quasi-2D a le même mécanisme de paroi et coûte cent fois moins : quatre résolutions y sont possibles.
**Critères** : (6) à 25 cm, la tranche reproduit l'écart de la 3D (≥ 20 %) ; (7) l'écart décroît avec la maille
— ordre mesuré ; ≥ 1 : erreur de résolution, sinon défaut de structure ; (8) l'amplitude moyenne converge, et
la valeur extrapolée dit quelle surface — S332 ou couvercle partiel — s'en approche le plus vite.

**Second amendement, après P4, déclaré avant le code.** Le couvercle partiel est la bonne physique : il
converge, S332 non. Il reste éteint pour une seule raison — une colonne dont l'ouverture se referme garde son
excès d'eau, que `1/a` change en pointe. Remède : l'eau de surface de la part qui vient d'être recouverte,
`(surface)·(1 − a_nouveau/a_ancien)`, va aux voisines de la couche du haut, au prorata de l'ouverture de la
face partagée et du couvercle voisin — la paroi pousse l'eau devant elle. Somme exacte, aucune allocation.
**Critères** : (9) contre-épreuve de S333 — coque tenue fixe, qui glisse de ± 25 cm dans δ — vitesse sur les
faces ouvertes ≤ 1 m/s avec le couvercle partiel (12,2 m/s sans ce remède, 0,32 avec le couvercle de S332) ;
(10) le volume de δ suit toujours la coque plongée — pilonnement de S332 ≤ 10⁻⁹ m³, glissement mesuré ;
(11) par défaut : les valeurs S3xx ne changent que là où un couvercle est en partie couvert, et la liste des
valeurs changées est publiée.

### Notes de reprise
- **P2, critère 1 — A317 reproduit hors du jeu** (pilonnement imposé 5 cm, 4,484 rad/s ; bandes à 2,5–3,5 m ;
  fenêtre 3–6 s). Amplitude quadratique moyenne des flancs −y / +y, mm : φ 0,05/0,55 → 16,08 / 8,89 (1,81) ;
  0,08/0,52 → 15,81 / 8,28 (1,91) ; 0,30/0,30 → 10,89 / 10,91 ; 0,55/0,05 → 8,86 / 16,11 ; 0,80/0,80 → 15,99 /
  15,99. Moyenne 12,78 mm, **écart 35 %**. **Le diagnostic écrit ne suffit pas** : l'amplitude n'est pas
  monotone en l'ouverture du couvercle — 0,80 rayonne comme 0,05 ; minimum vers 0,5, paroi au centre de la
  maille. Un second mécanisme est probable.
- **P3, le correctif éprouvé ne suffit pas.** Pression du couvercle partiel `ρg(η − z₀)/a`, ouverture bornée par
  `dt²·g/dx`. **Premier piège** : `set_solid_rigid` dépose l'eau que la coque déplace dans la hauteur de
  remplissage *avant* la projection ; le flux de paroi la retire *pendant* le pas — divisée par `a`, pointe à
  10,7 m/s ; le dépôt est donc mis à part (`Base3::deposit`, remis à zéro après un pas réussi). **Second
  piège, sans remède ici** : une colonne dont l'ouverture se referme — coque qui glisse — garde son excès
  d'eau, que `1/a` change en pression : 12,2 m/s dans la contre-épreuve de S333. **Et sur le pilonnement pur**
  (ouvertures fixes) : amplitudes 27,4 / 17,1 (0,05/0,55), 25,1 / 16,9, 15,8 / 15,9, 17,0 / 27,4, 19,2 / 19,2 mm —
  toutes plus fortes (+20 à +90 %), **écart 36,5 %, dissymétrie 1,60** : la mollesse du couvercle partiel est
  réelle, mais elle **n'est pas** le mécanisme d'A317. **Décision** : commutable, **éteint par défaut** ; les
  18 valeurs imprimées des essais S3xx sont alors identiques au bit (critère 3).
- **P4, convergence en tranche** (`--tranche`, pilonnement imposé, bandes à 2,5–3,5 m ; amplitudes en mm,
  placement φ et fraction de la seconde paroi entre parenthèses). **Couvercle de S332** : 25 cm → 23,9 (0,05) ;
  11,3 (0,55) ; 17,3 (0,30) ; 21,1 (0,80) — moyenne 18,4, **écart 38,5 %** ; 12,5 cm → 24,6 ; 19,9 ; 11,1 ; 20,4 ;
  13,7 ; 16,5 ; 19,6 ; 10,5 — moyenne 17,0, **écart 44 %** : **ne converge pas**. **Couvercle partiel** : 25 cm →
  35,9 ; 23,3 ; 22,3 ; 26,2 — moyenne 26,9, écart 33 % ; 12,5 cm → 24,4 ; 22,9 ; 21,4 ; 21,4 ; 21,2 ; 21,8 ; 22,2 ;
  20,6 — moyenne 22,0, **écart 10,9 %** : **converge**, ordre ≈ 1,6. **La conclusion de P3 était prématurée** :
  la mollesse du couvercle partiel est bien le défaut de structure ; à 25 cm, le correctif laisse une erreur
  de résolution ordinaire. 3,125 cm : gradient conjugué non convergé au pas 23 (8 000 itérations), les deux
  variantes — petites cellules sous le fond de la coque.
- **P4, 6,25 cm.** S332 : 23,2 ; 15,5 ; 14,4 ; 15,6 ; 17,5 ; 19,2 ; 19,0 ; 17,9 — moyenne 17,8, **écart 30,5 %**.
  Partiel : 20,3 ; 20,8 ; 20,8 ; 20,5 ; 20,9 ; 20,2 ; 20,3 ; 20,8 — moyenne 20,6, **écart 1,8 %**. **Bilan** :
  critère 6 tenu (tranche à 25 cm, 38,5 % ≥ 20 %) ; critère 7 — S332 **ne converge pas** (38,5 → 44,2 →
  30,5 %), défaut de structure ; partiel **converge** (33,4 → 10,9 → 1,8 %) ; critère 8 — moyenne partielle
  26,93 → 21,99 → 20,56 mm, ordre 1,79, **extrapolée 19,98 mm** ; S332 reste à 17–18 mm, 11 % sous la limite.
- **P5, l'eau poussée par une paroi qui glisse** : faite (transferts vers les voisines de la couche du haut,
  somme exacte). Contre-épreuve de S333 avec le couvercle partiel : 12,2 → **5,47 m/s** — critère 9 **non
  tenu**. Localisée par mode : glissement pur 0,39 m/s, pilonnement relatif pur 0,34 m/s ; **dès que la coque
  tourne par rapport à l'eau**, 1,37 à 5,47 m/s, toujours dans une colonne de coin au couvercle ouvert à 7–8 % :
  le résidu de rotation de S332 — vitesse de paroi au centre des faces, non au centroïde de leur part couverte
  — que `1/a` amplifie. **Décision, selon le plan** : couvercle partiel **éteint par défaut** ; S3xx au bit.
  **Préalable nommé** : la vitesse de paroi au centroïde de la part couverte (remède de S332), puis défaut
  allumé. Valeurs changées si on l'allume : S332 pilonnement 2,7266 → 2,7434·10⁻¹⁰ m³, décalage 0,0667 →
  0,0666 m (couvercles « poussière » du cube), contre-épreuve S333 0,32 → 5,47 m/s.
- **P5 bis, scène de la porte D** (`--couvercle-partiel`, flancs à 2,5–3,5 m, 3–8 s ; quatre scènes en parallèle,
  ≈ 345 ms/pas). Couvercle de S332 : 30/30 → 34,3 / 34,3 mm ; **8/52 → 51,2 / 15,0 mm, rapport 3,42**. Partiel :
  30/30 → 46,5 / 46,5 mm ; **8/52 → 55,3 / 45,4 mm, rapport 1,22** — carte à 8 s presque identique au 30/30.
  **Critère 5 non tenu** (± 5 %) : résolution de 25 cm et résidu de rotation — bruit près des coins, visible
  à ×5 ; vitesse sur face ouverte 0,69 m/s (30/30), 1,07 m/s (8/52). Critère 3 tenu au bit ; critère 4 :
  8,3 et 9,3·10⁻⁹ m³ (S332 : 3,8), toujours ~3·10⁻⁴ de la borne d'arrondi. Défaut (éteint) : valeurs de S333
  retrouvées au bit (0,09385 m ; 3,8169·10⁻⁹ m³). Images `viewer/captures/s334` : scène / carte 2 s
  `0xdc67e48d93ed4533` / `0x6e476e9fd79ea811` ; 4 s `0x4ac9326e7d7003d6` / `0xc8c7f61964126c39` ; 6 s
  `0x2db0f7edc09d42bf` / `0x12ed29a898550da5` ; 8 s `0x02c587bba1e7f64a` / `0x67457176f1526445` ; carte 8/52 à 8 s
  `0xf14d3c45be994d27`. Lecture de la tranche à 30 % : S332 ~13 % sous la limite, partiel ~12 % au-dessus.
