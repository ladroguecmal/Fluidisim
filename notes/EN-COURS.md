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
État             : terminée
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
- [x] **P2** — **encadrer `λ_cut` par le corpus seul**, sans aucune mesure.
  *Thèse : ADR-005 §5 donne `L_s = λ_cut/2` et S08 (écart E08) donne une contrainte sur `λ_cut/dx`.
  Les deux bornent `λ_cut` par le haut et par le bas. Si elles ne se croisent pas, B2 n'a pas de
  réponse admissible sous l'hypothèse d'un `λ_cut` global — et il vaut mieux le savoir avant de
  monter le banc que pendant.*
- [x] **P3** — dossier `B2` §1–3 : ce que le banc décide et ce qui en dépend · les candidats et ce
  qu'on sait déjà d'eux sans mesurer · l'encadrement de P2.
- [x] **P4** — dossier `B2` §4–6 : les scénarios en fichiers concrets · le protocole iso-qualité
  appliqué · les deux critères de recevabilité.
- [x] **P5** — dossier `B2` §7–9 : la procédure de décision, les préalables, les pièges de mesure.
- [x] **P6** — notes correctives dans les documents touchés, et les points ouverts que le dossier
  ferme ou déplace.
- [x] **P7** — index, angles morts, décomptes.
- [x] **P8** — rituel de fin (`REPRISE.md` §6) : journal S16, leçons, index, jeton libéré.

### Notes de reprise

- **Forme retenue** : un document de `docs/validation/`, `DOSSIER-B2.md`, et non un ADR. B2 ne décide
  rien par lui-même : il produit une mesure. Le dossier est un mode d'emploi, pas une décision — et
  `PLAN-BENCHMARK` reste la vue d'ensemble des onze bancs, qu'il ne remplace pas.
- **Attente honnête** : si l'encadrement de P2 se referme proprement autour de la valeur proposée de
  4 m, le dossier sera utile mais sans surprise. S'il ne se referme pas, c'est la trouvaille de la
  session, et elle change le protocole du banc.

#### P2 — l'encadrement de `λ_cut`, et il ne se referme pas

**Borne haute — l'éponge.** ADR-005 §5 : « l'éponge est dimensionnée par `λ_cut` », avec
`L_s = λ_cut/2` par face. Sur un domaine de largeur transverse `W`, l'intérieur utile vaut
`W − λ_cut` (deux faces opposées). Appliqué aux trois domaines de référence de SPEC-001 §2.3 et
§2.4 :

| Domaine | Emprise | `λ_cut` = 3 m | 4 m | 6 m | 10 m |
|---|---|---|---|---|---|
| Impact | 6 × 6 m | intérieur 50 % | **33 %** | 0 % | — |
| Bateau | 24 × 12 m | 75 % | 67 % | 50 % | 17 % |
| Déferlement | 120 × 20 m | 85 % | 80 % | 70 % | 50 % |

En surface au sol, c'est le carré de ces fractions : à `λ_cut = 4 m`, un domaine d'impact est
**89 % d'éponge**. La borne haute est donc dictée par le **plus petit** domaine, et elle est serrée :
`λ_cut ≤ 3 m` pour qu'un domaine d'impact garde la moitié de son emprise.

**Borne basse — l'échantillonnage du champ de fond.** Écart E08 (S08) : le fond est échantillonné
une cellule sur quatre puis interpolé, et la validité tient au rapport `λ_cut/dx`. À 40, dix points
par longueur d'onde après décimation : confortable. À 16, quatre points : deux fois Nyquist, et
l'interpolation perd une fraction notable de l'amplitude du terme source. Il faut donc `λ_cut`
**grand devant `dx`**, et le `dx` le plus grossier est celui du déferlement, 0,25 m :

| `dx` | `λ_cut` = 3 m | 4 m | 6 m | 10 m |
|---|---|---|---|---|
| 0,25 m (déferlement) | ratio 12 → **3 pts** | 16 → 4 pts | 24 → 6 pts | 40 → **10 pts** |
| 0,10 m (bateau) | 30 → 7,5 pts | 40 → 10 pts | 60 → 15 pts | 100 → 25 pts |
| 0,05 m (impact) | 60 → 15 pts | 80 → 20 pts | 120 → 30 pts | 200 → 50 pts |

**Les deux bornes ne se croisent pas.** Le déferlement veut `λ_cut ≈ 10 m` pour que la décimation
×4 reste valide ; l'impact veut `λ_cut ≤ 3 m` pour garder un intérieur utile. **Aucune valeur
globale ne satisfait les deux**, et l'écart est d'un facteur trois.

