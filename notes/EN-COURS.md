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

Session : S407 — **en cours**. Demande de l'utilisateur (2026-09-27) : *« Continue »*. Suite proposée par S406 : **C5d**, la densité
au raccord, sous la condition d'une cinquième session sur A316 — un témoin court (5 cm) qui la relève à 7,6 au moins par un geste
nommé d'avance ; sinon C6 sur la frontière telle qu'elle est. Agent : Claude, session cloud Claude Code ; fichiers, git, cargo,
Python ; ni carte graphique, ni Godot. Branche `claude/eager-volta-lf0kw3`, la plus avancée.

**Où en est la densité** ([RACCORD-3D-S398](../docs/validation/RACCORD-3D-S398.md) §7). Particules par maille occupée de la dernière
colonne de la bande, contre APIC seul (8,01 à 5 cm) : 7,59 / 7,62 / 7,69 à 5 cm, 7,26 / 7,24 / 7,31 à 2,5 cm (critère 8 ± 0,4), sur
toute la profondeur à 2,5 cm ; la masse de la bande, elle, est juste (niveau à 0,5 mm près à 2,5 cm). Écartés en S406 : la face,
le débit mouillé, la quantité de mouvement absorbée, le lieu du retrait. **Ce qui n'a pas été regardé** : si le déficit est
**local** à la dernière colonne ou réparti ; et **ce que fait l'échange** — combien de particules sont absorbées, retirées, posées.

**Thèse.** Une masse juste et une dernière colonne creuse, c'est des particules déplacées de la dernière colonne vers l'intérieur
de la bande — par l'échange (retraits contre la face plus nombreux que les poses, ou poses qui repartent) ou par le mouvement des
particules près d'une frontière qui n'a pas de particules réelles de l'autre côté (la séparation, le transport). **D'abord le
diagnostic** : compteurs de l'échange et densité des quatre dernières colonnes, sans rien changer au calcul ; **puis un geste
nommé d'avance**, dans les notes avant le calcul, éprouvé par un témoin court.

**Critères, écrits avant.** (1) L'instrument ne change rien : la ligne du banc à 5 cm au chiffre près de S406. (2) Le diagnostic
publié : absorptions, retraits, poses par tranche de 10 s ; densité des quatre dernières colonnes, raccord contre APIC seul.
**Prédiction** : le déficit est **local** — deux colonnes plus loin, ≥ 7,9 — et les retraits dépassent les poses d'au moins 10 %.
(3) Le geste, nommé d'avance, éprouvé à 5 cm (30 s) : densité ≥ 7,6 dans les trois tranches, le reste du critère 4 de S399 tenu
(courant ≤ 5 mm/s, niveau ±2 mm, saut < 0,5 maille, période et amortissement à 1 point). (4) S'il tient à 5 cm : à 2,5 cm, densité
≥ 7,6. (5) Suite entière, zéro avertissement. **Arrêt** : si le geste ne relève pas la densité à 7,6 à 5 cm, rien ne devient
défaut ; publié ; la suite est C6 sur la frontière telle qu'elle est.

### Plan

- [x] **P1** — jeton, plan seul.
- [x] **P2** — l'instrument : compteurs de l'échange dans `Apic3` (sans effet sur le calcul), densité des quatre dernières colonnes
  au banc ; critère 1.
- [>] **P3** — le diagnostic, 5 cm et 2,5 cm, raccord et APIC seul ; critère 2 ; le geste nommé dans les notes.
- [x] **P4** — le geste en option d'essai ; témoin à 5 cm ; critère 3.
- [>] **P5** — s'il tient : le défaut, 2,5 cm ; critère 4. Sinon : rien ne change, publié.
- [ ] **P6** — suite entière, zéro avertissement.
- [ ] **P7** — preuve (RACCORD-3D-S398 §8) ; A316 ; liste 4.16, file, feuille de route.
- [ ] **P8** — rituel.

