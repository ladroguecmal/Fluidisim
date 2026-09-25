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

Session : S364 — **en cours**. **Physique : la bathymétrie entre dans B** — *« Continue »*, sans verdict R21 à R23 ;
alternance d'ADR-191 D3 après S363 (rendu). Suite de S362 : la référence existe, reste **l'entrée**, que S362 disait
devoir être tranchée par un ADR **mesuré** (coût, requêtes de jeu, déterminisme) ; ADR-004 §2.1 et §5 à réviser.
Agent : Claude Opus 5.5, application desktop ; fichiers, git, cargo, carte réelle, accès web ; Godot 4.4.1 local.

**La voie à mesurer.** W naît d'événements et ne porte pas une mer permanente ; relayer B au large par W près des
côtes mélangerait deux réalisations (ADR-004 §3). Candidat : **B transformé composante par composante** par des
**tables cuites** le long du profil — correction de phase (entière, Q32), facteur d'amplitude `K_s·K_r`, nombre
d'onde transversal, `coth kh` des vitesses —, tirées de la référence de S362 : le précalcul d'ADR-013, mais de
**paramètres**, pas de réalisations. Au large, les tables valent zéro et un : **B inchangé au bit**. Premier cas : côte
à isobathes droites (celui que la référence sait juger).

Critères, écrits avant le code :
1. **Précision** contre la référence (f64), houle d'1 m, 10 s, 30°, plage 1/50, jusqu'au déferlement : phase × amplitude
   ≤ **3 mm** (tolérance d'image, S201), facteur d'amplitude à 1 %. Balayage du pas des tables. **Prédiction** :
   l'erreur d'interpolation de la phase vaut `Δ²/8·dk_y/dy` ; 2 m tiennent (≈ 0,4 mm), 5 m à la limite (≈ 2 mm).
2. **Continuité** : au large du profil, l'évaluation côtière est **identique au bit** à celle de B.
3. **Déterminisme** : phases en entiers, deux passes identiques au bit (hash).
4. **Coût** par composante et par échantillon contre B (48 ns en B1) ; indépendant de la taille des tables.
5. **Mémoire** par composante, par kilomètre de profil — et ce qu'en coûterait une bathymétrie 2D.

### Plan

- [x] **P1** — jeton, plan seul.
- [x] **P2** — les tables d'une côte à isobathes droites (`bathymetrie_cote.rs`) : cuisson depuis la référence,
  interpolation ; critère 1, balayage du pas.
- [x] **P3** — B sur la côte : l'évaluation côtière (phase entière, amplitude, pente, vitesses en profondeur finie) ;
  critères 2 et 3 ; η, pente et vitesse contre la référence.
- [>] **P4** — coût et mémoire ; critères 4 et 5.
- [ ] **P5** — ADR-196, l'entrée de la bathymétrie ; preuve (BATHYMETRIE-S362, section datée S364) ; liste 2.7, file,
  dépendances, feuille de route, index.
- [ ] **P6** — rituel.

### Notes de reprise
- **P2, critère 1 tenu.** `bathymetrie_cote.rs` : `Cote::cuire` (référence de S362 en f64, Simpson à huit
  sous-intervalles par pas), tables de 16 octets par composante et par échantillon ; `interpoler` : phase entière (différence
  des deux échantillons × fraction Q16), le reste linéaire en f32. Houle d'1 m, 10 s, 30°, plage 1/50 de 80 à 2 m, sondes
  au quart et au milieu des pas : **pire hauteur 0,054 / 0,226 / 1,39 / 5,35 mm** aux pas de 1 / 2 / 5 / 10 m — en `Δ²`,
  sous la prédiction (0,4 et 2 mm) ; facteur à 1,5e-5. **Trouvé : le bord du large.** À λ₀/2 (78 m), le « fond qui cesse
  de se sentir » des manuels et de SPEC-005 §8, le facteur vaut encore **0,990 : une marche de 5 mm** entre B et la côte.
  Prédiction écrite avant la mesure, tenue : à λ₀ (156 m), `K − 1` = **−4,15e-5** (prédit −4,4e-5), marche 0,02 mm. Le
  profil doit commencer à λ₀ de la plus longue composante, pas à λ₀/2. Déferlement de cette houle : 1,69 m.
- **P3, critères 2 et 3 tenus.** `Cote::eval` : phase de B + correction entière, amplitude `a·K`, vecteur d'onde local
  `k_x·t + k_y·n` (pente), vitesse horizontale `a·ω·coth(kh)`, verticale `a·ω`. Huit composantes de 6 à 10 s, ±30°,
  Hs ≈ 1 m, plage 1/30 de 160 m (λ₀ de la plus longue) à 2 m, table au pas de 2 m (313 Ko). **Au large : 120
  évaluations identiques au bit à B.** Deux passes : même hash. Sur la plage, 711 points, trois instants : **η à
  0,133 mm** de la référence f64, pente à 1,4e-5 (max 0,086), vitesse horizontale à 2,7e-4 m/s (max 1,31) ; **marche au
  bord du large 0,0017 mm**. Essai rendu incrémental (50 s → 0,2 s en mode optimisé).
