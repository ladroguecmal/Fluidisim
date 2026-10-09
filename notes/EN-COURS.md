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

Session : S758 — **en cours**. En autonomie ; session longue. BALLOTTEMENT-S757 : la 3D corrigée a la période juste, mais l'oscillation
grandit de 2,8 % par période ; S752 avait mesuré la projection qui ajoute de l'énergie potentielle (+2 à +3 J en 4 s). **La question** : une
projection qui rend à chaque particule l'énergie de son déplacement vertical arrête-t-elle l'injection, sans perdre ce que la 3D corrigée
tient ?

**L'hypothèse nommée** (ADR-290 D1) : la projection élève des particules de Δz sans rien leur prendre. **La règle** (`set_density_energy_neutral`) :
chaque particule déplacée garde son énergie, `|v|² ← max(|v|² − 2g·Δz, 0)`, sa vitesse mise à l'échelle dans sa direction. C'est la règle
d'une bille qui monte ou descend dans la pesanteur.

**Signature prédite** :
- le bilan propre de la projection (S752) : l'énergie potentielle ajoutée compensée par l'énergie cinétique retirée ;
- le ballottement : l'amortissement près de zéro (ni négatif) ;
- la période et l'onde solitaire inchangées.

**Les critères, écrits avant** (ADR-290 D3, le repos d'abord ; ADR-293 D3, la bande de quantum) :
1. **le repos** sur l'escalier (1 cm/s ; 3 mm) ;
2. **le ballottement** : la période à 1 % ; l'amortissement entre **−0,5 % et +1 %** par période (la 3D sans projection, dans la cuve de
   S413 : 0,08 à 0,3 %) ;
3. **l'onde solitaire** sur le canal à 2,5 cm : la largeur 80 %, le creux 10 mm, la célérité à 1 % (ADR-293 D2).

Synolakis (45 min) vient ensuite, si les trois tiennent.

**Contrôles du plan** (ADR-276, ADR-287, ADR-290, ADR-293)

- **témoin** :
  - la 3D corrigée sans la règle (S744, S752, S757) ;
  - la dispersion exacte ;
  - l'onde exacte et sa célérité.
- **instrument** : le bilan propre (S752) ; ceux de S743, S757, S740.
- **calcul** : le repos 2 min, le ballottement 5 min, le canal 10 min.
- **ADR** : ADR-276 D2 (une différence) ; ADR-290 D1 ; ADR-293 D2, D3.
- **pièges** :
  - au repos, les particules n'ont pas de vitesse : la règle ne peut rien retirer, l'énergie d'un déplacement vers le haut y reste ;
  - une particule qui descend gagne de la vitesse : la règle doit le permettre, sans quoi l'énergie fuit.

### Plan

- [x] **P1** — jeton ; plan.
- [ ] **P2** — la règle ; (1), (2), (3).
- [ ] **P3** — preuve ; le lot ; fermeture.

### Notes de reprise