**Ce que cela ne signifie pas.** Ce n'est pas une impasse d'architecture. Les deux contraintes ne
pèsent pas sur les mêmes domaines, et elles ne pèsent pas de la même façon :

- l'éponge est une contrainte **dure** : à `λ_cut ≥ 6 m`, un domaine d'impact n'a **plus d'intérieur
  du tout**. Il n'y a pas de compromis possible, seulement un domaine inutile ;
- la décimation est une contrainte **de coût** : à quatre points par longueur d'onde, on n'a pas un
  résultat faux, on a une économie qui s'effondre. C'est ce qu'E08 disait déjà — « l'économie
  disparaît dans le type de domaine le plus gros ».

**Résolution proposée, à porter au protocole de B2 : `λ_cut` reste global, c'est le taux de
décimation qui s'adapte.** E08 avait écrit la contrainte sous la bonne forme — `dx ≤ λ_cut/N` — en
laissant `N` libre. `N` est donc le paramètre, et non `λ_cut` :

| Domaine | `λ_cut/dx` à 4 m | Décimation admissible | Facteur d'économie |
|---|---|---|---|
| Impact, `dx` 0,05 | 80 | ×4 par axe | **64** |
| Bateau, `dx` 0,10 | 40 | ×4 par axe | **64** |
| Déferlement, `dx` 0,25 | 16 | ×2 par axe, au mieux | **8** |

**Conséquence directe sur le protocole du banc** : B2 doit mesurer le coût du terme source dans une
zone de déferlement **à décimation réduite**, et non au facteur 64 nominal. Sans cela, le coût de δ
en régime substitutif sera sous-estimé d'un facteur voisin de huit — sur le domaine qui compte
384 000 cellules, c'est-à-dire le plus gros du corpus.

**Et l'encadrement se referme, une fois `N` libéré** : `3 m ≥ λ_cut` par l'éponge sur le domaine
d'impact, `λ_cut ≥ 2,5 m` pour que le déferlement garde seulement dix cellules par longueur d'onde
avant décimation. La fenêtre est **2,5 à 3 m**, et la valeur proposée depuis S01 — **4 m** — est
au-dessus. Ce n'est pas une réfutation : c'est une hypothèse chiffrée que le banc doit trancher, et
c'est exactement ce qu'un dossier d'exécution doit apporter avant qu'on monte le banc.

**Écart trouvé en chemin, à signaler.** ADR-005 §5 conclut que « l'éponge représente ≈2 m sur un
domaine de 20 m, soit **≈27 % du volume en 3D** ». Ce chiffre n'est pas reproductible à partir de ce
que le paragraphe donne : selon les faces qui portent l'éponge, la même géométrie donne **20 %**
(deux faces), **36 %** (quatre faces) ou **49 %** (six faces). Le document ne dit pas lesquelles.
La grandeur n'est pas anecdotique — c'est le coût d'entrée de tout domaine — et elle fonde la borne
haute ci-dessus.

#### P3 à P5 — dossier écrit d'un seul tenant

`docs/validation/DOSSIER-B2.md`, dix sections. Le découpage du plan en trois étapes s'est révélé
artificiel : les sections se tiennent, et couper au milieu aurait produit trois commits dont aucun
n'était lisible seul. Fait en un, et déclaré ici.

Trois apports que le protocole d'origine n'avait pas :
- **B2-05**, scénario d'arrivée en cours de partie. L'ajout S04 de PLAN-BENCHMARK demandait de
  mesurer le volume d'état à transmettre ; aucun des quatre scénarios existants ne l'exerce.
- **La métrique d'iso-qualité est nommée** : l'erreur de célérité relative sur C02, intégrée sur
  [λ_cut, 4·λ_cut]. SPEC-003 §5.2 imposait d'en choisir une ; B2 ne l'avait pas.
- **B2 produit un couple, pas un nombre** : `λ_cut` **et** le tableau des décimations admissibles
  par classe de domaine. L'un sans l'autre ne veut rien dire.

Et une vérification faite plutôt que supposée : le plus court phénomène gameplay ondulatoire du
corpus est le **sillage à 5 m/s, 16 m**. Le critère de fermeture d'ADR-021 §3.2 laisse donc de la
marge jusqu'à λ_cut ≈ 6 m — ce n'est pas lui qui mord, c'est l'éponge du domaine d'impact.
