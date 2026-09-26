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

Session : S392 — **en cours**. **La pluie, pièce 5** d'[ADR-205](../docs/adr/ADR-205-la-pluie-complete.md) : les surfaces
mouillées — sol, margelles, murs plus sombres et plus brillants. Demande de l'utilisateur (2026-09-26) : *« Continue avec la
pluie, pièce 5 »* ; session de rendu (alternance d'ADR-191). Agent : Claude Opus 5.5, Claude Code (application de bureau) au
poste — fichiers, git, cargo, Python, RTX 5070 Laptop, Godot 4.6.3. Sert **8.10** (crédibilité perçue). **Découpage
déclaré** : 5a, les surfaces mouillées (ici) ; 5b, les éclaboussures au sol (Cossali, Coghe et Marengo 1997), session suivante
de la pièce.

**Thèse, sources et prédictions.** Un matériau rugueux sous un film d'eau (Ångström 1925 ; Lekner et Dorf 1988, *Appl. Opt.*
27, 1278 — lus par leurs résumés, sans téléchargement) : la lumière entre dans le film, se diffuse sur le matériau, et à
chaque remontée une part `r̄ᵢ` est renvoyée vers le bas par l'interface eau–air (réflexion totale au-delà de 48,6°, partielle
en deçà). Radiance diffuse mouillée, rebonds sommés et réfraction de sortie comprise : `L = (E/π)·a·(1 − r̄ᵢ)/(1 − a·r̄ᵢ)·(1 −
R(θ))`, `R(θ)` le Fresnel du film vu de l'œil ; plus le **reflet du film**, `R(θ)·L_ciel(réfléchi)`. Pour l'eau (n = 1,333),
calculés par deux intégrations indépendantes : `r̄ₑ` = 0,06641, `r̄ᵢ` = 0,47459 (= `1 − (1 − r̄ₑ)/n²`), `R(0)` = 0,02037.
**Prédit** : béton (0,42) → 0,643 de sa radiance sèche vu d'aplomb ; sol (0,20) → 0,569. Non modélisé : le second effet de
Lekner et Dorf (l'indice relatif qui baisse sous l'eau), propre au matériau — **la photographie dira s'il manque**. **Où** :
la pluie tombe à la verticale (sans vent) ; une face tournée vers le ciel est mouillée si la verticale au-dessus d'elle est
libre (les occultants de S382) ; abritée, elle reste sèche ; une face verticale ne l'est qu'au pied, dans la bande des
rejaillissements, `exp(−h/0,1 m)` — **hypothèse déclarée**, à calibrer ; une face sous l'eau des bacs n'est pas touchée.
Régime établi : sous toute pluie, les faces exposées sont mouillées (le séchage et le mouillage progressifs : la météo).

**Critères, écrits avant.** (1) **Sans pluie, identique au bit** : 12 images (méthode de S379), référence avant P3. (2) **Les
nombres** : `r̄ₑ`, `r̄ᵢ` par deux intégrations et la réciprocité, à 10⁻⁴ ; un outil et ses essais. (3) **Sur l'image** : sur une
dalle exposée, rapport mouillé / sec de la part diffuse contre la formule à l'angle du pixel, ≤ 1 % ; le reflet seul contre
`R(θ)·L_ciel`, ≤ 1 %. (4) **L'abri** : vue d'aplomb à 5 mm/px, le bord sec sous le débord de la margelle à un pixel de sa
place géométrique. (5) **Photographie réelle** : le rapport mouillé / sec d'un même matériau sous la même lumière, mesuré
(pixels lus par un canevas, sans téléchargement), contre la formule — publié, l'écart attribué ou dit. (6) Coût ≤ +0,3 ms sur
trois vues de la piscine à 10 mm/h ; sans pluie, 0 ; revue **R33** de l'utilisateur.

### Plan

- [x] **P1** — jeton, plan seul.
- [x] **P2** — les nombres : `outils/sol_mouille.py` et ses essais ; critère 2. La photographie : recherche, mesure ; critère 5.
- [x] **P3** — les images de référence sans pluie ; le nuanceur : mouillure (verticale libre, orientation, rejaillissements,
  eaux), diffus mouillé, reflet du film (ciel et occultants) ; la scène (eaux déclarées).
