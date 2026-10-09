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

Session : S747 — **en cours**. En autonomie ; session longue. SANS-SURFACE-S745 : corriger la surface (`Complete`) garde l'onde mais freine la
lame ; ne pas la corriger (`WithoutSurface`) fait l'inverse. **La question** : une projection hybride tient-elle les trois essais du banc ?

**La variante** (`DensityVariant::Hybrid`, consciente du fond) :
- comme `Complete`, partout où la colonne porte au moins **trois mailles d'eau** (la règle du film du rivage, S678) ;
- **aucune correction** dans une colonne qui en porte moins : la lame mince, le jet de rive.

Le compte se fait sur les étiquettes du pas (les mailles d'eau de la colonne).

**Les essais, et leurs critères écrits avant** (les mêmes que S745, ADR-288 D1 : le repos d'abord) :
1. **Le repos sur l'escalier**, 1:30 et 1:12 : la vitesse sous 1 cm/s, l'écart par les particules sous 3 mm ;
2. **la remontée de S645** : à **10 %** de la loi (`Complete` −11 % ; `WithoutSurface` −5,5 %) ;
3. **l'onde solitaire sur le canal à 2,5 cm** : la largeur au-dessus de **80 %**, le creux sous **10 % de `H`** (`Complete` 9 mm ;
   `WithoutSurface` 40,5 mm).

**Les quanta** (ADR-288 D2) : le seuil de trois mailles vaut 7,5 cm d'eau. La lame de S645 en a moins, le canal (0,5 m) beaucoup plus.

**Contrôles du plan** (ADR-276, ADR-287, ADR-288)

- **témoin** : les deux variantes de S744 et S745, sur les mêmes essais ; le repos exact ; la loi ; l'onde exacte.
- **instrument** : ceux de S743, S645 et S740.
- **calcul** : le repos (2 min), la remontée (8 min), le canal (13 min), en série.
- **ADR** : ADR-276 D2 (une différence : le seuil de la lame mince) ; ADR-287 D1 ; ADR-288 D1, D2.
- **pièges** :
  - une colonne qui passe le seuil pendant le pas change de traitement : c'est le comportement voulu ;
  - le juge du déferlement, plus tard, dira si la variante garde le plongeon (S709).

### Plan

- [x] **P1** — jeton ; plan.
- [ ] **P2** — la variante ; (1), (2), (3).
- [ ] **P3** — preuve ; fermeture.

### Notes de reprise
