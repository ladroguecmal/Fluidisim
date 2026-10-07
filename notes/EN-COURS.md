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

Session : S638 — **terminée**. En autonomie (ADR-247 : la physique des partiels). **Le lot** (dû ; feuille de route S635–S637), puis **5.10 — le
cycle de vie de δ attaché à V** (un manque de S637 : « la destruction de δ quand V se calme »).

**Ce que la session fait.** `articulation::Vie` : à chaque pas de V, le changement de surface contre le dernier état publié (S564) — significatif :
publier, remettre le calme à zéro, et faire **naître** δ s'il n'existe pas ; sinon, compter le calme, et faire **mourir** δ après `calme_requis`
pas (l'hystérésis d'ADR-022 §2.6 : une fenêtre au moins égale au retour d'équilibre, ici 3 τ). Ne fait pas : le coût de restauration
d'un δ substitutif dans la fenêtre (ADR-022), plusieurs δ sur un nœud.

**Références, calculées avant** (ce script). La piscine de S637, le robinet ouvert 50 pas (5 s) puis fermé, le seuil de 500 µm, le calme requis
**30 pas** (3 s ≥ 3 τ). Publications aux pas **[13, 26, 39]** ; δ naît au pas **13** et meurt au pas **69** ; son retard de
masse à la mort : **2.382232398e-03 m³**. **Bornes du montage** (ADR-257 D1, assertées) : la mort tient dans la fenêtre de 200 pas ; le calme
couvre trois τ.

**Quantum** : le pas de V. **Critères, écrits avant.** (1) les publications [13, 26, 39], une naissance au pas 13, une mort au pas
69 ; (2) le retard de masse de δ à sa mort égal à la référence à 10⁻⁹ m³ ; (3) refus : un calme requis nul.

### Plan

- [x] **P1** — jeton ; le lot ; plan.
- [x] **P2** — `Vie` et son essai ; (1)–(3).
- [x] **P3** — preuve ; liste 5.10 ; rituel (`--lot`).

### Notes de reprise
- **P2 fini** — (1)–(3) tenus du premier essai. Suite : 822 essais listés.
- **P3** — preuve VIE-DELTA-S638 ; ligne 5.10 ; index ; journal ; le lot.
