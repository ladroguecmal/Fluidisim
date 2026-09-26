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

Session : S383 — **en cours**. Verdict R31 : *« Je valide R31, continue la pluie »*. Cette session : **la pluie, pièce 4
d'ADR-205 — les gerbes** (liste 8.4). Agent : Claude Opus 5.5, application desktop ; fichiers, git, carte réelle, accès
web ; Godot 4.4.1 local.

**Thèse.** À chaque impact qui laisse un anneau (D ≥ 1,5 mm, au taux de Marshall et Palmer × Atlas), une **gerbe**
au-dessus de l'eau : la couronne, puis le jet de Worthington et ses gouttelettes — hauteurs, durées et tailles tirées des
mesures publiées pour des gouttes de pluie à leur vitesse terminale sur eau profonde, selon le diamètre. **Les mêmes
impacts que les rides** : le tirage (couche, période, maille → instant, centre, diamètre) sort du nuanceur des rides en une
fonction partagée, que lit un système de particules (une particule par couche, maille et période, dans une fenêtre autour
de la caméra). Au loin, où une gerbe tient sous le pixel, sa **part d'aire moyenne** (taux × aire × durée) éclaire la
surface de l'eau, fondue avec les particules par l'empreinte du pixel. Sur le sol et les margelles, les éclaboussures au
seuil de Mundo, Sommerfeld et Tropea (1995), si le temps le permet.

**Critères, écrits avant.** (1) Sans pluie : les 12 images **identiques au bit**. (2) Sous la pluie, gerbes éteintes : les
images de la pluie de S382 **identiques au bit** (le tirage partagé ne change pas les rides). (3) Chaque gerbe naît au
centre d'un anneau et à son instant — compté sur une image de contrôle : **100 %** à moins d'1 mm. (4) Le nombre de gerbes
vivantes par m² égal à **taux × durée de vie à ±5 %** (ou deux écarts-types de Poisson). (5) Hauteurs et durées de la
couronne et du jet **dans l'intervalle des mesures publiées** pour chaque diamètre. (6) Coût mesuré ; photographies
réelles ; jugement de l'utilisateur (R32).

### Plan

- [x] **P1** — jeton, plan seul.
- [x] **P2** — sources : couronne, cavité, jet de Worthington, gouttelettes des gouttes de pluie sur eau profonde (selon D,
  à vitesse terminale) ; seuil sur surface sèche ; photographies ; chiffres retenus.
- [x] **P3** — le tirage partagé (`pluie_impact`) sorti des rides ; critère 2 (rides au bit).
- [x] **P4** — les gerbes sur l'eau de la piscine : particules (fenêtre de mailles sur les eaux), géométrie par diamètre et
  âge (couronne, dôme, jet), radiance de l'eau comme les gouttes.
- [x] **P4b** — la mer : la gerbe posée sur la surface déplacée (la somme des ondes de la bande, évaluée au point d'impact
  en coordonnées de Lagrange, comme les rides), fenêtre autour de la caméra.
- [ ] **P5** — au loin : la part d'aire moyenne des gerbes dans les nuanceurs d'eau, fondue par l'empreinte.
- [ ] **P6** — contrôles : critères 1, 3, 4, 5 ; coût.
- [ ] **P7** — les éclaboussures au sol (seuil de Mundo, Sommerfeld et Tropea) ou, faute de temps, leur déclencheur.
- [ ] **P8** — images de R32 ; REVUE-VISUELLE §37.
- [ ] **P9** — preuve `GERBES-S383` ; liste, file, feuille de route, index.
- [ ] **P10** — rituel.

### Notes de reprise

**P2 — les sources.**
- **A — Wang, Liu, Bayeul-Lainé, Murphy, Katz, Coutier-Delgosha (2023)**, *Analysis of high-energy drop impact onto deep
  liquid pool* (JFM ; arXiv 2302.02728), sur l'expérience de **Murphy et al. (2015, JFM 780)** : goutte de pluie de
  **4,1 mm à 7,2 m/s** (81 % de sa vitesse terminale), We 2 893, Fr 1 322, sur eau profonde. Texte : couronne à son rayon
  maximal (≈ 13 mm) vers 3 ms, à sa hauteur maximale vers 12 ms, quand son bord se referme (dôme, « bubble canopy ») ;
  cavité la plus profonde vers 24 ms ; jet central (Worthington) vers 40 ms ; ≈ 2 000 microgouttelettes, tailles en deux
  modes (50 et 225 µm), énergie au plus 8 % de celle de la goutte ; les petites partent en rasant dans la première
  milliseconde, les grosses plus haut, des ligaments de la couronne.
- **Mesuré sur leur figure 3** (500 × 321 px, lue par canevas ; barre 17 px = 10 mm, vérifiée par la goutte : 7 px =
  4,1 mm) — sommet de la gerbe au-dessus de l'eau : **1 ms 5,9 mm ; 3 ms 11,2 ; 7 ms 17,1 ; 12 ms 19,4** (dôme fermé) ;
  **18 ms 24,7** (jet central sur le dôme) ; **41 ms 23,5 ; 52 ms 21,8** (jet large). Largeur à la base : 14, 16, 21, 25, 26,
  29 mm (1 à 41 ms).