- [x] **P4** — les contrôles : critère 1, critère 3 (rapport, reflet), critère 4 (le bord de l'abri) ; coût.
- [x] **P5** — images de revue R33, REVUE-VISUELLE ; preuve ; file, feuille de route, ADR-205 (note datée).
- [ ] **P6** — rituel.

### Notes de reprise

**P2 — critère 2 tenu.** `outils/sol_mouille.py` et `test_sol_mouille.py` (6 essais) : `r̄ₑ` = 0,06641 (vu de l'air),
`r̄ᵢ` = 0,47459 (vu de l'eau, intégration directe) = réciprocité 0,47459 ; `R(0)` = 0,02037. Rapports mouillé / sec vus
d'aplomb : 0,540 (a = 0,1), 0,569 (0,2), 0,600 (0,3), 0,643 (0,42), 0,720 (0,6), 0,899 (0,9).

**P2 — critère 5 : mesuré, écart dit, non attribué.** La formule exacte de Lekner et Dorf n'est pas lisible sans télécharger
(résumés d'Optica et de PubMed ; le PDF de Jensen, Legakis et Dorsey 1999, qui la reprend, part en téléchargement dans le
navigateur : refusé, non retenté) ; la forme d'Ångström est reconstruite ci-dessus depuis ce que les résumés décrivent.
**Référence** (Wikimedia Commons, lue par un canevas, sans téléchargement) : *From dry pavement to wet pavement, Pillmawr
Road, Newport* (Jaggery, 7 août 2024, CC BY-SA 2.0, geograph 7844328), image de 960 × 1 280 : au premier plan, les
premières gouttes sur l'asphalte sec — même matériau, même lumière, même angle. Zone (150–710, 950–1 280), aiguilles de pin
écartées (R − B ≥ 30) : fond sec sRGB ≈ 88, taches ≈ 31 ; **rapport 0,08 en linéaire** (courbe sRGB supposée), 0,35 en valeurs
de code ; **modèle** ≈ 0,53 pour un asphalte d'albédo 0,1, vu à ≈ 50°. Écart d'un facteur ≈ 6, **non attribué** : courbe de
l'appareil inconnue (un téléphone écrase les ombres), reflet d'un environnement sombre (arbres, mur) dans les gouttes, second
effet de Lekner et Dorf (pores remplis) non modélisé. Conséquence : la formule est gardée telle quelle ; un facteur de matériau
`assombrissement` (1 par défaut, le second effet) est exposé pour que R33 dise si le rendu est assez sombre. Les recherches
sans résultat : Commons (« partially wet », « rain shadow », « dry patch »), Geograph (vérification anti-robot, non
contournée).

**P3–P4 en un commit** (le nuanceur s'est réglé sur ses contrôles). `godot/mouille.gdshaderinc` (inclus par `paroi.gdshader`) ;
`piscine.gd` : `MOUILLE=0`, `ASSOMBRISSEMENT=`, bacs et sol déclarés, nez de margelle (`ruissellement_haut`), vue
`pied_mur`, suffixe `_sec`, `VUES=` pour `--cout-pluie`, contrôle `--controle-mouille`. Godot 4.4.1 (celui des preuves).
**Critère 1 tenu** : 7 images sans pluie (ensemble, rasante, buse à 6,5 et 200 s ; sans δ à 40 s) identiques au bit avant et
après, deux fois. **Critère 3 tenu** : diffus / sec contre la formule, sol (0,19) et margelle (0,41), d'aplomb et à 60° —
**0,029 %** au pire (sol d'aplomb 0,56573 pour 0,56572) ; reflet contre `R(θ)·L_CIE` — **0,29 %** au pire (le cône de rugosité).
**Critère 4 tenu**, au second essai : le bord sec sous la margelle ouest à **0,00 mm** de la verticale de l'arête (x = −4,3000),
sec 0,000 dessous, mouillé 1,000 dehors. **Vu en chemin** : le test de dalles avec un rayon vertical divise par zéro
(composantes x et z nulles), indéfini sur la carte — la verticale sous le débord passait pour libre ; remplacé par un test
exact (emprise et haut de la boîte), et les composantes nulles écartées de zéro dans le test général.
**Critère 6, coût (10 mm/h, médiane de 240 images, sans → avec mouillure)** : ensemble 1,039 → **1,381 ms (+0,34)**, proche
1,678 → 1,984 (+0,31), rasante 0,915 → 1,135 (+0,22) ; sans pluie 0. **Manqué de peu** sur la vue d'ensemble (seuil 0,3 ms ;
bruit ±0,05 ms), publié tel quel. Chemin : +0,96 ms au premier jet ; tests précoces (verticale et reflet contre les boîtes
élargies) et ciel couvert sans les nuages du ciel clair, +0,38 ; un seul rayon loin des occultants (moyenne exacte, CIE
linéaire en `sin h`), +0,34.

**P5.** Images de R33 (`viewer/captures/s392/`, locales) : planche `r33_mouille.png` (10 mm/h, sec à gauche, mouillé à droite ;
pied du mur ouest à hauteur d'œil, ensemble, proche) et les six images en pleine taille. REVUE-VISUELLE §38 ; preuve
[SURFACES-MOUILLEES-S392](../docs/validation/SURFACES-MOUILLEES-S392.md) ; ADR-205 (note datée : 5a faite, 5b à suivre) ;
file (ligne du rendu, aussi remise à trois colonnes), feuille de route (§3 ter), index.

