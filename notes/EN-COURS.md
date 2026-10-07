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

Session : S672 — **en cours**. En autonomie vers la v2 ; 2.7 et 12.3 (« manquent le courant de dérive littorale »). `Cote2D` déferle
(S670), mais la mer qui déferle pousse aussi l'eau : elle relève le niveau moyen au rivage et entraîne un courant le long de la côte.

**Ce que la session fait.** Le module `houle_moyenne.rs` (catégorie O) traite une côte uniforme le long de ses bords, rangée par rangée.

- **La contrainte de radiation** (Longuet-Higgins et Stewart 1964) : `S_ss = Σ E·(n·k_s²/k² + n − ½)` et `S_sn = Σ E·n·k_s·k_n/k²`.
- **Le niveau moyen** : `dη̄/ds = −(dS_ss/ds)/(g·(h + η̄))`, implicite en `η̄`. `S_ss` peut dépendre de `η̄` (le déferlement saturé).
- **Le courant de dérive** : `c_f·⟨|u|·u_n⟩ = −dS_sn/ds`. La moyenne est prise sur le temps des vitesses au fond de toutes les ondes,
  sans linéariser le frottement ; `V` est trouvé par bissection.

La côte 2D les recevra en S673.

**Contrôles du plan** (ADR-266, ADR-267, ADR-268)

- **témoin** : sans objet (un module nouveau).
- **instrument** : trois solutions analytiques. Ce qui départagerait : un `S_ss` faux (le `n − ½` oublié, un `cos²` de trop) s'écarte du
  creux de (1) de dizaines de % ; une intégration fausse s'écarte de la pente de (2) ; une dérive au mauvais signe ou un frottement mal
  moyenné s'écarte de (3).
  - (1) le creux hors du déferlement (Longuet-Higgins et Stewart 1962) : `η̄ = −H²k/(8·sinh 2kh)`, une houle de 1 m et 10 s de face
    sur la plage 1:50, de 80 m à 5 m ;
  - (2) la remontée saturée (Bowen, Inman et Simmons 1968), `H = γ·(h + η̄)` : `dη̄/ds = K·|dh/ds|`, `K = 1/(1 + 8/(3γ²))`, `γ` = 0,78,
    une houle de 30 s (l'eau peu profonde des formules), de 3 m à 0,3 m ;
  - (3) le courant de Longuet-Higgins (1970), sans mélange, le frottement linéarisé faible :
    `V = (5π/16)·(γ/c_f)·tan β·√(gh)·sin θ`, `c_f` = 0,01, 30 s, 1° à 2 m, de 1,5 m à 0,5 m.
- **calcul** (scratchpad `s672_calc.py`, le même équilibre intégré en Python ; ce script asserte les bornes au double du plancher au
  moins) : (1) 0,10 % → borne **0,5 %** ; (2) 0,73 % (`n` < 1 au large de la bande) → **2 %** ; (3) 1,0 % à 1° → **3 %**. À 5°, l'écart
  est de 10,9 % : `V/u_m` vaut 0,26 et le frottement n'est plus linéaire. C'est l'effet que la moyenne exacte porte et que la formule
  ignore : il est rapporté, pas jugé.
- **ADR** : ADR-196, ADR-262 (le réalisme), ADR-268.
- **pièges** : le signe de la dérive (`s` croît vers la côte : `−dS_sn/ds` > 0 dans la bande, le courant va dans le sens de `k_n`) ;
  `E = g·a²/2` par `ρ` ; `n` au `k` de la profondeur totale ; l'échantillonnage du temps (une suite équirépartie, pas une période
  commensurable).

**Critères, écrits avant.** (1) Le creux à **0,5 %** de Longuet-Higgins et Stewart, à chaque rangée. (2) La pente de la remontée saturée
à **2 %** de `K`, à chaque rangée. (3) La dérive à **3 %** de Longuet-Higgins 1970, de 1,5 m à 0,5 m, à 1° ; l'écart à 5° rapporté.

### Plan

- [x] **P1** — jeton ; plan.
- [ ] **P2** — `houle_moyenne.rs` ; les essais ; (1)–(3).
- [ ] **P3** — preuve ; liste 2.7, 12.3 ; rituel.

### Notes de reprise
