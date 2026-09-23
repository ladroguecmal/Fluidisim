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

Session : S330 — **en cours**. **Lot 3 : la frontière mobile** — essai 3 de la piscine, une boule à
mouvement imposé ; chemin de la v1 ([ADR-189](../docs/adr/ADR-189-la-v1-d-abord.md)).
Agent : Claude Opus 5.5, application desktop ; fichiers, git, cargo, carte réelle, accès web.
Entrée : *« continue, jusqu'à la v1 »* ; suite déclarée par S329.

**Ce que la session doit rendre possible.** Un solide qui **bouge** dans la référence 3D, mode linéaire —
celui où un bateau percera un jour le couvercle. L'hôte fournit, avant chaque pas, la distance signée du
solide à sa nouvelle position et sa vitesse de translation. δ recoupe sa géométrie **en place, sans
allocation**, depuis une copie du fond seul comptée à la configuration ; la part d'une face que le solide
couvre avance à sa vitesse dans la divergence (formulation pondérée par les ouvertures) ; une face qui
s'ouvre naît à la vitesse du solide ; l'eau que le solide déplace dans une colonne en élève la surface.
Consommateur : le corps du lot 4, puis le bateau de la porte D.

Critères, écrits avant le code :
1. **Immobile** : reposer le même solide à vitesse nulle ne change rien, au bit.
2. **Volume** : une sphère immergée qui se déplace — `Σ(η − η₀)·dx²` suit la variation du volume discret
   du solide à 10⁻⁹ m³ près, pas après pas.
3. **Faces** : divergence sous la tolérance à chaque pas ; aucune vitesse sur une face fermée ; aucune
   valeur non finie quand des faces naissent et meurent.
4. **Masse ajoutée** : départ impulsif d'une sphère immergée, un pas — la force de pression donne
   `C_m = m_a/(ρV)` ; **à 5 % de 0,5** (sphère en fluide illimité) à la maille la plus fine, et convergent.
5. **Rien de changé** pour les solides fixes et le fond : suite complète verte.

### Plan

- [x] **P1** — jeton, plan seul.
- [x] **P2** — `set_solid` : géométrie recoupée en place depuis le fond seul, faces qui naissent et
  meurent, eau déplacée ; flux du solide dans la divergence ; critères 1, 3, 5.
- [x] **P3** — volume (critère 2) ; force de pression sur la paroi depuis la pression de δ.
- [x] **P4** — banc : départ impulsif, trois mailles, `C_m`.
- [x] **P5** — preuve : section datée de [FACES-COUPEES-3D-S324](../docs/validation/FACES-COUPEES-3D-S324.md),
  avec « Reproduire » ; file, liste.
- [ ] **P6** — rituel.

### Notes de reprise
**P2 (23:16).** `Volume3::set_solid(nœuds, vitesse)` : la découpe refaite en place depuis celle du fond
seul (`Base3`, comptée à la configuration avec le volume du solide par colonne) ; `add_solid` vérifie
d'abord (`check_solid`), écrit ensuite — refus atomique ; face qui s'ouvre = vitesse du solide, face
fermée = 0 ; l'eau déplacée monte dans sa colonne, le nouveau volume calculé dans `rhs`, libre entre
deux pas — aucune allocation ; la diagonale de Jacobi suit. Divergence : la part couverte par le solide
(`ouverture du fond − ouverture`) avance à sa vitesse. `solid_force` : la pression de la maille sur ses
polygones de coupe. Essais : **reposer le solide immobile, 50 pas au bit** ; sphère à 1 m/s sur deux
mailles, faces qui naissent, divergence sous la tolérance, faces fermées à zéro, refus sans écriture.
Une fausse alerte : l'essai calculait sa sphère avec un pas en f64, le volume en f32 converti. Cœur : 481.
**P3 (23:18).** L'eau déplacée entre dans `η` par la somme compensée du transport (S233). Sphère à 1 m/s,
50 pas : `Σ(η − reste − z₀)·dx²` suit la variation du volume discret à **3,6·10⁻¹¹ m³** au pire (critère :
10⁻⁹). Le volume discret de la sphère ne varie que de 4·10⁻¹⁰ m³ sur deux mailles de déplacement : l'erreur
de l'interpolation linéaire tient à la courbure, pas à la position. Cœur : 482 réussis.
**P4 (23:19).** `delta3d_fond_coupe --masse-ajoutee` : sphère de 0,3 m au centre d'un cube de 2,4 m, départ
impulsif à 0,1 m/s, un pas ; 11 s. `C_m` = **0,48969 / 0,50506 / 0,50792** à 3 / 6 / 12 mailles par rayon ;
incréments 0,0154 puis 0,0029 (ordre ≈ 2,4), limite extrapolée ≈ 0,509. **Critère 4 tenu** : 1,6 % de 0,5.
L'excès va dans le sens du confinement — murs rigides à quatre rayons ; une sphère dans une sphère rigide de
rayon quadruple aurait `(1 + 2q)/(1 − q)` = 1,048 avec `q = (1/4)³`, le cube, plus grand, moins.
