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

Session : S459 — **en cours**. *« Parfait continue »* (après R38) — **C10-2, le saut dans la mer δ**
([C10-SCENES-S454](../docs/validation/C10-SCENES-S454.md) §2), premier pas, au CPU, avec le raccord qui existe (`BandInSea`).

**Une question de l'utilisateur, répondue** : les rendus n'ont pas tous le même moteur — l'afficheur (R14–R18), Godot (R19–R33 :
fond, caustiques, pluie), des bancs (R34–R37), puis le lancer de rayons sur la carte (S452–S458, R38 : la lumière de Godot portée à
la main, sans caustiques, surface fine, pluie ni AgX). Voulu : la campagne du solveur vit dans l'afficheur ; δ entre dans Godot en C11
(S386). Proposé : avancer C11 si l'utilisateur veut juger la v1 sous le rendu Godot complet.

**Ce que la session fait.** Le banc `saut_en_mer` : une mer `Volume3` relative à B (4,8 m × 1,6 m, 1,4 m d'eau, 5 cm), au milieu
une bande `Apic3` de 1,6 m en eau totale, raccordée par `BandInSea` ; le saut de B10 au centre de la bande. **La réponse au déclencheur
de c3** (§23.6 : la mer instable au bord quand des colonnes de particules touchent le raccord) : **les colonnes de la couronne du
raccord épinglées en colonnes** (`ColumnsSwitch::pinned_columns`, nouveau) — jamais de particules au raccord.

**Critères, écrits avant.** (1) **la masse au raccord** sous 0,1 % du volume de la bande ; (2) **la mer stable** au bord de la bande :
aucun refus sur 3 s, sous une houle calme (2 cm, 2 m) et sous le saut ; aucune colonne épinglée en particules ; (3) **le cratère** à
`t·√(g/D)` = 1 à une maille de celui de la bande seule (même bande, parois), hors des colonnes qui traversent le corps.

### Plan

- [x] **P1** — jeton, plan seul.
- [ ] **P2** — `pinned_columns` (et son test) ; le banc `saut_en_mer` ; mesures (1) à (3).
- [ ] **P3** — preuve ; rituel (allégé).

### Notes de reprise
