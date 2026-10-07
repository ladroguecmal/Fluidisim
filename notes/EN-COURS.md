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

Session : S618 — **terminée**. En autonomie (ADR-247) : **11.5 — le matériel cible de livraison et la seconde cible** (B7 complet, A98 ;
absent). ADR-219 D2 : ce PC est la cible ; la seconde cible est le bridage de 9.10. Cette session mesure, sur ce PC, le coût des modules
construits pour la v2 et en dérive les capacités (I-16, `qualite::Capacites`), pour la cible et pour la seconde cible.

**Ce que la session fait.** Un exemple `b7_cible.rs` : le coût, en ns, d'un pas de maille de `SaintVenant2D` (200²), d'un pas de maille de
`Domaine1D` (400), d'un échantillon de `TrainW1D`, d'un échantillon de `tsunami::niveau`, d'une image du `Regulateur` ; chaque coût est la
**médiane de cinq répétitions** d'au moins 0,2 s, avec leur étalement (max/min) ; puis les capacités par tick — mailles de δ, échantillons de
W — pour un budget de **2.0 ms** (ADR-012 : `cpu_sim_ms`) et pour la seconde cible bridée (÷ 3 : **0.6667 ms**), et le
côté du domaine 2D carré qui tient dans chacun. Ne fait pas : A98 (le sinus déterministe : `phase.rs` le traite ; sa conformité entre
plateformes demande une seconde plateforme), le GPU bridé (WARP), la scène représentative entière.

**Quantum** : la nanoseconde ; la mesure est bruitée. **Critères, écrits avant.** (1) le banc rend les cinq coûts, finis et positifs ; un
étalement au-delà de 1,5 marque la mesure « instable » sans l'écarter ; (2) les capacités suivent I-16 — `⌊budget / coût⌋` — et celles de la
seconde cible sont celles d'un budget divisé par 3 ; (3) le rapport inscrit, pour chaque module, les deux capacités ; aucun seuil de
performance n'est posé avant la mesure (ADR-012 §8 : les valeurs se mesurent sur la cible).

### Plan

- [x] **P1** — jeton ; plan.
- [x] **P2** — `b7_cible.rs`, sa mesure ; (1)–(3).
- [x] **P3** — preuve ; liste 11.5 ; rituel.

### Notes de reprise
- **P2 fini** — cinq coûts, étalements 1,05–1,24 ; capacités inscrites. **Défaut relevé** : `SaintVenant2D::pas` alloue à chaque pas
  (I-06) — point de file : préallouer ses tableaux de travail, re-mesurer.
- **P3** — preuve B7-CIBLE-S618 ; liste 11.5 (absent → partiel) et décompte ; index ; journal.
