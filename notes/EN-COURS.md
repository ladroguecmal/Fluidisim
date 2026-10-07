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

Session : S617 — **en cours**. En autonomie (ADR-247) : **le lot** (dû ; feuille de route S614–S616), puis **9.10 — les profils de qualité,
l'adaptation au matériel et à la charge** (I-16 ; absent). I-16 : un profil ne déclare que des allocations ; une capacité se calcule depuis
des coûts mesurés. ADR-012 §5 : un régulateur PI sur `q ∈ [0, 1]` — **non écrit** (note de S351).

**Ce que la session fait.** Un module `qualite.rs` : `Capacites::depuis(profil, coûts)` — les capacités dérivées (paquets W, blocs δ) ;
`Regulateur` — le PI d'ADR-012 §5 : le consommé filtré (`τ` = 0,5 s), la descente en une image sur le consommé brut (coupe proportionnelle
de `q`), la remontée rampée (≤ 1 par seconde), la décision engagée 30 images, l'intégrale plafonnée à `q` pendant l'engagement
(anti-emballement). **Gains choisis au plan** (ADR-012 §8 : les valeurs se mesurent) : `kp` 0,5, `ki` 2,0 pompaient (12 inversions en 12 s) ;
`kp` 0,3, `ki` 1,0 avec le plafonnement : 3. Ne fait pas : la mesure du coût réel sur ce PC bridé (WARP), le branchement à l'ordonnanceur
(`scheduler::set_profile`), les profils nommés.

**Références, calculées avant** (`s617_ref.py`, Python indépendant). Installation simulée `consommé = charge·(0,4 + 1,6·q)` ms, budget 1,6 ms,
30 images/s. **Trace** (12 s : charge 1, 1,8 de 2 à 5 s, puis 1) : `q` 0.750024 → **0.416670** dès la première image de
l'événement (le consommé 1.9200 ms à la suivante), **0.305555556** à 5 s (l'équilibre 0.305555556), **0.750000000** à 12 s
(0,75) ; retour à 99 % en **3.60 s** ; **3 inversions** notables (> 10⁻³). **Matériel faible** (coûts × 3) : `q` → **0.083333333**
(l'équilibre 1/12), 0 inversion. **Capacités** : 2,0 ms / (109/4096) ms par impact → **75 paquets W** ; matériel faible
**25** ; 384 Mio / (8³ × 16 o) → **49152 blocs**.

**Quantum** : f64 ; des entiers pour les capacités. **Critères, écrits avant.** (1) les trois `q` d'équilibre (trace, faible) à 10⁻⁶ et la
trajectoire égale à la référence à 10⁻¹² image par image ; (2) la descente à la première image de l'événement ; (3) au plus 3 inversions
notables sur la trace, aucune sur le matériel faible ; le retour à 99 % au moins 1 s (rampé) ; (4) la remontée jamais plus vite que 1 par
seconde, aucune remontée dans les 30 images d'une descente (assemblage : par construction) ; (5) les capacités 75, 25,
49152 ; (6) refus : `τ`, pas, budget non positifs ; un coût non positif.

### Plan

- [x] **P1** — jeton ; le lot ; plan.
- [ ] **P2** — `qualite.rs` et ses essais ; (1)–(6).
- [ ] **P3** — preuve ; liste 9.10 ; rituel (`--lot`).

### Notes de reprise
