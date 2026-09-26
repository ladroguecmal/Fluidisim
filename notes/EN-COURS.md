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

Session : S375 — **en cours**. **δ 3D dans le bassin** de la piscine de S374 : *« La dynamique de fluide doit se faire
en 3D volumétrique »* (ADR-200). À deux maillons : viser **5.10** (articulation V↔δ), absent → partiel.
Agent : Claude Opus 5.5, application desktop ; fichiers, git, cargo, carte réelle, accès web ; Godot 4.4.1 local.

**Thèse.** Le pas à surface mobile de `Volume3` (S296 ; murs sur les six côtés, fond plat) porte l'intérieur du bassin.
Il lui manque **une entrée de volume par colonne** qui ne remette pas la pression à zéro (`set_free_surface` le fait) :
`add_column_volume`, avec la compensation d'arrondi du transport. Par elle passent, à chaque pas de δ : le **jet** de la
pompe (volume sur la colonne d'impact, et sa quantité de mouvement dans les mailles sous l'impact), le **puits** du
déversoir (la bande de colonnes contre le mur est), et le **forçage vers V** (ADR-025 : `(M_V − M_δ)·dt/τ`, τ = 1 s,
uniforme). V garde la masse ; δ fait le mouvement. La lame et le jet dans l'air restent pour APIC (ADR-200 D3).

**Critères, écrits avant.** (1) `add_column_volume` : le volume de δ change **exactement** de `Σ dh·dx²` (à l'arrondi
f32 compensé, ≤ 10⁻⁹ m³ par pas) ; refus atomique hors bornes ; sans entrée, δ au repos reste au repos (au bit). (2)
Dans la piscine : le niveau moyen de δ suit celui de V à **1 mm** près au-delà de 5 τ. (3) Physique : le front de l'onde
née de l'impact atteint le mur opposé au temps des ondes longues, `d/√(g·h)`, **à ±15 %** (repéré au premier dépassement
de 1 mm). (4) Dans Godot, la surface rendue est celle de δ, au bit du fichier ; jugement de l'utilisateur (R27).

### Plan

- [x] **P1** — jeton, plan seul.
- [x] **P2** — `Volume3::add_column_volume` et ses essais ; critère 1.
- [x] **P3** — `examples/piscine_delta.rs` : V et δ au pas, jet, puits, forçage ; coût mesuré ; export des surfaces ;
  critères 2 et 3.
- [x] **P4** — Godot : la surface de δ en maillage de hauteur dans le bassin (`piscine.gd`), rejouée ; critère 4.
- [ ] **P5** — images de R27, preuve `PISCINE-DELTA-S375`, liste 5.10, file, feuille de route, index.
- [ ] **P6** — rituel.

### Notes de reprise

**P2 — fait.** `Volume3::add_column_volume(dh)` (`delta3d_mobile.rs`) : ajout compensé comme le transport, pression et
vitesses intactes, refus atomiques (`Shape`, `NotFinite`, `Domain`). **Critère 1** : 200 ajouts, écart de volume **0**
(au bit), repos au repos après 20 pas, refus sans écriture.

