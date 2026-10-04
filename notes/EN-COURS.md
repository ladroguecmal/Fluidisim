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

Session : S474 — **en cours**. En autonomie : **le type d'eau** (ADR-217), premier temps.

**Ce que la session fait.** (1) **Le modèle** : les propriétés optiques d'une eau tirées de trois constituants — le phytoplancton
(`Chl`, Morel et Maritorena 2001, Table 2), la matière dissoute (`a_g(440)`, pente de Babin et al. 2003), les particules minérales
(`MES`, Babin et al. 2003) — ajoutés à l'eau pure d'ADR-177 dans `a`, `b`, `b_b` ; d'où `R0`, `kd`, `c` et la visibilité.
`outils/type_eau.py` (la référence) et `godot/type_eau.gd` (le rendu), le même calcul. (2) **Les préréglages** (eau pure, océan clair,
Méditerranée, côtier, lac, rivière, eau trouble), choisis dans les plages publiées ; `TYPE_EAU=<nom>` dans `mer.tscn` et `saut.tscn`.
(3) **Jugés** contre V1 (Méditerranée) et V5 (côtier), compressés comme la référence (ADR-216 D8). **Le champ qui varie dans
l'espace** (une texture de concentrations lue par fragment) : le second temps, S475.

**Critères, écrits avant.** (1) `type_eau.gd` et `type_eau.py` d'accord à 10⁻⁶ près sur tous les préréglages ; le préréglage `pure`
redonne l'eau d'ADR-177 (`kd` et `c` exacts, `R0` à l'arrondi de la table) ; sans `TYPE_EAU`, les images au bit ; (2) les
visibilités dans les plages publiées des milieux ; (3) **V5** : avec le préréglage côtier (réglé dans les plages publiées), les
teintes de l'eau proche (B/G, B/R des creux et des crêtes) dans leur tolérance ; (4) **V1** : l'eau pure est la plus bleue possible
(ADR-177 D4) et V1 est plus bleue que notre eau pure — **le type d'eau ne peut pas fermer cet écart** ; le dire, mesuré, et nommer ce
qui le peut (le ciel reflété, l'étalonnage du téléphone) ; (5) les images montrées.

### Plan

- [x] **P1** — jeton, plan seul.
- [ ] **P2** — le modèle (Python, GDScript), les préréglages, `TYPE_EAU` ; l'égalité ; le défaut au bit.
- [ ] **P3** — V5 et V1 mesurées par préréglage ; le réglage du côtier ; les images.
- [ ] **P4** — preuve ; rituel (allégé).

### Notes de reprise
