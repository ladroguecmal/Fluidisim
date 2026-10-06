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

Session : S523 — **terminée**. En autonomie, **C07 peu profond, la branche supercritique** : par 5 m de fond, le sillage au-delà du
critique est contenu dans un coin de demi-angle `arcsin(1/Fr_h)` (CAS-CANONIQUES C07, SPEC-001 §5) ; W en profondeur uniforme (S522) le
rend mesurable.

**Ce que la session fait.** `c07_profondeur` généralisé (vitesse, durée, grille en arguments ; ses défauts rendent S522) ; à `U` = 10 et
15 m/s (`Fr_h` = 1,43 et 2,14), 24 s, σ 2 m, recette 512 × 256 à coupure 3, la zone de mesure près de l'origine (rayon honnête 179 m).
**L'instrument éprouvé d'abord sur la référence de la même famille** (ADR-233 : profondeur finie, supercritique, même source) — la
moyenne de |η| le long des rayons issus de la source, de 20 à 60 m derrière, l'angle de son maximum (le front où s'empilent les ondes
longues) ; on regarde le profil ; s'il ne lit pas `arcsin(1/Fr_h)` sur la référence, il est changé avant W. Puis W, mêmes points.

**Ordre de grandeur, calculé.** `√(gh)` = 7,00 m/s ; demi-angles **44,46°** (10 m/s) et **27,83°** (15 m/s) ; à 60 m derrière, le front
passe à 58,9 et 31,7 m de l'axe ; parcours 240 et 360 m ; le transitoire du départ s'étend à 168 m de son point de départ, loin derrière
la zone. **La résonance** (la pente −½ en deçà du critique) n'est pas de cette session : avec σ = 2 m, le facteur de la source à l'onde
transverse varie de 5·10⁻⁵ (2,1 m/s) à 0,94 (6,3 m/s) et masquerait la pente — il faut σ ≤ 0,25 m (0,86 à 1), une autre recette.

**Critères, écrits avant.** (1) Sur la référence (convergée sur trois grilles), l'instrument lit `arcsin(1/Fr_h)` à 1° aux deux vitesses.
(2) **C07 peu profond, supercritique** : sur W, à 2° de 44,46° et de 27,83°. (3) W contre la référence, écart quadratique ≤ 10 % sur la
zone de mesure.

### Plan

- [x] **P1** — jeton, plan seul.
- [x] **P2** — l'exemple généralisé, la référence, l'instrument éprouvé ; (1).
- [x] **P3** — W ; (2), (3).
- [x] **P4** — preuve ; listes 3.2, 13.2 ; CAS-CANONIQUES ; rituel.

### Notes de reprise
- **P2 fini** — l'instrument déclaré (le maximum des rayons, 20–60 m, 24 s) lit sur la référence **14,5° et 11,25°** : le sillage
  intérieur d'ondes courtes ; changé avant W, comme prévu. Le profil (regardé, ADR-233) : un sillage intérieur, puis une **crête d'ondes
  longues juste en dedans du coin de Mach**, qui l'approche avec la distance (U = 10 : 41,5 / 43,25 / 43,75 / 44,0 / 44,5° de 20 à 120 m ;
  U = 15 : 24,25 / 26,0 / 26,75 / 27,0 / 27,25°). **Figé** : la dernière crête (le maximum local le plus extérieur), fenêtre 80–100 m,
  40 s (le transitoire hors de la zone). Sur la référence, trois grilles : **43,75–44,00° (attendu 44,46) et 27,00° (27,83)** → (1) tenu.
- **P3 fini** — montage 1 (512 × 256, coupure 3, 40 s) : 23 % / 62 % → localisé : hors du domaine d'ADR-132 (trajets 400–600 m, rayon 179 m).
  Montage 2 (512 × 512, coupure 1,5) : 0,03 % à 10 m/s, mais l'instrument lit 77° sur la référence même (Gibbs) — famille non éprouvée.
  **Montage retenu** (512 × 512, coupure 3, 40 s / 16 s) : 0,38 % / < 0,01 % ; **44,00°** à 10 m/s (tenu) ; 79,75° à 15 m/s (une crête du
  bruit f32 à 8·10⁻⁸ m : manqué, pas d'instrument changé après coup). A331 ouverte.
- **P4** — preuve C07-PEU-PROFOND-S523 ; listes 3.2, 13.2 ; CAS-CANONIQUES ; A331 ; index ; journal.

