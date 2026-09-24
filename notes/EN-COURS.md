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

Session : S345 — **en cours**. **Porte C, la cadence de δ** ; chemin de la v1. Porte en cours de §3 bis ; la série
S341–S343 a été interrompue par S344 (§6.4).
Agent : Claude Opus 5.5, application desktop ; fichiers, git, cargo, carte réelle, accès web.
Entrée — le pas coûte 3,68 ms (S343), dont 2,06 de projection, déjà bornée par la mémoire ; fusionner ses
réductions au bit obligerait chacun des 5 880 groupes à relire 5 880 partiels, plus cher que le gain (lu en S345).
**ADR-012 §7 a tranché l'architecture** : *« Tick de simulation fixe à 30 Hz, indépendant du taux d'images. Le rendu
interpole »*, δ avec au plus une image de retard. Un pas de 3,7 ms étalé sur deux images de 60 Hz contribue
≈ 1,85 ms par image — sous les 2 ms. **Condition physique, à éprouver d'abord** : un pas de 33,3 ms au lieu de 16,7.

Critères, écrits avant le code :
1. **La cuve de S305** (mode (1, 1), `nx` = 32, 64 cycles, 4,3 s — deux périodes) sur la carte à 1, 16,7 et
   33,3 ms : période et amplitude du mode. À 33,3 ms, la période s'écarte de celle à 1 ms de **moins de 1 %**,
   l'amplitude après deux périodes de **moins de 1 %** ; sinon l'écart est publié et la cadence n'est pas reçue.
2. **La scène de B** à 30 Hz contre 60 Hz, 12 s, avec témoins : aucune colonne hors bornes ; l'onde isolée (avec
   − témoin) — position de son maximum à **moins d'une maille** (25 cm), amplitude à **moins de 5 %** de celle à
   60 Hz, chaque seconde.
3. Si 1 et 2 tiennent : la cadence de 30 Hz est recevable pour la physique ; le découpage du pas sur deux images
   et l'interpolation du rendu sont la suite. Preuve (§8 de COUT-DELTA3D-S341), file.

### Plan

- [x] **P1** — jeton, plan seul.
- [x] **P2** — le banc de cadence : la cuve à trois pas de temps ; critère 1.
- [ ] **P3** — la scène de B à 30 et 60 Hz ; critère 2.
- [ ] **P4** — preuve, file ; critère 3.
- [ ] **P5** — rituel.

### Notes de reprise
- **P2, critère 1 tenu** (`--delta3d-cadence-cuve`, cuve de S305, `nx` 32, 64 cycles, deux périodes). Période /
  amplitude au dernier extrême : **1 ms** 2,150743 s (+0,376 % à la théorie, spatial) / 1,00005 ; **16,7 ms**
  2,150533 / 1,00002 ; **33,3 ms 2,149899 / 0,99987** — soit −0,039 % de période et −0,013 % d'amplitude contre
  1 ms, pour 1 % permis. Le pas de temps ne pèse presque pas sur ce mode.

