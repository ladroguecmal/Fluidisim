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

Session : S339 — **terminée**. **Porte B, revue R16** : une onde circulaire née d'un impact, sur la mer de
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
- [x] **P3** — banc sans fenêtre ; critère 2, choix de `A` et `σ`.
- [x] **P4** — captures, aperçus ; critère 3.
- [x] **P4 bis** *(ajoutée après P4)* — **l'anneau** : un paquet circulaire préparé comme le front de R11 —
  `η = a·cos(k·(r − r₀))·e^{−(r−r₀)²/2s²}`, vitesses sortantes de la théorie linéaire, `a·k` = 0,26 comme le paquet
  de S302 —, `--anneau` ; banc (zéro colonne hors bornes ; rayon de la crête au trajet de la vitesse de groupe,
  facteur 1,5) ; captures, aperçus.
- [x] **P5** — preuve (SCENE-DELTA3D-S302 §8) ; R16 au registre ; file.
- [x] **P6** — rituel ; arrêt pour le verdict.

### Notes de reprise
- **P2, critère 1 tenu.** `Impact` (cratère de Ricker, au repos) et `Config::impact_review()` — A 0,4 m, σ 1,5 m,
  centre (0, 12), à régler en P3 ; `--impact` ; les captures δ 3D rendent la main après les options de rendu ;
  dossier `s339`. `INSTANTS=60,120 --houle --delta3d --captures` avant et après : **16 empreintes identiques au
  bit** (1 s : référence avec `0xa460b685bdc480b4` … ; 2 s : haute sans `0xd5be2b4afa89d233`).
- **P3, critère 2.** `IMPACT=A,σ --delta3d-scene-mesure --impact`, 12 s, centre (0, 14). Six cratères, **tous sans
  colonne hors bornes**, pas dégradés comme le témoin (tous, S302). Divergence franche max : 0,018 (0,4 ; 1,5),
  0,026 (0,6 ; 1,5), 0,040 (0,8 ; 2), 0,059 (1 ; 2,5), 0,034 (0,65 ; 2,5), 0,057 (0,78 ; 3) — témoin 0,022, paquet
  de S302 0,022–0,034. **Retenu : A 0,65 m, σ 2,5 m** — pente 0,252 (prévue 0,255) sous 0,26 (critère 1) ; volume
  net −1,1·10⁻⁵ m³ pour 9,4 m³ de creux, arrondi f32 ; à σ 3 m la queue tronquée laisse −1,9·10⁻³ m³, rejeté ;
  pentes 0,39 écartées par le critère 1. Anneaux (moyenne azimutale de l'onde isolée) : 0,105 m à 6,75 m (3 s),
  0,079 à 10,25 (5 s) ; maximum isolé 0,13–0,17 m. Rayon / trajet de groupe (2,08 m/s) : 3 s 1,08 ; 4 s 0,81 ;
  5 s 0,98 ; 6 s 0,78 — **tenu** ; avant 3 s le maximum est le bord du cratère qui s'effondre ; 7 s 0,63, 8 s 0,74 :
  l'anneau dominant a passé 11 m, l'éponge. Coût 4,6 ms, inchangé.
- **P4, critère 3 tenu, et ce qu'il montre.** `INSTANTS=0,60,180,300 --meilleur --eau-physique=2 --delta3d --impact
  --captures` : 32 images `viewer/captures/s339`, aperçus PNG, différences ×6. **Le cratère se voit** (0 et 1 s,
  poses rasante et proche) ; **les anneaux de 10 à 17 cm ne se voient pas** dans la mer de `Hs` 2,5 m — à 3 s, avec
  et sans δ indiscernables à l'œil ; la différence ×6 les montre, déformés. Physiquement attendu ; mais une revue
  sur une onde invisible n'informe pas le critère 3. D'où P4 bis. Octets changés, pose de référence : 4,95 % (0 s),
  15,7 (1 s), 13,0 (3 s), 14,6 (5 s). Bande claire en bas de la pose rasante, avec et sans δ : la mer coupée au
  plan proche, préexistante.
- **P4 bis, tenu.** `Ring` et `Config::ring_review()` : 41 cm, λ 10 m (`a·k` 0,258), crête à 5 m du centre
  (0, 16), enveloppe 3,5 m ; pente initiale 0,226 ; volume net 9,55 m³ (une crête, pas un train : 1,1 cm sur le
  domaine). Banc 12 s : **zéro colonne hors bornes**, pas dégradés comme le témoin, divergence franche max 0,043
  (témoin 0,022). Rayon de la crête / (r₀ + trajet de groupe 1,98 m/s) : 1 s 1,33 ; 2 s 1,37 ; 3 s 1,08 ; 4 s 1,18 ;
  5 s 0,96 ; 6 s 1,02 ; 7 s 1,05 ; 8 s 0,83 — dans le facteur 1,5. Moyenne azimutale de la crête 0,40 → 0,26 (2 s)
  → 0,11 m (5 s). Captures `INSTANTS=0,60,120,180 … --anneau --captures`, préfixe `s339a` : **l'anneau se lit** —
  proche 1 et 2 s, haute 1 s ; il se déforme en traversant. Une fine ligne claire suit sa crête (vue haute) :
  question posée, non attribuée. Empreintes : proche avec 1 s `0x5a79f01d64083ef3`, 2 s `0x2e37efc6f485f73d` ;
  haute avec 1 s `0xd4e6d1e43e57c1cb` ; les images « sans » de 0 et 1 s égales au bit à celles de l'impact.

