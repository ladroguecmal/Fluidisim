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

Session : S749 — **en cours**. En autonomie ; session longue. SURFACE-DILATEE-S748 : la projection dilate la couche de surface, parce qu'elle
n'y corrige que l'excès. **La question** : corrigée dans les deux sens vers sa densité attendue, la surface reste-t-elle à sa place, et la 3D
tient-elle les trois essais ?

**La densité attendue d'une maille de surface** : le noyau de la densité (le chapeau trilinéaire, d'une maille de demi-largeur) appliqué à
une eau uniforme sous une surface plane, située à `a = −φ/dx` mailles au-dessus du centre (φ lu à la maille) :
- `ρ*(a) = 0,5 + a − a²/2` pour `0 ≤ a ≤ 1` ;
- `0,5 + a + a²/2` pour `−1 ≤ a < 0` ;
- 1 au-dessus, 0 en dessous.

Au repos, la surface est sur la face haute de la dernière maille : `a = 0,5`, et `ρ* = 0,875`, la valeur exacte du réseau de pose (deux
particules par maille, aux quarts). **Corriger `ρ − ρ*` dans les deux sens ne bouge donc rien au repos**, et empêche la surface de se
dilater.

**La variante** (`DensityVariant::SurfaceTarget`, consciente du fond) : l'intérieur dans les deux sens vers 1, la surface dans les deux sens
vers `ρ*`. Le seuil de la lame mince (S747) n'est pas repris : une seule différence avec `Complete` (ADR-276 D2).

**Les critères, écrits avant** (ADR-288 D1 : le repos d'abord) :
1. **Le repos** sur l'escalier, 1:30 et 1:12 : la vitesse sous 1 cm/s, l'écart par les particules sous 3 mm ;
2. **l'onde solitaire** sur le canal à 2,5 cm : la largeur au-dessus de 80 %, le creux sous 10 % de `H` ;
3. **la remontée de S645** : à 10 % de la loi ;
4. **le niveau** : sur le canal, le niveau moyen de la surface loin derrière l'onde (de 0,5 à 2,5 m) reste à **1 mm** du repos, là où
   `Complete` le soulevait de 3 à 5 mm (S748).

**Les quanta** (ADR-288 D2) : la lecture du niveau par les particules vaut `dx/4` près (6 mm) ; le niveau moyen sur 2 m (80 colonnes) en
tire un plancher d'environ 1 mm. Le critère 4 est donc à la limite : il est rapporté, et ne fait pas échouer seul.

**Contrôles du plan** (ADR-236, ADR-276, ADR-287, ADR-288)

- **témoin** :
  - le repos exact ;
  - l'onde exacte ;
  - la loi de Synolakis ;
  - les variantes de S744 à S747 sur les mêmes essais.
- **instrument** : ceux de S743, S740 et S645 ; le niveau moyen, une lecture nouvelle sur le canal, éprouvée sur la variante `Complete` (elle
  doit montrer les 3 à 5 mm de S748).
- **calcul** : le repos (2 min), le canal (13 min), la remontée (7 min), en série.
- **ADR** : ADR-276 D2 ; ADR-287 D1 ; ADR-288 D1, D2.
- **pièges** :
  - φ lu à une maille dont le centre est presque à la surface : `a` voisin de 0, `ρ*` voisin de 0,5 ;
  - une surface en pente : la formule suppose une surface plane, horizontale ; elle est approchée.

### Plan

- [x] **P1** — jeton ; plan.
- [ ] **P2** — la variante ; (1), (3).
- [ ] **P3** — (2), (4) sur le canal.
- [ ] **P4** — preuve ; le lot ; fermeture.

### Notes de reprise
