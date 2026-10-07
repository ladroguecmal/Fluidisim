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

Session : S593 — **en cours**. En autonomie (ADR-247) : **le lot** (dû ; feuille de route S590–S592), puis **2.4 — les rivières** (absent ;
« débit macroscopique qui contraint les perturbations locales »). Première pièce : **l'état macroscopique d'une rivière** dans V — sa
ligne d'eau, avec le **remous** qu'un seuil aval lui impose, et la vitesse moyenne de chaque bief (ce que δ et W devront respecter).

**Le montage.** La loi de Manning de S592 : vingt biefs de 100 × 5 m, `S` = 10⁻³, `n` = 0,015, 5 m³/s apportés en amont ; le dernier bief
se vide par un déversoir de 5 m (`C_d` = 0,62) dont la crête est à 1,5 m au-dessus de son fond ; trois heures au pas de 0,05 s (le pas de 1 s
oscillerait : sous le remous, la surface est presque plate et `dQ/dΔh` grand — calculé : l'amplitude d'oscillation `(dt·C/A)²` passe de
~8 cm à ~10⁻⁵ m).

**Références, calculées avant** (ce script les écrit). (1) **L'état stationnaire exact du même découpage** (de l'aval vers l'amont, chaque
surface par bissection sur la loi de l'arête) : les profondeurs **0.7618, 0.7861, 0.8181, 0.8585, 0.9075, 0.9646, 1.0291, 1.0998, 1.1756, 1.2557, 1.3392, 1.4255, 1.5139, 1.6042, 1.6958, 1.7887, 1.8825, 1.9771, 2.0724, 2.1682** m, de l'amont à l'aval ; la
charge sur le seuil 0.66819 m. (2) **La ligne d'eau continue de l'onde diffusive** (`dy/dx = S − S_f` : le modèle de V, sans inertie) :
l'écart au découpage, **0.75 mm** au plus. (3) **La ligne d'eau complète** (`dy/dx = (S − S_f)/(1 − Fr²)`) : l'écart de l'onde
diffusive, **40.4 mm** au plus — l'inertie que V n'a pas, publiée, sans critère.

**Quantum** (ADR-236 D1 ; ADR-249 D1 vérifié par le script) : 1 ml sur 500 m² ; le pas de temps (l'oscillation ~10⁻⁵ m). **Critères,
écrits avant.** (1) chaque profondeur à 1 mm de la référence (1) ; (2) à 1.5 mm de la ligne diffusive continue ;
(3) la vitesse moyenne de chaque bief, `Q/(b·y)`, publiée et à 10⁻³ de `Q/(b·y_réf)` ; le bilan au millilitre ; (4) la ligne complète :
l'écart publié.

### Plan

- [x] **P1** — jeton ; le lot ; plan.
- [ ] **P2** — l'essai ; (1)–(4).
- [ ] **P3** — preuve ; liste 2.4 ; rituel (`--lot`).

### Notes de reprise
