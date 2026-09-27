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

Session : S406 — **en cours**. Demande de l'utilisateur (2026-09-27) : *« Continue, je confirmes »* — la priorité du solveur passe
avant la règle des maillons (file, décisions) ; suite : le raccord **C5**. Agent : Claude, session cloud Claude Code ; fichiers,
git, cargo, Python ; ni carte graphique, ni Godot. Branche `claude/eager-volta-lf0kw3`, la plus avancée.

**Où en est C5** ([RACCORD-3D-S398](../docs/validation/RACCORD-3D-S398.md) §5–6). La zone des colonnes dans APIC 3D ballotte à
0,03 point de δ ; la bande et l'échange tiennent la masse au bit ; S400 a reçu le repos (2·10⁻⁵ m/s) et la migration. **Manqué**
au critère 4 de S399 (ballottement (1, 0) de 30 s, frontière au nœud, contre APIC seul) : le **courant moyen de surface** sur la
face de frontière, −6,7 mm/s à 5 cm (≤ 5 demandé ; APIC seul −0,5 ; la zone seule +0,5), −1,8 à 2,5 cm — une circulation fermée,
l'eau entrant dans la bande par la rangée du haut —, et la **densité** de la dernière colonne de la bande, 7,54 à 5 cm et 7,2 à
2,5 cm (8 ± 0,4), le déficit le plus fort en haut, contre la face. S400 laisse trois suspects, à éprouver par des témoins courts.

**Thèse.** Trois gestes de la frontière traitent la face bande | zone d'un seul côté : (a) sa vitesse avant projection vient du
transfert des seules particules de la bande ; (b) son débit, rangée par rangée, est mouillé à la hauteur de la **colonne**, alors
que les particules de la bande suivent leur propre surface — l'échange retire ou pose alors, dans la rangée du haut, ce que la
bande n'y porte pas ; (c) la quantité de mouvement d'une particule absorbée est perdue. Chacun reçoit une option d'essai (sans
effet par défaut, au bit) ; le banc de S399, 30 s à 5 cm (≈ 45 s de calcul), désigne celui qui porte le courant ; le remède
devient le défaut, et le critère 4 se rejoue aux deux mailles.

**Critères, écrits avant** — ceux de S399, **inchangés**. (1) Sans zone, au bit ; toutes colonnes, les chiffres de S398 ; les
options d'essai éteintes, S400 au chiffre près. (2) Repos moitié-moitié ≤ 1 cm/s. (3) Volume à 10⁻⁶ sur 30 s. (4) Ballottement
(1, 0), 30 s, contre APIC seul : niveau ±2 mm par 10 s ; densité 8 ± 0,4 ; saut < 0,5 maille ; période et amortissement à
1 point ; **courant moyen sur la face ≤ 5 mm/s**. **Prédiction** : le suspect (b) porte l'essentiel du courant (déficit en haut,
contre la face, là où passe le courant) ; son remède ramène le courant sous 5 mm/s à 5 cm et relève la densité du haut. (5) Suite
entière, zéro avertissement. **Arrêt** : si aucun témoin ne déplace le courant de plus d'un tiers, publier l'attribution manquée,
ne rien rendre défaut, et le dire.

### Plan

- [x] **P1** — jeton, décision de l'utilisateur à la file, plan seul.
- [x] **P2** — options d'essai de la frontière dans `Apic3` (a, b, c) et leur passage au banc `apic3d_raccord` ; critère 1.
- [>] **P3** — témoins courts, 5 cm, 30 s : chaque option seule, puis la combinaison ; attribution.
- [ ] **P4** — le remède attribué devient le défaut ; essais ; critères 1 à 3.
- [ ] **P5** — le critère 4 aux deux mailles (5 cm, 2,5 cm ; ≈ 12 min), contre APIC seul.
- [ ] **P6** — suite entière, zéro avertissement.
- [ ] **P7** — preuve (RACCORD-3D-S398 §7) ; liste 4.16, file (campagne, lot 5), A316 (note datée), feuille de route, index.
- [ ] **P8** — rituel.

### Notes de reprise
- **P2** — `apic3d_columns.rs` : `Apic3::TRIAL_FACE_BOTH_SIDES` (1 : face de frontière = moyenne du transfert de la bande et de
  la vitesse advectée de la zone, ou la seule zone sans poids), `TRIAL_MEAN_HEIGHT` (2 : débit mouillé à la moyenne de `η` et de
  la hauteur de la bande lue sur `φ`), `TRIAL_KEEP_MOMENTUM` (4 : une particule absorbée rend sa quantité de mouvement aux faces
  de la zone, poids 1/8 réparti comme le transfert) ; `set_columns_trials` ; banc : `APIC3D_ESSAI=<bits>`. Essais s398–s400
  tenus tels quels (6).
- **P3** (en cours) — 5 cm, 30 s, rangée du haut mouillée (k = 9) : **base** (options éteintes) — S400 au chiffre près (densité
  7,542/7,767/7,951, saut 0,101, période +0,77 %, amortissement +0,39 %, courant **−6,7**, marche +0,069 mm ; le niveau imprimé est
  absolu, S400 publiait l'écart à APIC seul) ; **(a) face des deux côtés : −3,6 mm/s** (46 % ôtés), densité du haut 7,62/7,01 ;
  **(b) hauteur moyenne : −7,2** (pire) ; **(c) quantité de mouvement : −6,3**. Prédiction (b) **manquée** ; (a) porte près de la
  moitié. Lancés : (a)+(b), (a)+(c), variante « zone seule » (8), APIC seul.
