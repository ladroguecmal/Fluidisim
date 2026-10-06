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

Session : S553 — **terminée**. En autonomie, **6.6 — plusieurs compartiments et leurs cloisons** : l'envahissement progressif — la brèche
envahit un compartiment, qui envahit le suivant par une cloison percée.

**Ce que la session fait.** Un essai, sans code neuf (les pièces de S548–S552) : la barge de S548 ; deux compartiments de V de 5 × 8 × 4 m,
l'avant (`x` ∈ [0 ; 5]) et l'arrière (`x` ∈ [−5 ; 0]), formes volumiques ; une brèche de 0,1 m² au fond de l'avant, un trou de 0,05 m² au
pied de la cloison qui les sépare (deux arêtes d'orifice, une par sens : l'eau va du plus haut au plus bas) ; la mer volumique recentrée sous
la brèche (S552) ; la pesanteur du navire ; l'eau de chaque compartiment en son centre mouillé. Pendant l'envahissement la barge prend de
l'assiette (l'avant d'abord) ; à l'équilibre, tout est symétrique.

**Ordre de grandeur, calculé.** La flottabilité perdue des deux : `T' = T·A/(A − A₁ − A₂)` = 1,5·160/80 = **3,000 m**, **120 m³** dans
chacun ; franc-bord 1 m ; `GM` transversal après envahissement 1,278 m (stable : les extrémités intactes portent l'inertie de flottaison).

**Critères, écrits avant.** (1) Le tirant final au centre (vertical) à 1 % de 3,000 m ; l'assiette finale sous 0,1°. (2) L'eau de chaque
compartiment à 2 % de 120 m³. (3) Pendant l'envahissement, l'arrière en retard sur l'avant (la cloison limite) ; la masse de V exacte.

### Plan

- [x] **P1** — jeton, plan seul.
- [x] **P2** — l'essai ; (1)–(3).
- [x] **P3** — preuve ; liste 6.6 ; rituel.

### Notes de reprise
- **P2 fini** — à 900 s l'envahissement n'était pas fini (2,64 m) ; la trace : équilibre vers 1 800 s ; à 3 000 s, tirant 2,9985 m, assiette
  0, 119,88 m³ chacun ; l'arrière en retard ; masse exacte. Suite 712.
- **P3** — preuve CLOISON-PERCEE-S553 ; liste 6.6 ; index ; journal.

