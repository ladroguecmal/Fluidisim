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

Session : S494 — **en cours**. En autonomie, **6.2, les forces de W sur un corps** : le corps rigide (S331–S336) n'interroge que B
(`BackgroundWater`) ; « Manquent W, le courant, la turbulence ». Les impacts de W (`RadialImpact`, composés à B par
`composition::compose`, ADR-077) n'entrent pas dans sa requête.

**Ce que la session fait.** Une requête `MixedWater` (B + impacts confirmés, par la composition autoritaire, refus comptés et repliés
sur B) derrière `WaterQuery` ; l'accélération de W par différence centrée de 1 ms. Un essai : une bouée de 0,5 × 0,5 × 0,4 m à
500 kg/m³ à 5 m d'un impact d'1 kJ (λ = 4 m), sur une houle de B.

**Ordre de grandeur, écrit avant.** Un impact d'1 kJ sur ≈ 4 m de rayon : `a ≈ √(2E/(ρgπR²))` ≈ 6 cm au centre, ≈ 1 à 3 cm à 5 m ;
pente `k·a` ≈ 0,03 à 0,1. La bouée : tirant 0,195 m, `ωₙ = √(g/tirant)` ≈ 7,1 rad/s contre `ω` ≈ 3,9 pour λ = 4 m — elle suit
la surface amplifiée de ≈ 1,4 ; son empreinte voit la pente à `sin(kL/2)/(kL/2)` ≈ 0,975.

**Critères, écrits avant.** (1) sans impact, la requête mixte rend la trajectoire de `BackgroundWater` à 10⁻⁶ m sur 20 s (la seule
renormalisation de la normale) ; (2) le pilonnement suit l'oscillateur de référence `m·z'' = ρgA(η̄ − z)` forcé par la surface moyenne
sous l'empreinte (RK4 à 0,1 ms) à 3 % de max|η̄| (prévu < 1 %), et l'impact fait bouger la bouée d'au moins 30 % de son amplitude ;
(3) le déplacement horizontal suit l'excursion de la particule de surface `∫u dt` à 5 % de son maximum (prévu ≈ 2,5 %, l'empreinte) ;
(4) aucun refus de la composition. 6.2 reste partielle (le sillage de pression, le courant, la turbulence).

### Plan

- [x] **P1** — jeton, plan seul.
- [ ] **P2** — `Prepared::sample_local`, `MixedWater` ; essais (1)–(4).
- [ ] **P3** — preuve ; liste 6.2 ; lot des registres (dû) ; rituel.

### Notes de reprise
