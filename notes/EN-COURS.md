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

Session : S610 — **en cours**. En autonomie (ADR-247) : **9.6 — le précalcul avant l'impact : domaines, allocations, collisions, état
initial, avance plus rapide que le temps réel** (absent). ADR-013 §3 : en T2, δ vaut 0 — un domaine préparé ne porte que des blocs ; un
déplacement de l'impact prévu se corrige en translatant l'ensemble, sans erreur ; les seuls seuils sont réallouer, rebâtir, libérer. §4 :
l'avance temporelle n'est légitime que pour un domaine substitutif, qui naît faux et doit s'établir.

**Ce que la session fait.** Un module `precalcul.rs` : `Preparation` — les blocs (colonnes) d'un domaine préparé autour d'un impact prévu,
sur le réseau de blocs du référentiel, sa capacité réservée, son `dx`, δ = 0 ; `reviser(nouvelle prévision)` → `Decision` : **translater**
(un déplacement d'un nombre entier de blocs : ré-indexer, exact), **rebâtir l'ensemble** dans la capacité (un déplacement hors réseau),
**réallouer** (au-delà de la capacité), **rebâtir au nouveau `dx`**, **libérer** (l'événement n'aura pas lieu) ; `etablissement` — le temps
qu'un domaine substitutif né au repos met à rejoindre B (S609), mesuré, contre la borne d'ADR-013 §4 (`L/c_g` à `L/c_g + 2T`). Ne fait pas :
les collisions et proxys, l'avance mesurée en temps réel d'un domaine 3D, la graine qui remplace l'établissement (4.11, 12.3).

**Références, calculées avant** (ce script). **L'établissement** (le domaine de S609 né au repos, B de 0,1 m et 40 m : `T` = 9.030473 s,
`L/c` = 45.152364 s) : l'écart à B passe définitivement sous 5 % de l'amplitude à **60.65 s**, dans la borne
[45.152 ; 63.213] s ; l'écart à 120 s : 1.913472e-03 m. **La préparation** (blocs de 2 m, rayon 9 m, impact prévu en
(100,3 ; 50,7)) : **86 blocs** ; déplacé de (6 ; −4) m — trois blocs, moins deux — l'ensemble translaté est celui qu'on aurait
préparé là (asserté) ; déplacé de (0,3 ; 0,3) m, l'ensemble rebâti compte **89 blocs**.

**Quantum** : le pas de temps (0,05 s) ; des blocs entiers. **Critères, écrits avant.** (1) l'établissement à un pas près de la référence,
dans la borne d'ADR-013 §4 ; (2) la préparation : 86 blocs, δ = 0 au bit ; translatée de (6 ; −4) m, identique au bit à la préparation
directe, décision « translater » ; (3) déplacée de (0,3 ; 0,3) m : 89 blocs — « rebâtir » avec une capacité de 90, « réallouer » avec une
capacité de 86 ; un autre `dx` : « rebâtir au `dx` » ; l'événement annulé : « libérer » ; (4) refus : rayon
ou `dx` non positifs, une capacité sous l'ensemble initial.

### Plan

- [x] **P1** — jeton ; plan.
- [ ] **P2** — `precalcul.rs` et ses essais ; (1)–(4).
- [ ] **P3** — preuve ; liste 9.6 ; rituel.

### Notes de reprise
