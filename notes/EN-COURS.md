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

Session : S735 — **en cours**. Le diagnostic de S4 (TEMOINS-SELECTEUR-S734). **La question** : sur la pente de 1:3, le « front » d'eau se
fige à 0,48 m (escalier) ou 0,60 m (fond lisse) au-dessus du niveau, sous une couche d'environ 1 cm qui ne redescend pas ; et le juge voit un
« retournement » à 4,14 m, loin de la plage. Est-ce de l'eau réellement collée à la pente, ou une lecture fausse ? **Aucun remède dans cette
session** (ADR-226 D1 : localiser d'abord).

**Ce que la session fait.**
- `temoin_plage_s734` prend des **instantanés** à des instants donnés, écrits dans `calculs/s735_*.csv` :
  - chaque particule au-delà du pied (x, y, z, la vitesse) ;
  - par colonne de la plage (la rangée du milieu) : le fond (analytique et lisse), l'épaisseur lue par φ (`volume_surface_s708`), le nombre
    de particules au-dessus du fond, les étiquettes et φ de chaque maille.
- Au premier « retournement », les mêmes colonnes autour du lieu détecté.
- S4 sur fond lisse jusqu'à **5,0 s**, instantanés à 3,5, 4,1, 4,5 et 5,0 s (≈ 25 min).
- `outils/diagnostic_s735.py` lit les instantanés et dit, par colonne, l'épaisseur lue contre l'épaisseur des particules (le compte ×
  `dx³/8` / `dx²`).

**Le critère du diagnostic, écrit avant.** La cause est nommée si **deux lectures indépendantes** concordent :
- **« eau collée »** : au-delà de 12,5 m après 4,5 s, des particules sont présentes, au moins l'équivalent de 5 mm sur une colonne, et
  presque immobiles (|v| < 5 cm/s) ;
- **« lecture fausse »** : l'épaisseur lue par φ dépasse 5 mm là où les particules en donnent moins de 1 mm.

Si aucune des deux ne tient, la question reste ouverte et le dit. Pour le « retournement » à 4,14 m, les étiquettes de la colonne montrent ce
que le juge a vu.

**Contrôles du plan** (ADR-226, ADR-233, ADR-280, ADR-285)

- **témoin** : le même montage que S734 (S4 sur fond lisse), au bit jusqu'à 5 s ; les instantanés ne changent rien au calcul.
- **instrument** : les particules, comptées (la comptabilité) ; φ, lue (ce que voit le solveur) ; les deux, nommées à part (ADR-280 D1).
- **calcul** : ≈ 25 min, seul.
- **ADR** : ADR-226 D1 (localiser avant le remède) ; ADR-233 D1 (un instrument éprouvé sur sa famille) ; ADR-285 D1.
- **pièges** :
  - le fond lisse étiquette autrement les mailles coupées ;
  - un CSV de 280 000 lignes : seulement la plage (x > 9,8 m).

### Plan

- [x] **P1** — jeton ; plan.
- [x] **P2** — les instantanés ; l'outil ; le calcul ; le diagnostic.
- [ ] **P3** — preuve ; fermeture.

### Notes de reprise
- **P2 fini** — le calcul (1 574 s) et `outils/diagnostic_s735.py`.
  - **« Lecture fausse » : tenu.** φ lit 5 à 37 mm là où les particules en donnent zéro : 4 colonnes à 3,5 s, 8 à 4,1 et 4,5 s, 17 à
    5,0 s. À 5,0 s, la colonne figée (13,113 m) lit encore 10 mm, sans aucune particule.
  - **« Eau collée » : non tenu.** Deux colonnes seulement, à 4,5 s (6 et 9 mm, 2–3 cm/s), disparues à 5,0 s. Au-delà de 12,5 m, il y a 172
    particules à 4,5 s, puis 74 à 5,0 s, qui redescendent à 0,67 m/s.
  - **La remontée comptée par les particules** (la rangée du milieu) : 0,53 m au seuil de 5 mm, 0,45 m au seuil de 10 mm, de 4,1 à 4,5 s,
    contre 0,328 m exacts (+37 à +60 %). La 3D remonte réellement trop haut ; une autre question.
  - **Le « retournement » à 4,14 m** : à x = 4,1375 m, la maille 18 est « air » avec φ = +1 mm (0,04 maille), entre de l'eau à φ = −2 mm
    et à −6 mm. Le juge compte un vide d'une maille au contact de la surface.
