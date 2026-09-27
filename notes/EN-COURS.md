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

Session : S402 — **en cours**. Demande de l'utilisateur (2026-09-27) : *« Continue »*. Suite proposée par S401 : **C8c**, les niveaux
de `dx` et la famine. Agent : Claude Opus 5.5, session cloud Claude Code ; fichiers, git, cargo, Python ; ni carte graphique, ni
Godot. Branche `claude/eager-volta-lf0kw3` (S401), la plus avancée ; aucune autre copie.

**Trouvé en lisant le lot.** ADR-006 §3.2 dit qu'un changement de niveau est « une destruction/création de domaine, gratuite
visuellement (ADR-005 §5) » ; I-12 et ADR-012 §4 renvoient au même §5. **Ce paragraphe n'existe plus dans le fichier** : le commit
de S35 (`c2eb75ba`) a remplacé ADR-005 entier par la note qu'il devait lui ajouter (−202 lignes), celui de S39 (`16e48d60`) a fait de
même avec la sienne. Aucun autre ADR ni aucune spécification n'a perdu de lignes (audit par `git log --numstat`). Le fichier juste
est : le corps et les notes jusqu'à S16 (`c0df00f7`), puis la note B-S26 (S35), puis la note B-S27 (S39), qui dit rétracter « la note
de B-S26 qui la précède immédiatement ».

**Thèse (C8c, première part).** ADR-005 §5 fait détruire l'ancien domaine (transduction, puis amortissement sur τ ≈ 0,5 à 1,5 s) et
naître le nouveau à δ = 0 : **tout ce que le domaine contient est perdu**, et seul ce qui sort par son bord passe à W. Un
**transfert d'état** d'un niveau à l'autre — surface par recouvrement, reconstruction linéaire conservative, volume exact ; vitesses
interpolées ; pression remise à zéro — garde ce que le niveau d'arrivée sait porter. Les deux se mesurent sur les mêmes cas, contre
le domaine fin tenu tout du long : le rang 4 d'ADR-012 (25 → 50 cm), puis le retour.

**Critères, écrits avant.** **A1** — ADR-005 : chaque part restaurée identique au bit à sa source, dans l'ordre ; une note datée dit
ce qui s'est passé. **A2** — un contrôle de l'outil : chaque ADR commence par son titre `# ADR-NNN` ; **vu échouer** sur les versions
de S35 et S39, tenu sur tous les ADR restaurés. **T1** — un état uniforme traverse 25 → 50 → 25 cm inchangé (au bit ou à un ulp).
**T2** — volume de perturbation conservé à l'arrondi f64. **T3** — aller-retour d'une surface sinusoïdale de 8 m (16 mailles
grossières) : erreur **≤ 1 %** de l'amplitude (prédiction : 0,3 %, calculée en 1D) ; **vu échouer** sans pente (prédiction : ≈ 10 %).
**T4** — refus : fenêtre, repos, densité, gravité différents ; ensemble épars ou découpe. **B1** — banc, bosse de 5 cm et σ = 1 m :
le saut d'image au passage 25 → 50 cm **≤ 3 mm** (tolérance d'image, I-12 ; prédiction ≈ 1 mm). **B2** — au retour 50 → 25 cm, le
saut **≤ 3 mm** de l'image grossière d'avant. **B3** — publiés : l'écart pendant la période grossière (le prix du rang 4, « visible
de près ») ; le même cas selon ADR-005 §5 (prédiction : la bosse perdue, écart de l'ordre de l'amplitude) ; la source mobile de S401.

### Plan

- [x] **P1** — jeton, plan seul.
- [x] **P2** — ADR-005 restauré, note datée ; critère A1.
- [x] **P3** — le contrôle dans `etat_projet.py` (vu échouer sur S35 et S39) ; protection de METHODE, leçon ; critère A2.
- [x] **P4** — `delta3d_levels.rs` : le transfert d'état entre niveaux ; essais T1 à T4, vu échouer sans pente.
- [x] **P5** — le banc `delta3d_niveaux` : bosse et source mobile, 25 → 50 cm à 2 s, retour à 5 s, contre le domaine fin ; ADR-005 §5
  (fondu de 0,5 s, naissance à zéro) ; critères B1 à B3.
- [ ] **P6** — suite entière, zéro avertissement.
- [ ] **P7** — preuve `NIVEAUX-S402` ; un ADR si la mesure tranche le mécanisme du rang 4 ; liste (4.5, 9.9), file, feuille de route,
  index.
- [ ] **P8** — rituel.

### Notes de reprise

