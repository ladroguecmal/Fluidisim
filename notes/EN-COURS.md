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

Session : S416 — **terminée**. Demande de l'utilisateur (2026-09-30) : *« Reprends le projet »* — la suite déclarée : **C7**, APIC
sur la carte (conception S384 §5, ADR-211 D1, ADR-212 D5). Agent : Claude Code (Opus 5.5), au poste ; RTX 5070 Laptop.

**Ce que C7 demande, et ce qu'une session en fait.** La référence de la bande étroite est `Apic3` avec sa zone de colonnes et
son fond (≈ 3 100 lignes du cœur) ; la porter entière est plusieurs sessions. C7 se découpe (P2) ; **S416 fait C7a** : le pas
d'APIC **nu** — particules triées par maille, particules → grille, surface reconstruite, gravité, parois, projection à fluide
fantôme, extrapolation, grille → particules, advection RK2, séparation — sur la carte, contre la référence, étage par étage puis
sur le ballottement (1, 0) de S388. Zone des colonnes, fond, corps : C7b et suivantes.

**Critères, écrits avant.** (1) Chaque étage, sur le même état d'entrée, à l'arrondi `f32` de la référence : tri (comptes par
maille identiques), vitesses de grille à 10⁻⁵ m/s, `φ` à 10⁻⁵ m, étiquettes identiques, vitesses corrigées à 10⁻⁴ m/s après
projection (le solveur diffère : résidu relatif ≤ 10⁻⁵ des deux côtés). (2) **Le ballottement (1, 0) de S388** à 5 cm, 10 s :
la surface de la carte à **3 mm** de la référence (le critère de production, ADR-175 D1), période à 0,5 %. (3) Le **coût par
particule** publié, par étage, au 99ᵉ centile. (4) Zéro avertissement, suite du cœur inchangée. **Arrêt** : si un étage
diverge au-delà de l'arrondi, le publier, ne pas avancer au suivant.

### Plan

- [x] **P1** — jeton, plan seul.
- [x] **P2** — conception de C7 : découpage (C7a nu, C7b corps, C7c zone et fond, C7d relative à B et critère de vitesse, C7e
  budget), choix du transfert (tri par maille, collecte par face — déterministe, sans atomique flottant), critères.
- [x] **P3** — le cœur : accès de banc au pas étage par étage (`step_stage`), à comportement inchangé ; essai.
- [x] **P4** — la carte : module `apic3d_carte`, tampons, tri par maille (compte, préfixe, rangement) ; banc contre le cœur.
- [x] **P5** — particules → grille, collecte par face ; banc.
- [x] **P6** — surface reconstruite et étiquettes (reflets des parois) ; banc.
- [x] **P7** — gravité, parois, projection (gradient conjugué préconditionné par la diagonale) ; banc.
- [x] **P8** — extrapolation, grille → particules, advection, séparation ; le pas entier ; banc d'un pas.
- [x] **P9** — le ballottement (1, 0) sur la carte contre la référence, 10 s ; coût par particule ; critères 2 et 3.
- [x] **P10** — suite entière, zéro avertissement ; preuve ; liste, file, feuille de route, index.
- [x] **P11** — rituel.

### Notes de reprise
- **P2** — conception : [APIC-CARTE-S416](../docs/validation/APIC-CARTE-S416.md) §1 ; C7a à C7e ; Gao 2018 non relu (résumé seulement).
- **P3** — `ApicStage`, `step_upto` (`step` = `step_upto(Full)`, au bit) ; accesseurs de banc `affine`, `face_weights`, `pressure`, `bins`, `settings`, `physics` ; constantes du pas rendues publiques. Essai `_s416`.
- **P4** — `viewer/src/apic3d_carte.rs` + `.wgsl` ; banc `--apic3d-carte-etages` (DX, CHAUFFE) : ballottement (1, 0) à 5 cm, 20 pas de chauffe, 12 800 particules — **débuts et ordre identiques** à la référence ; tri 0,020 ms.
- **P5** — collecte par face (poids bornés de `weights` en opérations vectorielles : FXC refuse l'écriture indexée dans un vecteur) : écart de vitesse **1,6·10⁻⁷ m/s** (vitesse max 0,13), poids 2,3·10⁻⁵ (sommes ≤ 8), mêmes faces alimentées ; tri + transfert 0,10 ms.
- **P6** — reconstruction en collecte, reflets des parois : écart de φ **1,6·10⁻⁶ m**, étiquettes identiques (1 600 mailles d'eau) ; **0,39 ms** — chaque maille lit 125 mailles et leurs images : le poste de coût, pour C7e. Piège : le `!` d'un script en ligne casse le shell de l'outil — écrire les scripts par l'outil d'écriture.
- **P7** — gravité, parois, gradient conjugué diagonal sur la carte (scalaires sur la carte, drapeau de fin lu uniformément par `workgroupUniformLoad`), correction : **94 itérations des deux côtés**, résidu 7,05·10⁻⁷ contre 7,04·10⁻⁷, pression à 5,4·10⁻³ Pa sur 4 694 (10⁻⁶), vitesses à **2,7·10⁻⁶ m/s**. Coût : **les dispatchs enregistrés**, même sautés — 4,07 ms au plafond de 400, 1,01 ms à 120 (`ITERATIONS=`) : la multigrille (C7e) est la voie.
- **P8** — extrapolation (5 668 faces non nulles des deux côtés, écart 2,7·10⁻⁶ m/s), retour (vitesses 2,6·10⁻⁶ m/s, `C` 1,1·10⁻⁴ s⁻¹ — le gradient amplifie par 1/dx), advection (positions **1,2·10⁻⁷ m**), séparation (idem) ; 0,05 / 0,02 / 0,01 / 0,19 ms. Critère 1 tenu, tous les étages.
- **P9** — `--apic3d-carte-ballottement` (DX, DUREE, ITERATIONS) : 5 cm, 10 s, 500 pas au pas de la référence : **surface à 0,456 mm** au pire (t = 7,14 s), période 1,9965 s contre 1,9964 (**+0,003 %**), 10 passages de chaque côté, itérations 93,5 des deux, aucune non convergée ; une particule isolée à 1,72 mm en fin (divergence des trajectoires, la surface tient). **Coût p99 2,05 ms, 160 ns par particule** : projection 1,39 (plafond de 200 itérations enregistrées), surface 0,41, séparation 0,15, transfert 0,10, le reste < 0,02. Critères 2 et 3. 2,5 cm, 2 s : au calcul.
- **P10** — suite du cœur : **753 réussis** (752 + `_s416`), 19 ignorés, zéro avertissement ; afficheur zéro avertissement. 2,5 cm, 2 s : surface à 0,77 mm, 193 itérations des deux côtés, 4,77 ms pour 102 400 particules (47 ns). Preuve APIC-CARTE-S416 §3–6 ; liste 4.19 (état inchangé, partiel), file, feuille de route, index. `--check` : 0.
- **P11** — journal ; jeton libre ; maillons 4 (justifiés : S406) ; suivant : S417, C7b puis C7c.
