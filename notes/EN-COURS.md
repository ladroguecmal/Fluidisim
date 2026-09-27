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

Session : S410 — **en cours**. Demande de l'utilisateur (2026-09-27) : *« Réalise C6b »*. Conception S384 §5, C6 : reçu si,
« sur B10 **et sur une vague qui déferle** : particules seulement dans la bande, colonnes ailleurs ; aucune bascule qui oscille
(hystérésis mesurée) ; coût compté ». C6a (S408, [BASCULE-S408](../docs/validation/BASCULE-S408.md)) l'a éprouvé sur un corps qui
entre ; **le pli prédit par la pente n'a rien déclenché qui compte** (§5). Agent : Claude Code (Opus 5.5), application de bureau,
au poste — fichiers, git, cargo, Python, RTX 5070 (non utilisée : C6b est de la référence CPU). Branche `poste`.

**Le cas.** Chen, Kharif, Zaleski et Li (1999, *Phys. Fluids* 11, 121) : une houle de Stokes d'ordre 3 en profondeur infinie,
`ε = ka = 0,55`, `η = (λ/2π)[(ε + ε³/8) cos θ + ½ε² cos 2θ + ⅜ε³ cos 3θ]` — le jet se forme à `t₁ = 0,72` et touche la face
avant à `t₂ = 1,56`, en unités `√(λ/g)` (VOF, 256 mailles par longueur d'onde, périodique ; lu sur arXiv comp-gas/9605002).
Ici : `λ` = 2 m, 5 cm (40 mailles par longueur d'onde), 1 m d'eau (`kd` = π), **un bassin de quatre longueurs d'onde à parois**
— la phase posée pour que `u` = 0 aux parois à `t` = 0 (deux crêtes du milieu, à 2,5 et 4,5 m, loin des réflexions pendant la
seconde utile) ; vitesses de la théorie à l'ordre 3 (`u = aω e^{kz} cos θ`, `w = aω e^{kz} sin θ`, `ω = √(gk)(1 + ε²/2)`), la
matrice affine `C = ∇u`. APIC seul d'abord, puis la bande dynamique, la zone posée après le premier pas.

**Thèse.** Le critère de S408 — non convertible, pente > 1, dilatation, maintien — tient les particules aux crêtes qui se raidissent
**avant** qu'elles se retournent, les colonnes dans les creux ; le déferlement de la bande suit celui d'APIC seul.

**Critères, écrits avant.** (1) **APIC seul déferle** dans la fenêtre des deux crêtes du milieu : **retournement** (une verticale
eau / air / eau au-dessus du niveau moyen, deux particules par maille au moins) puis **impact** (air enfermé > `(dx)³·8` au-dessous
du jet) avant `t` = 2,5 ; *comparé à Chen, non exigé* — prédiction : retournement dans [0,6 ; 1,1], impact dans [1,3 ; 1,9]. (2) **La
bande** (défauts) : retournement et impact **à 3 %** du temps d'APIC seul, abscisse du jet à l'impact à deux mailles ; volume
relatif ≤ 10⁻⁹. (3) **Le pli prédit** : à la première verticale retournée de la bande, sa colonne était en particules **au moins un
pas avant**. (4) **Hystérésis** : au plus **deux** bascules par colonne, aux défauts et au maintien court (0,05 s). (5) **Coût**
: part moyenne de la bande (*prédiction* ≤ 40 %), part au retournement, particules et temps de calcul contre APIC seul. (6)
**La crête courte**, si le temps le permet : amplitude modulée le long de la crête (`ε` de 0,55 au milieu à 0,275 aux parois,
sous le seuil) — aucune colonne de la bande dans le quart extérieur de la largeur. (7) Suite entière, zéro avertissement.
**Arrêt** : si APIC seul ne déferle pas avant `t` = 2,5, publier et chercher pourquoi, sans changer le cas pour faire passer.

### Plan

- [x] **P1** — jeton, plan seul.
- [x] **P2** — `Apic3::set_particle_velocities` (vitesse et `C` d'un champ donné) ; essai : un champ affine passe à la grille exactement.
- [x] **P3** — le banc `apic3d_deferlement` : la houle de Chen, les mesures (retournement, impact, abscisse du jet, crête), la
  bande (`APIC3D_BASCULE`, comme B10), la crête courte en option.
- [>] **P4** — APIC seul ; critère 1.
- [ ] **P5** — la bande : défauts et maintien court ; critères 2 à 5.
- [ ] **P6** — la crête courte ; critère 6.
- [ ] **P7** — suite entière, zéro avertissement ; critère 7.
- [ ] **P8** — preuve : BASCULE-S408 §6 (un fil, une preuve) ; liste (4.10, 4.16), file, feuille de route, index.
- [ ] **P9** — rituel.

### Notes de reprise
- **P2** — `Apic3::set_particle_velocities(field)` (`apic3d.rs`) : vitesse et `C = ∇v` de chaque particule active ; refus
  `NotFinite` sans rien changer. Essai `_s410` : le champ posé exactement, le refus sans effet, l'aller et retour affine de S388
  (1e-5). Réussi.
- **P3** — `examples/apic3d_deferlement.rs` : `[mailles_par_lambda=40] [ny=4] [courte]` ; 160 × 4 × 32 à 5 cm. Mesures sur
  l'occupation (eau : ≥ 2 particules ou sous `η` d'une colonne ; air : aucune) dans la fenêtre [2 ; 6,5] m : crête, retournement
  (eau / air / eau), impact (air enfermé > 8 mailles, abscisse moyenne). Bande : `APIC3D_BASCULE` (`pente`, `dilatation`,
  `maintien`), zone posée après le premier pas (`clear_counts` ensuite), avance de la bande sur le retournement à sa colonne ;
  `APIC3D_TRACE=1` : intervalles de la bande sur la rangée du milieu. Construit sans avertissement.
