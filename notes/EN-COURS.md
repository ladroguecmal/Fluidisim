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
Session          : S17
État             : en cours
Battement        : 2026-09-05
Objectif         : le dossier de réunion des onze destinataires extérieurs
```

### Plan

Tout ce qu'on demande à l'extérieur existe — dispersé sur 26 ADR et 7 spécifications. Rien n'est
présentable. C'est le dernier travail disponible qui ne demande ni mesure ni décision humaine
préalable.

- [ ] **P1** — déclarer le plan, prendre le jeton, mettre à jour le battement.
- [ ] **P2** — **classer par ce que la réponse débloque**, et non par l'importance du sujet.
  *Thèse : l'ordre de `00_INDEX.md` classe par gravité de conséquence. Le bon critère est
  l'irréversibilité — ce qui bloque la première ligne de code passe devant ce qui bloque le format
  d'une autre équipe, qui passe devant ce qui bloque un banc. Je m'attends à ce que l'ordre change,
  et notamment que « qui possède le harnais » remonte très haut : il conditionne H1, qui conditionne
  la première ligne du solveur.*
- [ ] **P3** — dossier §1–2 : comment le lire, et le tableau de synthèse ordonné.
- [ ] **P4** — dossier §3 : les fiches de rang 1 et 2 — ce qui bloque du code, ce qui bloque un
  format extérieur.
- [ ] **P5** — dossier §4 : les fiches de rang 3 et 4 — données à obtenir, cadrages.
- [ ] **P6** — dossier §5–6 : **ce que nous ne demandons pas** — la section qui évite les
  malentendus coûteux — et ce qui reste ouvert.
- [ ] **P7** — index, angles morts, décomptes.
- [ ] **P8** — rituel de fin (`REPRISE.md` §6) : journal S17, leçons, index, jeton libéré.

### Notes de reprise

- **Forme** : `docs/DOSSIER-REUNIONS.md`, à la racine de `docs/` et non dans un sous-dossier — c'est
  le seul document du corpus destiné à être **sorti du dépôt** et lu par quelqu'un qui n'y reviendra
  pas. Une fiche par destinataire, tenant seule, sans renvoi obligatoire.
- **Règle d'écriture propre à ce document** : chaque fiche porte **un chiffre**. Une demande sans
  chiffre se discute ; une demande avec un chiffre se traite. C'est L14 appliquée à une réunion.