**P3 — en cours, trouvé en chemin.**
- `support/piscine.rs` : la piscine définie une fois ; `piscine_v` rebranché, **export identique au bit**.
- **Défaut de la référence mobile 3D** : toute surface décalée uniformément du repos (≥ 1 µm) est refusée
  (`Convergence`) sur tout domaine — le gradient conjugué converge (36–46 itérations), mais le critère de divergence
  divise deux arrondis (vitesses 10⁻¹⁰ m/s) : 2 à 4. C'est le cas même d'ADR-025 (V élève le niveau d'un bloc).
  **ADR-201** : plancher de l'échelle de vitesse, 10⁻⁴ m/s ; 76 essais δ 3D inchangés ; essai
  `a_uniform_rise_is_a_state_without_motion_s375` (vitesses 1,5·10⁻¹⁰ et 1,2·10⁻⁹ m/s, surface uniforme au bit).
- **Coût à 10 cm** (80 × 40 × 16) : ≈ 100 itérations, ≈ 0,5 s par pas sur la référence séquentielle — deux heures ; 20 cm.
- **Premier essai du jet** (quantité de mouvement entière dans les 30 cm du haut, sans dissipation) : la surface sort du
  domaine à t = 7,4 s (2,4 s après le lancement) — courant accéléré sans fin (≈ 56 N sur quelques décilitres), énergie
  sans puits. **Panache à calibrer** : σ 20 cm, 60 cm de profondeur, verticale seule, horizontale dissipée ;
  amortissement 0,5 s dans le panache, 30 s partout.

**P3 — interrompue par la limite d'usage (2026-09-26, ≈ 04:25), à reprendre à chaud.** Committé dans ce commit :
- `Volume3::shift_rest` (le repos suit le niveau de V ; essai `shifting_rest_to_the_mean_level_changes_no_physics_s375`,
  surfaces à un ulp, pression moyenne 78,4 → −2,0 Pa ; seuil écrit d'abord à 10⁻⁷ m, **sous la résolution f32** à 2 m,
  réécrit en deux ulps et dit) ; `last_refused_report` (diagnostic). Cause du refus à t = 305,4 s : plancher f32 atteint,
  divergence 1,06·10⁻⁵ pour 10⁻⁵ — le décalage de 8,5 mm au repos d'origine (83 Pa uniformes).
- `examples/piscine_delta.rs` : **exécution complète** 330 s, 13 200 pas, 357 s, 27 ms/pas, 62 itérations en moyenne,
  0 refus. **Critère 2 tenu** (≤ 0,0001 mm). **Critère 3 manqué** : 2,475 s pour 1,759 (+40,7 %) au seuil de 1 mm ;
  diagnostic sur l'export (le critère reste manqué) : 0,3 mm → 1,70 s (−3 %) ; 0,1 mm → 1,50 s, précurseur incompressible
  plus rapide que √(g·h). Amplitudes : écart-type 0,2–0,36 mm, pire 1,5–4,9 mm. Export 6 600 images, 10,6 Mo.
- Godot : maillage de hauteur de δ (`piscine.gd`, `bassin.gdshader`) ; **critère 4 tenu** (460 hauteurs, 0 écart).
  18 images faites (`godot/captures/piscine_*`), **pas encore regardées**.
**Reste** : le contrôle S374 compare la cote du maillage à V — faux en mode δ (le maillage est au repos) : ne le faire
qu'avec `DELTA=0` ; regarder les images ; suite complète du cœur (après `shift_rest`) ; P5 (preuve `PISCINE-DELTA-S375`,
ADR-201 note sur `shift_rest`, liste 5.10, file, feuille de route, index, R27) ; P6 rituel.

**Reprise à chaud (09:53)** : arbre propre, l'étape interrompue était committée ; complétée. Contrôle de S374 limité au bac
tampon en mode δ (`DELTA=0` : entier) — les deux modes tenus. Suite du cœur : 539 réussis, 0 avertissement.
**Les images** : à l'échelle réelle, **la surface de δ paraît plane** (vues ensemble, déversoir, buse, rasante) ; cartes de
hauteur (données) : dôme à l'impact (5,3 s), creux (5,6 s, −4,7 mm), anneaux et réflexions (6,5–8 s), interférences
(40 s), creux stable sous le jet (200 s), oscillations résiduelles (260 s) ; écart-type 0,2–0,46 mm. **Témoin
`EXAGERE=100`** : creux et anneaux visibles — la chaîne de rendu est bonne ; l'amplitude physique (mm, maille de 20 cm,
panache amorti) ne se voit pas. Suite : δ sur GPU dans Godot à 5–10 cm, panache calé sur une mesure, APIC pour le jet.
