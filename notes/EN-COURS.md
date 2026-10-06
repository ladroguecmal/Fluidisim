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

Session : S547 — **en cours**. En autonomie, **5.9 — l'évent à débit limité** (ADR-015 §2 : « l'inondation d'un compartiment fermé est
limitée par la sortie de l'air ; `Q_eau ≤ Q_air` ») : S538 n'a qu'un nœud scellé ou ouvert.

**Ce que la session fait.** `Flow::Vent { area_mm2 }` : l'évent d'un nœud scellé vers l'air libre — aucune eau n'y passe ; dans `step_air`,
l'air de la poche en sort à `Q_a = C_d·A·√(2Δp/ρ_a)`, `ρ_a = 1,2·p/p_atm` (isotherme), et le produit `p·V_air` de la poche baisse de
`p·Q_a·dt` (les moles qui sortent). `air` devient mutable. Sans évent, S538 au bit.

**Ordre de grandeur, calculé (incompressible, quasi permanent).** Le débit d'eau par la brèche égale le débit d'air par l'évent :
`k = √((C²A_v²/ρ_a)/(C²A_v²/ρ_a + C²a²/ρ))`, le remplissage de Torricelli ralenti de `k`. C17 (brèche 1 dm², 99 % en 463,5 s ouvert) :
évent de **5 cm²** → `k` = 0,8253, **99 % en 561,6 s**, surpression initiale 6,4 kPa (6,3 % de p_atm) ; 1 cm² → `k` = 0,2805, 1 652 s,
18,5 kPa (18 %). La compressibilité de l'air, négligée par cette loi, compte de l'ordre de la surpression rapportée à p_atm.

**Critères, écrits avant.** (1) Sans évent, `step_air` au bit de S538 (ses essais). (2) Évent de 5 cm² : 99 % à 5 % de 561,6 s (la
compressibilité, ≈ 6 %, en marge). (3) Évent de 1 cm² : plus de trois fois plus lent qu'ouvert (`1/k` = 3,6) ; un compartiment scellé ne se
remplit pas (S538). (4) La masse d'eau exacte.

### Plan

- [ ] **P1** — jeton, plan seul.
- [ ] **P2** — l'évent, les essais ; (1)–(4).
- [ ] **P3** — preuve ; liste 5.9 ; rituel.

### Notes de reprise
