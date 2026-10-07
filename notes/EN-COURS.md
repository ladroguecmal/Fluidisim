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

Session : S600 — **terminée**. En autonomie (ADR-247), **9.13 — le dépassement critique temporaire sans retard global perceptible**
(absent) : ADR-012 §6 — « une réserve d'événement : +50 % pendant 0,5 s au plus, un rechargement de 5 s, pour le seul domaine dont
`W_gameplay` est maximal ; sans rechargement, la réserve devient le budget nominal ».

**Ce que la session fait.** `ReserveEvenement` (dans l'ordonnanceur) : par tick de simulation (30 Hz, ADR-012 §7), `budget(nominal_ms,
critique, demande)` rend le budget accordé — le nominal ×1,5 si le domaine est **critique** (son `W_gameplay` est le maximum) et
**demande** la réserve, tant qu'il reste des ticks de réserve ; épuisée, la réserve est **verrouillée** 150 ticks (5 s) puis pleine ; un domaine
non critique ne la touche jamais. Le dépassement le plus fort d'une image est donc borné à +50 % du budget de l'eau, et sa durée à 0,5 s.
Ne fait pas : la recharge partielle d'une réserve entamée sans être épuisée (elle reste entamée jusqu'à épuisement — le choix le plus
prudent), le branchement à `Scheduler::allocate`, la mesure sur le banc B7.

**Références, calculées avant par ce script** (sa propre machine d'état). Une demande critique continue pendant 60 s : **165 ticks
renforcés sur 1 800** (9.17 %), soit un dépassement moyen de **4.58 %** du budget. Un événement de 2 s, 10 s
de calme, un second de 2 s : **15** puis **15** ticks renforcés (la réserve rechargée entre les deux).

**Quantum** : le tick (des comptes entiers, exacts). **Critères, écrits avant.** (1) le compte sous demande continue, exact ; (2) les deux
événements, exacts ; (3) un domaine non critique : aucun tick renforcé ; jamais plus de 15 ticks consécutifs renforcés ; le budget accordé
jamais au-dessus de 1,5 × le nominal ; (4) refus : un nominal non positif.

### Plan

- [x] **P1** — jeton ; plan.
- [x] **P2** — `ReserveEvenement` et ses essais ; (1)–(4).
- [x] **P3** — preuve ; liste 9.13 ; rituel.

### Notes de reprise
- **P2 fini** — 165 ; 15 et 15 ; les bornes ; refus ; un avertissement de compilation (une constante empruntée en mutable) corrigé. Suite 776.
- **P3** — preuve RESERVE-EVENEMENT-S600 ; liste 9.13 (absent → partiel) et décompte ; index ; journal.

