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

Session : S670 — **terminée**. En autonomie vers la v2 ; 2.7. S669 a mis le déferlement d'une mer dans la marche parabolique ; `Cote2D`
marche encore chaque composante seule, sans dissipation, et s'arrête à 2 m de fond.

**Ce que la session fait.** **`Cote2D::cuire_deferlante`** : toutes les composantes de B cuites ensemble par `propager_spectre_periodique`,
avec Battjes et Janssen (`γ` de Battjes et Stive, `Hrms₀` et `f̄` tirés de B), jusqu'à 1 m de fond. `cuire_decime` la rappelle sans
déferlement, au bit.

**Contrôles du plan** (ADR-266, ADR-267, ADR-268)

- **témoin** : les essais S664–S668, sans déferlement, au bit (la marche spectrale sans dissipation est au bit des marches séparées, S669).
- **instrument** : l'équilibre d'énergie 1D de S669, rendu par composante (`a_c(s) = √(2·E_c)`), parti de l'eau profonde par la
  conservation du flux (`c_g0 = g/(2ω)`), indépendant de `transformer`. Ce qui départagerait : une cuisson juste rend le facteur de
  chaque composante à la précision de la marche ; un départ mal normalisé (le facteur WKB oublié dans `Hrms`) déplace le déferlement et
  s'écarte de plusieurs %, dès 6 m de fond.
- **calcul** (ce script) : le plancher de l'instrument — 0,56 % par composante sans déferlement (S665), plus 0,28 % avec (S669) — d'où
  la borne de **2 %** ; au rivage, la correction avance de 1.71 rad par nœud des tables à 8 m (tenu), de 3.41 rad à 16 m
  (refusé, `PasTropGrand`).
- **ADR** : ADR-196, ADR-268.
- **pièges** : `Hrms` en amplitudes physiques (`a_c·|A_c|`, le départ WKB compris) ; `Hrms₀` tiré des amplitudes de B (en eau
  profonde) ; la décimation `m` = 8 refusée au rivage.

**Critères, écrits avant.** (1) Sans déferlement, `cuire_decime` au bit d'avant (les essais S664–S668). (2) La mer de S667 sur la plage
de 80 m à 1 m : le facteur de chaque composante à moins de **2 %** de l'équilibre 1D, aux nœuds de la côte ; au large (`s ≤ 0`), B au
bit. (3) Rapportés : `Hrms/h` au rivage ; l'écart de `η` au rivage entre la côte avec et sans déferlement ; la mémoire à `m` = 4.

### Plan

- [x] **P1** — jeton ; plan.
- [x] **P2** — `cuire_deferlante` ; l'essai ; (1)–(3).
- [x] **P3** — preuve ; liste 2.7 ; rituel.

### Notes de reprise
- **P2 fini** — (1) l'empreinte des tables inchangée ; (2) 0,40 % au plus, B au bit au large ; (3) `Hrms` 0,513 m au rivage (1D 0,511), `|Δη|` avec et sans déferlement jusqu'à 1,19 m ; 1,7 s ; 2,0 Mo à `m` = 4.
