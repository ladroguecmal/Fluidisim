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

Session : S324 — **en cours**. **Lot 3, premier lot : les faces coupées de δ en trois dimensions**
— la moitié basse d'I3 ([TROIS-SYSTEMES-S308](../docs/registres/TROIS-SYSTEMES-S308.md) §5), sur le
chemin de la porte D, donc de la v1.
Agent : Claude Opus 5.5, application desktop ; fichiers, git, cargo, carte réelle, accès web.
Entrée : *« Je suis ta recommandation »* (2026-09-22) — **le lot 3 prend la place du lot 2 dans
l'alternance d'ADR-184 tant que l'ordre E est bloqué** ; à consigner en ADR-188. Maillons à 3 : la
session doit recevoir une capacité.

**Ce que la session doit rendre possible.** Un fond qui n'est pas plat dans la référence 3D : la
découpe de S232 — fractions de volume et ouvertures de faces sur un fond **linéaire par morceaux**
— portée à la grille x-y-z, dans l'opérateur, la divergence et la correction du mode linéaire. C'est
la géométrie sur laquelle reposeront obstacles fixes, puis corps. Consommateur : le lot 4 (corps
rigides), la porte D, et le point 4.15 de la liste.

**La géométrie.** Fond fourni au centre des colonnes, ramené aux coins par moyennes emboîtées ;
chaque empreinte de colonne coupée en deux triangles par une diagonale fixe, le fond **linéaire sur
chacun** : toutes les intégrales sont alors exactes — ouverture d'une face latérale par la formule 1D
de S232 le long de son arête, ouverture d'une face horizontale par l'aire où le fond est dessous,
fraction de volume par l'intégrale exacte de `clamp((haut − fond)/dx)` sur chaque triangle. Quand le
fond ne dépend pas de `y`, **les formules de la 2D elles-mêmes**, pour garder l'identité au bit.

Critères, écrits avant le code :
1. **Géométrie** : fond indépendant de `y` → ouvertures et fractions identiques **au bit** à la 2D ;
   fond plan `αx + βy + γ` → fractions exactes à l'arrondi f32 contre une forme fermée indépendante ;
   `Σ_k fraction·dx` = profondeur moyenne de l'empreinte à 10⁻⁶ près ; aucun coin étroit perdu.
2. **Opérateur** : fond plat → toutes les réceptions 3D antérieures **au bit** ; lac au repos sur fond
   coupé → vitesse **nulle en bits** ; `ny` = 1 sur les trois fonds de S232 → vitesses du premier pas
   identiques au bit à la 2D.
3. **Fond vraiment 3D** : débit à travers `x = L/2`, un pas depuis le repos, à trois mailles → ordre
   **≥ 1,8**, incréments de même signe et décroissants (garde de S197), comme S232 en 2D.
4. Aucune allocation dans le pas ; coût publié ; rien du mode mobile ni du couplage touché.

### Plan

- [x] **P1** — jeton, plan seul.
- [x] **P2** — ADR-188 : la décision de l'utilisateur ; file, feuille de route.
- [x] **P3** — la géométrie coupée 3D et ses essais (critère 1).
- [x] **P4** — l'opérateur, la divergence et la correction pondérés ; `configure_with_bottom` ; essais du
  critère 2.
- [>] **P5** — le banc du fond 3D : ordre de convergence du débit ouvert (critère 3).
- [ ] **P6** — preuve `docs/validation/FACES-COUPEES-3D-S324.md`, avec « Reproduire » ; file, liste 4.15,
  feuille de route.
- [ ] **P7** — S320 P5b : §5 bis au retour du calcul lancé à 20:11 — asynchrone.
- [ ] **P8** — rituel.

### Notes de reprise

**P3 (22:04).** `delta3d_cut.rs` : coins par moyennes emboîtées, **quatre triangles** autour du centre
(symétrique en `x` et `y`), intégrales exactes ; les formules 2D (`cut_fraction`, `open_below`, sortie
de `cut` sans changer une opération) servent quand l'empreinte ne dépend pas de `y`. Cinq essais verts
au premier passage : identité **au bit** avec la 2D sur les trois fonds de S232 ; plan exact à 2·10⁻⁶ ;
colonnes à 2·10⁻⁶ ; coin étroit contre quadrature ; symétrie miroir à 10⁻⁶. Les 67 essais δ 2D passent.

**P4 (01:06, le 23).** Chemin coupé séparé du chemin plat (`cut: Option<Cut3>`, `None` = S295 au bit) :
opérateur, divergence, second membre, erreur inverse, correction et flux de colonne pondérés comme la
2D ; couvercle entièrement mouillé exigé ; pas mobile et couplé refusent la découpe. Six essais verts :
`ny` = 1 sur les trois fonds de S232 **identique au bit à la 2D sur 200 pas** (surface, `u`, `w`,
itérations) ; lac au repos exact ; opérateur symétrique défini positif ; divergence ouverte tenue ;
faces fermées sans vitesse ; refus du pas mobile. Suite : **589 réussis**, 0 échec. Heurt : le long bloc
de texte passé au shell ne s'analysait plus (`unexpected EOF`) — passer par un fichier. Pause de
l'utilisateur entre 22:05 et 01:03.
