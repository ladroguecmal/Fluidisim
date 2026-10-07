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

Session : S627 — **en cours**. En autonomie (ADR-247 : la physique des partiels). **4.14, côté déferlement** : en eau peu profonde, une vague
brisée est un ressaut mobile ; Saint-Venant le porte comme un choc. Cette session juge la capture des chocs de l'ordre deux (S620) sur deux
ruptures de barrage analytiques — **Stoker** (fond mouillé, un ressaut) et **Ritter** (fond sec : C04, jusqu'ici en 1D seulement, porté en
2D).

**Ce que la session fait.** Aucun code nouveau dans le cœur : un essai, une bande de trois mailles, 100 m, le barrage à 50 m, 1 m d'eau à
gauche, 0,5 m (Stoker) ou rien (Ritter) à droite ; t = 6 s ; l'écart L1 de `h` aux solutions exactes, mailles 0,5 / 0,25 / 0,125 m. Ne fait
pas : le déferlement d'une houle sur une pente (le passage du front lisse au ressaut), le rouleau 3D.

**Références, calculées avant** (`s627_ref.py`, numpy). Stoker : l'état intermédiaire `h_m` = 0.726920446 m, `u_m` = 0.923363902 m/s, le
ressaut à **2.957918120 m/s**. Écart L1 — Stoker : **3.672946492e-03, 1.734845787e-03, 8.606719835e-04** (rapports 2.117, 2.016) ;
Ritter : **6.745603398e-03, 3.375485355e-03, 1.693812017e-03** (1.998, 1.993) — l'ordre un, attendu aux chocs et au front sec. Le
front de Ritter au millimètre : 79.75, 81.625, 83.0625 m pour **85.802285 m** exact — il s'en approche lentement (6.052, 4.177,
2.740 m). Masse exacte, `h ≥ 0`. **Sensibilité** (ADR-256 D1 : `h` gauche perturbée d'un ulp) : au plus 8.8e-17 sur l'écart L1.

**Quantum** : f64 ; la tolérance d'accord avec numpy, **10⁻¹²**, au moins dix fois la sensibilité mesurée (asserté). **Critères, écrits avant.**
(1) les six écarts L1 égaux aux références à 10⁻¹², les trois fronts égaux ; (2) chaque raffinement divise l'écart L1 par au moins 1,9 ; (3) le
front sec s'approche de sa position exacte, la distance divisée par au moins 1,3 à chaque raffinement ; (4) masse à 10⁻¹³, `h ≥ 0`.

### Plan

- [x] **P1** — jeton ; plan.
- [ ] **P2** — l'essai ; (1)–(4).
- [ ] **P3** — preuve ; liste 4.14 ; rituel.

### Notes de reprise
