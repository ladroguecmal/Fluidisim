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

---

## Session en cours

Session : S237 — en cours
Agent : Claude Code, Opus 5 (fichiers, git, cargo, Python/numpy/sympy et GPU local disponibles)
Entrée : « Continue », master propre à f5f78bf, trois copies au même commit, jeton libre,
secteur. Cœur : 419 réussis, 5 ignorés.

**Objectif.** Première surface **géométriquement mobile** dans le candidat δ MAC x-z : la frontière
à pression imposée se trouve à la hauteur réelle `η(x)` et non plus au couvercle fixe `z₀` ; les
mailles entrent et sortent du fluide ; l'advection quadratique est présente. Reçue contre une
référence **non linéaire indépendante**.
**Ce que la lecture fixe avant le plan.** Mode S233 : Dirichlet `p_dyn = ρg(η−z₀)` au couvercle
`z₀ = nz·dx`, facteur de demi-maille 2 dans l'opérateur, η transporté par flux de colonnes jusqu'à
`z₀`, advection retirée. Le dépôt possède un véhicule potentiel **non linéaire d'ordre 3** reçu
contre Stokes (S193, `examples/support/nl_surface.rs`, ADR-122) : périodique de longueur `2L`,
condition initiale paire, il donne l'onde stationnaire d'amplitude finie d'un bassin à murs.
**Thèse.** Fonction hauteur sur la grille MAC ; condition de Dirichlet par **fluide fantôme**
(Gibou) à la distance `θ·dx` de la surface, verticale et horizontale — opérateur symétrique, CG
conservé ; flux de colonne intégré jusqu'à la hauteur mouillée de chaque face ; vitesses
extrapolées dans la bande d'air ; niveau de référence au repos distinct du sommet du domaine, pour
un repos exact. À petite amplitude, le mode mobile doit rejoindre S233 ; à amplitude finie, il doit
produire l'harmonique `2k` que le mode linéaire ne peut pas produire.
**Critères, déclarés avant construction.** Repos exact au bit à un niveau intérieur ; opérateur
symétrique au bit ; volume conservé à l'arrondi ; petite amplitude à ≤1 % du mode linéaire S233 ;
amplitude finie : profil contre HOS M=3 sur une période, erreur décroissante en raffinant et
harmonique `2k` captée, le mode linéaire servant de témoin (harmonique nulle) ; tolérances de banc
écrites en P2 avec leur motif, jamais ajustées après mesure. Refus atomiques : surface à moins
d'une maille du fond ou du sommet, garde de pas, non-convergence ; zéro allocation.
**Arrêt.** Mode mobile construit et reçu, ou constat chiffré de ce qui manque. Aucune ambition
réduite ; déferlement, mouillage du fond, air, cavité et 3D restent hors lot et nommés.

### Plan

- [x] **P1** — amorce, jeton, plan seuls.
- [x] **P2** — lectures ciblées (ADR-122, SURFACE-LIBRE-NL-S193, API `nl_surface`, ADR-141, tests
  δ surface) ; protocole écrit : modèle discret, gardes, domaine d'amplitude, tolérances et motifs.
- [x] **P3** — référence : onde stationnaire HOS M=3 (bassin 8×4 m, ka 0,01/0,1/0,2), séries
  `η(x,t)` aux centres de colonnes ; recoupement sympy de l'ordre deux à petite amplitude.
- [x] **P4a** — cœur : géométrie mobile (ensemble fluide, `θ`, opérateur, second membre et
  correction à Dirichlet mobile) ; tests repos exact et symétrie.
- [x] **P4b** — cœur : extrapolation, flux mouillé, advection, `step_surface_mobile` atomique ;
  tests petite amplitude contre S233, volume, changement de topologie, refus, allocation.
- [ ] **P5** — réception : banc MAC contre HOS, trois résolutions, deux amplitudes, harmonique,
  témoin linéaire, coût ; document de validation.
- [ ] **P6** — rituel §6, file, feuille de route, jeton ; copies à synchroniser.

### Notes de reprise

Base : S236 — cœur 419/5 ; S233 mode linéaire (erreurs fines 0,066 % Terre / 0,017 % Lune).

P2 : protocole dans `docs/validation/SURFACE-MOBILE-S237.md` §1. Choix motivés : bassin `L = h = 2 m`
(`kh = π`) plutôt que 8×4 m du plan — à 8×4 m (`kh = π/2`, λ = 16 m) Ursell `U = 4a` n'admet que
`a ≲ 0,025 m` dans le domaine d'ADR-122, harmonique ≈ 3·10⁻⁵ m, invisible sur la grille ; à 2×2 m,
`U = 2a`, `a = 0,10 m` → `U = 0,2`, harmonique de l'ordre du cm. Grilles 32/64/128 (32 mailles par
λ au plus fin), `nz` jusqu'à 2,25 m. `θ_min = 10⁻³` avec préconditionnement diagonal du CG mobile.
Coût estimé au plus fin : ≈18 000 mailles, une période ≈1,6 s à 1 ms.

