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

Session : S476 — **terminée**. **Réponses de l'utilisateur** aux six faits du plan de complétion (F1 à F6) : le réseau à nous ; un
seul ordinateur, celui-ci ; le jeu sera **DyingStar** (<https://github.com/DyingStar-game>), **une surprise pour l'équipe** ; pas
d'outil de terrain en tête ; l'écume par les vidéos ; les verdicts aux jalons.

**Ce que la session fait.** (1) **ADR-219** : les six réponses ; les points à second matériel reformulés sur ce PC ; ce que les dépôts
publics de DyingStar disent (lus seulement — aucun contact : la surprise) ; un point ajouté, **13.4 — l'eau dans le jeu**. (2) **La
liste** : les reformulations aux points, 13.4, le décompte. (3) **Le registre des dépendances** : les attentes extérieures levées, les
fronts recalculés. (4) **Le plan de complétion** : §3 résolu, 13.4 en K13. Le type d'eau dans l'espace passe en S477.

**Critères, écrits avant.** (1) ADR-219 écrite et indexée ; (2) la liste et le registre d'accord (`--check` à 0), 13.4 compté ;
(3) chaque point ouvert dans exactement une campagne (le calcul de S475 refait) ; (4) aucune action vers DyingStar ni son équipe.

### Plan

- [x] **P1** — jeton, plan seul.
- [x] **P2** — ADR-219 ; la liste ; le registre ; le plan.
- [x] **P3** — preuve ; rituel (allégé).

### Notes de reprise
- **P2** — ADR-219 (indexée). DyingStar lu dans ses dépôts publics seulement (pages et API publique de GitHub, en lecture) — Godot
  4.5, C#, double précision, Jolt, Wwise, serveur Horizon en Rust, planètes de 6 356 km en tuiles HEALPix ; **aucune action vers le
  jeu ni son équipe**. La liste : 1.7, 9.10, 10.2, 10.3, 11.5 reformulés sur ce PC ; 7.1 la suspension levée ; 7.8 par Wwise ; 12.4 le
  terrain du jeu ; **13.4 ajouté** — 121 points, 120 au périmètre. Le registre : onze attentes levées, **46 points au front 0** (36
  avant), 13 en E. Le plan : §3 répondu, 13.4 en K13 ; **117 points ouverts, chacun dans une seule campagne** (le calcul refait).
- **P3** — la preuve : ADR-219 et le plan de complétion ; journal ; jeton libre ; maillons 1 ; suivant : S477, K1 — le type d'eau dans l'espace.
