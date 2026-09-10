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

---

## Session en cours

Session : S157 — en cours
Agent : Claude Code (Opus 5 ; fichiers, git et cargo disponibles)
Objectif : S156-1, A214. Remplacer **deux encadrements par une loi**. S156 a montré que le pas
radial borne la durée d'un sillage par périodicité, mais n'a mesuré que deux seuils — radial 128
décroche entre 15 et 20 s, radial 256 entre 45 et 50 s — et la formule candidate se trompe d'un
facteur 2,5 sur le second. Sans loi, pas de garde-fou : c'est exactement ce que dit A214.

### Plan

- [x] **P1** — état réel, jeton, plan déclaré et committé seul.
- [ ] **P2** — un critère **lisse** et sa vérification de monotonie. Le critère de S156 — premier
      rayon qui dépasse le seuil — n'était pas monotone en temps ; une dichotomie posée dessus
      donnerait un chiffre faux avec l'apparence d'un chiffre précis.
- [ ] **P3** — trois seuils par dichotomie, radial 64 / 128 / 256, angulaire fixé à 512.
      Trois points suffisent à trancher entre les deux exposants candidats.
- [ ] **P4** — dépendance à `sigma` : mêmes seuils à `sigma` 4 m, `cutoff` 1,5 pour garder le
      **même produit réduit** `sigma*cutoff = 6` et donc la même forme spectrale à échelle près.
- [ ] **P5** — décider : loi écrite si elle tient sur les deux familles, refus argumenté sinon.
      Garde-fou reçu seulement si la loi le mérite — A214 dit pourquoi un garde faux est pire.
- [ ] **P6** — livrable, rituel de fin, fusion `--ff-only`.

### Notes de reprise

Départ dc78f2e = master, trois copies coïncidentes, rien en attente.
298 tests/cinq ignorés, 107 ADR, 214 angles, 234 leçons, 18 invariants.

Ce que S156 laisse et qu'il ne faut pas refaire :
- `examples/wake_reach.rs` porte `profil(radial, angular, us)` et `rayon_honnete` ;
- mécanisme établi : période spatiale `2*pi*radial/cutoff`, soit 67 / 134 / 268 / 536 m ;
- le rayon est borné indépendamment par `angular`, proportionnellement — fixer `angular` à 512
  met cette seconde borne hors du chemin, et c'est pour cela qu'on la fixe ;
- plafond de la grammaire : `radial` et `angular` au plus 512. Donc `t_max(512)` restera
  **extrapolé**, jamais mesuré, et cela devra être écrit comme tel.

**Prédiction écrite avant la mesure.** Deux exposants sont en lice. Si la vitesse pertinente est
celle du plus petit nœud du maillage, `t_max` croît comme `sqrt(radial)` — c'est la formule de
S156, qui se trompe d'un facteur 2,5 sur 256. Si elle est fixée par l'échelle où vit l'énergie,
donc indépendante du maillage, `t_max` croît comme `radial`. Les deux seuils connus donnent un
rapport de 2,7 pour un doublement : **plus proche de la seconde**, et même au-delà. Le troisième
point tranchera, et s'il donne un rapport franchement différent de 2,7 c'est qu'aucune loi de
puissance simple ne décrit le phénomène — ce serait le résultat le plus utile, parce qu'il
interdirait le garde-fou pour de bon.

Piège de méthode déjà payé en S156 : un critère qui compare `radial` à `2*radial` mesure la
défaillance **de la plus grossière des deux**, à condition que la plus fine soit encore dans son
domaine. Pour radial 256 contre 512 à 50 s, cette condition n'est pas vérifiable. Le dire.
