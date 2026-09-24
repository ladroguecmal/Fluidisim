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

Session : S339 — **en cours**. **Porte B, revue R16** : une onde circulaire née d'un impact, sur la mer de
référence provisoire ; chemin de la v1 ([ADR-174](../docs/adr/ADR-174-arbitrages-du-2026-09-19.md) D4), porte en
cours de §3 bis.
Agent : Claude Opus 5.5, application desktop ; fichiers, git, cargo, carte réelle, accès web.
Entrée — S338 : porte D reçue ; la v1 demande encore A, B et C. Critère 3 de la porte B
([ADR-175](../docs/adr/ADR-175-architecture-d-execution-de-delta-en-3d.md) §4) : *« une onde traverse une mer
étalée et s'y déforme »*, jugée convaincante. **R11** (S303) : δ sans artefact, fondu invisible, mais *« je ne
sais pas s'il s'agit d'une onde circulaire ou bien linéaire »* — le front injecté ne disait pas son origine —, et
la mer pas réaliste. **R14** (S308) : la troisième image, `--meilleur --eau-physique=2`, devient la référence
interne provisoire de l'océan. Aucune revue n'a montré δ sur cette mer.

**Ce que la session doit rendre possible.** Le verdict du critère 3, sur la mer que l'utilisateur a retenue
et avec une onde qu'il peut lire. **Deux défauts de chemin, trouvés en lisant** : les captures de `--delta3d`
rendent la main **avant** que `--eau-physique` (et `--tonalite`, `--ciel-mesure`, `--coupure`) soient lus — la
revue R11 n'aurait pas pu montrer la couleur de R14 ; et la scène ne sait injecter qu'un front.

Critères, écrits avant le code :
1. **L'impact** : un cratère à bord relevé, au repos — `η₀ = −A·(1 − r²/2σ²)·e^{−r²/2σ²}`, volume net nul —,
   vitesses nulles (problème de Cauchy–Poisson). Pente maximale ≈ 0,98·A/σ, tenue sous la cambrure du paquet de
   S302 (0,26). `Config::review()` inchangée ; les captures de `--houle --delta3d --captures` identiques au bit
   avant et après le changement (deux instants, empreintes relevées sur la même carte).
2. **Banc sans fenêtre** (`--delta3d-scene-mesure --impact`), 12 s : zéro colonne hors bornes ; pas dégradés et
   divergence franche comme le témoin *(corrigé en P2, avant toute mesure : « zéro pas dégradé » était faux —
   S302 §2 les déclare tous dégradés à 32 cycles, tolérance d'ADR-144, point de la porte C)* ;
   volume net initial de δ nul à l'arrondi ; les anneaux isolés (avec − témoin) **s'éloignent** du point
   d'impact, rayon du maximum publié chaque seconde et comparé à la vitesse de groupe du nombre d'onde dominant
   `√2/σ` — dans un facteur 1,5.
3. **Captures** sur `--meilleur --eau-physique=2`, les poses de S302, avec et sans δ ; aperçus PNG.
4. **R16** au registre, questions écrites ; **arrêt pour le verdict** (ADR-189 D3).

### Plan

- [x] **P1** — jeton, plan seul.
- [x] **P2** — l'impact dans la scène ; options lues avant les captures ; critère 1.
- [ ] **P3** — banc sans fenêtre ; critère 2, choix de `A` et `σ`.
- [ ] **P4** — captures, aperçus ; critère 3.
- [ ] **P5** — preuve (SCENE-DELTA3D-S302 §6) ; R16 au registre ; file.
- [ ] **P6** — rituel ; arrêt pour le verdict.

### Notes de reprise
- **P2, critère 1 tenu.** `Impact` (cratère de Ricker, au repos) et `Config::impact_review()` — A 0,4 m, σ 1,5 m,
  centre (0, 12), à régler en P3 ; `--impact` ; les captures δ 3D rendent la main après les options de rendu ;
  dossier `s339`. `INSTANTS=60,120 --houle --delta3d --captures` avant et après : **16 empreintes identiques au
  bit** (1 s : référence avec `0xa460b685bdc480b4` … ; 2 s : haute sans `0xd5be2b4afa89d233`).

