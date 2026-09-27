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

Session : S411 — **en cours**. Conception, au poste, sans code. Demande de l'utilisateur (2026-09-27), verdict R34 et réflexion :
*« Il s'agit de 2D et de bille, encore loin du finale, qui est en 3D et une topology sans interstice visible dans l'eau sauf pour
les jets. Donc difficile de réaliser un retour, mais le maintien de 0.3s parait bien, la formation de la vague est visible. Point
de réflexion est il intéréssant de simuler les billes en dessous en profondeur, car on ne les voit pas et ne sont pas en grand
mouvement ou possibilités d'être arraché. Il faut réfléchir a comment pouvoir avoir une simulation digne des logiciels 3D comme
HOUDINI ou autres spécialisé tout en étant en temps réel et dynamique. Je pense qu'il faut réfléchir a des astuces, trucages pour
réussir. C'est comme pour le courant, peut être avoir un système de LOD pour le courant et avoir des courants plus généraux en
profondeur et des courants détaillé en zone mouvementé, proche du joueur etc.... »* Agent : Claude Code (Opus 5.5), au poste.

**Ce que l'existant dit déjà** : la conception de la campagne (S384 §4.1, A1) voulait « des particules APIC dans une bande **sous
la surface** » — l'implémentation (S398–S410) a mis en particules des colonnes **entières**, du fond à la surface ; c'est le
**Narrow Band FLIP** de Ferstl, Ando, Wojtan, Westermann et Thuerey (Eurographics 2016), repris dans Houdini 16.5 ; les courants
à niveaux de détail sont conçus depuis S01 ([ADR-011](../docs/adr/ADR-011-courants-et-ecoulements-diriges.md) : C0 à C3), jamais
construits (2.6 absent).

**Livrable.** Un document de conception : ce qui fait la qualité d'un logiciel spécialisé, et **les trucages** qui la rendent
possible en temps réel, chacun rapporté à nos couches (B, W, δ, V), à son état dans le dépôt et à son coût ; l'ordre proposé
pour la campagne ; les questions qui demandent l'utilisateur. Aucun ADR avant sa réponse.

### Plan

- [x] **P1** — jeton, plan seul.
- [x] **P2** — verdict R34 consigné (REVUE-VISUELLE, file, preuve §6.4) ; maintien 0,3 s retenu pour C6c, défaut inchangé d'ici là.
- [>] **P3** — le document `docs/registres/TRUCAGES-TEMPS-REEL-S411.md` ; index, file.
- [ ] **P4** — rituel.

### Notes de reprise
- **P2** — R34 consigné : REVUE-VISUELLE §39 (verdict), BASCULE-S408 §6.4, file (décision en tête ; ligne de la campagne : C6c =
  bande étroite + critère qui suit la crête). Le lien vers le document de P3 est posé d'avance (navigation vérifiée au rituel).
