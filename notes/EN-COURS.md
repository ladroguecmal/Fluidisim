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

Session : S577 — **terminée**. En autonomie, **2.2 — la marée** (« manquent la marée, le niveau moyen variable… ») ; elle sert aussi 7.7
(la prévision des gués, SPEC-006 §5.4 : « la marée, analytique, fiable à l'horizon publié »).

**Ce que la session fait.** `maree.rs` : une marée **harmonique**, `η(t) = Z₀ + Σ Aₖ·cos(ωₖ·t − gₖ)`, au plus huit composantes (M2, S2, N2,
K2, K1, O1, P1, Q1 : leurs périodes sont des faits astronomiques), chaque phase **entière** par `PhaseQ32::from_time` (I-03 : identique sur
toute plateforme) ; les fréquences converties une fois (`freq_hz_to_q32`). L'amplitude et la phase de chaque composante sont celles du
lieu (une carte cotidale viendra avec les régions, 11.2) ; le niveau moyen `Z₀` est un paramètre.

**Références, calculées avant** (ce script les écrit). L'arrondi des fréquences en Q32 : au pire **1.0e-05** relatif (M2 :
1.6e-06) — la phase de M2 dérive de **4.8e-05 tour en 15 jours**, 1.2e-03 tour en un an (0.9 min) :
déterministe, et sous la précision d'une table de marée. Vives-eaux et mortes-eaux, M2 (1 m) + S2 (0,46 m) : battement de
**14.765 jours** ; sur 30 jours, η entre **-1.4600 et 1.4600 m** (f64 idéal, pas de 60 s) ; marnages 2,92 et 1,08 m.
Un gué (fond à −0,5 m) sous M2 + S2, depuis la pleine mer de vives-eaux (`t` = 0, 1,96 m d'eau) : il repasse sous la nage (1,30 m) dans
**6974.05 s**.

**Quantum** (ADR-236 D1) : η en f32 (10⁻⁷ m sur 1,5 m) ; la dérive de phase ci-dessus (M2 à 15 jours : 3.0e-04 m sur 1 m
d'amplitude). **Critères, écrits avant.** (1) M2 seule contre `cos(2πt/T)` idéal à 10⁻³ m sur 15 jours (rapport 3
à la dérive) ; (2) M2 + S2 : le maximum et le minimum sur 30 jours à 10⁻³ m de 1.4600 / -1.4600 ; (3) le gué à 1 s de 6974.05 s
par `prochain_franchissement` ; (4) au bit à `t` donné, deux évaluations ; à un an, η fini et borné par `Σ Aₖ` ; (5) refus : plus de huit
composantes, une période non positive.

### Plan

- [x] **P1** — jeton ; plan.
- [x] **P2** — `maree.rs` et ses essais ; (1)–(5).
- [x] **P3** — preuve ; listes 2.2, 7.7 ; rituel.

### Notes de reprise
- **P2 fini** — M2 à 2,96·10⁻⁴ m de l'idéal (la dérive prédite) ; M2 + S2 de −1,4600 à 1,4600 m ; le gué à 6 974,03 s ; au bit ; refus.
  Suite 752.
- **P3** — preuve MAREE-S577 ; listes 2.2, 7.7 ; index ; journal.

