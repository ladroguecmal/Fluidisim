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

Session : S390 — **terminée**. **C3**, première part : **la multigrille sur la carte** ([conception](../docs/registres/CAMPAGNE-SOLVEUR-3D-S384.md)
§5, [ADR-207](../docs/adr/ADR-207-la-campagne-du-solveur-volumique-3d.md)). Demande de l'utilisateur (2026-09-26) : *« Reprends le
projet »*, puis, entre la pluie (pièce 5), C3 et les gerbes, **C3**. Agent : Claude Opus 5.5, Claude Code (application de
bureau) au poste — fichiers, git, cargo, Python, RTX 5070 Laptop, Godot 4.6.3.

**Maillons à 3** : ce fil (la campagne) n'a pas encore changé l'état d'un point ; C3 vise le coût de δ (4.19) et la maille fine
de la haute mer. Choisi par l'utilisateur ; la colonne graduée sur la carte n'en fait pas partie (S387 : elle sert l'eau calme,
pas la haute mer) — reportée à l'usage qui la consomme, les contenants.

**Thèse.** La production (afficheur, `Step3`) résout la pression par **32 cycles** de gradient conjugué préconditionné par
Jacobi : 2,06 ms des 3,68 du pas, 0,062 ms par cycle (S341), et des pas **déclarés dégradés** (S302). La référence a reçu en S385
un cycle en V qui fait 9 à 11 itérations quelle que soit la maille. **Le porter tel quel sur la carte** — Jacobi amorti ω = 6/7,
deux lissages avant, deux après, huit au plus grossier, restriction par moyenne, prolongation par injection, niveaux grossiers
rediscrétisés depuis la surface à chaque projection — comme préconditionneur du même gradient conjugué, à travail fixe.
**Prédiction** (*estimée*, avant mesure) : sur la scène de la porte B (120 × 112 × 28, deux niveaux grossiers), un cycle
multigrille coûte 4 à 6 cycles de Jacobi (six passes fines contre une, plus les niveaux grossiers, où domine le coût
d'appel) ; il atteint le résidu de 32 cycles de Jacobi en 3 à 6 cycles. Le gain à 25 cm serait donc modeste (projection
≈ 1,2 à 1,6 ms) ; il grandit avec la maille, là où Jacobi rampe — la maille de 10 cm est C3b.

**Critères, écrits avant.**
1. **Éteinte par défaut** : surface publiée identique au bit (empreintes de `--delta3d-empreinte`), dispatchs inchangés.
2. **L'instrument** : sur la géométrie de la scène après chauffe, le cycle en V de la carte contre une réplique CPU en `f64`,
   même résidu d'entrée — écart ≤ 10⁻⁵ du maximum ; symétrie `⟨u, M⁻¹v⟩` contre `⟨M⁻¹u, v⟩` ≤ 10⁻⁵ relatif, positivité ;
   l'essai **vu échouer** sur un cycle rendu asymétrique (un lissage après au lieu de deux).
3. **La convergence** sur la scène de la porte B : vrai résidu relatif et divergence franche contre les cycles, Jacobi 8 à 64,
   multigrille 1 à 8 ; **la multigrille atteint le résidu de 32 cycles de Jacobi en 8 cycles au plus**.
4. **Le coût** : cycle multigrille publié ; à résidu égal, pas entier ≤ celui de Jacobi à 32 cycles ; la porte C tient (deux
   parts ≤ 2 ms au 99ᵉ centile).
5. **La référence** : production multigrille à **3 mm** de la référence sur les trois cas de cuve ; divergence publiée.
6. Suite inchangée, zéro avertissement. **Non visé ici** : 10 cm (C3b), A298 remesurée (C3b, sur le pas retenu), activation
   par défaut dans la scène vivante (décidée en P6 sur les chiffres, empreintes nouvelles expliquées si oui).

### Plan

- [x] **P1** — jeton, plan seul.
- [x] **P2** — les noyaux WGSL : lissages fins et grossiers, résidu restreint, prolongation, géométrie des niveaux ; les
  variantes du gradient conjugué (mise à jour sans `z`, produit `r·z`, première direction).
- [x] **P3** — le branchement dans `Step3` : tampons réservés à la configuration (I-06), multigrille éteinte par défaut, nombre de
  dispatchs du profil ; critère 1.
- [x] **P4** — l'instrument : réplique `f64`, symétrie, positivité, vu échouer ; critère 2.
- [x] **P5** — convergence et coût sur la scène de la porte B, deux parts à 30 Hz ; critères 3 et 4.
- [x] **P6** — les trois cas de cuve avec la multigrille ; critère 5 ; activation par défaut décidée.
- [x] **P7** — critère 6 ; preuve (section datée de MULTIGRILLE-3D-S385) ; file, feuille de route, liste.
- [x] **P8** — rituel.

### Notes de reprise

