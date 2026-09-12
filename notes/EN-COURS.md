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

Session : S194 — en cours
Agent : Claude Code (Opus 5 ; fichiers, git et cargo 1.97.0 disponibles)
Objectif : S193-1, mesurer le **couplage de deux trains** sur le véhicule non linéaire
dispersif de S193 — écart entre la **somme des évolutions** et l'**évolution de la
somme** — sous le critère d'ADR-120. C'est la mesure qu'ADR-112 attend et qu'A217
nomme ; elle n'était pas possible avant S193. Aucun choix δ, aucun seuil de bascule.

### Plan

- [x] **P1** — reprise, état réel (quatre copies au même commit), jeton et plan seuls.
- [ ] **P2** — dérivation et protocole **avant tout code** : décomposition des termes
  croisés, deux régimes prédits (harmoniques liées croisées, non cumulatives, pente 1 ;
  modulation croisée de fréquence, cumulative en temps, pente 2), exigence de bande
  pour contenir k₁±k₂ et leurs harmoniques, couples résonants et non résonants,
  normalisation, réceptions chiffrées et contre-épreuves déclarées.
- [ ] **P3a** — banc de couplage `nl_coupling_2d.rs` réutilisant `support/nl_surface.rs`
  sans le modifier ; tests propres dont **train unique** et **M=1** à écart nul, et
  couple à modes disjoints exact au bit.
- [ ] **P3b** — campagne : échelle d'amplitude, partage d'amplitude, croissance en
  temps, échelle M, couples résonant/non résonant, deux profondeurs ; ajustement
  `écart = α·s + β·s²·N` ; frontière des 2 % en (cambrure × durée) ; reproductibilité.
- [ ] **P4** — documenter, propager A217/A216/A50/B4 et la file ; ADR seulement si une
  décision de projet est prise ; ne **pas** dériver de seuil de bascule W/δ.
- [ ] **P5** — rituel de fin (§6) : journal, angles, leçons, index/README/décomptes,
  jeton `libre`, copies avancées sans suppression non prouvée.

### Notes de reprise

Hérité de S193 : véhicule `NlSurface` (bande spectrale Q, convolution tronquée,
relèvement de S192 à symbole horizontal exact, ordres M=1/2/3, RK4), reçu contre
Stokes — `b₂` à 0,4555 %, décalage de fréquence à 1,6454 %, empreinte
`0x41fc3b13793bee10`. ADR-122 retient **M=3**. Seuil 2 % d'ADR-120 fixé, jamais
redemandé. ADR-112 : une superposition indépendante ne reçoit pas le couplage.

Thèse déclarée avant mesure : l'écart de superposition n'est **pas un nombre** mais un
domaine en (cambrure × durée), parce qu'il a deux parts de natures différentes — une
part instantanée non cumulative d'ordre un en cambrure, et une part de phase cumulative
d'ordre deux multipliée par le nombre de périodes observées. Le détail est écrit en P2
avant tout code.

Leçons de S193 à appliquer ici : **L271** — déclarer au moins une configuration où
l'effet est nul par construction (ici train unique, et M=1) ; **L272** — toute ligne de
base doit être la grandeur que le véhicule porte, pas celle du continu ; **L273** — une
prédiction d'ordre se mesure avant d'être commentée.
