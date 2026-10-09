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

Session : S739 — **terminée**. En autonomie ; session longue. DISTANCE-PARCOURUE-S738. **La question** : comment l'onde solitaire posée
aujourd'hui se transforme-t-elle dans la 3D sur un fond plat, et une onde de départ dotée de son profil vertical (celui de SGN, S698) se
garde-t-elle mieux ?

**Ce que la session fait.**
- **Un canal plat** (`canal_s739`) : `d` = 0,5 m, `H` = 0,1 m (`H/d` = 0,2, celle de S4), 24 m, deux rangées, des murs. L'onde est centrée à
  4 m ; 5 s.
- **Deux ondes de départ**, une seule différence (ADR-276 D2) :
  - **A**, celle d'aujourd'hui : `η` de Boussinesq (`OndeSolitaire`), `u = c·η/(d + η)` uniforme sur la verticale, `w = 0` ;
  - **B**, l'onde de Rayleigh (`OndeDepart { rayleigh: true }`, S719, la solitaire de SGN) avec le profil vertical de SGN (S698) :
    `u(z) = ū + (h²/6 − z²/2)·ū_xx`, `w(z) = −z·ū_x`.
- **Les mesures, tous les 0,25 s, par la surface** (ADR-280 D1), affichées au fil du calcul (ADR-281 D1) :
  - la crête (la hauteur, la place) ;
  - la largeur à mi-hauteur ;
  - le creux derrière la crête (une traîne) ;
  - et, comme seconde lecture, la crête par les particules (ADR-286 D2).
- 5 cm d'abord (≈ 77 000 particules, quelques minutes), puis 2,5 cm pour l'onde retenue (≈ 307 000, ≈ 45 min ; ADR-274 D1).

**Les bornes** (ADR-286 D1) : la célérité vaut 2,43 m/s ; à 5 s, la crête est à 16,1 m et le front (5 %) à 18,9 m. Le mur est à 24 m :
5,1 m de marge.

**Les critères, écrits avant.**
- **E1 — A**, rapporté : la crête, la largeur et le creux selon la distance. S738 attend une crête qui oscille de ±10 %.
- **E2 — B** est retenue si, sur les 12 m :
  1. la crête reste à **±3 %** de sa hauteur de départ ;
  2. la largeur à mi-hauteur à **±5 %** ;
  3. le creux de la traîne sous **2 % de `H`** ;
  4. B fait mieux que A sur les trois.
- **E3 — l'onde retenue à 2,5 cm** : les mêmes critères.

S'il n'y en a pas, la question reste ouverte, avec les nombres. L'application à S4 (l'approche de 3 m) est pour S740.

**Contrôles du plan** (ADR-226, ADR-273, ADR-276, ADR-280, ADR-281, ADR-286)

- **témoin** : l'onde de SGN (stable, S694) et sa forme exacte ; A contre B, partis du même canal.
- **instrument** : la surface par φ, et la crête par les particules (une seconde lecture) ; le creux mesuré sur 3 m derrière la crête.
- **calcul** : E1 et E2 à 5 cm, quelques minutes chacun ; E3 ≈ 45 min ; en série, par chemins absolus.
- **ADR** : ADR-276 D1 (une seule fonction d'onde : `OndeDepart`) ; ADR-286 D1, D2.
- **pièges** :
  - la largeur à mi-hauteur d'une onde qui oscille : lue par interpolation sur la surface lissée (10 cm) ;
  - à 5 cm, `H` fait deux mailles : 5 cm explore, 2,5 cm juge.

### Plan

- [x] **P1** — jeton ; plan.
- [x] **P2** — le canal, les deux ondes, les mesures ; E1, E2 à 5 cm.
- [x] **P3** — **E3 changé, après E1–E2** (aucune onde retenue) : **A à 2,5 cm**, pour savoir si la déformation tient à la maille. Critère
  écrit avant : à 2,5 cm, la largeur à mi-hauteur reste au-dessus de **80 %** de sa valeur à 0,25 s, et le creux sous **10 % de `H`**
  jusqu'à 4,25 s. Si oui, la maille de 5 cm est la cause ; sinon, la 3D à la maille des témoins déforme l'onde.
- [x] **P4** — preuve ; fermeture.

### Notes de reprise
- **P2 fini** — E1 et E2 à 5 cm (349 s) : **les deux ondes se déforment pareil**.
  - A : la largeur à mi-hauteur passe de 2,33 m (0,25 s) à 1,44 (1,25 s), 0,67 (2,25 s), 0,55 (3,25 s) et 0,49 m (4,25 s) ; la crête 98,
    86, 114, 111, 100 mm ; le creux derrière −17, −22, −45, −54 mm.
  - B : la largeur 2,43, 1,41, 0,64, 0,57, 0,82 m ; la crête 104, 80, 120, 129, 94 mm ; le creux −4, −16, −27, −43, −47 mm.
  - **B non retenue** : l'onde de départ n'est pas la cause. La 3D à 5 cm ne garde pas une onde solitaire : elle la raidit et la rétrécit,
    comme un modèle sans dispersion, et un creux se creuse derrière.
  - Le résumé imprimé (±inf) est faux : la mesure à t = 0 est prise avant toute reconstruction de la surface. Les séries affichées font foi.
- **P3 fini** — A à 2,5 cm (873 s) : la largeur 2,35 → 0,45 m, la crête 96 → 150 mm, le creux −42 mm, la célérité 2,19 m/s (−10 %). **Manqué** : la 3D à la maille des témoins déforme l'onde. Le suspect suivant : la pression (le gradient conjugué plafonné à 4 000 itérations).
