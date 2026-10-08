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

Session : S698 — **terminée**. En autonomie, sans arrêt. S697 : la zone de colonnes du raccord du large (hydrostatique, vitesse uniforme)
fait tout l'écart. **Le raccord du large par particules**, sans zone de colonnes.

**Ce que la session fait.**

- **Le bord gauche d'APIC par particules** (`apic3d_gauche.rs`), le miroir de S682–S683. Les particules qui sortent par la gauche sont
  retirées et comptées. Le volume qui entre est posé par quanta, chaque particule avec la vitesse que l'appelant donne à sa hauteur.
- **Le porteur SGN** (S694) au large. Au raccord, `ū = q/h`, puis `ū_x` et `ū_xx` par différences sur ses mailles. Le profil vertical
  de SGN sur fond plat (z depuis le fond) donne `u(z) = ū + (h²/6 − z²/2)·ū_xx` et `w(z) = −z·ū_x`. C'est la vitesse du bord ouvert,
  couche par couche, et celle des particules posées.

**Contrôles du plan** (ADR-266, ADR-267, ADR-268, ADR-273, ADR-276)

- **témoin** : le montage sans raccord de S697, par la même fonction (2,637 s, 9,988 m). Le raccord est la seule chose qui change
  (ADR-276 D2).
- **instrument** : le premier retournement, au raccord à 5,0 m. Ce que rendrait chaque hypothèse :
  - si la zone de colonnes était la cause, et que le bord par particules avec le profil de SGN transmet l'onde, le témoin à 0,02 s près ;
  - si le bord par particules garde une faute (le profil, la pose), un écart entre le témoin et 2,524 s.
- **calcul** : aucun nombre neuf. Le coût est celui de S693, ≈ 10 min ; le profil de SGN est calculé à chaque pas.
- **ADR** : ADR-271, ADR-273, ADR-275, ADR-276.
- **pièges** :
  - la masse : la 3D + le rivage + les réservoirs − la dette − ce qui est entré par la gauche + ce qui en est sorti ;
  - `ū_xx` par différences centrées sur SGN ;
  - les couches au-dessus de l'eau : la vitesse du bord y est sans effet (faces d'air).

**Critères, écrits avant.**

1. Au raccord à 5,0 m : le retournement à moins de **0,02 s** et **0,15 m** du témoin (2,637 s, 9,988 m).
2. L'air après lui, en avant ; la masse à 10⁻¹² ; la dette sous un quantum.

### Plan

- [x] **P1** — jeton ; plan.
- [x] **P2** — le bord gauche ; l'essai ; (1)–(2).
- [x] **P3** — preuve ; rituel.

### Notes de reprise
- **P2 fini** — la pose par rangée laissait un trou d'air (un faux retournement à 0,14 s) : posée face par face. (1) **échoue** : 2,590 s (−0,047 s ; colonnes −0,113 s) ; le témoin à 1,0 m −0,017 s (colonnes −0,055 s) ; (2) tenu.
