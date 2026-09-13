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

---

## Session en cours

Session : S221 — en cours
Agent : Claude Code, Opus 5 (fichiers, git et cargo disponibles)
Entrée : « Continue » juste après S220, même conversation ; master et trois copies à 32e7afd,
jeton libre, maillons 0. Lectures S220 conservées, état réel revérifié. Copie principale.

### Thèse et plan

A260 : un mode à grande largeur de phase paie `2 c_k` (exclu) ou `D²/2` presque autant (inclu),
**mode par mode**. Toute partition des modes en `R ∪ U` donne `|S(p)| ≤ |S_R(p)| + |S_U(p)|`,
et `|S_U(p)| ≤ G(U)`, l'enveloppe directionnelle ADR-134 du seul sous-ensemble, valable en tout
point. Famille de coupures `U_j = {D_k ≥ D*_j}`, `D* ∈ {2, 1, 1/2}`, accumulée par classes
dans la même passe O(N) ; minimum avec ADR-135 et ADR-136, qui restent **au bit**. Pour la coupure
`D* = 2`, gain garanti non négatif sur les exclus à `2 c_k` : `G(U) + |S_U(c)| ≤ 2 C_U`.

**Limite annoncée avant mesure** : `G(U)` ne dépend pas du point. Une grande cellule loin du
sillage garde `G(U)` si `U` porte l'essentiel de la masse — l'enveloppe ne voit pas la
localisation spatiale du paquet, qui vit dans la cohérence des phases. Hypothèse testée, sans
chiffre : gain à 8191 évaluations seulement si la masse `C_U` de `D ≥ 2` porte la majorité
de `C` sur les feuilles 2 × 1,5 m ; aucun gain attendu à 2047. Critères : sondes sous la borne,
domination au bit sur ADR-136, gain strict sur un rectangle à spectre étalé, S220 inchangée au
bit, décomposition `C_U`/`G(U)` publiée par classe.

- [x] **P1** — jeton et plan seuls.
- [x] **P2** — ADR-137 : coupure spectrale, famille de seuils, domination, réserve, limite spatiale.
- [x] **P3** — construire la passe générique (ordre deux inchangé au bit), ordre `Spectral`, Prepared.
- [x] **P4** — tests : couverture, domination, gain strict, identité S220, refus, partition.
- [x] **P5** — exemple S221 : décomposition par taille de maille, partitions 2047→32767 ; campagne isolée.
- [x] **P6** — publier la réception S221 et ses relevés bruts.
- [>] **P7** — rituel §6, journal, registres, index, file active, jeton et copies.

### Notes de reprise

Suite S220 : ordre deux à 1,006–1,012 × maximum à 32767 évaluations ; plateau à 8191 (A260).
Campagne détachée : signaler la fin par un fichier marqueur, **pas** par `tail -f` d'un journal
écrit par un autre processus (verrou Windows constaté S220).

P3 : bits de l'ordre deux figés **avant** refactorisation (test `second_order_bits_frozen_before_spectral_s221`, 6 modes dont deux courts, 5 rectangles), puis passe générique `second_order_pass::<SPECTRAL>` ; classes accumulées derrière la constante. Module `pressure_spectral_bound.rs` : trois coupures D* = 2/1/½, `SlopeOrder::Spectral`, exposition Prepared. 11 tests de borne debug réussis, bits figés compris.

P4 : quatre tests S221 (bits figés, couverture/domination sur 14 modes dont 8 courts à k = 9, gain strict un long + huit courts isotropes — borne < 0,85 × globale, 160 801 sondes —, refus et partition Spectral) + contrôles contexte/instant/domaine de Prepared pour l'ordre deux et la coupure. Une attente fausse corrigée : la masse d'une classe unique arrondie vers le haut est un ulp au-dessus (encadrement, pas égalité). Release : 370 réussis (272+4+1+93), 5 ignorés, zéro échec.

P5 : exemple `coupure_spectrale_s221` — grilles 4×3, 2×1,5 et 1×0,75 m (les partitions uniformes de 1024/4096/16384 feuilles) avec fractions de masse par classe, branche gagnante et détail du pire rectangle ; partitions Second puis Spectral à 2047/8191/16383/32767 avec détail de feuille maximale hors chronométrage. Campagne détachée lancée 16:42:33 (`scratchpad/campagne_s221.ps1`, sorties `scratchpad/s221`) : base1, lent, long, base_tard, base2 ; fin signalée par `fin_*.txt` et `TERMINE.txt`.

P5 fin : campagne 16:42:33–16:52:42, cinq processus, base1/base2 identiques au bit. **Hypothèse du plan réfutée sur son mécanisme** : à 2×1,5 m, la classe D ≥ 2 ne porte que 1,1–2,2 % de la masse (lent/base/long) ; la classe [1, 2) en porte 44–67 %. La coupure utile est **D* = 1** (gagne sur les feuilles maximales). Partition Spectral contre Second : 2047 → gain 1,003–1,024 (quasi nul) ; **8191 → 0,1020/0,02654/0,1205/0,1031 contre plafond** (gain 1,115–1,230) ; 16383 → 0,0919/0,02083/0,1107/0,09139 contre 0,0987/0,02456/0,1178/0,09142 ; 32767 → identiques au plancher de réserve. Pire rectangle 2×1,5 m base : G(U) = 0,1097 pour C_U = 0,1267 (94 % de C) — la limite spatiale annoncée dans ADR-137. Coût : Spectral 26,1–26,3 s contre Second 31,5–32,1 s à 32767. Micro-mesure alternée (2000 appels × 6 tours, 2×1,5 m) : ordre un 485–528 µs, ordre deux 845–922 µs, spectrale 709–758 µs. **Expérience** : ordre deux passé par `second_order_pass::<true>` → 710 µs, bits inchangés (le test figé garantit) ; donc écart de **code machine**, pas d'opérations. Rétabli `<false>` (ADR-137 §5, comparabilité S220), consigné comme levier de coût.

P6 : COUPURE-SPECTRALE-S221 et relevés bruts (dont micro-mesure et expérience `<true>`) publiés. Relus contre les relevés et corrigés avant commit : victoires de la coupure 1 « 96–100 % » (et non 97–100), feuille maximale tardive à 16383 gagnée par la coupure 2 d'un arrondi, ordre un micro 491–528 µs, part de masse tardive 63 % ; attribution à [1, 2) appuyée par les restes mesurés (0,0919 contre 0,0138). À porter au rituel : suivis A259/A260 (mécanisme corrigé), A261 (localisation spatiale), L300 (code machine), L301 (compter/peser).
