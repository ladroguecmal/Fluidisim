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

---

## Session en cours

Session : S291 — en cours
Agent : Claude Opus 5, application desktop Claude Code ; fichiers, git, cargo, outils locaux.
Entrée : demande de l'utilisateur, qui recouvre A293 — décomposer le temps par étape de calcul,
trouver où il part, chercher les erreurs et les calculs redondants, expérimenter, proposer.
Contrainte donnée : **garder un rendu visuellement valide et un temps de réponse court**.
Objectif : un pas dont **tous** les postes sont connus, puis supprimer ce qui est prouvé inutile
sans déplacer d'un bit ce que le pas publie.

### Plan

- [x] **P1** — état réel, jeton et plan seuls.
- [ ] **P2** — cœur : accumuler le temps **par phase** (les huit de `delta_budget`), exposé sans
  changer les structures publiques ; **mesurer ce que l'instrument lui-même coûte**, et le dire.
- [ ] **P3** — banc de décomposition : part de chaque phase sur un vrai pas, deux tailles, deux
  fonds, avec et sans candidat GPU. C'est cette carte qui décide de la suite, pas la lecture.
- [ ] **P4** — supprimer les recalculs **identiques** repérés à la lecture, chacun avec sa preuve
  d'identité, réception **au bit** sur la trajectoire :
  a. `‖b‖²` réduit **deux fois** sur le même `rhs` (garde `warm`, puis `b2`) ;
  b. `correct_into_uw` **et** `divergence_metric` refaits sur le **même** `p` quand la porte
     physique d'ADR-144 est franchie — c'est-à-dire à chaque pas accepté ;
  c. double `ctl.poll` par maille dans la boucle `w` de l'advection.
- [ ] **P5** — phase `Validate` : elle parcourt **onze** tableaux, dont six ne sont **pas
  publiés** (`us`, `ws`, `rhs`, `res`, `dir`, `tmp`). Distinguer ce qui protège la publication de
  ce qui ne protège rien, mesurer, et **refuser le changement si l'argument ne tient pas**.
- [ ] **P6** — coût de l'horloge : le sondage lit l'horloge toutes les 64 mailles. Mesurer le pas
  avec horloge réelle contre horloge figée, et l'effet du grain. Les mesures de S289/S290 ont été
  prises avec une horloge figée : dire de combien elles sous-estiment la production.
- [ ] **P7** — re-mesure complète et **propositions chiffrées**, y compris celles qui ne seront pas
  construites ici. Vérifier que le rendu reste valide : trajectoire au bit, ou écart expliqué.
- [ ] **P8** — rituel §6 : preuve, journal, registres/index/feuille, jeton libre.

### Notes de reprise

Point de départ (S290, [preuve](../docs/validation/ENCODAGE-CYCLE-S290.md)) : à 6 656 mailles le
pas coûte 4,6315 ms dont **2,1271 d'appel de pression**. Les ≈ 2,5 ms restants n'ont jamais été
mesurés : S244 n'avait cartographié que la boucle de pression, qui n'est plus le poste dominant.

**Suspects relevés à la lecture, avant toute mesure** — à confirmer ou écarter, pas à croire :
1. `project_with` réduit `norm2(&self.rhs)` pour décider `warm`, **puis** à nouveau pour `b2`.
   Deux réductions complètes sur le même tableau inchangé.
2. Sur un pas accepté, la porte d'ADR-144 appelle `correct_into_uw` puis `divergence_metric` ;
   la queue de `project_with` **les rappelle tous les deux**, sur le même `p`, les mêmes `us`/`ws`.
   Deux corrections de faces complètes et deux passes de divergence par pas.
3. `Validate` vérifie la finitude de onze tableaux — ≈ 60 000 valeurs à 6 656 mailles — dont six
   sont des tampons de travail jamais publiés.
4. La boucle `w` de `advect` sonde le contrôle **deux fois** par maille (une fois par colonne, une
   fois par maille).
5. `wet_cell(c)` refait une division et un modulo par maille, plus `height(i)`, et le chemin du
   candidat externe rezéroie deux fois les mailles sèches.
6. `surface_in_bounds` est parcouru deux fois par pas (Prepare et Validate).
7. **Le sondage lit l'horloge une fois par 64 mailles.** Toutes les mesures de S289 et S290 ont
   été prises avec une horloge **figée** (`now_ns → 0`), donc quasi gratuite : elles ne disent
   rien du coût réel du sondage en production. À quantifier avant de proposer quoi que ce soit.

Ce qui reste hors de ce lot : multigrille GPU, 3D, solides, boucle d'image, multiplateforme,
calibration automatique de la longueur de cycle. Et **aucune porte d'acceptation ne bouge** :
ADR-143/144 sont le garde-fou qui rend ces suppressions vérifiables au bit.