- **P2** — ADR-005 restauré : le texte de S16 (`c0df00f7`, 9 791 premiers octets), la note B-S26 (S35), la note B-S27 (S39), chacun
  au bit et dans l'ordre (vérifié par position), puis une note datée de S402. 285 lignes ; titre, §1 à §6 retrouvés. **§5, le
  cycle de vie** : création et croissance à δ = 0 (coût nul) ; rétrécissement, « transduction δ→W puis amortissement sur τ ≈ 0,3 s » ;
  destruction, « idem, τ ≈ 0,5–1,5 s selon l'énergie résiduelle » (négligeable) ; bascules perturbatif ↔ substitutif continues.
- **P3** — `adr_heads` dans `outils/etat_projet.py` : un ADR commence par son titre `# ADR-NNN` de son fichier (des lignes vides
  avant sont admises : neuf ADR intacts — 092 à 099, 111 — en ont une ; le premier jet, trop strict, les refusait). **Vu échouer**
  sur les versions réelles : `c2eb75ba` (S35) et `16e48d60` (S39), une anomalie chacune ; `c0df00f7` (S16) et la restaurée, aucune ;
  tous les ADR du dépôt passent. Essai `test_an_adr_begins_with_its_title_s402` (17 essais de l'outil). METHODE : une ligne « en
  écrivant » (dix-huit protections) ; leçon **L373**.
- **P4** — `delta3d_levels.rs`, `Volume3::resample_from` (+ `LevelChange`) : surface par recouvrement d'une reconstruction
  **bilinéaire** (pentes centrées, décentrées au bord, et terme croisé `∂²h/∂x∂y`), positions rapportées à la fenêtre d'arrivée ;
  vitesses trilinéaires aux centres des faces, moyennées sur `n × n` sous-faces quand l'arrivée est plus grossière (rapport 2 :
  exactement les faces couvertes) ; pression à zéro ; murs refermés. **T1 tenu** (surface uniforme au bit, vitesse uniforme au bit
  loin des murs). **T2** : 25 ↔ 50 cm, **écart 0,0** ; 10 → 25 cm, **−1,1·10⁻⁸ m³** = l'écart d'aire des fenêtres (0,1 m non
  exact en f32 : 3·10⁻⁸) — la **hauteur moyenne** conservée au bit près (1,191245712·10⁻² m). **T3** : **manqué d'abord, 1,031 %**
  — la reconstruction plane oubliait le terme croisé, 0,96 % calculé en 2D (`sin²(θ/4)`) ; avec lui, **0,304 %** (prédiction 2D :
  0,3 %) ; λ = 16 m, 0,037 % : **ordre trois** sur cette mesure (rapport 8,2 ; le terme d'ordre deux s'annule aux demi-mailles) —
  ma prédiction « ordre deux » était fausse, l'essai tient désormais « au moins l'ordre deux ». **Vu échouer sans pente : 10,20 %**
  (prédiction ≈ 10 %), ordre un (5,02 % à 16 m). **T4 tenu** (étendue, repos, gravité, ensemble épars ; rien d'écrit). Un état
  transféré repart : dix pas grossiers, volume gardé.
- **P5** — `examples/delta3d_niveaux.rs` : bassin 24 × 16 m, 25 cm (96 × 64 × 12) ; passage à 50 cm à 2 s, retour à 5 s, fin à
  7 s ; référence à 25 cm tout du long ; image sur la grille fine (le grossier reconstruit par le même transfert) ; saut = variation
  de l'image sur le pas du passage moins celle de la référence ; ADR-005 §5 : nouveau domaine au repos, l'ancien continue sans
  la source et s'efface linéairement en 0,5 s (pas de transduction en référence). Quatre calculs en parallèle, ≈ 6 min chacun.
  **Bosse** (5 cm, σ = 1 m) — transfert : saut **0,24 mm** au passage, **0,13 mm** au retour, autres pas ≤ 0,18 mm ; écart
  **1,82 mm** pendant la période à 50 cm, 2,32 mm après ; ADR-005 §5 : sauts 0,36 / 0,49 mm (le fondu, 0,65 mm par pas), écart
  **10,2 mm**, 12,8 mm après. **Source** (dipôle de S401, 24 mm) — transfert : sauts 1,95 / 1,32 mm, écart 15,7 mm, 16,7 après ;
  ADR-005 §5 : 0,68 / 1,01 mm (2,28 par pas), écart 17,0 mm, 23,6 après. **B1, B2 tenus** partout (≤ 3 mm ; prédiction ≈ 1 mm
  sur la bosse : 0,24). **B3** : le transfert garde le contenu résolu (1,8 mm, sous la tolérance d'image) ; ADR-005 §5 le perd ;
  la source, sous-résolue à 50 cm (σ = une maille), coûte 15 à 17 mm aux deux — le « visible de près » d'ADR-012 §4.