P3 (`examples/delta_mobile.rs oracle`) : ordre deux depuis le repos dérivé par sympy (script
`standing2.py`, bloc-notes) puis écrit en forme fermée dans l'exemple : `B₂'' + Ω²B₂ = σ₂D₂ + K₂'`,
`B₂(0) = B₂'(0) = 0` ; `ω = 3,918171`, `T = 1,603601 s`, `Ω = 5,5515` (harmonique libre),
`B₂ = −0,7913 cos Ωt + 0,3986 cos 2ωt + 0,3927` (constante `k/4`), max|B₂| = 1,4780 sur une période
→ `b₂` max 0,148 mm / 3,69 mm / 14,78 mm à a = 1/5/10 cm. HOS M=3, Q16 : **K64** écart relatif
0,31 % dès a = 1 cm — plancher du symbole discret (4,8·10⁻³, A242/L277) ; **K256** : 0,034 % /
0,823 % / 3,283 % à a = 1/5/10 cm, rapport 3,99 entre 10 et 5 cm = `(ka)²` exact → termes d'ordre
quatre absents de l'analytique, pas défaut du véhicule. M=2 : 0,025 / 0,757 / 3,078 %. Précision
propre (a = 10 cm, K256) : Q32 1,0·10⁻⁸ a, K512 5,3·10⁻⁵ a, dt 0,5 ms 1,6·10⁻¹¹ a. Référence
retenue : **M=3, Q16, K256, dt 1 ms** (`REF_BAND`, `REF_LEVELS`). Volume ≤ 2·10⁻¹⁹.

P4a : `src/delta_mobile.rs` (sous-module de `delta_projection`) — `wet`, `ghost_up`,
`ghost_side`, `apply_mobile`, `rhs_mobile` (second membre + inverse de diagonale en une passe),
`precondition_into_dir`, `dot_prec`, `correct_mobile`, `set_free_surface(eta, rest)`,
`SURFACE_THETA_MIN = 1e-3`. Champs `rest`, `mobile`, `prec` (+4·nx·nz octets ; test de
comptabilité passé de six à sept f32 par maille). `project` : branches `jacobi` pour second
membre, CG préconditionné, correction et diagnostic (mailles fluides seules) ; chemin non mobile
écrit pour rester au bit (`alpha = rz/dq` avec `rz ≡ rr`). Tests : **symétrie `A_ij = A_ji` au bit**
et diagonale du préconditionneur au bit, surface ondulée 2×3 m à fantômes latéraux et verticaux
(premier essai avec amplitude 0,3 : deux faces latérales seulement, sous l'exigence de quatre —
amplitude portée à 0,5, exigence inchangée) ; projection d'un champ quelconque convergée,
divergence < 1e-4 ; **repos exact au bit** à 2,0 / 2,013 / 1,9 m sur fond bosselé, zéro itération.
Non-régression : 26 tests δ, 8 d'exécution, **empreinte S232 `0xc5ab1eadb094d058` inchangée**.

P4b : **protocole corrigé avant mesure** — critère 4 porté de a = 1 cm à **1 mm** (l'harmonique
physique vaut 1,5 % de a à 1 cm, P3) ; coefficient d'advection `dt` construit en f64 puis arrondi,
écrit dans le document. `step_surface_mobile` (gardes avant/après, sauvegarde, advection,
projection mobile, `extrapolate_mobile`, `transport_mobile`, validation), `surface_in_bounds`,
`wet_cells`. **Premier essai** (exemple `essai`, nx32, a = 5 cm, une période contre HOS) : profil
**0,85 % de a**, `b₂` 3,705 mm contre 3,673 mm (**2,4 %**), dérive de volume 3,7·10⁻⁹ m, mailles
fluides 1021..1029, ≤143 itérations, 4,5 ms/pas. Tests : repos exact 50 pas (1,9 m, fond bosselé) ;
onde 10 cm nx16 400 pas, volume à l'arrondi, topologie changeante ; mobile − linéaire à 1 mm sur
800 pas nx16 **< 1 %** ; refus sommet/fond avant pas, **refus d'après pas sur dynamique réelle**
(crête au sommet admis moins rien, dépassement au demi-cycle, pas > 400), `Convergence`, g = 0,
état au bit, mode désarmé ; exécution : **580 points d'expiration**, zéro allocation, reprise
identique, cinq refus d'entrée, horloge reculante. **Impasse** : champ à divergence nulle écrit à la
main pour le refus d'après pas → second membre d'arrondi, pression f32 plafonnée à **3,3·10⁻⁶** de
résidu relatif, CG simple (vérifié en forçant M = I) comme préconditionné, à 1 ms comme à 1 µs →
`Convergence` avant la garde. Suite du cœur **425 réussis, 5 ignorés** ; empreinte S232 inchangée.