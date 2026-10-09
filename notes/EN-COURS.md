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

Session : S733 — **en cours**. En autonomie ; session longue. SELECTEUR-DOMAINES-S732, **P1 : le prédicteur**. **La question** : SGN, calculé
en avance depuis l'état de départ, prévoit-il où et quand la vague de R43 se retourne, et où retombe son jet ?

**Ce que la session fait.**
- **SGN sur un fond doux** (`Serre1D::nouveau_fond`) :
  - la surface `η = h + z` reconstruite (MUSCL), la hauteur aux faces lue sous elle, le fond continu aux faces ;
  - la source du fond centrée, `−g·(h⁺ + h⁻)/2·Δz/dx` : le lac au repos est tenu exactement ;
  - le terme dispersif sur fond doux, `g·η_xx` au lieu de `g·h_xx` (l'approximation de pente douce : les termes en `z_x` négligés).
  Le domaine reste périodique : le fond monte jusqu'à 4 cm d'eau, puis redescend en miroir. L'onde se retourne bien avant (0,14 m).
- **Le prédicteur** (`selecteur.rs`, `prevoir`) : SGN sur une copie, jusqu'à un horizon donné. À chaque pas, trois critères de déclenchement
  publiés sont évalués à la crête :
  - (K) la vitesse de montée de la surface, `η_t > α·√(g·h)` (Kennedy et al., 2000 ; α = 0,65) ;
  - (H) le rapport `η/h`, la hauteur de la crête sur la profondeur locale, au-delà de 0,8 ;
  - (F) le nombre de Froude de la crête, `u/√(g·(h))`, au-delà de 0,8.

  Il rend, pour chaque critère, le premier instant et le lieu `(t, x)`, la hauteur de la crête `H_b` et la profondeur au repos `h_b`, et
  `L_jet = c_b·√(2·H_b/g)` (S732).
- **La trajectoire de la crête du témoin** (`outils/crete_film.py`), lue dans le film de S730 E2 (`calculs/s730_tout3d_12.bin`) : la
  plus haute particule et sa place, à chaque image, jusqu'au retournement.

**Les essais, et leurs critères écrits avant.**
- **E1 — le fond doux** :
  1. le lac au repos sur la plage de R43 (le fond monte et redescend), 2 s : la vitesse sous **10⁻¹² m/s**, la masse au bit ;
  2. la levée d'une onde longue (`kd` ≈ 0,1, 1 mm) sur une pente de 1:50 : la hauteur suit la loi de Green, `H ∝ h^(-1/4)`, à **5 %** de
     la profondeur 0,5 m à 0,2 m.
- **E2 — la prévision de R43**, depuis l'état de départ de S730 (l'onde de Boussinesq, `x₁` = 3,4 m), contre le témoin (le retournement à
  2,620 s et 9,938 m ; l'air à 2,804 s et 10,375 m) :
  3. la crête de SGN contre celle du témoin, jusqu'à 2,3 s : l'écart de place et de hauteur, **rapporté** (deux modèles ; S713 a vu la 3D
     trop haute de 40 % sur Synolakis) ;
  4. pour chaque critère, l'avance `(t_témoin − t, x_témoin − x)` rapportée. Le critère est retenu si son lieu tombe à **0,3 m** du
     retournement du témoin et son avance entre 0 et 0,3 s : une avance fixe le corrige alors. C'est un calibrage sur une seule scène, et
     le registre le dit : il se juge à chaque nouveau témoin (P2) ;
  5. le jet prévu, `x_b(témoin) + L_jet(SGN)`, à **0,15 m** de l'air enfermé du témoin (10,375 m) ;
  6. **le coût** : une prévision de 3 s sous **100 ms**.

**Contrôles du plan** (ADR-266, ADR-273, ADR-276, ADR-280, ADR-284, ADR-285)

- **témoin** : le tout-3D de S730 E2 (raccord au-delà du jet), son film ; pour E1, des solutions exactes (le repos, la loi de Green).
- **instrument** : la crête du témoin lue sur ses particules (la plus haute, une rangée), notée comme lecture ponctuelle (ADR-280 D1) et
  rapportée seulement ; les critères 4 et 5 portent sur le retournement et l'air, juges éprouvés (S647, S648).
- **calcul** : E1 et E2, quelques secondes ; aucun calcul long.
- **ADR** :
  - ADR-276 D1 : l'état de départ vient d'`OndeDepart`, la même fonction que le témoin ;
  - ADR-285 D1 : le témoin a son raccord mesuré (S730, `J` = 0) ;
  - ADR-278 D2 : les tolérances.
- **pièges** :
  - SGN ne déferle pas : après le déclenchement, ses nombres n'ont plus de sens, et la prévision s'arrête au premier ;
  - la pente de 1:12 n'est pas douce : l'approximation de pente douce y est rapportée comme telle ;
  - la hauteur doit rester positive : le fond s'arrête à 4 cm.

### Plan

- [x] **P1** — jeton ; plan.
- [x] **P2** — le fond doux ; E1 ; (1)–(2).
- [x] **P3** — le prédicteur, la crête du témoin ; E2 ; (3)–(6).
- [ ] **P3b** — **E3, ajouté après E2** (le coût manqué) : le même prédicteur à `dx` = 5 cm. Critères écrits avant : (7) la prévision de 3 s
  sous **100 ms** ; (8) la convergence : pour chaque critère (et chaque variante rapportée), l'instant à **0,05 s** et le lieu à **0,1 m** de
  ceux de 2,5 cm ; la crête à 2,6 s à 5 % de celle de 2,5 cm. Jugé contre le prédicteur à 2,5 cm, non contre le témoin.
- [ ] **P4** — preuve ; fermeture.

### Notes de reprise
- **P2 fini** — **tenu** : (1) le lac au repos sur la plage de R43, 2 s, la vitesse 4,7·10⁻¹⁵ m/s, la masse 0 ; (2) la levée sur 1:50, de
  0,5 m à 0,2 m, Saint-Venant **1,2292** contre Green 1,2574 (−2,2 %) ; rapporté à l'usure du schéma (la même bosse sur fond plat, 0,9823),
  1,2514 (−0,5 %) ; SGN 1,1338 (une bosse de σ = 2 m se disperse ; rapporté). **Écart au plan** : l'onde longue sinusoïdale (`kd` ≈ 0,1) aurait
  31 m, plus que la pente ; à sa place, une bosse gaussienne de σ = 2 m (σ = 0,5 m d'abord : l'écrêtage du limiteur l'usait de 20 % en
  800 mailles), et le témoin de l'usure sur fond plat. S694 inchangé (sans fond, au bit).
- **P3 fini** — E2 : (4) **manqué** pour les trois seuils publiés : Kennedy 0,65 déclenche à 2,729 s, 10,463 m (0,52 m trop loin) ; le
  rapport de hauteur 0,8 à 2,339 s, 9,312 m (0,63 m trop tôt) ; Froude 0,8 à 2,970 s, 10,788 m. (5) **tenu pour les trois** : le jet à
  10,24–10,28 m, contre 10,375 m pour l'air du témoin. (6) **manqué** : 277 ms. Rapportées, hors critère : Kennedy 0,35 (2,404 s ;
  9,812 m) et le rapport 1,0 (2,523 s ; 9,738 m) tomberaient dans la fenêtre, mais un seuil choisi sur la scène qui le juge ne juge rien
  (ADR-248 D2) : il faut d'autres témoins (P2 du registre). (3) La crête : la même place que le témoin (9,113 contre 9,106 m à 2,3 s ;
  9,838 contre 9,874 m à 2,6 s) ; une hauteur bien plus basse au dernier mètre (168 contre 228 mm à 2,6 s ; `outils/crete_film.py`). La
  pente douce à 1:12, ou la 3D trop haute (S713), restent à départager. L'essai n'affirme plus que (5).
