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

Session : S292 — en cours
Agent : Claude Opus 5, application desktop Claude Code ; fichiers, git, cargo, outils locaux.
Entrée : demande de l'utilisateur — régler le gel de la carte refroidie. C'est A294.
Objectif : que le **pire** pas tienne, y compris après une inactivité de la carte. Un pic de
467 ms annule le gain de médiane de milliers de pas.

### Plan

- [x] **P1** — état réel, jeton et plan seuls.
- [ ] **P2** — **où tombe le gel** : décomposer le premier appel après inactivité en ses cinq
  postes déjà instrumentés (empaquetage / encodage / soumission / attente / lecture). Cela
  distingue le pilote, la carte et notre code, et interdit de deviner.
- [ ] **P3** — **caractériser** : le coût du premier appel en fonction de la durée d'inactivité
  (0 à plusieurs secondes), et le nombre d'appels avant retour au régime. Publier la courbe,
  pas un seul point.
- [ ] **P4** — **discriminer le mécanisme** par des essais qui s'excluent : un envoi *trivial*
  réchauffe-t-il autant qu'un cycle complet ? le gel dépend-il de la taille des tampons
  (résidence mémoire) ou non (état d'alimentation) ? une simple lecture d'horodatage suffit-elle ?
- [ ] **P5** — **construire l'entretien** que la mesure désigne, au bon endroit : le solveur
  expose le geste, l'hôte décide de la cadence. Mesurer son coût propre et ses allocations.
- [ ] **P6** — **éprouver sur le chemin réel** : pas couplé de la bande δ avec des coupures
  franches, pire pas mesuré avec et sans entretien, trajectoire inchangée au bit.
- [ ] **P7** — rituel §6 : preuve, journal, registres/index/feuille, jeton libre.

### Notes de reprise

Ce que S291 a mesuré, et qui est le point de départ ([preuve](../docs/validation/PAS-DECOMPOSE-S291.md) §7) :
un pas à **467 ms**, puis **132 ms** à la reproduction, toujours au premier appel venant après
≈ 7 s sans GPU. Un envoi de préchauffage **au démarrage** ne le supprime pas. Ne pas laisser la
carte inactive le supprime : maximum retombé à 26,12 ms. Donc **chemin refroidi, pas chemin neuf**.

**Le piège de cette session est de bâtir l'entretien avant de savoir ce qui refroidit.** Trois
mécanismes possibles, et ils ne demandent pas le même geste :
1. **état d'alimentation** de la carte — elle redescend en fréquence, et le premier envoi paie la
   remontée. Un envoi trivial suffirait à l'empêcher, et le coût serait indépendant de la taille.
2. **résidence mémoire** — les tampons sortent de la VRAM et le premier envoi paie leur retour.
   Le coût dépendrait alors de la taille des tampons, et un envoi trivial ne protégerait rien.
3. **caches internes de la pile** (allocateurs de commandes, ceinture de transfert) — le coût
   serait dans l'encodage ou la soumission, pas dans l'attente.
P2 et P4 existent pour trancher entre les trois, et le décompte des cinq postes le dit presque
seul : dans l'**attente**, c'est la carte ou le pilote ; dans la **soumission**, le pilote ;
dans l'**encodage**, wgpu.

Ce qui reste hors de ce lot : budget 2 ms, longueur de cycle calibrée, multiplateforme, 3D,
solides, boucle d'image complète. Et **aucune porte d'acceptation ne bouge** : ADR-143/144
restent le garde-fou qui rend toute mesure d'identité vérifiable au bit.
