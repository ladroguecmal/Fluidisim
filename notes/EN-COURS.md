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

```
Session          : S18
État             : en cours
Battement        : 2026-09-05
Objectif         : trancher les cinq arbitrages, sur délégation explicite de l'utilisateur
```

### Plan

**Changement de mandat.** `CLAUDE.md` et `REPRISE.md` §5 posent depuis S06 que les arbitrages de
design ne sont pas à moi. L'utilisateur les délègue explicitement — « prends les décisions ». Cette
règle vient de lui ; il peut la lever, et il la lève.

**Conduite adoptée pour une décision prise par délégation** : elle doit être **plus argumentée**
qu'une décision ordinaire, pas moins, et **bon marché à défaire**. Chaque section porte donc son
motif, son chiffre, et ce qu'il faudrait changer si la réponse était l'inverse.

- [ ] **P1** — déclarer le plan, prendre le jeton, mettre à jour le battement.
- [x] **P2** — `ADR-027` §1–2 : le cadre de la délégation · **arbitrage 1, l'échelle du temps**.
  *Thèse : la question mêle trois besoins de design — voyage rapide, pause, mode photo — dont aucun
  n'exige de mettre à l'échelle le temps **par joueur**. La réponse est non, et elle ne coûte rien
  au design une fois les trois besoins traités séparément.*
- [x] **P3** — `ADR-027` §3–4 : **la glace** · **le trait de côte mobile**.
  *Thèse : le second se dissout. « Qui porte le trait de côte » suppose qu'il soit stocké ; il est
  dérivé de la marée analytique, donc personne ne le porte — au même titre que l'écume permanente.*
- [x] **P4** — `ADR-027` §5–7 : **la durée de vie d'un nœud V** · **la propriété du harnais** ·
  ce qui reste ouvert.
  *Thèse : le premier se dissout aussi — l'eau d'un objet suit la politique de cet objet, et n'a
  pas besoin d'une règle propre.*
- [ ] **P5** — notes correctives dans ADR-003, ADR-010, ADR-011, ADR-017, ADR-018, SPEC-003,
  SPEC-006.
- [ ] **P6** — `CLAUDE.md`, `REPRISE.md`, `00_INDEX.md`, `DOSSIER-REUNIONS.md` : le mandat a changé,
  et ce qui reste non décidable doit être dit précisément.
- [ ] **P7** — angles morts, décomptes.
- [ ] **P8** — rituel de fin (`REPRISE.md` §6) : journal S18, leçons, index, jeton libéré.

### Notes de reprise

- **Ce que je ne peux toujours pas décider, et qui n'est pas un arbitrage** : nommer les personnes
  (fiches 1 et 2 du dossier de réunion), constater l'état réel du projet — du terrain a-t-il été
  sculpté, un format réseau existe-t-il, du code existe-t-il — et créer un dépôt distant. Ce sont
  des **faits** et des **actions sur l'infrastructure**, pas des décisions de conception. La
  distinction doit être écrite dans `CLAUDE.md`, sans quoi une session suivante croira que tout a
  été délégué.
- **Deux des cinq se dissolvent.** Le trait de côte et le nœud V posaient chacun une question dont
  la prémisse est fausse. C'est la troisième fois que la méthode le produit (L03), et cela vaut
  d'être noté : un arbitrage qui traîne est souvent un arbitrage mal posé, et l'attente d'une
  réponse humaine masque le fait qu'il n'y a rien à trancher.

#### P2 à P4 — ADR-027, sept sections

Écrit d un tenant : les cinq décisions se tiennent par leur cadre commun, et le §1 (par quelle
autorité, à quelles conditions) ne se sépare pas des décisions qu il encadre.

**Deux se sont effectivement dissoutes**, comme la thèse du plan l anticipait.
- *Trait de côte* : « qui le porte » suppose qu il soit stocké. Il est dérivé de la marée
  analytique — même famille que l écume permanente et les sites turbulents. Ce qui est stocké, c est
  la bibliothèque à 16 états : **45 Mo**, tout le prix de la décision.
- *Nœud V* : 20 octets par nœud, **4 Ko pour la flotte d un joueur**. Une politique de rétention
  propre à l eau coûterait plus en complexité qu elle n économiserait, et créerait un joueur qui
  retrouve son navire intact mais asséché.

**Le harnais s est dissous à moitié** : la question cherchait un propriétaire unique là où il en faut
deux. Le code et les scénarios à l eau, **les seuils d acceptation à la qualité** — c est la seule
barre qu on est tenté de déplacer quand on ne la passe pas. Les seuils vivent dans un fichier séparé
qui exige une approbation ; ajouter un scénario ne passe par personne (L19).

**Le temps** est le seul des cinq dont l inversion tardive détruirait du travail fait. La question
mêlait trois besoins — voyage rapide, pause, mode photo — dont aucun n exige une échelle **par
joueur**. L échelle **globale** reste disponible et ne coûte rien : c est un degré de liberté que
personne n avait relevé.
