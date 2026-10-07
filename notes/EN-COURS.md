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

Session : S629 — **en cours**. En autonomie (ADR-247 : la physique des partiels). **Le lot** (dû ; feuille de route S626–S628), puis **3.4 —
« le raffinement à la côte »** : S622–S624 nourrissaient le domaine local d'une onde solitaire ; ici, de l'objet macroscopique lui-même,
`tsunami::niveau` (S582).

**Ce que la session fait.** Aucun code nouveau dans le cœur : un essai. Un rayon de 4 000 m à 10 m sur 50 km, prolongé de 2 km de fond plat
— le domaine local, d'ordre deux ; ses deux bords caractéristiques (S622, S628) reçoivent `tsunami::niveau` (f32) et `u = √(g/h)·η` ; une
impulsion de demi-durée 60 s. À la jauge (1 km dans le domaine), l'écart maximal du niveau local au niveau macroscopique, rapporté à la crête.
Ne fait pas : le domaine local sur la pente (S624 le fait avec une onde solitaire), la dispersion.

**En route, avant ce plan** : un balayage de la remontée d'une houle en amplitude (vers le déferlement) a été abandonné sans commit — la
remontée dépassait Keller & Keller de 1,5 %, 3,3 %, 20 %, l'incidence linéaire au bord et le raidissement sur le fond plat mêlés au
déferlement (ADR-256 D2).

**Références, calculées avant** (`s629_ref.py`, numpy). Crête à la jauge 5 mm (`A₀` = 1,118 mm) : écart relatif, maille 4 / 2 / 1 m,
**3.128833e-03, 1.279170e-03, 1.242549e-03** ; crête 20 mm : 5.052937e-03, 4.970474e-03,
4.951854e-03. À maille fine, l'écart est **proportionnel à l'amplitude** (rapport 3.985 pour 4) : la non-linéarité du domaine
local, absente du modèle macroscopique linéaire ; la part numérique converge (D2). Après le passage : 3.122e-07 m (5 mm),
4.992e-06 m (20 mm). **Sensibilité** à un ulp (D1) : 2.1e-12 sur l'écart relatif, 2.5e-14 m sur le reste.

**Quantum** : f64 ; les tolérances d'accord avec numpy, **10⁻¹⁰** (écart relatif) et **10⁻¹² m** (reste), au moins dix fois la sensibilité
(asserté). **Critères, écrits avant.** (1) les six écarts relatifs et les six restes égaux aux références à ces tolérances ; (2) à 5 mm,
l'écart décroît avec la maille et passe sous 0,2 % à 1 m ; (3) à 1 m, le rapport des écarts 20 mm / 5 mm entre 3,5 et 4,5 (la
non-linéarité) ; (4) à 5 mm, le reste après passage sous 10⁻⁴ de la crête.

### Plan

- [x] **P1** — jeton ; le lot ; plan.
- [ ] **P2** — l'essai ; (1)–(4).
- [ ] **P3** — preuve ; liste 3.4 ; rituel (`--lot`).

### Notes de reprise
