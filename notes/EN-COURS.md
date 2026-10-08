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

Session : S710 — **en cours**. En autonomie, sans arrêt ; session longue (ADR-279 D3). S709 : la projection de densité tient le volume,
mais, corrigeant 100 % de l'écart à chaque pas, elle lisse le front et empêche le plongeon. **Une projection faible** : on ne corrige
qu'une fraction κ de l'écart par pas.

**Le calcul de κ.** La perte à combattre est de ≈ 1,3 %/s, soit 0,013 % par pas de 10 ms. À l'équilibre, `κ·(ρ − 1)` compense cette
perte, d'où un biais de densité de `0,013 % / κ`. Avec **κ = 0,05**, le biais est de 0,26 %, et le volume doit tenir à ≈ 0,3 %. Le
lissage de la dynamique rapide est vingt fois moindre qu'en S709.

**Les essais, dans l'ordre ; chacun a ses critères, écrits avant lui.** La variante `Complete` (S709), seul κ change (ADR-276 D2).

| essai | ce qu'il juge | critères |
|---|---|---|
| **E1** | l'onde plate (10 m, 1,6 s), κ = 0,05 | `V_φ/V_φ(0)` à 1,6 s à moins de **0,5 %** (sans projection : −1,87 % ; κ = 1 : −0,10 %) |
| **E2** | le tout-3D de S690, κ = 0,05 | `V_φ/V_n` à 2,5 s à moins de **0,5 %** de sa valeur au départ (sans projection : −3,2 %) ; **le plongeon gardé** : le retournement à 0,1 s et 0,15 m du juge sans projection (ADR-278 D2 : 2,637 s, 9,988 m), et l'air après lui ; le coût mesuré |

Si E2 tient, la session propose d'allumer la projection faible par défaut. Ce sera une décision (un ADR), avec le banc de non-régression
réinscrit, car la scène `--v1` changerait. Elle se prendra en session suivante.

**Contrôles du plan** (ADR-266, ADR-267, ADR-268, ADR-276, ADR-277, ADR-278, ADR-279)

- **témoin** : S708 (sans projection) et S709 (κ = 1), par les mêmes fonctions.
- **instrument** : `V_φ`, étalonné en S708 ; le lecteur de retournement de S647. Ce que rendrait chaque hypothèse :
  - si la force de la correction faisait le lissage, le plongeon revient près du juge, et le volume tient à 0,3 % ;
  - si la correction de surface, même faible, comble la lèvre, aucun plongeon encore.
- **calcul** : κ et le biais attendu, ci-dessus. Le coût : celui de S709, ≈ +40 %.
- **ADR**, et comment chacun est tenu (ADR-277 D1) :
  - ADR-278 D2 : la tolérance du plongeon ;
  - ADR-279 D1 : la convergence du juge n'est pas mesurée au-delà de S647 ; la tolérance est celle d'ADR-278 ;
  - ADR-276 D2 : seul κ change.
- **pièges** :
  - κ multiplie le second membre, donc le déplacement ; la borne du quart de maille reste ;
  - le défaut (κ = 1 avec `enable_density_projection`) reste celui de S709.

**Critères de la session.** E1 et E2 tenus, ou leur échec nommé.

### Plan

- [x] **P1** — jeton ; plan.
- [x] **P2** — κ ; E1.
- [ ] **P3** — E2.
- [ ] **P4** — preuve ; rituel.

### Notes de reprise
- **P2 fini (E1)** — **tenu** : κ = 0,05, `V_φ/V_φ(0)` 0,9975 (0,4 s), 0,9954 (1,0 s), 0,9955 (1,6 s) ; un équilibre à −0,45 %, comme le calcul l'attendait (sans projection −1,87 %).
