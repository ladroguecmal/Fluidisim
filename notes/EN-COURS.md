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
- [ ] **P3** — `examples/piscine_delta.rs` : V et δ au pas, jet, puits, forçage ; coût mesuré ; export des surfaces ;
  critères 2 et 3.
- [ ] **P4** — Godot : la surface de δ en maillage de hauteur dans le bassin (`piscine.gd`), rejouée ; critère 4.
- [ ] **P5** — images de R27, preuve `PISCINE-DELTA-S375`, liste 5.10, file, feuille de route, index.
- [ ] **P6** — rituel.

### Notes de reprise

**P2 — fait.** `Volume3::add_column_volume(dh)` (`delta3d_mobile.rs`) : ajout compensé comme le transport, pression et
vitesses intactes, refus atomiques (`Shape`, `NotFinite`, `Domain`). **Critère 1** : 200 ajouts, écart de volume **0**
(au bit), repos au repos après 20 pas, refus sans écriture.