- **B — Watson et al. (2024, PNAS 121)**, gouttes de 4 mm à 2,2–6 m/s (Fr 127–850) : cratère `κ₁/D ∼ Fr^0,25`, premier
  jet `δ₁/D ∼ Fr^0,27`, `δ₁ ∝ κ₁^1,03` (des exposants ; les valeurs absolues, dans leurs figures, non lues).
- **C — Mundo, Sommerfeld et Tropea (1995)** : sur surface sèche, éclaboussure si `K = Oh·Re^1,25 > 57,7` ; une goutte de
  2 mm à 6,55 m/s : Re ≈ 13 000, Oh ≈ 0,0026, **K ≈ 360** — toute goutte de pluie visible éclabousse sur le béton.
- **Refusés ou non lus** : JPO 2018 (vent) et HAL (Michon, Josserand, Séon 2017) bloqués ; PDF de l'arXiv 2604.10491
  illisible par l'outil. Résumés automatiques de PNAS contradictoires : écartés, seuls les exposants gardés.
- **Le modèle retenu** (hypothèse dite) : la gerbe de A, à l'échelle `s(D) = (D/4,1 mm)·(Fr/1 322)^0,26` en longueur (B :
  exposant moyen de κ₁ et δ₁), `√s` en temps (effondrement de cavité gouverné par la gravité aux grands Fr), `Fr = v²/(g·D)`,
  `v` d'Atlas. Pour D = 1,5 / 2 / 4,1 mm à leur vitesse terminale : s ≈ 0,41 / 0,56 / 1,10 — dôme ≈ 8 / 11 / 21 mm.
- Auteurs de B vérifiés (écrits d'abord sans lecture, puis contrôlés) : Daren A. Watson, M. R. Thornton, H. A. Khan,
  R. C. Diamco, D. Yilmaz-Aydin, A. K. Dickerson, PNAS 121 (5), e2315667121.

**P3 — le tirage partagé.** `pluie_phase(j)`, `pluie_graine(j, k)`, `pluie_decalage(hk)`, `pluie_impact(...)` (naissance,
centre, diamètre) dans `pluie.gdshaderinc` ; les rides les appellent. **Critère 2** : 8 images sous la pluie (piscine 10 et
50 mm/h, trois vues ; mer 10 mm/h, deux poses ; `pluie8.sh`) — d'abord rendues deux fois au commit d'avant : **identiques
entre elles** (la pluie est déterministe, contrairement à ce que S380 n'exigeait pas) ; après le découpage : **8 / 8 au
bit**. **Critère 1** : nouvelle référence, la fin de S382 (occultation comprise) : 12 / 12 au bit.

**P4 — les gerbes de la piscine.** `gerbes.gd` (un `GPUParticles3D` : couches × 2 périodes × mailles d'une fenêtre qui couvre
les deux eaux — 26 × 12 mailles, 24 336 particules à 10 mm/h), `gerbe.gdshader` (le tirage des rides, `pluie_impact` ;
vie `0,08·√s` s plus une pose ; hors de l'eau, rien), `gerbe_dessin.gdshader` (relevés de P2 interpolés ; coupe jusqu'à
10 ms, puis dôme et jet de 4 mm à tête de 2,5 mm ; moyenne sur quatre instants de la pose de 1/60 s ; radiance des gouttes ;
opacités réglées 0,25 / 0,6 / 0,3 / 0,55 / 0,8). **Impasses** : `return` interdit dans `process()` des particules (tout
dans une fonction) ; **rien ne se dessinait** — même des carrés rouges forcés — : l'eau du bassin, transparente et qui lit
l'écran, était dessinée après les particules et les recouvrait (tri par la distance du centre des boîtes) ; le dessin passe
après elle (`render_priority` 1). **Vu** : de petites coupes blanches semées sur l'eau de près, des points clairs de loin.

**P4b — la mer.** Fenêtre carrée de 12 m dans le plan de B, en coordonnées de Lagrange (celles des rides de la mer), à
4,5 m devant la caméra ; la gerbe posée à `q + d(q)`, hauteur `η(q)` (`bande_au_point`). La bande et `bande_au_point`
sorties de `surface_b.gdshaderinc` dans **`bande_b.gdshaderinc`** (que `surface_b` inclut) : `surface_b` dépend de
`optique_eau` (`sous_eau`), illisible depuis un nuanceur de particules — une seule source gardée. **Défaut trouvé,
antérieur (S380)** : les captures de la mer n'appelaient pas `phases()` après `pose()` — la boîte des gouttes restait devant
la caméra de départ (« proche ») : **en pose « référence », les gouttes de R29 et R30 tombaient ≈ 11 m trop loin** ; les
gerbes, de même (elles apparaissaient près de l'horizon). Corrigé : après chaque pose, la pluie suit. **Vu** au ras de l'eau
(`demi_dessus`) : dômes et jets posés sur les vagues, une couronne ; en pose « référence », les traînées au premier plan.
Contrôles : sans pluie **12 / 12 au bit** (référence fin de S382) ; sous la pluie sans gerbes **7 / 8 au bit**, la mer en
pose « référence » change — la correction, voulue.
