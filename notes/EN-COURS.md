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

Session : S674 — **en cours**. En autonomie vers la v2 ; 2.7. `Cote2D` (S673) relève le niveau de 13 cm au rivage, mais sa mer déferle
encore sur la profondeur au repos : 13 % de profondeur manque au rivage.

**Ce que la session fait.** **Le point fixe du niveau.** `cuire_deferlante` marche sur `h + η̄(s)`, recalcule `η̄`, et recommence
jusqu'à `|Δη̄|` < 1 mm (au plus 8 marches ; le nombre est gardé). Les tables (`k`, `coth`) sont cuites sur la profondeur totale. Le
courant n'est calculé qu'à la dernière marche.

**Contrôles du plan** (ADR-266, ADR-267, ADR-268)

- **témoin** : l'itération 0 est S673 (12,94 cm au calcul, 13,0 cm mesurés) ; sans déferlement, au bit (aucune itération).
- **instrument** : le même point fixe en 1D dans l'essai — l'équilibre d'énergie de S669 sur `h + η̄_ref`, `η̄_ref` par
  `houle_moyenne`. Ce qui départagerait :
  - une rétroaction juste suit la référence au plancher de S673 ;
  - une profondeur totale oubliée dans les tables, ou dans le déferlement, laisse `Hrms` au rivage à 0,512 m au lieu de 0,552 m (8 %) ;
  - une itération mal raccordée ne converge pas en trois marches.
- **calcul** (scratchpad `s674_calc.py`, et ce script qui asserte) : au rivage, `Hrms` 0,512 → **0,552 m** (+7,8 %), `η̄` 12,94 → **12,56
  cm** ; `|Δη̄|` 129 mm, 4,0 mm, 0,14 mm, **trois marches**. Les bornes de S673 : 3 % du pic pour `η̄`, 5 % pour `V`, 2 % par composante.
- **ADR** : ADR-196, ADR-268.
- **pièges** : la profondeur des tables (`coth`, `k̄` de `ψ`) prise sur la même `h + η̄` que la marche ; le départ (`η̄(0)` = 0) inchangé ;
  la dernière marche faite avec l'avant-dernier `η̄` (l'écart, sous 1 mm).

**Critères, écrits avant.** (1) Sans déferlement, au bit ; l'itération 0 au bit de S673. (2) Contre le point fixe 1D : le facteur de
chaque composante à **2 %**, `η̄` à **3 %** du pic, `V` à **5 %** du pic. (3) Convergence sous 1 mm en **au plus 4** marches. (4)
Rapportés : `Hrms` et `η̄` au rivage avant et après, le coût.

### Plan

- [x] **P1** — jeton ; plan.
- [ ] **P2** — le point fixe dans `cuire_interne` ; l'essai ; (1)–(4).
- [ ] **P3** — preuve ; liste 2.7 ; rituel.

### Notes de reprise
