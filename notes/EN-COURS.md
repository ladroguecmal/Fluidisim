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

Session : S514 — **en cours**. En autonomie, **6.7, l'acteur poussé, renversé ou déplacé par l'eau** — absent. ADR-018 (seuils de
profondeur humanoïdes 0,15 / 0,50 / 1,00 / 1,30 m ; produit d'emportement `HR = d·(v + 0,5)` et ses classes) et ADR-023 §3 (le nageur :
un corps commandé en surface bascule en mode contraint quel que soit son `ω·dt`, la commande ajoutée dans le repère de la surface) en
fixent les règles ; rien ne les construit.

**Ce que la session fait.** `actor` (cœur) : la classe de progression d'un humanoïde selon la profondeur, le produit d'emportement et sa
classe, l'adulte emporté (`HR ≥ 1,25`) ; `Floating` d'un corps commandé (toujours contraint) ; le pas contraint commandé (la vitesse
horizontale relaxée vers la vitesse de l'eau **plus** la commande).

**Ordre de grandeur, écrit avant (ADR-023 §3.4).** Un nageur à 0,7 m/s ne fait plus route quand la vitesse orbitale de surface `πH/T`
dépasse sa vitesse : à `T` = 5 s, `H* = 0,7·T/π` = 1,114 m — à la relaxation près (taux `ω` du corps ≫ `2π/T` : quelques pour mille).

**Critères, écrits avant.** (1) les seuils de profondeur et les classes de `HR` d'ADR-018, de part et d'autre de chaque seuil ; l'exemple
« 0,5 m à 2 m/s » emporte un adulte, « 0,5 m à 1,4 m/s » non ; (2) un corps commandé est contraint même à `ω·dt` = 0,14 ; (3) le nageur
commandé à 0,7 m/s face à une houle de 5 s : sa vitesse sur le fond change de signe pour `H` au-dessus de `H*` (corrigé de la relaxation,
calculé dans l'essai) et jamais en dessous, à 2 % près. 6.7 passe à partiel (manquent la poche d'air, le rouleau plongeant qui décolle le
nageur).

### Plan

- [x] **P1** — jeton, plan seul.
- [ ] **P2** — `actor` ; le mode contraint commandé ; essais (1)–(3).
- [ ] **P3** — preuve ; liste 6.7 ; rituel.

### Notes de reprise
