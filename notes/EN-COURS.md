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

Session : S346 — **en cours**. **Porte C, attribuer A318** ; chemin de la v1.
Agent : Claude Opus 5.5, application desktop ; fichiers, git, cargo, carte réelle, accès web.
Entrée — S345 ([preuve](../docs/validation/COUT-DELTA3D-S341.md) §8) : la cadence de 30 Hz d'ADR-012 §7 tient sur la
cuve (0,04 %), pas sur la scène de B : l'onde isolée y garde jusqu'à 6,3 % d'amplitude de plus qu'à 60 Hz, cinq fois
le témoin des cycles ; ni la projection ni l'éponge. **A318**, sévérité 2 : il bloque l'adoption de la cadence.

**Ce que la session doit rendre possible.** Savoir d'où vient l'écart, pour le corriger ou le faire juger. Deux
sources possibles : le **couplage à la mer** — le fond de B, lu au début du pas, force δ sur tout le pas (ordre un en
temps) — ou la **dynamique propre** de l'onde — son advection par elle-même, les bascules de mouillure (A297).

Critères, écrits avant le code :
1. **Le front sur une mer au repos** (fond de B d'amplitude nulle), 30 contre 60 Hz, 12 s : amplitude de l'onde
   chaque seconde. Si l'écart tombe au niveau du témoin des cycles (≤ 1,3 %), le couplage à la mer est en cause ;
   s'il reste de l'ordre de S345, c'est la dynamique propre.
2. **Une troisième cadence**, 40 Hz (25 ms), sur la scène de B : l'écart suit-il le pas — ordre un — ou non.
3. Preuve (§9), file, A318 : attribué, ou ce qui reste à éprouver ; la suite de la porte C en dépend.

### Plan

- [x] **P1** — jeton, plan seul.
- [x] **P2** — le front sur une mer au repos, aux deux cadences ; critère 1.
- [x] **P3** — la scène de B à 40 Hz ; critère 2.
- [x] **P4** — preuve, file, A318 ; critère 3.
- [ ] **P5** — rituel.

### Notes de reprise
- **P2, critère 1 : partagé.** `MER=repos … --delta3d-cadence-scene` (fond de B d'amplitude nulle) : 30 contre 60 Hz,
  amplitude **+1,0 à +3,9 %** (1,72 ; 1,89 ; 1,31 ; 1,60 ; 1,03 ; 3,11 ; 1,32 ; **3,87** ; 0,30 ; −0,70 ; 0,47 ; 0,16),
  presque toujours positive ; avec la mer, 6,3 % ; témoin des cycles, 1,3 %. **La dynamique propre de l'onde dépend
  du pas** — moins d'amortissement quand les pas sont moins nombreux —, **et la mer en rajoute**.
- **P3, critère 2.** `CADENCES=16667:32,25000:32` : 40 contre 60 Hz, amplitude **≤ 0,73 % jusqu'à 5 s** (0,46 ; 0,31 ;
  −0,03 ; −0,73 ; 0,42) — le niveau du témoin des cycles —, puis −3,28 (6 s), +5,64 (8 s), **+9,18 % (12 s)**, plus qu'à
  30 Hz au même instant (2,2). **Deux régimes** : tant que l'onde est groupée (≤ 5 s), l'écart croît avec le pas — 40 Hz
  au niveau du témoin, 30 Hz jusqu'à 5,7 % ; une fois dispersée, elle diverge sans ordre, comme toute scène au-delà
  de l'horizon d'A297 (S298). À 40 Hz, un pas par image et demie : ≈ 2,45 ms par image, au-dessus des 2 ms.

