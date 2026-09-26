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

Session : S374 — **en cours**. Demande : *« commence à permettre de visualiser le système de piscine avec déversoir et
pompe »*. La demande prime sur l'alternance et sur le compteur de maillons (1).
Agent : Claude Opus 5.5, application desktop ; fichiers, git, cargo, carte réelle, accès web ; Godot 4.4.1 local.

**Thèse.** Une **piscine à débordement** : un bassin de 8 × 4 m, un déversoir de 4 m sur un petit côté, qui tombe dans
un bac tampon plus bas, et une pompe qui rend l'eau au bassin par une buse. **V la calcule, Godot la montre** — V reste la
seule source : le cœur (exemple `piscine_v`) joue un scénario (pompe arrêtée, lancée, arrêtée) et publie chaque pas —
volumes, surfaces par `Shapes::surface_plane` (I-01 : le consommateur ne reconstruit rien), débits d'arête, commandes ;
Godot le rejoue, interpolé à l'image. **Premier pas, pas l'intégration** : l'intégration native (godot-rust, accord de
téléchargement) ou un lien local en direct viendront ensuite ; la commande en direct (touche pour la pompe) en dépend.
La lame du déversoir et le jet de la buse sont de l'habillage tiré des débits de V (hauteur critique `(q²/g)^⅓`,
trajectoires balistiques) ; la buse n'existe pas dans V.

**Critères, écrits avant.** (1) Le scénario fermé conserve le volume **exactement** à chaque pas. (2) Régime établi :
débit du déversoir = débit de la pompe au point de fonctionnement **analytique** (charge sur le seuil par la loi des
trois demis, hauteur statique de la pompe) **à ±1 %**. (3) Dans Godot, la surface rendue de chaque bac est celle publiée
par V **au dixième de millimètre**. (4) Jugement de l'utilisateur (R27).

### Plan

- [>] **P1** — jeton, plan seul.
- [ ] **P2** — `examples/piscine_v.rs` : la piscine dans V, le scénario, l'export (`godot/donnees/piscine_v.json`) ;
  critères 1 et 2.
- [ ] **P3** — `godot/piscine.tscn`, `piscine.gd` : les bacs, les murs, l'eau (`bassin.gdshader`, l'optique de
  `optique_eau`), le rejeu ; critère 3.
- [ ] **P4** — la lame du déversoir, le jet de la buse, les indications à l'écran.
- [ ] **P5** — images de R27 ; preuve `PISCINE-V-S374`, liste, file, index.
- [ ] **P6** — rituel.

### Notes de reprise
