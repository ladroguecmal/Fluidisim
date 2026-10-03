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

Session : S465 — **terminée**. En autonomie : la pluie d'ADR-205, reçue sur la piscine et la mer (R28 à R33), sur la scène du saut.

**Ce que la session fait.** Dans `saut.tscn` : **les rides** (`pluie.gdshaderinc`, `pluie_rides`) sur la normale de l'eau du domaine et
de la mer au-delà, sur une horloge de pluie à part (l'enregistrement boucle, la pluie non) ; **le ciel couvert** (`couvert` : le ciel,
l'éclairement, l'éclat et les caustiques éteints) ; **les gouttes dans l'air** (`pluie_air.gd`) et **les gerbes** (`gerbes.gd`) sur la
nappe de la mer ; **l'extinction** par les gouttes (la brume, `Pluie.extinction`). `PLUIE=<mm/h>` ou la touche P (0, 2, 10, 50).

**Critères, écrits avant.** (1) le taux des anneaux celui de `pluie.gd` (Marshall-Palmer × Atlas, publié) ; (2) sous la pluie, plus
d'éclat ni de caustiques (le soleil direct éteint) ; (3) 60 images/s au moins sous 10 mm/h ; (4) les images montrées (10 mm/h).

### Plan

- [x] **P1** — jeton, plan seul.
- [x] **P2** — la pluie dans `saut.tscn` ; mesures ; images.
- [x] **P3** — preuve ; rituel (allégé).

### Notes de reprise
- **P2** — `saut_optique.gdshaderinc` : `pluie_rides` sur la normale (horloge `temps_pluie`), l'éclat et les caustiques éteints sous le
  ciel couvert ; `saut.gd` : `regler_pluie` (les uniformes de `pluie.gd`, `couvert`, `pluie_air.gd`, `gerbes.gd`, l'extinction),
  `PLUIE=`, la touche P. **Mesuré** : (1) le taux des anneaux de `pluie.gd`, **447 m⁻²·s⁻¹ à 10 mm/h** — tenu ; (2) sous la pluie,
  `couvert` = 1 : plus d'éclat ni de caustiques — tenu ; (3) **174 images/s à 10 mm/h** (60 à 50 mm/h) — tenu ; (4) images
  `godot/captures/saut_t{0.85,1.60,6.30}_pluie10.png` envoyées.
- **P3** — C10-SCENES-S454 §14 ; journal ; jeton libre ; maillons 1 ; suivant : S466, le joueur — une capsule debout au lieu de la sphère, dans la physique et le rendu.