### Notes de reprise
- **P2** — `Columns3::counts` (absorbées, retirées, posées ; `columns_exchange_counts`), sans effet sur le calcul ; au banc,
  `quatre_colonnes` (densité des quatre dernières colonnes de la bande, de la frontière vers l'intérieur) et `echange_abs_ret_pos`
  par tranche. **Critère 1 tenu** : la ligne à 5 cm au chiffre près de S406 (densité 7,593/7,617/7,685, niveau
  +1,007/+1,772/+2,181, période +0,70 %, amortissement +0,52 %).
- **P3** (5 cm) — **la densité n'est pas perdue, elle est déplacée d'une colonne** : dernière colonne 7,59 / 7,62 / 7,69, **l'avant-
  dernière 8,45 / 8,65 / 8,66**, puis 8,02 / 8,01 / 8,08 et 8,01 / 8,00 / 8,00 (APIC seul : 8,0 partout) ; la somme des deux premières
  vaut 16. **L'échange** : absorbées 377 / 1 / 14, retirées 1 398 / 1 591 / 1 531, posées 1 785 / 1 569 / 1 559 — passé les dix
  premières secondes, **aucune particule ne traverse la face** : tout passe par le retrait (la plus proche de la face, celle qui
  allait traverser) et la pose. Prédictions : déficit local, **tenue** (8,02 deux colonnes plus loin) ; retraits > poses de 10 %,
  **manquée** (égaux). **Le geste, nommé avant le calcul** : la pose se fait **à la face**, au centre de la tranche d'eau entrée —
  une particule vaut une tranche de `dx/8` sur la face-maille, son centre à `dx/16` de la face — et non à `dx/4`, dans la maille :
  la pose à `dx/4` porte chaque volume entré un quart de maille trop loin, et le mouvement d'aller-retour (±½ maille au nœud)
  l'amasse dans l'avant-dernière colonne. **Prédiction** : à 5 cm, dernière colonne ≥ 7,8, avant-dernière ≤ 8,2.
- **P4** — témoin à 5 cm, 30 s, la pose à la face (`TRIAL_POSE_AT_FACE`) : **densité 7,785 / 7,847 / 8,002** (≥ 7,6 dans les trois
  tranches) ; les quatre colonnes 7,79:8,01:8,01:8,00 / 7,85:7,98:7,91:7,99 / 8,00:8,05:8,02:8,02 — **le dipôle a disparu** ;
  l'échange redevient naturel : absorbées **1 810 / 1 726 / 1 714**, retirées 108 / 1 / 0, posées 1 922 / 1 715 / 1 703 — les
  particules traversent la face au lieu d'être retirées avant ; niveau, écart à APIC seul, **−0,15 / −0,02 / +0,46 mm** (S406 :
  +1,76) ; courant ≤ 0,2 mm/s ; saut 0,113 ; période +0,59 contre +0,98 % (0,39 point) ; amortissement +0,43 contre +0,32 % (0,11) ;
  volume −1,2·10⁻⁹. **Critère 3 tenu.** Prédiction « dernière colonne ≥ 7,8 » : tenue aux deux dernières tranches, à 0,015 près
  à la première (7,785) ; « avant-dernière ≤ 8,2 » tenue (≤ 8,05).
- **P5** (en cours) — le défaut : **la pose à la face** (`dx/16`) ; `TRIAL_POSE_QUARTER` (16) rend la pose de S399–S406,
  `TRIAL_S400` l'implique. Essai neuf `_s407` (6 s, 16 s de calcul) : à la face, dernière colonne 7,842, avant-dernière 7,990,
  absorbées / retirées / posées 1 833 / 104 / 1 936 ; à `dx/4`, 7,958 / 8,101 et **380 / 1 305** / 1 693 — en 6 s le dipôle ne
  s'est pas encore formé, mais le mécanisme se lit : à un quart de maille, les retraits l'emportent. Lancés : 2,5 cm au défaut ;
  à 5 cm, le défaut, `APIC3D_ESSAI=16` (S406) et `=1` (S400), pour le critère 1.
- **P5** (critère 1 du changement de défaut, 5 cm) — le défaut rend le témoin au chiffre près (7,785 / 7,847 / 8,002) ;
  `APIC3D_ESSAI=16` rend **S406** au chiffre près (7,593 / 7,617 / 7,685, niveau +1,007 / +1,772 / +2,181) ; `=1` rend **S400** au
  chiffre près (7,542 / 7,767 / 7,951 ; son échange aussi dominé par les retraits : 357 absorbées pour 1 399 retraits). Attente :
  les trois calculs à 2,5 cm (le diagnostic de S406, APIC seul, le défaut).
