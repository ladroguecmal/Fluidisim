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
Session          : S08
État             : en cours
Battement        : 2026-09-05
Objectif         : recroiser les cinq SPEC entre elles (S05 n'avait confronté que les ADR)
```

### Plan

- [ ] **P1** — déclarer le plan, prendre le jeton, mettre à jour le battement.
- [x] **P2** — croisement chiffré SPEC-001 × SPEC-002 × SPEC-005 : toute valeur numérique
  apparaissant dans deux documents doit y valoir la même chose, ou l'écart doit être motivé.
  *Thèse : les chiffres recopiés d'un document à l'autre se périment en silence — S07 en a déjà
  trouvé un cas dans le README.*
- [x] **P3** — croisement SPEC-004 × SPEC-001/002 : chaque grandeur que les fiches chiffrées
  déclarent nécessaire doit être atteignable par une signature existante.
  *Thèse (L20) : une exigence qui n'a pas d'argument dans une signature n'est pas implémentable,
  et cela ne se voit qu'en confrontant les deux documents.*
- [ ] **P4** — croisement SPEC-003 × SPEC-004/005 : le harnais peut-il instrumenter ce que les
  interfaces exposent, et la cuisson réutilise-t-elle réellement le cœur qu'elle prétend réutiliser.
  *Thèse : un harnais qui exige une observation que l'interface ne permet pas de nommer est un
  harnais non écrivable (L19 pris à l'envers).*
- [ ] **P5** — rédiger `docs/registres/REVUE-CROISEE-S08.md` : écarts trouvés, gravité, résolution,
  et la liste des contrôles **passés sans écart** — sans elle la revue n'est pas vérifiable.
- [ ] **P6** — appliquer les résolutions : notes correctives datées dans les documents touchés,
  nouvel ADR si une décision change, angles morts enregistrés.
- [ ] **P7** — rituel de fin (`REPRISE.md` §6) : journal S08, leçons, index, jeton libéré.

### Notes de reprise

*(Vide au démarrage. Y déposer au fil de l'eau ce qui n'est pas encore dans un fichier.)*

#### P2 — croisement chiffré SPEC-001 × SPEC-002 × SPEC-005

**Contrôle de fond effectué avant tout croisement** : une quarantaine de valeurs de SPEC-001 et
SPEC-002 ont été recalculées depuis leurs formules — dispersion profonde et peu profonde, CFL et
sous-pas à 30 Hz, loi `dx⁻⁴`, comptages de cellules, énergie `E = ρgHs²/16`, fetch SMB, Kelvin,
`λ = 2πv²/g`, ulp `f32` du temps, Monahan, Weber, Stokes des bulles, Boyle, bilan du vide (14 % /
86 %), Stefan `h ≈ 0,035√FDD`, produit d'emportement, transmission acoustique (−29,5 dB), Snell et
atténuation optique. **Toutes justes.** Les deux fiches chiffrées sont saines ; les écarts trouvés
plus bas portent sur la provenance et sur les croisements, jamais sur l'arithmétique.

- **E01, gravité 3** — SPEC-001 §2.4. La colonne « cellules éparses » applique trois taux
  d'occupation différents sans les nommer : bateau 432 k = 1,73 M ÷ 4 ; impact 1,15 M = comptage
  **plein** (6×6×4 m à 0,05 = 1 152 000) ; déferlement 384 k = 768 k ÷ 2. §2.3 annonce « ≈4 » et un
  lecteur l'applique partout. Deux consommateurs : le contrôle de cohérence du budget 384 Mo, et
  SPEC-005 §6 (197 Mo par plage). Les deux conclusions tiennent, la provenance manque — I-14.
- **E02, gravité 2** — SPEC-005 §6 : « le volume 3D est ré-établi en 2 à 3 secondes ». Aucune
  provenance. La structure verticale d'un train de houle s'établit en ≈1 période, soit **4,4 à
  8,0 s** pour λ = 30 à 100 m (SPEC-001 §1). Et ce n'est pas anodin : la fenêtre de préparation
  utile vaut `t ≤ √(2·R/a_max)`, soit **7,8 s** pour R = 60 m (demi-longueur d'une zone de
  120 m) et `a_max = 2 m/s²` (ADR-013 §2). À 2–3 s la marge est confortable, à 8 s elle est nulle.
  Le chiffre doit être dérivé ou étiqueté « à calibrer ».
- **E03, dérivation nouvelle, pas un écart** — SPEC-002 §4 exige `Hs < 0,15 m` pour une formation
  en plaque ; SPEC-001 §4 donne `Hs(U10, F)`. Croisées, elles **bornent la glace par le fetch** :
  `F_max = g·(0,15/(0,0016·U10))²`, soit **3,4 km à U10 = 5 m/s**, 0,86 km à 10 m/s, 9,6 km à
  3 m/s. La glace en plaque est un phénomène de lac et de baie abritée, jamais de haute mer.
  Chiffre à porter à l'arbitrage n°2 (ADR-017, « le projet veut-il de la glace ? ») : il en réduit
  la portée à une classe de plans d'eau, ce qui change le coût de la réponse « oui ».

#### P3 — croisement SPEC-004 × SPEC-001/002

**Trouvaille structurante — le chemin manquant.** SPEC-004 spécifie deux chemins : le chemin
**tiré** (`EvalWaterBatch`, `sample_batch`) et le chemin de **branchement de solveur**
(`IFluidSolver`, `IWaveSolver`, services d'hôte). Trois documents en exigent un troisième, que
personne ne porte : un chemin **poussé**, où le système d'eau *publie* par tick, à basse fréquence,
des champs et des signaux que personne ne vient chercher point par point.

- ADR-014 §2 — champ de moussage `F(x,t)`, deux canaux, texture 2D ancrée au monde ;
- ADR-018 §1 — `TraversabilitySample` par cellule `HydroGrid`, publié « jamais en interrogation
  continue, sans quoi la navigation devient un consommateur majeur de `EvalWater` » ;
- ADR-016 §2 et §6 — bus d'événements audio, et trois champs à ajouter à `WaveEvent`.

C'est L22 en clair : trois ADR ont chacun inventé sa propre publication parce que le document qui
aurait dû fournir le mécanisme ne l'a pas. Écarts qui en découlent :

- **E04, gravité 1** — trois des quatre interfaces inter-équipes en attente d'accord humain
  (audio, IA/navigation, et la part écume du rendu) **n'ont aucune signature dans SPEC-004**, le
  document dont le titre est « signatures des interfaces » et dont le statut est « dernier document
  avant l'écriture de code ». On ne peut pas présenter à ces équipes le document censé porter leur
  interface. Aggravant : `WaveEvent` n'est défini nulle part dans SPEC-004 — il vit dans ADR-009 §2
  — alors qu'il traverse trois frontières (réseau, transduction δ→W, audio) et qu'ADR-016 §6
  demande de l'élargir de trois champs « avant de figer le format ».
- **E05, gravité 2** — SPEC-002 §5 : `HR = d·(v + 0,5)`, où `v` est un **courant**.
  `WaterSample.u` de SPEC-004 §2 est explicitement « orbitale + courant », et aucun champ ne les
  sépare. Un consommateur qui prend `u` obtient un danger qui **oscille à la période de la houle** :
  à Hs = 1 m et T = 5 s, la vitesse orbitale de crête vaut `πHs/T ≈ 0,63 m/s`, du même ordre que le
  courant qu'on cherche à mesurer. ADR-018 §1 fait le bon choix (`flow_speed` = courant de surface)
  mais sa structure n'est pas dans SPEC-004 : le seul canal spécifié livre la mauvaise grandeur.
  Même famille que L12 — le paramètre auquel on pense en premier n'est pas celui qui gouverne.
- **E06, gravité 3, même cause que E04** — SPEC-002 §1 pose une écume résiduelle de demi-vie 30 s,
  et ADR-016 §7 remplace les émetteurs audio lointains par le « lit d'écume ». `WaterSample` porte
  `steepness` (le *déclencheur* instantané) et `aeration`, jamais la couverture accumulée. L'audio
  ne peut donc pas lire ce dont ADR-016 dit qu'il dépend. Symptôme de E04, pas défaut distinct.

**Contrôles passés.** `SampleHints` / LOD spectral ↔ ADR-004 §4 : cohérent. Séparation
`WaterSample` (≈56 o) / `BackgroundSample` (≈80 o) ↔ SPEC-001 §2.4 (24–32 o par cellule) :
les ordres de grandeur ne se contredisent pas. `steepness` ↔ SPEC-001 §3 (`H/λ ≈ 1/7`) et
SPEC-002 §1 : le déclencheur d'écume est bien atteignable sans simulation. `aeration` ↔
SPEC-002 §2 (`ρ_eff = (1−α)·ρ`) : la portance réduite est calculable par l'appelant.
