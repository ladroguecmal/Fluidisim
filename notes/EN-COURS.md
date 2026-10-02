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

Session : S440 — **en cours**. Demande de l'utilisateur (2026-10-02) : *« Continue sinon j'accepte l'écart »* — **C7d-3b reçu**,
l'écart de 0,01 % accepté ([preuve](../docs/validation/APIC-CARTE-S416.md) §22.10) ; puis la suite de la campagne.

**Ce que la session trouve en entrant.** C7d-3a est bloqué (A320, cause non trouvée) ; C7d-3c et C7d-3d l'attendent. **A322** (à 10 cm,
30 Hz explose vers 62 s, la mer seule) précède toute scène à 10 cm (C10). Son attribution (S409, MULTIGRILLE-3D-S385 §6.4) : retirer
**le résidu de quantité de mouvement du fond** suffit à la supprimer — **ce que fait le mode relatif**, désormais sur la carte.

**Ce que la session fait.** Le banc d'A321 (`--delta3d-a321`) gagne `RELATIF=1` (`Step3::set_relative`). La scène d'A322 :
`MAILLE=0.1 EMPRISE=80,80 MULTIGRILLE=1 CYCLES=8`, 30 Hz : le témoin (le pas de S297) rejoué, puis le mode relatif.

**Critères, écrits avant.** **A322 levée sous le mode relatif** si : (1) le témoin explose encore (vers 62 s) ; (2) en mode relatif, la
scène tient **120 s** à 30 Hz, `max_u` de δ sous 3 m/s à chaque seconde et la part de l'échelle de la maille de `w` sous 0,05 ;
(3) la divergence et le résidu de la projection restent dans leur ordre de grandeur du témoin avant son explosion. Autrement : A322
reste ouverte, avec ce qui a été mesuré. A322 étant levée, elle ne l'est que **dans le mode relatif** : la production par défaut garde
le pas de S297 tant que C7d-3a n'est pas reçu.

### Plan

- [x] **P1** — jeton, plan seul ; l'arbitrage inscrit.
- [x] **P2** — `RELATIF` au banc d'A321 ; le témoin ; le mode relatif ; critères.
- [ ] **P3** — preuve ; A322 ; registres ; rituel.

### Notes de reprise
- **P2** — `--delta3d-a321` : `RELATIF=1`. `MAILLE=0.1 EMPRISE=80,80 MULTIGRILLE=1 CYCLES=8`, 30 Hz. **Témoin** : explose au pas
  **1 860** (62 s), comme en S409 — (1) tenu ; avant, `max_u` ≤ 1,92 m/s, part de maille de `w` > 0,05 trois secondes sur 60 (0,115
  au plus). **Mode relatif** : **tient 120 s** (3 600 pas) ; `max_u` ≤ **1,25 m/s** ; divergence ≤ 1,4·10⁻³, résidu ≤ 6,8·10⁻⁵ (le
  témoin : 2,3·10⁻³ et 8,8·10⁻⁵) — (3) tenu ; mais **la part de l'échelle de la maille de `w` dépasse 0,05 pendant 65 secondes sur
  120** (jusqu'à 0,62 à 67 s, `max_u` 0,61 m/s sur une face `w` près de la surface) — (2) **manqué**. Des bouffées transitoires à la
  surface, qui retombent en quelques secondes. **A322 non levée telle qu'écrite** : l'explosion disparaît sous le mode relatif ;
  apparaissent des bouffées à l'échelle de la maille, à 10 cm, que le pas de S297 n'a pas avant d'exploser.

