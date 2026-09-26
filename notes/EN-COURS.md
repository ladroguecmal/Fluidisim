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

Session : S389 — **terminée**. **C4b**, première part : **la surface d'APIC** ([APIC3D-S388](../docs/validation/APIC3D-S388.md) :
la lecture de la surface commande la période). Demande de l'utilisateur (2026-09-26) : *« continue »* — troisième session du
fil (C4) : la voie de la v1 a été proposée à l'utilisateur en fin de S388, qui a demandé la suite ; sa demande prime
(REPRISE §6). Agent : Claude Opus 5.5, session cloud Claude Code ; fichiers, git, cargo, Python ; ni carte graphique, ni Godot.

**Thèse.** Calculée sur un réseau en `f64` pour une surface qui parcourt **continûment** une maille (huit positions), la lecture
de S388 — noyau d'une maille — se trompe jusqu'à **9,9 %** de maille (S388 n'avait regardé que deux positions, ±6,1 %). Le
**rayon du noyau** commande cette erreur, pas le nombre de particules : 3,6 % à 1,5 maille, **2,5 % à 2 mailles** ; 27
particules par maille n'y changent presque rien (2,4 %). Retenir un noyau de deux mailles, rayon au repos minimax sur les
positions continues ; puis remesurer les ballottements, dont la période devrait suivre.

**Critères, écrits avant.** (1) Le modèle `f64` et la reconstruction 3D lisent la même hauteur à 0,1 % de maille près, sur les
huit positions. (2) Lecture ≤ 2,5 % de maille sur toute position (critère 2 de S388, 1 %, reste manqué ; ce qui change est
publié). (3) Repos : ≤ 1 cm/s à 5 cm, masse exacte. (4) Les critères de S388, **inchangés** : (1, 0) ≤ 1 % à 2,5 cm, (1, 1) ≤ 2 %.
(5) **L'énergie, sur une mesure qui la porte** : l'amplitude du moment ne croît jamais — amortissement par période ≥ 0 par
régression sur les extremums (S354) ; publié. (6) Attribution : noyau d'une maille contre deux, même cas. (7) Suite
inchangée, zéro avertissement.

### Plan

- [x] **P1** — jeton, plan seul.
- [x] **P2** — le modèle de lecture en `f64` dans le cœur, positions continues, rayon du noyau en paramètre ; rayon minimax ;
  critère 1.
- [x] **P3** — la reconstruction à deux mailles ; critères 2 et 3.
- [x] **P4** — les ballottements remesurés, l'amortissement par régression, l'attribution ; critères 4 à 6.
- [x] **P5** — critère 7 ; preuve (section datée d'APIC3D-S388) ; liste, file, feuille de route.
- [x] **P6** — rituel.

### Notes de reprise

**P2–P3 en un commit** (le modèle et la reconstruction se règlent ensemble). `KERNEL_CELLS` = 2 ; modèle `f64`
(`lattice_mean_distance`, `lattice_read_error`, `minimax_radius`) sur huit positions continues ; rayon minimax 0,05280 m à
10 cm (noyau 1 : 0,03600 ; S318 : 0,02743). **Critère 1 tenu** : lecture 3D = modèle à 10⁻³ % près sur les huit positions,
noyaux 1 et 2. **Critère 2 tenu** : noyau 2, pire lecture **2,46 %** de maille (noyau 1 : 9,93 %).

**Trouvé en chemin — les parois** : au premier passage, le noyau de deux mailles faisait courir le repos à **15 cm/s**. Près
d'une paroi, le noyau ne trouve des particules que d'un côté : la moyenne se décale vers l'intérieur, la surface y paraît plus
basse. Les parois **reflètent** désormais les particules (images seulement pour les centres à moins d'un rayon de noyau d'une
paroi latérale ou du fond). **Critère 3 tenu** : repos **6,2 µm/s** (noyau 2), témoin noyau 1 avec images **1,5 µm/s** — **les
5,6 mm/s de S388 venaient des parois**, pas du noyau.

**P4 — les ballottements remesurés** (noyau 2, parois reflétées ; `apic3d_ballottement`, amortissement par régression) :

| cas | 5 cm | 2,5 cm | critère |
|---|---|---|---|
| (1, 0) | +1,01 % (S388 +2,05) | **+0,39 %** (S388 +1,04) | ≤ 1 % **tenu** |
| (1, 1) | +2,63 % (S388 +7,64) | **+1,01 %** (S388 +2,69) | ≤ 2 % **tenu** |
| amortissement par période | +0,21 % ; (1, 1) +0,63 % | +0,10 % ; +0,78 % | ≥ 0 **tenu** |
| énergie « créée » (oscillation, % de l'onde analytique) | +7,0 % ; +3,6 % | +3,3 % ; +5,3 % | publiée |
| calcul | 24 s ; 29 s | 224 s ; 274 s | — |

**Attribution** (5 cm) : noyau 1, parois reflétées — (1, 0) **+0,54 %**, amortissement 3,78 %/période ; (1, 1) **+5,57 %**,
2,33 %/période. S388 (noyau 1, sans images) : +2,05 % et +7,64 %. **Les images aux parois** corrigent surtout (1, 0) ; **le
noyau large** corrige l'oblique et divise l'amortissement par 18. Critères 4 à 6 tenus.

**P5 — critère 7 tenu** : suite Rust 680 réussis, 18 ignorés, zéro avertissement ; outils Python 32 réussis. Preuve :
§5 d'[APIC3D-S388](../docs/validation/APIC3D-S388.md) ; liste 4.16, file de la campagne, feuille de route.
