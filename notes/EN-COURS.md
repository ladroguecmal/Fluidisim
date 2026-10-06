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

Session : S498 — **terminée**. En autonomie, **6.1, C11 — le petit objet léger** : la liste dit « Manquent W derrière la requête (fait
S494–S495), l'amortissement des autres degrés de liberté, C11 et B6 sur ses cinq archétypes ». ADR-008 §3 fixe trois régimes selon
`ω·dt` — normal (≤ 0,3), sous-cyclé (≤ 1, 2 à 4 sous-pas), **contraint** (> 1 : projeté sur la surface, orienté sur sa normale, vitesse
horizontale amortie vers l'orbitale) ; le corps rigide n'en a aucun.

**Ce que la session fait.** `RigidBody` : la raideur de flottaison mesurée sur le proxy (les points dans leur rampe), `ω`, le régime, un
pas qui l'applique ; le mode contraint (position d'équilibre trouvée une fois en eau calme, puis `z = η + c`, inclinaison de la surface,
vitesse relaxée vers celle de l'eau). B6 (le nombre de points par archétype) : S499.

**Ordre de grandeur, écrit avant.** Le pas symplectique est stable tant que `ω·dt < 2` : la balle de ping-pong à 30 Hz (`ω·dt` ≈ 2,25)
diverge — la valeur propre `1 − x²/2 − √((1 − x²/2)² − 1)` ≈ −2,7 par pas ; la caisse (0,33) et la barque (0,29) restent bornées, leur
`|G|` vaut 1 à l'arrondi (le pas symplectique conserve une énergie modifiée).

**Critères, écrits avant (C11 réécrit en S30).** (1) les régimes : `ω` du corps contre `√(ρgA/(m + m_a))` à 10⁻⁹, et la bascule aux
seuils 0,3 et 1 ; (2) en mode contraint, sur 120 s de houle de B et d'impacts de W à 30 Hz : `max|z − (η + c)| = 0` et l'axe du corps
sur la normale à 10⁻¹² rad ; (3) le témoin : la même balle en pas normal diverge (`|G|` > 1, prévu ≈ 2,7 par pas) ; (4) hors mode
contraint, `|G| ≤ 1 + 10⁻⁹` par période sur 120 s (caisse sous-cyclée, barque normale).

### Plan

- [x] **P1** — jeton, plan seul.
- [x] **P2** — les régimes et le mode contraint ; essais (1)–(4).
- [x] **P3** — preuve ; liste 6.1 ; rituel.

### Notes de reprise
- **P2** — `heave_stiffness`, `equilibrium_offset`, `floating`, `step_floating`. (1) tenu ; (2) 0 et ≤ 4,2·10⁻¹⁷ rad ; (3) 3,052 = prédit ;
  (4) 1 à 10⁻¹². Impasses : témoin à 1 mm sorti de l'eau au premier pas → 0,1 µm ; `|G|` par maxima paraboliques bruité à 6·10⁻⁴ (la
  barque à 1 + 1,6·10⁻⁹) → enveloppe ajustée à la pulsation exacte du pas ; seuil inchangé. 658 essais.
- **P3** — preuve PETIT-OBJET-S498 ; liste 6.1 ; index ; journal.
