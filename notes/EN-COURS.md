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

Session : S535 — **terminée**. En autonomie, **5.5 — l'assèchement du sol** : depuis S530–S533 la pluie entre dans le sol, mais il ne
rend jamais rien.

**Ce que la session fait.** Deux lois d'arête de V. `Flow::Drainage { conductivity_nm_s, exponent_pm }` : le **drainage gravitaire**
d'un sol (`from`) vers le dessous (`to` : une nappe, ou dehors), `q = K·Sᶜ` par unité d'aire (Brooks–Corey, gradient unitaire), `S` le
remplissage du sol rapporté à sa capacité ; l'aire et la lame de stockage sont celles du nœud. Intégré **exactement** sur le pas :
`dS/dt = −a·Sᶜ`, `a = K/lame`, `S₁ = (S₀^{1−c} + (c − 1)·a·dt)^{1/(1−c)}` (`c` = 1 : `S₀·e^{−a·dt}`). `Flow::Evaporation { area_mm2,
rate_nm_s }` : l'**évaporation** d'un nœud vers dehors à taux potentiel d'auteur (en attendant la météo, ADR-197 D5), fois la commande
(l'exposition), bornée par ce qu'il contient.

**Ordre de grandeur, calculé.** Un sol de 100 mm de stockage (1 m²), K = 10,9 mm/h, `c` = 4 : `S` = 0,910 / 0,616 / 0,484 à 1 / 10 / 24 h
(91,0 / 61,6 / 48,3 mm restants). Évaporation de 3 mm/jour (34,7 nm/s) : une flaque d'1 cm en 3,33 jours.

**Critères, écrits avant.** (1) Les lois d'avant au bit (suite). (2) Le drainage contre la forme fermée à 10⁻³ à 1, 10 et 24 h ; masse
exacte. (3) L'évaporation : la flaque décroît au taux, à 1 ml près par jour, et s'arrête à zéro exactement. (4) Un cycle complet — pluie,
infiltration, drainage, évaporation — garde la masse au millilitre.

### Plan

- [x] **P1** — jeton, plan seul.
- [x] **P2** — les lois, les essais ; (1)–(4).
- [x] **P3** — preuve ; liste 5.5 ; rituel.

### Notes de reprise
- **P2 fini** — `Flow::Drainage`, `Flow::Evaporation` ; essais `s535` : drainage 2–7·10⁻⁶ ; évaporation 3 024 ml exact, à sec exact ;
  cycle à la masse exacte. Suite 696.
- **P3** — preuve ASSECHEMENT-S535 ; liste 5.5 ; index ; journal.

