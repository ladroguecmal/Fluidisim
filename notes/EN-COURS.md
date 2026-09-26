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

Session : S399 — **en cours**. **C5b, deuxième part** ([ADR-207](../docs/adr/ADR-207-la-campagne-du-solveur-volumique-3d.md) D5 ;
[RACCORD-3D-S398](../docs/validation/RACCORD-3D-S398.md)) : **la bande de particules et l'échange**, dans une même projection.
Demande de l'utilisateur (2026-09-27) : *« Continue »* (objectif : terminer le solveur). Agent : Claude Opus 5.5, session cloud
Claude Code ; fichiers, git, cargo, Python ; ni carte graphique, ni Godot. Sert 4.16, 4.12, A316.

**Thèse.** Dans `Apic3`, une colonne est soit de la zone (surface `η`), soit de la **bande** (particules). Deux gestes, les
leçons de 2D portées :
- **La reconstruction voit les colonnes** — l'idée de Chentanez, Müller et Kim (le champ de densité de la grille ajouté à celui
  des particules) : près de la zone, la reconstruction compte des **particules virtuelles** des colonnes, rangées étirées sur
  `[0, η]` (S327 : pas de marche d'un quart de maille), images aux parois comprises — sans quoi la frontière serait une paroi
  vue d'un seul côté (le biais de S389 et de S393).
- **L'échange par le flux de la face** (le « solde » de S327, le meilleur montage après S397) : sur chaque face de frontière et à
  chaque profondeur, le volume `u·mouillé·dx²·dt` passe à `η` de la colonne ; le solde de la face-maille le doit aux particules,
  ou le leur doit ; une particule entière due est **retirée là où elle arrive** (la plus proche de la face, dans sa maille), une
  particule entière reçue est **posée contre la face**, au sous-réseau le plus libre ; une particule qui franchit la frontière est
  absorbée et paie d'avance le solde. Masse : particules + `Σ η·dx²` + soldes.

**Critères, écrits avant.** (1) Sans zone, au bit (la ligne de S389) ; toutes colonnes, les chiffres de S398 (+0,02 %, +0,49 %).
(2) Repos, moitié particules moitié colonnes, 2 s : vitesse ≤ 1 cm/s ; la surface lue dans la dernière colonne de particules à
≤ 5 % de maille du repos. (3) Masse (particules + `η` + soldes) à 10⁻⁶ relatif sur 30 s. (4) Ballottement (1, 0), frontière au
nœud, 30 s, 5 et 2,5 cm, contre APIC seul : niveau d'eau équivalent de la bande à ±2 mm par tranche de 10 s ; densité 8 ± 0,4
particules par maille occupée dans la dernière colonne de la bande ; saut de surface < 0,5 maille ; période et amortissement
à 1 point ; circulation sur la face |ū| ≤ 5 mm/s.

### Plan

- [x] **P1** — jeton, plan seul.
- [x] **P2** — la reconstruction voit les colonnes (particules virtuelles) ; critère 2 (repos, surface lue).
- [x] **P3** — l'échange (flux de face, soldes, retrait, pose, absorption) ; critère 3 et le repos échangé ; critère 1.
- [>] **P4** — l'exemple `apic3d_raccord` : 30 s, 5 et 2,5 cm, contre APIC seul ; critère 4.
- [ ] **P5** — suite ; preuve (§5 de RACCORD-3D-S398) ; A316, file, liste.
- [ ] **P6** — rituel.

### Notes de reprise
**P2 — les particules virtuelles des colonnes** (`virtual_column_sums`) : pour une maille de la bande à portée de la zone,
chaque colonne compte `2 × 2` particules par rangée, `round(2η/dx)` rangées étirées sur `[0, η]`, images aux parois comprises ;
les mailles de la zone ne sont plus reconstruites (leur `φ` est `z − η`). **Critère 2, première moitié, tenu** : au repos, la
surface lue dans la dernière colonne de la bande se trompe de **2,19 %** de maille, comme au milieu de la bande ; **vu échouer**
sans les virtuelles : **14,7 %** (la paroi vue d'un seul côté). **Le repos dynamique** demande l'échange : sans lui, 8,2 cm/s —
la pression fait passer de l'eau par les faces de la frontière, que ni `η` ni les particules ne transportent. Jugé en P3.

**P3 — l'échange** (`columns_transport`, `columns_exchange`) : une face bande | zone est une frontière — hauteur mouillée de la
colonne, le volume passé porté au solde de la face-maille (`f64`) ; après l'advection, absorption des particules entrées dans la
zone (solde payé d'avance), retrait de la plus proche de la face (profondeurs voisines ensuite), pose contre la face au
sous-réseau le plus libre ; tri par maille, retraits marqués puis compactés. **Critère 3 tenu (sur 2 s)** : volume total
(particules + `η` + soldes) à **1,7·10⁻¹¹** au repos, **2,9·10⁻¹⁰** sous une onde qui traverse (6 576 → 6 304 particules, aucun
refus). **Critère 1 tenu** : sans zone, la ligne de S389 ; toutes colonnes, celle de S398. **Critère 2, seconde moitié, manqué** :
repos **1,007 cm/s** (4 s : 0,5 à 1,03 cm/s, sans décroître, aucune particule échangée) — la bande lit sa surface 1,1 mm sous
les colonnes (biais de lecture 2,19 % de maille contre une surface exacte) ; la marche excite une seiche d'un millimètre
(`u ~ A·ω` ≈ 5 mm/s) que rien n'amortit. L'essai garde le volume ; sur la vitesse, une garde de non-régression à 1,2 cm/s,
écrite comme telle.
