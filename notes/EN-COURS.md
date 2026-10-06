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

Session : S539 — **en cours**. En autonomie, **7.5 — l'air comprimé** (absent) par le cas d'ADR-015 §2–3 : « une coque retournée flotte
grâce à l'air qu'elle emprisonne » ; la poche se comprime avec la profondeur, « un bateau chaviré flotte, puis passe un point de non-retour
et coule d'un coup ».

**Ce que la session fait.** `RigidBody::air_pocket : Option<AirPocket { body, volume_surface, thickness }>` — une poche d'air portée par
le corps (son centre dans le repère du corps, son volume à la pression atmosphérique, son épaisseur verticale) : noyée à la profondeur `d`
de son centre, elle déplace `V₀·p_atm/(p_atm + ρ g d)` (isotherme, ADR-015 §3 « lente »), une poussée `ρ g V` appliquée en son centre
(fraction d'immersion comme un point du proxy, la pente de la surface comme S333). `None` : rien ne change.

**Ordre de grandeur, calculé.** La table d'ADR-015 (eau de mer) : 100 / 50,19 / 33,50 / 25,14 % du volume à 0 / 10 / 20 / 30 m. Un corps de
2 000 kg et 0,5 m³ de matière (4 000 kg/m³) portant 2 m³ d'air à la surface : il faut 1,451 m³ d'air pour le porter → **point de
non-retour `d*` = 3,811 m**.

**Critères, écrits avant.** (1) La poussée de la poche à 0, 10, 20, 30 m à 10⁻¹² de Boyle (le quantum : l'arrondi f64, rapport > 10⁶,
ADR-236 D1). (2) Lâché au repos à `d*` − 0,3 m, le corps remonte (centre au-dessus de −1 m à 60 s) ; à `d*` + 0,3 m, il coule (sous −20 m
à 60 s). (3) Sans poche, la suite du cœur au bit.

### Plan

- [ ] **P1** — jeton, plan seul.
- [ ] **P2** — la poche, les essais ; (1)–(3).
- [ ] **P3** — preuve ; liste 7.5 ; rituel.

### Notes de reprise