**P2–P4 en un commit** (noyaux, branchement et instrument se règlent ensemble). `viewer/src/delta3d_mg.wgsl` (compilé à la
suite de `delta3d_cg.wgsl`), `viewer/src/delta3d_mg.rs` (réserve, séquences, réplique, banc `--delta3d-mg-cycle`).
`Step3::enable_multigrid` réserve à la configuration ; **211 680 flottants** (0,85 Mo) pour la porte B ; deux niveaux,
60 × 56 × 14 et 30 × 28 × 7. Premier lissage fusionné à la mise à jour ; `q` sert de tampon de lissage ; `r·z` replié
dans le dernier lissage. **24 dispatchs par cycle** (5 pour Jacobi). Quatre avertissements « jamais employé » restent
jusqu'à P5 (compte de dispatchs, bascule) — à zéro au commit de P5.

**Critère 1 tenu** : `--delta3d-empreinte` identique avant et après — 60 pas `0x6e90a7ae36e713e5` / `0x0ac01c724307b1ac`,
600 pas `0x8687dbea5acaab0f` / `0xde9ce81588ac0609`, précédente = publiée d'avant.

**Critère 2 tenu** (après 30 pas de Jacobi, 190 644 mailles mouillées) : `M` = 1/diagonale à 1,4·10⁻⁷, mouillage
cohérent ; carte contre réplique `f64` **4,4·10⁻⁷** du maximum (aléatoires 7,3 et 7,4·10⁻⁸, second membre 4,4·10⁻⁷) ;
symétrie **1,8·10⁻⁷** (réplique `f64` 2,6·10⁻¹⁴) ; positivité. **Vu échouer** (`ASYMETRIQUE=1`, zéro lissage après au
niveau intermédiaire) : symétrie **0,233**, sur la carte comme sur la réplique ; la réplique suit toujours la carte à
3,6·10⁻⁷ — l'instrument distingue la recette, pas seulement l'implémentation. Un mot réservé du WGSL (`active`) a
refusé la première compilation du nuanceur, à l'exécution : corrigé.

**P5 — la scène de la porte B à 30 Hz** (`--delta3d-mg-scene`, pas de 33,333 ms, 300 pas, secteur 96 % avant et après) :

| variante | résidu médian / max | div. franche médiane | dégradés | projection méd. / q99 | pas q99 |
|---|---|---|---|---|---|
| Jacobi 8 | 2,2·10⁻³ / 1,3·10⁻² | 6,9·10⁻² | 300/300 | 0,585 / 0,589 ms | 2,253 |
| Jacobi 16 | — | — | — | 1,075 / 1,082 | 2,744 — **explose** au pas 270 |
| **Jacobi 32** (production) | **7,1·10⁻⁵ / 1,3·10⁻³** | 1,7·10⁻³ | 300/300 | **2,078 / 2,088** | **3,765** |
| Jacobi 64 | 7,4·10⁻⁶ / 1,3·10⁻⁴ | 1,3·10⁻⁴ | 300/300 | (4,06, S341) | — |
| MG 1 ; MG 2 | — | — | — | 0,341 ; 0,519 | **explosent** aux pas 30 et 90 |
| MG 3 | 7,9·10⁻⁴ / 1,4·10⁻² | 9,2·10⁻³ | 300/300 | 0,632 / 0,682 | 2,319 |
| MG 4 | 2,0·10⁻⁴ / 2,8·10⁻³ | 4,5·10⁻³ | 300/300 | 0,784 / 0,793 | 2,405 |
| **MG 6** | **4,9·10⁻⁵ / 3,6·10⁻⁴** | 8,0·10⁻⁴ | 300/300 | **1,077 / 1,094** | **2,751** |
| MG 8 | 1,2·10⁻⁵ / 1,1·10⁻⁴ | 2,0·10⁻⁴ | 300/300 | 1,371 / 1,409 | 3,087 |
| réf. MG 24 ; Jacobi 512 | 1,1·10⁻⁷ ; 2,7·10⁻⁷ | 8,9·10⁻⁷ ; 2,8·10⁻⁶ | 0 ; 1 | — | — |

**Critère 3 tenu** : la multigrille atteint le résidu de Jacobi-32 en **6 cycles** (médiane 4,9 contre 7,1·10⁻⁵ ; maximum 3,6·10⁻⁴
contre 1,3·10⁻³) — haut de la fourchette prédite (3 à 6) : ≈ 0,43 de réduction par cycle. **Critère 4 tenu** : un cycle coûte
**0,147 ms** (2,4 cycles de Jacobi ; prédit 4 à 6) ; à résidu égal, projection **−48 %**, pas q99 **2,751 contre 3,765 ms**. Deux
parts, MG 4 : `k` = 1 → 1,627 / 0,796 ms q99 ; MG 6 s'en déduit à ≈ 1,63 / 1,09 (non mesuré tel quel).

