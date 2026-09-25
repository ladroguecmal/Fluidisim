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

Session : S362 — **en cours**. **Physique : la bathymétrie, 1 — la référence** (liste 2.7).
Agent : Claude Opus 5.5, application desktop ; fichiers, git, cargo, carte réelle, accès web ; Godot 4.4.1 local.
Entrée — *« Continue »*, sans verdict R21/R22 (attendus). **Choix du lot, à deux maillons** (REPRISE §6) : la suite
proposée — la coque qui bouge (6.4) ou le lot 5 — laisserait chaque point dans sa case ; **2.7, la bathymétrie**, est
*absente*, vingt points en dépendent (registre), et le registre la dit faisable : « profondeur finie et fond variable, sur
cas de référence publiés ». La scène côtière de S359 la montre manquer : B ne voit pas le fond. **Ce qui ne se tranche
pas ici** : où la bathymétrie entre — ADR-004 §2.1 garde les composantes de B identiques partout et place réfraction et
levée dans W (§5) ; ADR-054 et ADR-156 renvoient le choix à B2 et J5. Cette session construit la **référence** — la
physique que tout candidat devra reproduire —, pas l'intégration.

Critères, écrits avant le code (module `bathymetrie` du cœur, f64, référence et non chemin déterministe) :
1. **Dispersion en profondeur finie** `ω² = g·k·tanh(k·h)` : résidu relatif ≤ 10⁻¹² ; limites profonde (`k = ω²/g`) et
   peu profonde (`k = ω/√(gh)`) à 10⁻⁶ près dans leurs domaines ; écart à l'approximation explicite de **Fenton et McKee
   (1990)** au plus **1,7 %** — la borne que ses auteurs publient.
2. **Levée** `K_s = √(c_g0/c_g)` : son minimum, **0,913 vers k·h ≈ 1,2**, la valeur des manuels (Dean et Dalrymple).
3. **Réfraction sur contours droits** (Snell, `k·sin θ` conservé) et **flux d'énergie** `a²·c_g·cos θ` constant à 10⁻¹⁰
   près le long d'un profil de plage ; la phase `∫k_y dy` a pour dérivée `k_y` à 10⁻⁶ près.
4. **Déferlement** borné par profondeur, `H ≤ 0,78·h` (McCowan 1894) ; la profondeur de déferlement d'une houle se
   calcule et se publie.
5. Preuve ouverte par « Reproduire » ; liste 2.7 (**absent → partiel** si 1 à 4 tiennent), registre, file, index.

### Plan

- [x] **P1** — jeton, plan seul.
- [x] **P2** — le module `bathymetrie` : dispersion, vitesse de groupe, levée, réfraction, phase, déferlement.
- [x] **P3** — ses essais contre les résultats publiés ; critères 1 à 4.
- [ ] **P4** — preuve, liste, registre, file, index ; critère 5.
- [ ] **P5** — rituel.

### Notes de reprise
- **P2.** `code/water-core/src/bathymetrie.rs` (f64, référence) : `nombre_d_onde` (Eckart puis Newton),
  `rapport_de_groupe`, `vitesse_de_groupe`, `coefficient_de_levee`, `transformer` (Snell, `K_s`, `K_r`),
  `phase_transversale` (Simpson), `profondeur_de_deferlement` (McCowan 0,78, pas de 1 % puis dichotomie), `MCCOWAN`.
  Calculé en Python avant l'essai : Fenton–McKee au pire 1,63 % à k₀h = 0,34 ; levée minimale 0,91299 à kh = 1,1997.
- **P3, critères 1 à 4 tenus.** `tests_bathymetrie.rs`, `cargo test -p water-core --release --offline s362` :
  résidu de dispersion **4,39·10⁻¹⁶** ; profond 0 ; peu profond (kh ≈ 2·10⁻³) 6,71·10⁻⁷ ; **Fenton–McKee 1,63 %** ≤ 1,7 % ;
  **levée minimale 0,91299 à kh = 1,1995** (0,913 des manuels) ; plage 1/50, 8 s, 30° : Snell 1,1·10⁻¹⁶, flux
  d'énergie 4,9·10⁻¹⁶, dérivée de la phase **3,27·10⁻⁸** ; 30° au large, **10° par 2 m de fond** ; déferlement d'une
  houle de 1 m et 10 s : **h_b = 1,783 m**, H_b = 1,391 m (de face) ; 1,688 m à 30°. **L'instrument, deux fois corrigé**
  (le seuil jamais) : la différence centrée tombait au coin du profil (2,45·10⁻³), puis, à ±0,5 m, sa troncature
  `k_y''·d²/(6·k_y)` ≈ 3·10⁻⁶ par 2 m de fond dépassait le critère (mesuré 3,11·10⁻⁶) ; à ±5 cm, 3·10⁻⁸ prévu, 3,27·10⁻⁸
  mesuré.
