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

Session : S463 — **terminée**. *« Continue en autonomie »* — les sessions s'enchaînent sans « Continue » (ADR-215 D1, D2), sur la
suite de C11 ; inscrit aux décisions.

**Ce que la session fait.** **La surface fine** de S360 sur la scène du saut : les deux cascades FFT de la queue de B (`detail.gd`,
32 m et 4 m, calculées sur la carte de Godot depuis `donnees/detail_h0.bin`) ajoutent leurs pentes à la normale de la surface simulée
et de la mer au-delà. **Tranché (ADR-215 D2)** : la queue exportée est celle de la mer de R14 (un vent établi) ; la scène du saut est
une mer calme — ses pentes sont échelonnées (`FORCE_DETAIL`, 0,5 par défaut), à juger sur image.

**Critères, écrits avant.** (1) la pente quadratique moyenne ajoutée publiée (la cascade × l'échelle) ; (2) le raccord domaine | mer
de B sans saut de texture (la même surface fine des deux côtés) ; (3) toujours 60 images/s au moins ; (4) les images montrées.

### Plan

- [x] **P1** — jeton, plan seul ; décision inscrite.
- [x] **P2** — les cascades dans `saut.tscn` ; mesures ; images.
- [x] **P3** — preuve ; rituel (allégé).

### Notes de reprise
- **P2** — `pente_fine()` dans `saut_optique.gdshaderinc` (les deux cascades au niveau de détail de l'empreinte du pixel, ajoutées à la
  normale des faces tournées vers le haut), `couleur_eau(…, empreinte)` ; `saut.gd` charge `detail.gd` depuis `mer_b.json` et le calcule
  à l'instant de chaque image (`DETAIL=0`, `FORCE_DETAIL=`). **Mesuré** : (1) pente quadratique ajoutée **0,0087** (les cascades : 0,0091
  et 0,0257, × 0,5²) ; (2) le raccord domaine | mer de B ne se voit plus (la même surface fine des deux côtés) — tenu ; (3) **391
  images/s** — tenu ; (4) images envoyées.
- **P3** — C10-SCENES-S454 §12 ; journal ; jeton libre ; maillons 2 ; registres : le lot dû en S460 fait en S464 ; suivant : S464, le lot des registres puis le direct.
