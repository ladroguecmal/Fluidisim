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

Session : S703 — **terminée**. En autonomie, sans arrêt. S702 : la pose par la grille, nourrie par la 3D (la vitesse des faces, `h`), est à
+0,021 s. **Elle est maintenant nourrie par SGN.**

**Ce que la session fait.**

- **`Large::GrilleSgn`.** Le bord à particules, avec les données de S698 : la vitesse du bord et le volume de chaque face, tirés du
  profil vertical de SGN. La pose est celle de S702, par la grille.
- **Le volume au plan, pendant l'enregistrement.** Deux volumes entrés, comparés :
  - celui que donnent les données de R4 : la vitesse des faces, la couche de surface au prorata de `h` ;
  - celui des particules réellement passées.

  Il dit si la couche de surface explique le retard de R4.

**Contrôles du plan** (ADR-266, ADR-267, ADR-268, ADR-273, ADR-276, ADR-277)

- **témoin** : R4 (S702, +0,021 s) pour la pose ; S698 (−0,047 s) pour les données. Le tout-3D (2,637 s), par la même fonction.
- **instrument** : le premier retournement de `GrilleSgn` ; le rapport des deux volumes. Ce que rendrait chaque hypothèse :
  - les vitesses de SGN en cause (S700 : −0,121 s à pose égale) : `GrilleSgn` nettement plus tôt que R4 ;
  - la pose seule en cause : `GrilleSgn` près de R4 ;
  - la couche de surface : un volume de R4 sous celui des particules passées.
- **calcul** : aucun nombre neuf ; ≈ 13 + 7 min.
- **ADR**, et comment chacun est tenu (ADR-277 D1) :
  - ADR-273 D1 : la pose a été jugée contre la 3D en S702 ; elle est nourrie par SGN ici ;
  - ADR-276 D2 : `GrilleSgn` diffère de S698 par la pose seule, et de R4 par les données seules ;
  - ADR-277 D2 : un mode nommé de plus.
- **pièges** :
  - le pas de chaque enregistrement est la durée entre deux fins de pas ;
  - le quantum est `dx³/8` (deux particules par axe) ;
  - au-dessus de `h`, la vitesse du bord est nulle.

**Critères, écrits avant.**

1. `GrilleSgn` à moins de **0,02 s** et **0,15 m** du tout-3D (2,637 s, 9,988 m).
2. Le rapport des volumes mesuré ; la masse à 10⁻¹², la dette sous un quantum.

### Plan

- [x] **P1** — jeton ; plan.
- [x] **P2** — `GrilleSgn` ; le volume au plan ; l'essai ; (1)–(2).
- [x] **P3** — preuve ; rituel.

### Notes de reprise
- **P2 fini** — (1) **échoue** : 2,569 s (−0,068 s ; les données de SGN −0,089 s à pose égale) ; (2) le rapport des volumes 0,986 ; 2,9·10⁻¹⁵, sous un quantum.
