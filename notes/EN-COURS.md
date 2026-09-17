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

Session : S266 — en cours
Agent : Codex, GPT-6 ; fichiers, git, cargo, Python, GPU local et accès web.
Entrée : « Très bien continue », après l'avant/après R7. Aspect filtré accepté ; poursuivre
la réduction du coût en conservant cet aspect. Master propre c4f9bb9, copie unique, jeton libre.

Objectif : réduire le coût de la variante de reflets consommée par l'image, à qualité vérifiée
contre R7 ; préserver le chemin historique et les requêtes. Pas de nouveau réglage de vent.

### Plan

- [x] **P1** — amorce, jeton, plan seuls.
- [x] **P2** — consigner R7 accepté ; choisir et contractualiser une optimisation mesurable
  de l'éclairage (pré-calcul du ciel de banc), critères d'erreur et de coût avant construction.
- [x] **P3** — implémenter et tester le pré-calcul réutilisé sur le chemin de l'hôte.
- [x] **P4** — vérifier images, coût, invariance et tests ; retenir ou rejeter selon les critères.
- [>] **P5** — optimisation algébrique : sommes suffixes de covariance, témoin conservé,
  tests et réception aux critères écrits dans CIEL-CACHE-S266.
- [ ] **P6** — rituel §6, capacités, limites, file entière, journal et jeton.

### Notes de reprise

S265 : eau GPU 2,49–2,71 ms en 3×3 dont cuisson du sillage 1,06–1,11 ms ; ciel procédural
réévalué neuf fois par fragment. Aspect accepté ne reçoit ni coût ni convergence de la quadrature.
Le lot J2 bords ouverts reste utile, mais rendre le visuel accepté moins cher est la suite explicite.

P2 : R7 accepté consigné ; ADR-162 et critères CIEL-CACHE-S266 écrits avant code.

P3 : cache cubique 512² RGBA16Float, cuisson GPU et réutilisation/invalidation construits.
Tests hôte 18/1/0. 8 501 directions par ciel ; cycle clair/brume/clair identique,
aucune recuisson caméra/temps. Cuisson 0,056 ms mesurée, 12 Mio. Images P4 en cours.

P4 : 512² rejeté en précision (maximum RGB 22 > 16, MAE ≤0,074). Essai 1024² prévu par
ADR-162. Premier coût 512² NON RECEVABLE : capture GPU concurrente encore active ; à refaire
après fin des captures. Ne pas employer ce passage comme preuve de gain.

P4 : cache rejeté 512/1024 (maximum RGB 22/19 >16 ; gain1024 <5 % contre 10 requis).
Code cache retiré, expérience conservée à e60b8fa. P5 déclaré avant construction.
