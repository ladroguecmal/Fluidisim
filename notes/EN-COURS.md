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

Session : S737 — **en cours**. En autonomie ; session longue. DIAGNOSTIC-S4-S735. **La question** : lus par les particules, le front et la
remontée suivent-ils l'eau, et un juge du retournement qui ignore le bruit de surface garde-t-il les vrais retournements en écartant le faux
de S4 ?

**Ce que la session fait.**
- **Le front par les particules** (`front_particules_s737`) : par colonne, l'épaisseur comptée `n·dx/(8·ny)` (toutes les rangées). Le front
  est la colonne la plus avancée au-dessus de 5 mm dont la voisine d'amont l'est aussi, ce qui écarte une goutte isolée. La remontée vaut
  `zb(front) − d`. **Sa seconde lecture** (ADR-286 D2) : le front par φ de S734, rapporté à côté au fil du calcul.
- **Le juge robuste** (`retournement_robuste_s737`) : celui de S647, mais une maille « air » du vide ne compte que si son φ dépasse
  `dx/4`. Plus près, c'est l'interface (S735 : φ = +1 mm). **Le juge de S647 reste tel quel**, et les deux sont rapportés.
- `temoin_plage_s734` rapporte les deux juges, les deux fronts, et la série de la remontée par les particules tous les 0,1 s.

**Les essais, et leurs critères écrits avant.**
1. **Le cas d'école de S647** (une couche plate ; une lèvre d'eau au-dessus d'un vide) : le juge robuste ne trouve rien sur la couche, et
   trouve la lèvre.
2. **S4 sur fond lisse**, 6 s : le juge robuste **ne voit aucun retournement** (celui de S647 en voyait un à 4,94 s, à 4,14 m). La
   remontée par les particules **redescend** : à 6 s, sous 80 % de son maximum. Son maximum est rapporté contre 0,328 m (S735 attend 0,45
   à 0,53 m).
3. **S2**, 5,65 s : le juge robuste voit le retournement à **0,05 s et 0,1 m** de celui de S647 (3,290 s ; 12,01 m).

**Les bornes de durée** (ADR-286 D1, `python` au plan) :
- S4 : le mur à 15,31 m ; même à +80 % de la loi, la lame s'arrête à 13,08 m, soit 2,2 m de marge ;
- S2 : le mur à 20,22 m ; le front de S734 s'arrêtait à 17,59 m, soit 2,6 m de marge.

**Contrôles du plan** (ADR-226, ADR-233, ADR-280, ADR-285, ADR-286)

- **témoin** :
  - le juge de S647 sur les mêmes calculs ;
  - le cas d'école de S647 ;
  - la loi de Synolakis, rapportée.
- **instrument** : le front par les particules, éprouvé sur S4 lui-même, contre φ (ADR-286 D2) ; le compte de toutes les rangées (et non
  plus d'une seule) divise le bruit par deux.
- **calcul** : S4 (≈ 35 min), puis S2 (≈ 28 min), en série, par chemins absolus (ADR-286 D3).
- **ADR** :
  - ADR-280 D1 : deux lectures nommées ;
  - ADR-245 D2 : un seuil dans une seule fonction (`dx/4`, une constante).
- **pièges** :
  - changer le juge de S647 changerait toutes les mesures passées ; il n'est pas touché ;
  - le seuil de `dx/4` doit garder la lèvre du cas d'école.

### Plan

- [x] **P1** — jeton ; plan.
- [x] **P2** — les deux lectures, le juge robuste ; le cas d'école (1).
- [ ] **P3** — S4 et S2 ; (2), (3).
- [ ] **P4** — preuve ; fermeture.

### Notes de reprise
- **P2 fini** — (1) **tenu** : la couche plate, rien ; la lèvre, trouvée (colonne 27, écart 1), comme S647. Le juge robuste, le front par les particules, les deux juges et les deux lectures dans le témoin.
