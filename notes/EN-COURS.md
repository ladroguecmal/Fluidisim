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
Session          : S16
État             : en cours
Battement        : 2026-09-05
Objectif         : préparer l'exécution de B2 — le banc qui fixe `λ_cut`, « à décider en premier »
                   depuis S01 et qui débloque quatre points ouverts
```

### Plan

Le corpus n'a plus de classe de contrôle non passée. Ce qui reste est du code, des mesures et des
réunions. Le plus utile de ce que je peux encore produire est donc le **dossier d'exécution** du banc
le plus rentable : tout ce qu'il faut pour le lancer le jour où le harnais existe, et surtout tout ce
qu'on peut déjà savoir **sans mesurer**.

- [ ] **P1** — déclarer le plan, prendre le jeton, mettre à jour le battement.
- [ ] **P2** — **encadrer `λ_cut` par le corpus seul**, sans aucune mesure.
  *Thèse : ADR-005 §5 donne `L_s = λ_cut/2` et S08 (écart E08) donne une contrainte sur `λ_cut/dx`.
  Les deux bornent `λ_cut` par le haut et par le bas. Si elles ne se croisent pas, B2 n'a pas de
  réponse admissible sous l'hypothèse d'un `λ_cut` global — et il vaut mieux le savoir avant de
  monter le banc que pendant.*
- [ ] **P3** — dossier `B2` §1–3 : ce que le banc décide et ce qui en dépend · les candidats et ce
  qu'on sait déjà d'eux sans mesurer · l'encadrement de P2.
- [ ] **P4** — dossier `B2` §4–6 : les scénarios en fichiers concrets · le protocole iso-qualité
  appliqué · les deux critères de recevabilité.
- [ ] **P5** — dossier `B2` §7–9 : la procédure de décision, les préalables, les pièges de mesure.
- [ ] **P6** — notes correctives dans les documents touchés, et les points ouverts que le dossier
  ferme ou déplace.
- [ ] **P7** — index, angles morts, décomptes.
- [ ] **P8** — rituel de fin (`REPRISE.md` §6) : journal S16, leçons, index, jeton libéré.

### Notes de reprise

- **Forme retenue** : un document de `docs/validation/`, `DOSSIER-B2.md`, et non un ADR. B2 ne décide
  rien par lui-même : il produit une mesure. Le dossier est un mode d'emploi, pas une décision — et
  `PLAN-BENCHMARK` reste la vue d'ensemble des onze bancs, qu'il ne remplace pas.
- **Attente honnête** : si l'encadrement de P2 se referme proprement autour de la valeur proposée de
  4 m, le dossier sera utile mais sans surprise. S'il ne se referme pas, c'est la trouvaille de la
  session, et elle change le protocole du banc.
