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

Session : S738 — **en cours**. En autonomie ; session longue. LECTURE-PARTICULES-S737. **La question** : pourquoi la 3D remonte-t-elle
S4 (`H/d` = 0,2, 1:3, `d` = 0,5 m) à 0,52 m au moins, quand S645, le même cas à `d` = 0,35 m, remontait à 98 % de la loi (0,2307 m contre
0,2295) ? **Une cause à la fois, du moins cher au plus cher (ADR-276 D2) ; aucun remède dans cette session.**

**Les essais, et leurs critères écrits avant.**
- **E1 — S645 relancé tel quel** (`ballistic_air_lets_the_swash_run_up_fine_s645`, ≈ 5 min). Le code d'aujourd'hui redonne-t-il 0,2307 m
  (la particule la plus haute) à 2 mm près ? Si non : **une régression**, la bissection de l'historique est la suite, et rien d'autre.
- **E2 à E5 — S4 rapproché de S645**, un écart à la fois. Chaque variante garde les précédentes :
  - **E2, le plafond** : le domaine monte à 1,0 m au-dessus du niveau (au lieu de 0,6 m) ;
  - **E3, l'approche** : l'onde à la distance canonique du pied, comme S645 (sans les 3 m de plus) ;
  - **E4, la vitesse sur la grille** : posée sur les particules seulement, comme S645 ;
  - **E5, l'échelle** : `d` = 0,35 m, `H` = 0,07 m, la même géométrie que S645 sauf le mur du large.

  Chacune rapporte trois lectures de la remontée :
  - la particule la plus haute au-delà du pied (l'instrument de S645) ;
  - le front par les particules (S737), au seuil de 5 mm ;
  - la loi.

  **L'écart désigné** est la première variante qui ramène la remontée (la particule la plus haute) à moins de 15 % de la loi.
- **Les bornes** (ADR-286 D1) :
  - avec le plafond à 1,0 m au-dessus du niveau, le fond le rejoint à 1,5/3 = 0,5 m après le rivage… *au plus 1,0 m de remontée* ;
  - la plage fait 4 m au-delà du rivage, soit 1,33 m de hauteur : le plafond est la frontière, et une remontée qui l'atteint est
    rapportée comme plafonnée ;
  - la durée, 6 s : la remontée culmine vers 4,5 s.

**Contrôles du plan** (ADR-226, ADR-276, ADR-285, ADR-286)

- **témoin** : S645, mesuré aujourd'hui ; la loi de Synolakis (le déferlement au reflux n'y change rien : la remontée précède le reflux).
- **instrument** : trois lectures de la même grandeur (ADR-286 D2), rapportées au fil du calcul.
- **calcul** : E1, 5 min ; E2 à E5, ≈ 30, 25, 25 et 10 min, en série, par chemins absolus.
- **ADR** :
  - ADR-276 D2 : une cause à la fois, le plan écrit ce que chaque variante change ;
  - ADR-286 D1 : les frontières, chiffrées.
- **pièges** :
  - S645 avait un mur au bout de la plage, à 0,26 m au-dessus du niveau ; S4 n'en a pas. E5 le dit ;
  - la variante qui « ramène » peut cacher deux causes : on s'arrête à la première, et le rapport le dit.

### Plan

- [x] **P1** — jeton ; plan.
- [x] **P2** — E1.
- [ ] **P3** — les variantes de la scène ; E2 à E5 (on s'arrête à la première qui ramène).
- [ ] **P4** — preuve ; fermeture.

### Notes de reprise
- **P2 fini** — E1 : **S645 redonne exactement 0,2307 m** (la particule la plus haute), 0,2250 m (étiquettes), la crête 0,0730 m ; 380 s. **Aucune régression** : l'écart est dans le montage. Les variantes E2 à E5 écrites (`s4_vers_s645_s738`), l'instrument de S645 ajouté au témoin.
