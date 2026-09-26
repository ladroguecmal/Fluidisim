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

Session : S385 — **en cours**. **C1** de la campagne ([ADR-207](../docs/adr/ADR-207-la-campagne-du-solveur-volumique-3d.md) D5) :
**la multigrille 3D de la référence**. Demande de l'utilisateur (2026-09-26) : *« Oui »* à C1 ; et le **verdict R32**, à
consigner. Agent : Claude Opus 5.5, session cloud Claude Code ; fichiers, git, cargo, Python ; ni carte graphique, ni Godot.

**Thèse.** Le pas mobile de la référence (`project_mobile3` : le pas couplé, la piscine, les solides) résout la pression par
un gradient conjugué préconditionné par Jacobi, dont les itérations croissent avec la maille. Le cycle en V de S245
(`delta_projection.rs`, 2D) porté en 3D — lissage de Jacobi amorti ω = 6/7 (dérivé pour sept points), restriction moyenne
des huit filles, prolongation par injection, autant de lissages avant qu'après, niveaux grossiers rediscrétisés (ouvertures
moyennées, maille active si une fille l'est, air si une fille l'est) — comme **préconditionneur** : il change le chemin,
jamais le test d'acceptation (ADR-144). **Désactivé par défaut** : tout ce qui existe reste au bit.

**Critères, écrits avant.** (1) Le cycle est **symétrique** (écart relatif ≤ 10⁻⁵ sur des vecteurs quelconques) et **défini
positif**, sur une surface libre, sur fond plat et sur fond coupé. (2) Projection **acceptée** avec et sans, et surfaces à
**10⁻⁶ m** l'une de l'autre après 20 pas mobiles. (3) **Itérations indépendantes de la maille** : à trois mailles d'un même
domaine (`dx`, `dx/2`, `dx/4`), le nombre d'itérations avec la multigrille ne croît pas de plus de 50 % de la plus grossière
à la plus fine, quand Jacobi croît au moins du double (L274 : trois points). (4) La bosse de S324 en mode mobile ne rampe
pas (A315). (5) Désactivée : suite entière inchangée (664 réussis, 18 ignorés), zéro avertissement. (6) Coût par itération
et temps total publiés, au même résidu.

### Plan

- [x] **P1** — jeton, plan seul.
- [x] **P2** — verdict R32 consigné (REVUE-VISUELLE §37, file : les défauts nommés, déclencheur « peaufinage »).
- [x] **P3** — l'instrument d'abord : `examples/delta3d_multigrille.rs`, trois mailles, fond plat et bosse, itérations et
  temps de Jacobi — la référence de mesure.
- [x] **P4a** — `delta3d_multigrid.rs` : niveaux 3D (géométrie, restriction, prolongation, opérateur, lissage) ; comptés
  auprès de l'hôte (I-06) ; essais de forme.
- [x] **P4b** — le cycle en V et son branchement dans `project_mobile3` (`enable_multigrid`) ; critère 1.
- [x] **P5** — critères 2 à 4 : essais et banc.
- [x] **P6** — critère 5 (suite entière), critère 6 (coût) ; preuve `MULTIGRILLE-3D-S385` ; liste, file, feuille de route,
  index.
- [ ] **P7** — rituel.

### Notes de reprise

**P3 — la référence de mesure, Jacobi** (`delta3d_multigrille -- --fin`, cas mobile de S328, 8 × 4 × 6 m, surface à 4 m,
sinusoïde de 1 cm ; premier pas depuis `p = 0`, puis dix pas chauds) :

| fond | nx | mailles | it. premier | it. moyen chaud | durée premier / chaud |
|---|---:|---:|---:|---:|---|
| plat | 32 / 64 / 128 | 12 288 / 98 304 / 786 432 | 102 / 191 / **365** | 36,5 / 58,4 / 85,9 | 0,06 / 0,90 / 13,4 s ; 0,02 / 0,27 / 3,43 s |
| bosse | 32 / 64 / 128 | idem | 117 / 219 / **444** | 36,0 / 58,8 / 86,3 | 0,08 / 0,88 / 16,3 s ; 0,02 / 0,26 / 3,65 s |

Croissance de 32 à 128 : premier pas ×3,58 (plat), ×3,79 (bosse) ; chaud ×2,35, ×2,40 — Jacobi croît bien au moins du
double (critère 3, partie Jacobi). Divergence max 9,5·10⁻⁶ (sous 10⁻⁵).

**P4a–P4b — un seul commit** : P4a seul laissait le module inutilisé, donc des avertissements (critère 5). Construit :
`delta3d_multigrid.rs` (niveaux, `coarsen3`, `row_sums`, `restrict3`, `prolong_add3`, `smooth3`, `coarse_cycle3` ;
`enable_multigrid`, `prepare_multigrid3`, `v_cycle3`, `prime_multigrid3`, `precondition_multigrid3`) ; branché dans
`project_mobile3` derrière `mg.is_some()` ; `open3` et `solid3` passés `pub(super)`. **Critère 1 tenu** : symétrie et
positivité sur fond plat et sur la bosse (`multigrid_cycle_is_symmetric_and_positive_s385`) ; **vu échouer** sur un
cycle rendu asymétrique (un lissage après au lieu de deux : 1,23 contre 1,13), puis rétabli. Critère 2 à 32 mailles :
surfaces à 10⁻⁶ m après 20 pas, deux fois moins d'itérations au moins (`…_with_fewer_iterations_s385`).

**P5 — critères 2 à 4** (banc rejoué deux fois, itérations identiques) :

| fond | méthode | it. premier (32 / 64 / 128) | croissance | it. chaud | pas chaud à 128 | premier pas à 128 |
|---|---|---|---:|---|---:|---:|
| plat | Jacobi | 102 / 191 / 365 | ×3,58 | 36,5 / 58,4 / 85,9 | 3,24 s | 12,3 s |
| plat | multigrille | **9 / 10 / 11** | **×1,22** | 3,2 / 3,7 / 4,0 | **1,32 s** | **2,89 s** |
| bosse | Jacobi | 117 / 219 / 444 | ×3,79 | 36,0 / 58,8 / 86,3 | 3,54 s | 16,8 s |
| bosse | multigrille | **9 / 10 / 11** | **×1,22** | 3,2 / 3,7 / 4,1 | **1,59 s** | **3,71 s** |

Critère 3 tenu (≤ +50 % contre ≥ ×2) ; critère 4 tenu (la bosse comme le fond plat, A315 ne revient pas). Critère 2 : essai à
32 mailles. **Divergence** : lignes franches ≤ 5,9·10⁻⁶ avec la multigrille (≤ 9,5·10⁻⁶ Jacobi) ; **sur toutes les lignes,
1,41·10⁻⁵ au fond plat à 128** avec la multigrille (Jacobi 8,9·10⁻⁶) — les lignes de surface, qu'ADR-144 ne juge pas ; dit, pas
expliqué. Une itération multigrille coûte ≈ 8,7 fois une de Jacobi (pas entier / itérations, à 128). **Critère 5** : suite
entière 667 réussis (664 + 3), 18 ignorés, zéro avertissement.
