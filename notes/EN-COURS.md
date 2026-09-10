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

Session : S138 — terminée
Agent : Claude Code (Opus 5 ; fichiers, git et cargo disponibles)
Objectif : S137-1 — auditer les renvois du corpus. S137 a trouvé un renvoi faux recopié par
trois ADR pendant soixante sessions, et deux numéros de leçon en collision. Chercher les autres,
par recherche de texte et non par relecture — la méthode de S11 et S15.

### Plan

- [x] **P1** — état réel, jeton, **plan déclaré et committé seul**. S137 ne l'a pas fait ;
      cette session commence par le faire.
- [x] **P2** — audit mécanique des **identifiants** : ADR, angles, leçons, invariants, cas.
      Collisions, trous, et références vers des numéros qui n'existent pas. C'est le plus
      automatisable, et S137 a montré qu'il y a des collisions.
- [x] **P3** — audit des renvois vers les **bancs** — c'est là que S137 a trouvé le défaut, et
      la même erreur peut viser B3, B4 ou B10.
- [x] **P4** — audit des renvois vers les **sections de spécification** : `SPEC-00x §y` où la
      section a pu bouger, disparaître, ou n'avoir jamais porté ce qu'on lui attribue.
- [x] **P5** — corriger ce qui est faux : notes datées pour les ADR, correction directe pour
      les documents qui ne sont pas des ADR, et **dire ce qui est douteux sans le trancher**.
- [x] **P6** — livrable, rituel de fin, fusion `--ff-only`.

### Notes de reprise

Départ 17aa26b = master, trois copies coïncidentes.

Ce que S137 a établi et qui oriente cet audit :
- un renvoi non vérifié **ferme** la question au lieu de la laisser ouverte (L217) ; les
  renvois les plus dangereux sont donc ceux qui attribuent une tâche — « à calibrer B2 »,
  « traité en Sxx », « voir tel banc » ;
- les numéros peuvent entrer en collision quand deux agents travaillent en parallèle, ce qui
  est le cas depuis S125 (Codex sur master, moi sur ce worktree).

Piège à éviter : corriger un renvoi douteux en devinant sa cible. Si la cible juste n'est pas
évidente, le dire et laisser ouvert vaut mieux qu'un second renvoi faux.

Attendu réaliste : un audit trouve surtout des choses mineures. Le résultat utile peut être
« le corpus est cohérent sur ces points », à condition d'avoir cherché pour de bon.

P2-P6 : AUDIT-RENVOIS-S138, deux notes correctives (ADR-058, ADR-085), A205, L218, journal,
index, README, REPRISE, jeton rendu, ff-only. Aucun code modifié, 269 tests inchangés.

Ce que l'audit a coûté et rapporté, pour la prochaine fois : l'automatique n'a produit que des
faux positifs — dix-sept, tous dus à mes motifs — et il a fallu vérifier chacun. Le vrai résultat
est venu d'une seule recherche ciblée sur la formulation fautive de S137. **Chercher la
formulation exacte de l'erreur connue rapporte plus que balayer le corpus au hasard.**

Pour S139 sans relire : S138-1 mesure le rapport entre `slope_bound` — la borne L1 du modèle,
`Σ|a_k|·k`, calculée dans `RadialImpact::new` — et la pente réelle maximale du champ, obtenue en
échantillonnant `sample().slope` sur le disque. Si le rapport est stable, `max_slope` se dérive
de la cambrure de Stokes (SPEC-001 §4, `H/λ ≈ 1/7`, pente `πH/λ ≈ 0,449`) divisée par ce rapport.
Méthode : la sonde `forme_initiale` fait déjà ce genre de balayage et peut servir de modèle.
