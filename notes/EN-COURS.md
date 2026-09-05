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
- Marquer `[>]` **avant** de commencer une étape, `[x]` **seulement après** son commit. Un `[x]`
  sans commit est un mensonge que la session suivante paiera.
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
Session          : S07
État             : en cours
Battement        : 2026-09-05
Objectif         : dispositif de reprise propre après interruption par limite d'usage
```

### Plan

- [>] **P1** — `notes/EN-COURS.md` : journal d'intention + procédure de reprise à chaud.
  *Thèse : ce qui est déclaré avant survit à une coupure ; ce qui est écrit après ne survit pas.*
- [ ] **P2** — `REPRISE.md` : jeton à trois états avec battement, §6 amendé, §7 reprise à chaud,
  §8 mis à jour (le dépôt est désormais sous git).
  *Thèse : le jeton doit distinguer « occupé » de « interrompu », sinon personne n'ose reprendre.*
- [ ] **P3** — `README.md` : mention du protocole dans les règles de tenue.
- [ ] **P4** — `notes/LECONS.md` : leçons généralisables de cette session.
- [ ] **P5** — rituel de fin : entrée de journal S07, index, jeton repassé à `libre`, ce fichier
  repassé à `terminée`.

### Notes de reprise

*(Vide au démarrage. Y déposer au fil de l'eau ce qui n'est pas encore dans un fichier.)*

- Le dépôt a été mis sous git au début de S07, commit de base `c6886a7`. Avant lui, aucun
  historique n'existe : ne pas chercher de trace des sessions S01 à S06 dans `git log`, elles sont
  toutes dans ce seul commit et dans `notes/JOURNAL.md`.
- Seuil de battement retenu pour présumer une interruption : **2 heures**. Choisi parce que les
  limites d'usage se réinitialisent à cette échelle ; plus court, deux sessions se marchent
  dessus ; plus long, on attend pour rien. Convention, pas mesure.
