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

Session : S439 — **en cours**. Demande de l'utilisateur (2026-10-02) : *« Continue »* — la suite déclarée : **C7d-3b**
([preuve](../docs/validation/APIC-CARTE-S416.md) §22.3, §22.9), sans la bascule des défauts.

**Ce que la session trouve en entrant.** Le pas résident (`viewer/src/delta3d_step.rs`, `delta3d_step.wgsl`, et `couple_columns`,
`couple_rhs` de `delta3d_background.wgsl`) porte le pas de S297 : le résidu de quantité de mouvement de B dans la prédiction, la
bande de B entière dans le transport, les fantômes de pression avec l'erreur entière de B. Le mode relatif de la référence
(`RELATIVE_ALL`, S369, plus le fantôme latéral d'A324, S436) change quatre choses : (1) la prédiction sans le résidu ; (2) la bande moins
celle de B seul (`own = repos + η_B`) ; (3) le fantôme du haut moins l'erreur de B à sa propre surface ; (4) le fantôme latéral interpolé
des fantômes verticaux des deux colonnes — au second membre (`couple_rhs`) comme à la correction (`correct`).

**Ce que la session fait.** Une constante de compilation `RELATIVE` dans les deux sources, éteinte par défaut ; des pipelines à part,
compilés à la demande (`Step3::enable_relative`) — le procédé de S391 : la production garde son code, donc ses bits (L345). Le banc
`--delta3d-trajectoire` gagne `RELATIF=1` (la référence en `RELATIVE_ALL`, la carte en mode relatif) et `TEMOIN=1` (aucune
perturbation initiale).

**Critères, écrits avant.** **Reçu si** : (1) **la production au bit** — le banc de trajectoire sans `RELATIF` rend les mêmes lignes
qu'avant le changement ; (2) **le témoin** — en mode relatif, sans perturbation, sous la mer de S298, la surface publiée et les vitesses
de la carte restent **nulles au bit** sur tout le banc ; (3) **la trajectoire** — en mode relatif, la carte suit la référence au moins
aussi bien que le pas de S297 suit la sienne : horizon du millimètre au moins aussi tardif, écart avant l'horizon au plus deux fois
celui du pas de S297 ; (4) la suite du cœur, zéro avertissement ; l'afficheur compile sans avertissement.

### Plan

- [x] **P1** — jeton, plan seul.
- [ ] **P2** — la ligne de base du banc de trajectoire (S297).
- [ ] **P3** — le mode relatif sur la carte : sources, pipelines, banc.
- [ ] **P4** — mesures ; critères.
- [ ] **P5** — preuve ; registres ; rituel.

### Notes de reprise
