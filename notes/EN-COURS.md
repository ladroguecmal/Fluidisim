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
Session          : S21
État             : en cours
Battement        : 2026-09-05
Objectif         : H3 — les cas canoniques analytiques et le mode `physics`
```

### Plan

H1 vérifie que le code est **reproductible**. Il ne vérifie pas qu il est **juste** : un hash stable
peut être stable et faux. H3 est le premier étage qui confronte le code à des références
**extérieures** — des solutions fermées que rien de ce que j écris ne peut influencer.

- [ ] **P1** — plan, jeton.
- [x] **P2** — mode `physics` : cadre d assertions, mesure contre référence, tolérance déclarée.
- [x] **P3** — les cas analytiques que `B` seul permet, et ils sont plus nombreux qu il n y paraît :
  dispersion **mesurée sur le champ** et non lue dans la configuration, restitution de `Hs` par la
  variance, identité de la vitesse orbitale, pente maximale, homogénéité spatiale.
  *Thèse : au moins un de ces cas va échouer. Une référence analytique n a d intérêt que si elle
  peut me contredire, et je n ai jamais vérifié la cinématique de `B` autrement qu en la relisant.*
- [x] **P4** — exécuter, constater, corriger.
- [ ] **P5** — **C10, le cube flottant** : premier calcul de force, et première référence fermée sur
  autre chose que la cinématique — tirant d eau `d = m/(ρA)`.
- [ ] **P6** — notes correctives, index, angles morts.
- [ ] **P7** — rituel de fin.

### Notes de reprise

- **Ce que H3 ne couvrira pas, et pourquoi.** Douze des seize cas canoniques ont une référence
  fermée ; la plupart demandent `W` ou `δ`, qui n existent pas. H3 livre donc les cas que la couche
  `B` permet, et **dit lesquels attendent quoi**. Un H3 qui prétendrait couvrir C04 sans solveur
  serait un H3 qui ment.
- **Le cas le plus fort est la dispersion mesurée sur le champ**, et non lue dans les paramètres :
  `configure` calcule `k = ω²/g`, donc vérifier `k` contre les paramètres ne prouverait que ma
  propre arithmétique. Mesurer une longueur d onde et une période **dans le champ échantillonné**,
  puis vérifier `λ = gT²/2π`, teste l implémentation entière — phases, sinus, conversion de
  position.

#### P2 à P4 — la thèse était juste, et le cas analytique a trouvé le bug

**Quatre échecs au premier passage. Un seul venait du cœur.**

**Le bug du cœur** : la vitesse orbitale était **en quadrature au lieu d en phase** avec
l élévation. Airy en eau profonde donne u = a·ω·sin(φ) — en phase avec η — et w = a·ω·cos(φ). Mes
deux lignes étaient inversées. Conséquence physique : **sous une crête, l eau n avançait pas**, elle
montait. Le cas `u/η = ω` l a fait tomber au premier passage ; la relecture ne l avait jamais vu, et
le hash de H1 était parfaitement stable — stable et faux, ce que H1 ne peut pas distinguer.

**Deux bugs de mes tests** : un temps renvoyé en secondes et réadditionné à l instant de départ
(période mesurée : 10¹⁵ s) ; et un contrôle d homogénéité échantillonnant à 5 000 m, au-delà du rayon
de référentiel. **Le champ avait raison** — I-08 refuse au-delà de 4096 m, et `eval` renvoyait
`None`. La propriété révélée par mon erreur méritait son propre cas : elle en a un.

Après correction, **8 cas sur 8** sur le scénario monochromatique. La dispersion, mesurée entièrement
dans le champ — longueur d onde par passages à zéro, période par passages à zéro en un point fixe —
retrouve λ = gT²/2π **à 0,000 %**.

**Et le mécanisme de non-régression a fonctionné** : le champ ayant changé, les deux hashs de
conformité sont tombés. C est l effet recherché, et la bénédiction se fait dans un commit séparé.

**Limite de mesure à consigner** : sur le scénario à 32 composantes, la restitution de Hs par la
variance donne 8,5 % d écart. Ce n est pas un défaut du champ mais de la **fenêtre** — la plus longue
composante fait 225 m de long et la fenêtre 384 m, soit 1,7 longueur d onde. Une estimation de
variance a besoin de plusieurs longueurs d onde de la **plus longue** composante.
