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
- [>] **P2** — l'instrument : compteurs de l'échange dans `Apic3` (sans effet sur le calcul), densité des quatre dernières colonnes
  au banc ; critère 1.
- [ ] **P3** — le diagnostic, 5 cm et 2,5 cm, raccord et APIC seul ; critère 2 ; le geste nommé dans les notes.
- [ ] **P4** — le geste en option d'essai ; témoin à 5 cm ; critère 3.
- [ ] **P5** — s'il tient : le défaut, 2,5 cm ; critère 4. Sinon : rien ne change, publié.
- [ ] **P6** — suite entière, zéro avertissement.
- [ ] **P7** — preuve (RACCORD-3D-S398 §8) ; A316 ; liste 4.16, file, feuille de route.
- [ ] **P8** — rituel.

### Notes de reprise
