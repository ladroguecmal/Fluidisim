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

Session : S344 — **en cours**. **Porte A, premier critère** ; chemin de la v1
([ADR-174](../docs/adr/ADR-174-arbitrages-du-2026-09-19.md) D4). §6.4 interdit une quatrième session de suite sur
la porte C (S343).
Agent : Claude Opus 5.5, application desktop ; fichiers, git, cargo, carte réelle, accès web.
Entrée — l'utilisateur : *« Continue »*, après le passage proposé à la porte A. Critères de la porte A (§3 bis) :
**plusieurs candidats réels se disputent un budget** ; un domaine **se déplace et se redimensionne** ; la
dégradation de rang 1 d'ADR-012 §4 existe. Sur des domaines 3D (ADR-175 D6). L'ordonnanceur (`scheduler.rs`, S278)
sait arbitrer plusieurs candidats, mais n'a jamais servi qu'une bande δ 2D (S279–S286).

**Ce que la session doit rendre possible.** Le premier critère, sur des domaines 3D réels : deux domaines δ de
production (la scène de la porte B, `Config::review`), à 60 m l'un de l'autre ; une caméra qui passe de l'un à
l'autre ; à chaque image, chacun soumissionne sa **part d'écran** (`screen_fraction`, ADR-012 §2) et son **coût
mesuré** (médiane des huit derniers pas payés, horodatés) ; l'ordonnanceur décide et alloue sous **un budget de
banc de 5 ms**, qui n'en tient qu'un (3,7 ms chacun, S343). Seuils d'allumage et d'extinction calibrés sur les parts
d'écran mesurées, et écrits (ADR-171).

Critères, écrits avant le code :
1. **Le budget n'est jamais dépassé** : la somme des budgets accordés ≤ 5 ms à chaque image.
2. **Le domaine regardé est servi** : hors des transitions, le domaine accordé est celui qui occupe le plus
   d'écran.
3. **Les transitions suivent ADR-013 §5** : allumage sans délai au-dessus du seuil ; extinction après 1 s sous le
   seuil bas ; l'état « vivant mais affamé » — l'ancien domaine, le temps de son délai — publié et borné.
4. **Les coûts sont mesurés**, publiés par domaine ; un domaine rallumé est de nouveau servi (pas d'exclusion
   absorbante, S279 §4).

### Plan

- [x] **P1** — jeton, plan seul.
- [x] **P2** — les parts d'écran le long du trajet de caméra ; les seuils calibrés.
- [ ] **P3** — le banc d'arbitrage, deux domaines 3D ; critères 1 à 4.
- [ ] **P4** — preuve, file, feuille de route, liste.
- [ ] **P5** — rituel.

### Notes de reprise
- **P2, fait** (`--delta3d-parts`). **Premier trajet écarté** : tourner seulement la tête vers B laisse B plus
  petit que A à l'écran (0,013 contre 0,034, B à 68 m) — la surface décide, pas le regard (ADR-012 §2). **Trajet
  retenu** : l'œil longe la côte, devant A (0–5 s), vers B à 15 m/s (5–9 s), devant B (9–13 s), retour (13–17 s),
  devant A (17–20 s). Parts d'écran : de face **0,1629** ; à mi-chemin 0,0389 chacun ; loin 0. **Seuils calibrés**
  (ADR-171) sur la part de face : allumage **0,10** (0,61 ×), extinction **0,05** (0,31 ×).

