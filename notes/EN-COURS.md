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

Session : S495 — **terminée**. En autonomie, **6.2, le sillage de pression derrière la requête du corps** : `MixedWater` (S494) compose
B et les impacts ; l'autre part de W, la pression d'un objet en marche (`bound_pressure`, ADR-103), n'y entre pas. Un corps ne sent pas
le sillage d'un autre.

**Ce que la session fait.** La composition de `mixed_water::sample_world_batch` (B, impacts, pression ; ADR-077) extraite par point, au
bit, et exposée au point local (`mixed::sample_local`) ; `MixedWater` la prend quand une pression est publiée à l'instant demandé. Un
essai : une source de pression gaussienne (σ = 1 m) en marche à 3 m/s, une bouée posée à 3 m de sa route.

**Ordre de grandeur, écrit avant.** Creux sous la source ≈ `p₀/(ρg)` : 2 cm pour 200 Pa ; sillage de Kelvin à 3 m de la route ≈ 0,5 à
1 cm, onde transverse `λ = 2πU²/g` ≈ 5,8 m, divergentes ≈ 2 m. Le second ordre cumulé `k·a·ω·t` (la leçon de S494) : ≈ 0,5 à 200 Pa
sur 3 s — le critère horizontal se tient donc au régime linéaire, 2 Pa (≈ 0,005). L'empreinte d'une bouée de 0,25 m sur 2 m : ≈ 0,7 %.

**Critères, écrits avant.** (1) l'extraction au bit : les essais de la composition mixte inchangés (`tests_mixed_water`), et sans pression,
`MixedWater` rend S494 à 10⁻⁶ m ; (2) à 200 Pa, le pilonnement suit l'oscillateur forcé par la surface sous l'empreinte à 3 % de max|η̄|,
et le sillage fait bouger la bouée d'au moins 30 % de max|η̄_P| ; (3) à 2 Pa, le déplacement horizontal suit `∫u dt` à 5 % de son maximum
(bouée de 0,25 m) ; (4) aucun refus. Avec S494, W entier (impacts et pression) derrière la requête du corps ; 6.2 reste partielle (le
courant, la turbulence).

### Plan

- [x] **P1** — jeton, plan seul.
- [x] **P2** — l'extraction au bit ; `MixedWater` avec la pression ; essais (1)–(4).
- [x] **P3** — preuve ; liste 6.2 ; rituel.

### Notes de reprise
- **P2 (en cours)** — l'extraction (`compose_local`, `mixed::sample_local`) : 11 560 points au bit contre `Prepared::sample_local` ;
  S494 inchangé aux chiffres près. `MixedWater.pressure`. **Critère 3 manqué d'abord à 40 %, au premier ordre** (même rapport à 2 et
  200 Pa). Impasses : la bouée 0,25 × 0,25 × 0,2 roule (GM ≈ 0 avec 4 points par axe) → bouée plate (0,4 × le côté) — l'écart ne
  change pas ; le corps égale `∫∫−g∇η` au µm : **l'écart est dans le champ** — `du/dt ≠ −g∇η` à 9 m de la source pendant ≈ 0,5 s
  (résidu ≈ 10⁻⁴ m/s² pour 2 Pa) : la recette d'essai 16 × 24 (`tests_mixed_water`) ne reconstruit la gaussienne qu'à
  `r ≲ 24/k_max` ≈ 4 m ; au-delà, une pression repliée agit sur l'eau, pas sur le corps. → recette 64 × 128, coupure 3 (le nombre de
  nœuds du sillage de production S212).
  **Recette 64 × 128 : 215 %** — l'hypothèse ne suffisait pas. Au point fixe (0, 3), le résidu `du/dt + g∇η` s'accumule au passage de
  la source : la **vraie** queue de la gaussienne à 3σ (`0,011·p₀`), dont le gradient l'emporte sur la pente du sillage (les deux ∝ p₀ ;
  rapport ≈ 3/(k·0,01)) — l'ordre de grandeur l'avait dit négligeable sans la comparer à la pente. → bouée en (−6, 6), à 6σ
  (e⁻¹⁸), route de −15 à 15 m sur 10 s, 12 s de mesure. Coût : ≈ 6,5 min l'essai (4 096 modes, ≈ 150 échantillons par pas).
  **6σ, coupure 3 : 25 % ; coupure 6 : 19 %** — encore au premier ordre. Diagnostic posé enfin (champ seul, sans pas du corps, au point
  de la bouée) : résidu `du/dt + g∇η` 4·10⁻⁸ m — **le champ est cohérent** ; `∫u − ∫∫(−g∇η)` croît **linéairement** : `u₀·t`, l'eau a
  une vitesse au départ (la source naît en marche), la bouée partait au repos — l'erreur de départ de S494, côté pression. → lâchée à
  la vitesse de l'eau composée. Leçon : trois remèdes essayés avant de séparer les chaînes (corps / champ / départ) — la séparation
  aurait dû venir d'abord (ADR-224 D1 : l'ordre de grandeur d'un remède — ici, le remède n'était pas diagnostiqué).
- **P2 fait** — lâchée à la vitesse de l'eau : (3) **0,85 %** à 2 Pa (0,25 m) ; à 200 Pa 12,5 % (second ordre, publié). (2) 0,05 % et
  0,03 % ; sillage : 9,6 mm de pilonnement pour 8,4 mm de surface. (4) 0. L'essai du sillage `#[ignore]` (≈ 8,5 min). Suite : 654
  essais, 17 ignorés.
- **P3** — preuve SILLAGE-CORPS-S495 ; liste 6.2 ; index ; journal.