**Ce que la scène dit aussi.** (1) Les deux références convergées s'écartent de **6 mm à 1 s, 102 mm à 8 s** : la scène
amplifie tout écart minime (A297) ; l'écart de surface à une référence n'y juge donc pas la projection — le résidu et la
divergence, si. (2) **Tout pas sous-convergé est dégradé** à 30 Hz, y compris la production (Jacobi 32) ; Jacobi 16, MG 1 et
MG 2 **explosent** en 1 à 9 s. (3) **Attribution du taux** (témoin : plus de lissages au grossier, même scène, 150 pas) :
32 lissages → 0,27 par cycle (MG 8 : 1,5·10⁻⁶), 128 → rien de plus ; mais 0,207 ms par cycle (0,45 à 128) : à la précision
de Jacobi-32, la recette du cœur (8) reste la moins chère. Un dispatch minuscule coûte ≈ 2,5 µs. **Dit, pas retenu.**

**P6 — les trois cas de cuve** (`MULTIGRILLE=`, écart de hauteur publiée au cœur, m) :

| cas | Jacobi, même binaire | multigrille 8 | multigrille 16 | niveaux |
|---|---|---|---|---|
| 3, `nx` 16 / 32 / 48 (1 s) | 64 : 2,6 / 2,4 / 2,4·10⁻⁸ ; 128 : 2,2 / 2,2 / 2,5·10⁻⁸ | 2,2 / 3,0 / 3,0·10⁻⁸ | 2,6 / 2,6 / 2,5·10⁻⁸ | 1 |
| 2, houle 5 cm (2 s) | 64 (S340) : 1,0·10⁻⁶ | 1,6·10⁻⁷ | 6,1·10⁻⁸ | 2 |
| 1, 5 cm, `nx` 32 / 64 | 64 (S340) : 3,9·10⁻⁷ / 1,3·10⁻⁶ | 2,5·10⁻⁷ / 1,2·10⁻⁶ | 1,1 / 3,4·10⁻⁷ | 0 |
| 1, 10 cm, `nx` 32 / 64 | 64 : 1,8·10⁻⁵ (S340) / **1,25·10⁻⁶** | 1,5·10⁻⁵ / **5,0·10⁻⁵** | 1,2·10⁻⁶ / **5,6·10⁻⁵** | 0 |

**Critère 5 tenu** : au plus **5,6·10⁻⁵ m**, pour 3 mm. Le pire, cas 1 à 10 cm et 64 mailles, est 40 fois le témoin : `ny` = 1,
**aucun niveau grossier**, le préconditionneur n'est que quatre lissages de Jacobi ; **64 cycles le ramènent à 3,9·10⁻⁷**
— une sous-convergence de ce cas, pas un défaut du cycle. Les 3·10⁻⁷ publiés en S305 pour le cas 3 datent d'avant
S342–S343 : le même binaire rend aujourd'hui 2,2 à 2,6·10⁻⁸ avec Jacobi.

**Épreuve d'une minute** (L369 ; Jacobi 16 explosait à 9 s). **À 30 Hz, tout explose en 24 à 40 s**, références
convergées comprises (multigrille 24 au pas 1 050, Jacobi 512 au pas 930 ; Jacobi 32, la production, au pas 1 200 ;
multigrille 6 au pas 720, 8 au pas 930). **À 60 Hz, la minute tient** pour les références, Jacobi 16 à 128, multigrille 6 et
8 ; explosent Jacobi 8 (32 s), multigrille 1 à 4 (0,5 à 58 s). **La cadence de 30 Hz de la porte C n'est pas stable sur une
minute dans cette scène, quel que soit le solveur de pression** : nouvel angle mort (P7), non attribué.

**Construit en P6** : option vivante `--multigrille[=n]` (6 par défaut ; `K_DEUX_PARTS_MG` = 1), éprouvée par
`--captures` à 30 Hz (divergence franche 4,3·10⁻⁴ au pas 63 contre 9,8·10⁻⁴) — les captures locales `s347` ont été
réécrites puis régénérées sans l'option ; instrument extrait (`mesurer_cycle`) ; deux essais —
`replica_vcycle_is_symmetric_and_seen_failing_s390` (CPU) et `card_vcycle_matches_the_replica_s390` (carte, ignoré par
défaut), tous deux réussis. Essais de l'afficheur : 36 réussis, 1 ignoré, avant l'ajout ; suite du cœur 680 réussis,
18 ignorés, inchangée (le cœur n'a pas changé).

**P7 — critère 6 tenu** : afficheur 37 réussis, 2 ignorés (dont l'essai de la carte), zéro avertissement ; cœur 680 réussis,
18 ignorés. Preuve : §5 de [MULTIGRILLE-3D-S385](../docs/validation/MULTIGRILLE-3D-S385.md). File (campagne ; A321, ligne
nouvelle), feuille de route (§3 bis porte C, §3 ter campagne), liste 4.19 — les trois premières lignes committées dès P6.
Angle mort **A321** (sévérité 3).

