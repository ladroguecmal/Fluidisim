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

Session : S673 — **en cours**. En autonomie vers la v2 ; 2.7 et 12.3. `houle_moyenne.rs` (S672) calcule le niveau moyen et le courant de
dérive d'une côte droite ; `Cote2D` (S670) cuit la mer qui déferle. Ils ne se parlent pas encore.

**Ce que la session fait.**

- **`Cote2D::cuire_deferlante` reçoit `c_f`.** Par rangée de la marche, l'énergie de chaque composante est moyennée le long de la
  côte ; son vecteur d'onde vient de Snell (`k_n = k₀·sin θ₀`). La rangée donne `S_ss` et `S_sn`, puis `η̄(s)` et `V(s)`, stockés
  par rangée des tables (8 octets par rangée).
- **`eval`** ajoute `η̄` à `η` et `V·t̂` à `u_total`. Sans déferlement, rien ne change (au bit).
- **`houle_moyenne`** : les vitesses au fond sont échantillonnées une fois par rangée, avant la bissection (le coût, ce script).

**Contrôles du plan** (ADR-266, ADR-267, ADR-268)

- **témoin** : `cuire_decime`, sans déferlement, au bit (l'empreinte de S670, `40c657593a299c83`) ; les essais S672 aux mêmes valeurs.
- **instrument** : l'équilibre d'énergie 1D de S669, donnant les amplitudes par composante, passé par `houle_moyenne` : `η̄_ref(s)`,
  `V_ref(s)`. Ce qui départagerait :
  - une moyenne le long de la côte juste suit la référence au plancher ;
  - un signe d'axe faux (`t̂` retourné) inverse le courant ;
  - un indice de rangée décalé déplace le pic de `V` de la bande.
- **calcul** (ce script) : le plancher — 0,8 % sur `S` (0,40 % par composante, S670) ; d'où **3 %** du pic pour `η̄` et **5 %** du pic
  pour `V` ; le coût, ≈ 0,8·10⁹ opérations pour 1 976 rangées.
- **ADR** : ADR-196, ADR-268.
- **pièges** : `t̂ = (−n_y, n_x)`, le même que `coordonnees` ; `η̄(0)` = 0, rapporté au bord du large (le creux y est de 10⁻⁵ m) ; `V` hors
  de la bande, où `dS_sn/ds` n'est que bruit numérique.

**Critères, écrits avant.** (1) Sans déferlement, au bit ; S672 aux mêmes valeurs. (2) `η̄` des tables à **3 %** du pic de `η̄_ref`, `V` à
**5 %** du pic de `V_ref`, à chaque rangée. (3) `eval` : `η` relevé de `η̄(s)` interpolé (à 10⁻⁵ m), `u_total` de `V·t̂` ; le courant
dans le sens de `S_sn` au large. (4) Rapportés : la remontée au rivage, le creux le plus bas, le pic de `V`, le coût de la cuisson.

### Plan

- [x] **P1** — jeton ; plan.
- [ ] **P2** — `Cote2D` ; `houle_moyenne` échantillonné ; l'essai ; (1)–(4).
- [ ] **P3** — preuve ; listes 2.7, 12.3 ; rituel.

### Notes de reprise
