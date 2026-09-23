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
- **Ce fichier ne porte que la session en cours** ([ADR-187](../docs/adr/ADR-187-methode-refondue-s321.md)
  D3). À la clôture, ce qui doit survivre des notes va à la preuve ou au journal ; la session
  suivante remplace ensuite toute la section. Aucune section d'archive, 300 lignes au plus :
  `outils/etat_projet.py --check` le vérifie. Notes de S301 à S320 : `git show 78622a19:notes/EN-COURS.md`.

---

## Session en cours

Session : S333 — **en cours**. **La porte D** : un bateau flotte et perturbe l'eau qui le porte, sans
autorité de δ sur le jeu ; chemin de la v1 ([ADR-189](../docs/adr/ADR-189-la-v1-d-abord.md)).
Agent : Claude Opus 5.5, application desktop ; fichiers, git, cargo, carte réelle, accès web.
Entrée : *« continue, jusqu'à la v1 »* ; suite déclarée par S332.

**Ce que la session doit rendre possible.** La scène de la porte D, sur la référence CPU (ADR-178 D4, D5) :
une coque de jeu sur une **houle de B**, et δ qui porte **la perturbation qu'elle ajoute** — rayonnement de
son mouvement relatif à l'eau qui la porte. **La théorie linéaire dit comment la poser** : la coque se place
dans δ à sa position **relative** à la surface incidente, et sa paroi avance à la vitesse **relative** —
corps moins houle, prise au centre de la coque, valable pour une coque courte devant la longueur d'onde.
Prendre sa pose absolue compterait deux fois le mouvement de la houle. Puis des images, et **l'arrêt pour
le verdict visuel de l'utilisateur** : ADR-178 D3 exige les deux validations, ADR-189 D3 dit de s'arrêter.

Critères, écrits avant le code :
1. **B derrière la requête d'eau** : une coque sur une houle longue pilonne à l'amplitude que donne
   `1/(1 − ω²/ωₙ²)` — réponse d'un pavé droit sous Froude–Krylov hydrostatique — à ± 5 %.
2. **Au repos relatif**, pas de perturbation : une coque qui suit exactement la houle (vitesse relative
   nulle) laisse δ au repos au bit.
3. **I-04** : la trajectoire de jeu identique au bit avec ou sans δ, dans la scène de la porte D.
4. **Conservation** : le volume de δ suit celui de la coque plongée à 10⁻⁹ m³ près.
5. **La perturbation existe et reste bornée** : élévation de δ non nulle, sous l'amplitude de la houle.
6. **Images** de la surface totale — houle B plus perturbation δ — autour de la coque, à plusieurs instants.

### Plan

- [x] **P1** — jeton, plan seul.
- [x] **P2** — `BackgroundWater` : la requête d'eau sur B ; critère 1.
- [x] **P2 bis** — *ajouté à la reprise* : la poussée suit le gradient de la pression que le proxy suppose
  déjà, `ρg(η − z)` ; critère 1 bis.
- [x] **P3** — la coque de δ relative à l'eau qui la porte ; critère 2.
- [x] **P4** — banc `porte_d` : coque 4 × 1,6 × 1 m sur une houle ; critères 3 à 5, champs écrits.
- [x] **P5** — images de la scène (critère 6).
- [ ] **P6** — preuve : `docs/validation/PORTE-D-S333.md`, avec « Reproduire » ; feuille de route (porte D :
  partie numérique), liste, file.
- [ ] **P7** — rituel ; arrêt pour le verdict visuel.

**Amendement de la reprise, déclaré avant le code.**
- **P2 bis, pourquoi.** Le proxy intègre `p = ρg(η(x) − z)` mais n'en garde que la composante verticale :
  sur la houle, la coque monte et descend sans être entraînée — le défaut qu'ADR-008 §2 nomme
  « immédiatement perceptible » — et δ verrait un cavalement relatif de l'ordre de `a`, 0,26 m/s, qui
  n'existe pas pour un corps libre. La force par volume plongé est `ρg(−∇η, 1)`. **Critère 1 bis** : sur
  la houle longue, la coque libre cavale avec l'eau, excursion à ± 5 % de `a` ; en eau calme, essais
  S331–S332 inchangés.
- **P3 précisé.** La coque entre dans δ à sa pose relative au **repère de l'eau qui la porte** :
  déplacement de la particule de surface au centre (`ξ`, `η`) et inclinaison de la surface ; vitesses de
  paroi par **différence finie de cette pose**, en f32 — découpe et paroi cohérentes au bit, pose
  constante pour une coque qui suit l'eau (critère 2).
- **P4 précisé.** Sur une houle longue, la perturbation d'une coque qui suit l'eau est d'ordre `kd`
  (~1 cm), invisible et du même ordre que l'erreur de l'approximation de coque courte. La coque est donc
  **lâchée 10 cm au-dessus de son équilibre** : son pilonnement relatif rayonne des anneaux que δ porte.

### Notes de reprise

- **Reprise à chaud, 2026-09-24 00:00** (nouvelle conversation, *« Reprends le projets »*) : la session
  s'était arrêtée à 23:47, P2 commencé sans `[>]` ; diff de 18 lignes, `BackgroundWater` dans
  `rigid_body.rs`, cohérent avec la thèse de P2 → **compléter**. Aucune autre session active (vérifié).
