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

Session : S123 — en cours
Agent : Claude Code (Opus 5 ; fichiers, git et cargo disponibles)
Objectif : A199 — le couloir d'acceptation du candidat radial, mesuré en S122, n'a jamais été
confronté aux impacts que le jeu produira. Établir quels régimes il couvre, lesquels il ne
couvre pas, et dire ce qui manque plutôt qu'élargir une borne au hasard.

**Session de conception**, pas de construction : la première depuis longtemps. Le bilan S69
reprochait aux sessions S47–S68 de n'avoir produit aucune conception du système d'eau ; la
mesure est ici au service d'une décision, pas l'inverse.

### Plan

- [x] **P1** — amorce, jeton, plan.
- [x] **P2** — inventaire (L209) : que disent SPEC-002, ADR-058 et ADR-060 du choix de la
      longueur d'onde et de l'usage visé ? **D'où vient `wavelength_m`** — dérivée, ou
      déclarée par l'appelant ? Aucune conclusion avant cette lecture.
- [x] **P3** — la relation entre un impact réel et la longueur d'onde qu'il engendre, avec
      provenance (I-14) : formule citée du corpus, ou étiquette « à calibrer » et le banc.
- [x] **P4** — le catalogue des cas du jeu confronté au couloir, **mesuré** et non supposé
      (L210) : une sonde qui construit le candidat pour chaque cas et rend le verdict.
- [x] **P5** — ADR-083, sur ce que la confrontation aura montré.
- [ ] **P6** — livrable, rituel de fin, fusion `--ff-only`.

### Notes de reprise

Départ c208d87 = master ; trois copies coïncidentes, 5134cd archivée, c107bf sur la ligne S44.

Le couloir mesuré en S122 (livrable §4) : ondes du mètre à la dizaine de mètres. Les bornes
dominantes tirées de la lecture du code — à revérifier en P4 plutôt qu'à croire :
`Reach` impose `hi·radius ≤ 64` avec `hi = 4π/λ`, donc **rayon ≤ ~5,09 λ** ; `Regime` impose
**profondeur > λ** ; `Resolution` fait entrer l'horizon.

Deux nombres déjà dans le corpus et utilisables : `λ = 2πv²/g` (64 m à 10 m/s, S01) et la
cambrure limite `H/λ = 0,78`. Ne pas en inventer d'autres sans provenance — I-14.

Piège : la tentation sera d'élargir une borne pour faire entrer un cas. Une borne existe pour
une raison (portée tabulée, régime d'eau profonde, résolution de phase) ; la déplacer sans
traiter cette raison remplacerait un refus franc par un résultat faux.

P2/P3, et le premier résultat précède la question posée : **la longueur d'onde ne vient de
nulle part**. ADR-055 la valide comme « positive en mètres » et reporte au « générateur
physique » qui n'existe pas ; ADR-060 dit que la bande est « à calibrer B2 » et que son
lambda=4 m est un « paramètre d'essai uniquement ». Le registre ne contient rien là-dessus.
Le candidat est donc piloté par une grandeur que personne ne sait produire, et parler de cas
qui « tombent dans le couloir » n'a pas de sens tant que ce lien manque.

Écriture retenue, conforme à I-14 : lambda = alpha * b, avec b la demi-largeur de l'objet
(SPEC-001 §5 bis, Wagner : l'étendue mouillée à la fin de l'impact vaut b, indépendamment de
v et beta) et alpha **à calibrer, banc B2**. La session ne fixe pas alpha ; elle mesure la
**sensibilité du verdict à alpha**, ce qui ne demande pas de le connaître.

`lambda = 2*pi*v^2/g` (SPEC-001 §5) écartée : elle décrit le sillage d'un mouvement horizontal
établi, pas une entrée verticale. L'employer ici serait un détournement.

Substitution dans les bornes : radius <= 5,09*alpha*b (Reach), depth > alpha*b (Regime).
Donc portée et profondeur requises sont toutes deux **proportionnelles à la taille de l'objet**.

P4 : `impact_envelope.rs`, onze cas, quatre valeurs d'alpha. **Un seul cas sur onze se construit
à la portée voulue** (vaisseau en haute mer), et le verdict ne dépend pas d'alpha — c'était
l'objet du balayage.

Trois chiffres qui portent la session :
- **portée = 5,09 lambda = 10,18 b** exactement, au-dessus de ~15 cm ; en dessous c'est
  `Resolution` qui mord avant. Un plongeon humain n'est calculable que dans 3 m.
- **vaisseau en port : aucune portée**, `Regime` — le modèle suppose l'eau profonde (ADR-059).
- **plafond d'énergie : 1e-6 à 1e-2** de l'énergie de référence (masse ajoutée ~rho b^3 à la
  vitesse d'entrée, SPEC-001 §5 bis). Ne dit pas que le candidat est insuffisant : la fraction
  réellement transférée n'a jamais été établie (ADR-055 la reporte). Donne le seuil que le
  générateur devra respecter.

La limite de portée est **numérique, pas physique** : table de Bessel tabulée jusqu'à x=64, et
ADR-060 range explicitement cette limite parmi les « choix numériques testés, pas des paramètres
gameplay ». Piste pour la lever : développement asymptotique J0(x) ~ sqrt(2/(pi x)) cos(x-pi/4)
au-delà de la table. **À ne pas décider sans mesurer** (L209/L210) — c'est une action, pas une
décision de cette session.

Erreur de méthode évitée de justesse et consignée dans le livrable : la première sonde prenait
1e-9 J pour « négligeable », ce qui faisait lire un refus de pente comme une impossibilité
géométrique. Il a fallu descendre à 1e-30.
