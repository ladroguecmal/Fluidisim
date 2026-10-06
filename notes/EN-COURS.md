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

Session : S543 — **en cours**. En autonomie, **la rotation de C16** (ADR-002 §2.2) : une cuve de 20 m à 100 m de l'axe d'une station qui
tourne à 0,313 rad/s — la pesanteur est centrifuge (Ω²R = 9,8 m/s²) ; « surface d'équilibre cylindrique de flèche 50 cm ».

**Ce que la session fait.** `Volume3::set_horizontal_gravity_field(g₀, Ω², centre)` : la part horizontale de `g_eff` affine dans le plan,
`g_h(x) = g₀ + Ω²·(x − centre)` — dans la cuve, la force centrifuge a une composante horizontale `Ω²·x` le long de la cuve (le rayon
s'incline de `x/R`) ; la verticale reste `g = Ω²R` de la configuration. La même force de volume au champ prédit (S542). Coriolis n'est pas
porté : il n'agit pas sur l'équilibre (l'eau au repos dans le repère tournant), seulement sur le ballottement.

**Ordre de grandeur, calculé.** R = 100 m, g = 9,81 : Ω = 0,3132 rad/s ; la flèche exacte du cylindre sur 20 m, 0,5013 m, la parabole du
modèle linéaire `x²/(2R)` : 0,500 m. Cuve de 20 m, 2 m d'eau (80 × 1 × 8 mailles de 25 cm) : premier mode 9,18 s ; quatre périodes 36,7 s.

**Critères, écrits avant.** (1) Sans champ : le pas d'avant au bit. (2) La surface moyenne sur quatre périodes, ajustée par une
parabole : sa courbure à 2 % de `1/R` (la flèche à 2 % de 0,500 m) ; le quantum : l'arrondi f32 (10⁻⁷ m sur 0,5 m).

### Plan

- [ ] **P1** — jeton, plan seul.
- [ ] **P2** — le champ, l'essai ; (1), (2).
- [ ] **P3** — preuve ; C16 ; liste 4.17 ; rituel.

### Notes de reprise
