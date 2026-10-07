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

Session : S628 — **terminée**. En autonomie (ADR-247 : la physique des partiels). **4.14 — le frottement** (un manque depuis S613), et le lien
avec l'éditeur de rivières (S604) : la hauteur normale de Manning.

**Ce que la session fait.** `SaintVenant2D::regler_frottement(n)` : le frottement de Manning, appliqué semi-implicitement après le pas,
`q ← q/(1 + dt·g·n²·|u|/h^(4/3))` ; `pas_avec_bords(dt, t, gauche, droite)` : le bord droit caractéristique, miroir de celui de S622 (ordre
deux). Deux essais : **(A)** un écoulement uniforme freiné (h = 1 m, u₀ = 1 m/s, n = 0,03, 1 000 m entre murs) — la vitesse au milieu à 20 s
contre `1/u = 1/u₀ + g·n²·t/h^(4/3)` (le schéma semi-implicite est exact pour `du/dt = −a·u²` : l'essai ne vérifie que le coefficient,
ADR-248) ; **(B)** un écoulement uniforme sur pente (S = 5·10⁻⁴, n = 0,035, q = 1,5 m²/s, 2 000 m) tenu entre deux bords nourris de l'état
normal — la hauteur au milieu après 600 s contre la hauteur normale d'un chenal large `(q·n/√S)^(3/5)` = **1.668801113 m**. Ne fait pas : le
frottement sur la plage (S625 sans frottement), un `n` variable, la loi de Manning d'un chenal étroit (le rayon hydraulique).

**Références, calculées avant** (`s628_ref.py`, numpy). (A) à 20 s, pas 0,04 / 0,02 / 0,01 s : `u` = 0.849920957350966, ... ; l'exact
0.849920957350966. (B) la hauteur au milieu, maille 4 / 2 / 1 m : **1.668500219, 1.668649333,
1.668724889 m** — écarts relatifs -1.803e-04, -9.095e-05, -4.568e-05 (l'ordre un) ; l'écart maximal le long du
chenal 6.363e-04, 3.210e-04, 1.612e-04 m. La formule rectangulaire de S604 pour un chenal de
10⁶ m : 1.668803341 m (à 1.3e-06 du chenal large). **Sensibilité** à un ulp (ADR-256 D1) : au plus 6.7e-16 m.

**Quantum** : f64 ; la tolérance d'accord avec numpy, **10⁻¹²**, au moins dix fois la sensibilité (asserté). **Critères, écrits avant.** (1) (A) :
la vitesse égale à l'exacte et à la référence à 10⁻¹² aux trois pas ; (2) (B) : la hauteur au milieu et l'écart maximal égaux aux références à
10⁻¹² ; l'écart relatif à la hauteur normale divisé par au moins 1,8 à chaque raffinement, sous 10⁻⁴ à 1 m ; (3) `riviere::hauteur_normale`
(S604) pour un chenal de 10⁶ m à moins de 10⁻⁵ de la hauteur normale large ; (4) les essais de S613–S627 inchangés (sans frottement, rien ne
change) ; (5) refus : `n` négatif ou non fini ; le bord droit à l'ordre un.

### Plan

- [x] **P1** — jeton ; plan.
- [x] **P2** — le frottement, le bord droit, leurs essais ; (1)–(5).
- [x] **P3** — preuve ; liste 4.14 ; rituel.

### Notes de reprise
- **P2 fini** — (1)–(5) tenus du premier essai ; les 30 essais S6xx passent. Suite : 814 essais listés.
- **P3** — preuve FROTTEMENT-S628 ; ligne 4.14 ; index ; journal.
