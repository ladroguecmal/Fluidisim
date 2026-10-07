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

Session : S602 — **terminée**. En autonomie (ADR-247) : **le lot** (dû ; feuille de route S599–S601), puis **9.4 — les objets contrôlables :
paliers de confiance ; confiance réduite par le jeu** (absent). Les paliers d'ADR-013 §2 existent (`ballistic::tier`, S405, pour 9.3) ;
manque ce qui est propre aux objets contrôlables.

**Ce que la session fait.** Dans `ballistic.rs` : `Confiance { facteur_jeu }` — le jeu réduit la confiance en multipliant la capacité de
manœuvre (`a_max` effectif = `a_max·facteur`, `facteur ≥ 1` : un pilote erratique, une perte de contrôle annoncée) ; `horizon_utile(a_max, R,
confiance)` = `√(2R/a_max_effectif)` ; `palier_controlable(t, a_max, R, confiance)` — le palier d'ADR-013 sous la confiance réduite ;
`reevaluation_s(palier)` — T4 réévalué à 2 Hz (0,5 s), les autres à chaque tick. Ne fait pas : la source des facteurs (le jeu), la table des
`a_max` par archétype (ADR-013 §8.2 : l'équipe véhicules), l'hystérésis entre paliers.

**Références, calculées avant** (ce script les écrit et vérifie la table d'ADR-013 au dixième). Horizons : avion de chasse **1.414214 s**
(1,4), avion en perte de contrôle **3.651484 s** (3,7), vaisseau lourd **4.898979 s** (4,9). Le vaisseau lourd à 4 s de
l'impact : l'enveloppe 40 m ≤ 60 m → **T2** ; le jeu divise la confiance par deux (facteur 2) : 80 m > 60 m → **T3**, l'horizon
tombé à **3.464102 s**.

**Quantum** : f64 ; des paliers (des valeurs discrètes). **Critères, écrits avant.** (1) les trois horizons à 10⁻¹² ; (2) les deux paliers ;
un facteur 1 rend le palier de `tier` au bit, sur une grille de 200 cas ; (3) la réévaluation : 0,5 s en T4, le tick ailleurs ; (4) refus :
un facteur sous 1 ou non fini.

### Plan

- [x] **P1** — jeton ; le lot ; plan.
- [x] **P2** — `Confiance` et ses essais ; (1)–(4).
- [x] **P3** — preuve ; liste 9.4 ; rituel (`--lot`).

### Notes de reprise
- **P2 fini** — les horizons ; T2 → T3 ; 200 cas au bit ; la réévaluation ; refus. Suite 777.
- **P3** — preuve CONFIANCE-S602 ; liste 9.4 (absent → partiel) et décompte ; index ; journal ; le lot.

