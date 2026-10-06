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

Session : S515 — **en cours**. En autonomie, **5.4, la vanne selon son ouverture, les pertes et l'énergie de la pompe** : ADR-199 laissait
« le `C_d` selon l'ouverture, à calibrer sur la courbe du constructeur », « ni puissance ni énergie consommée, ni pertes de charge » ; son
§3 dit la voie : une courbe tabulée à la place de la section, une seconde loi plutôt qu'une modification.

**Ce que la session fait.** Deux lois nouvelles de V, les anciennes intactes : `Flow::Valve { area_mm2, curve_pm }` — Torricelli sous une
courbe d'ouverture tabulée (onze points, de 0 à 1 000 ‰, interpolée linéairement : la courbe du constructeur, linéaire, à pourcentage
égal…) ; `Flow::PumpLine { …, loss_um_per_l2s2, efficiency_pm }` — la pompe avec perte de charge `K·Q²` sur sa conduite, point de
fonctionnement `Q = √((n²H₀ − Δh)/(H₀/Q²max + K))`, rendement ; `pump_operating_point` : débit, hauteur, puissance hydraulique et à
l'arbre, pour que l'hôte cumule l'énergie. L'empreinte et la validation les connaissent.

**Ordre de grandeur, écrit avant.** Une courbe à pourcentage égal de rapport 50 (`f = 50^(x−1)`) : à mi-ouverture, 14 % du débit plein
(contre 50 % en linéaire) ; tabulée tous les 10 %, l'interpolation linéaire s'en écarte au plus d'≈ 1,9 % du débit plein entre deux points.
Une pompe de 10 l/s et 10 m de barrage, `K` = 0,05 m/(l/s)² : à `Δh` = 3 m, le débit tombe de 8,37 à 6,32 l/s.

**Critères, écrits avant.** (1) la courbe linéaire rend les débits de l'orifice commandé à 10⁻¹² près ; la courbe à pourcentage égal exacte
aux points de la table (10⁻¹²) et dans la borne d'interpolation entre eux ; (2) `PumpLine` à `K` = 0 rend `Pump` à 10⁻¹² près ; à `K` > 0, la
forme fermée à 10⁻⁹ ; (3) l'énergie : en remplissant un bassin, `∫ P_hydraulique dt` égale l'énergie potentielle gagnée par l'eau (plus
`∫ ρgKQ³ dt` avec pertes) à 0,1 % ; la puissance à l'arbre `P_h/η` ; (4) la masse exacte ; la suite du cœur, l'instantané. 5.4 reste
partielle par 5.8 (le réseau fermé, v2 d'ADR-010).

### Plan

- [x] **P1** — jeton, plan seul.
- [ ] **P2** — `Valve`, `PumpLine`, `pump_operating_point` ; essais (1)–(4).
- [ ] **P3** — preuve ; note datée à ADR-199 ; liste 5.4 ; rituel.

### Notes de reprise