- **P2, critère 1 tenu.** Coque 4 × 1,6 × 1 m, 500 kg/m³, proxy 16 × 8 × 4, lâchée sur le régime forcé :
  houle 6 s (λ 56 m, 25 cm) → pilonnement **1,04903·a** ; `1/(1 − ω²/ωₙ²)` = 1,05767, écart −0,82 % (± 5 %) ;
  prédiction avec la houle vue par la flottaison `S = ⟨cos kx⟩` : 1,04892·a (10⁻⁴). Houle 3 s (λ 14 m) →
  1,11574·a pour 1,11576·a prédit, alors que `1/(1 − ω²/ωₙ²)` seul dirait 1,279 : **le critère 1 ne vaut
  que pour une houle longue**. Écart ponctuel au régime forcé 2,6·10⁻⁴ et 1,9·10⁻³ de Z.
- **P2 bis, critère 1 bis tenu.** Force `ρg(−∇η, 1)` par volume plongé ; S331–S332 inchangés (mêmes
  valeurs imprimées). Houle 6 s : trajectoire horizontale = `x₀ + U·t + c·ξ`, **c = 0,99059** pour
  `S` = 0,99172 (−0,11 %) ; **dérive U = 4,05 mm/s** dans le sens de la houle, second ordre — Stokes `a²ωk`
  = 7,3 mm/s ; publiée, pas jugée. Critère 1 re-mesuré avec cavalement, par moindres carrés sur la houle
  *sous la coque* : 1,04889·a pour 1,04892 prédit ; 3 s : 1,11914·a pour 1,11576 (+0,3 %).
  **Impasse évitée** : la demi-excursion comptait la dérive (1,157·a) puis le pilonnement libre que le
  cavalement excite au départ — non amorti, 5,2 % de Z à 3 s ; la projection les sépare.
- **P3, critère 2 tenu au bit.** `HullInDelta` : hauteur `z − η` et inclinaison lues au centre, position
  horizontale intégrée `Σ dt·(V − u)` — l'excursion eulérienne ne se lit qu'au second ordre près (une coque
  qui suit la particule verrait `ξ(x_b) ≠ ξ(α)`, 7 mm) —, pose arrondie en f32, paroi = différence finie.
  Coque qui suit la houle : pose constante, δ au repos au bit, 100 pas. Tenue immobile : 0,32 m/s, 8,2 cm.
  **Piège de mesure** : sur une coque qui perce le couvercle, les faces couvertes gardent des vitesses que
  rien ne lit (3,7 m/s) et les colonnes couvertes une hauteur de comptabilité ; mesurer sur les faces
  ouvertes (`open = 1`) et les colonnes au couvercle libre — vaut pour P4 et les images.
- **P4, banc `porte_d`** (houle 25 cm / 6 s, coque lâchée 10 cm au-dessus de l'équilibre relatif, δ 96 × 96 × 8
  à 25 cm, 800 pas de 10 ms). « Champs écrits » lu sous **I-17** : aucun champ δ n'est écrit ; mesures
  imprimées, images rendues en mémoire (P5). **Critère 3 tenu** : trajectoire au bit, 800 pas. **Critère 5
  tenu** : δ 13,7 cm au plus sur les colonnes libres (houle 25 cm), 0,61 m/s. **Critère 4 manqué au sens
  strict** : 2,28·10⁻⁹ m³ pour 10⁻⁹ — sans dérive, signe alterné ; borne d'arrondi du transport 1,8·10⁻⁵
  (rapport 1,3·10⁻⁴) ; **témoin** coque immobile, bosse 10 cm, 200 pas : 3,6·10⁻¹⁰ (rapport 6,3·10⁻⁴) → le
  plancher du transport f32, qui croît avec la grille (576 colonnes en S332, 9 216 ici), pas le couplage.
  Pilonnement relatif ±0,12 m ; décalage visuel 1,5 cm ; δ CPU **239 ms/pas**, 109 itérations.
- **P5, images** à 2, 4, 6, 8 s : scène 1600 × 600 — B + δ à l'échelle | B + 5·δ —, carte de δ 600 × 600
  (± 5 cm) ; `viewer/captures/s333`, PNG par `outils/apercu_ppm.py`. Empreintes scène / carte : 2 s
  `0xb9a974e189acbed2` / `0xa0366797d73e2f97` ; 4 s `0x32ca3288ed68258c` / `0x0cba3fe9c000a959` ; 6 s
  `0x4fd48091eeb78821` / `0x984edb9f4bddac25` ; 8 s `0x817b5a8d979abda7` / `0xe8a3a75362265bd9`.
  Rendu : lumière de côté, pas de reflet solaire — les pentes réelles de δ (~0,03) sont sous le seuil
  d'un ciel uniforme ; d'où le panneau ×5 (pratique de S297). **Pli droit au bord de δ** : murs, pas
  d'éponge en mode linéaire.
- **Trouvé en P5 → A317 (sévérité 2).** Premier placement : parois à 8 % et 52 % d'eau dans leurs mailles de
  bord en `y` → rayonnement symétrique à 2 s, **franchement dissymétrique à 8 s** (fort du côté de la lamelle
  de 8 %), δ max 13,7 cm. Le jeu, lui, ne roule pas (10⁻¹⁷ rad, y = 0 exact) : c'est δ. Placement 30 % / 30 %
  → symétrique ; δ max **9,4 cm** ; volume 3,8·10⁻⁹ (plancher 1,8·10⁻⁵). Scène retenue : 30/30 ;
  `--decalage-y 0.055` rend l'ancien au bit (carte 8 s `0xb614282e3e5cd161`). **Impasse écartée** : hypothèse
  d'un roulis paramétrique du jeu (ω_pilonnement 4,48 ≈ 2 × ω_roulis 2,45) — le proxy symétrique au bit ne
  l'amorce pas.
