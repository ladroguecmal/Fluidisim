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

Session : S235 — en cours
Agent : Claude Code, Opus 5 (fichiers, git, cargo, Python et GPU local disponibles)
Entrée : « Continue », master propre à f0159ca, trois copies au même commit, jeton libre,
alimentation secteur (BatteryStatus 2).

**Objectif.** Scène J1 **représentative à plusieurs sources** dans l'hôte GPU : plusieurs
sillages (journal commun, montage S222) et plusieurs impacts à naissances décalées, composés et
admis par le cœur ; **visibilité** de la grille et des impacts, **exactitude au retour dans le
champ** ; coût GPU et CPU publiés (ADR-131 D3, alimentation selon A270).
**Faits de départ.** Plusieurs sillages d'un même journal et d'une même recette publient
4 096 modes au total (S222) : la cuisson GPU ne devrait pas croître avec eux, la préparation CPU
si. Le shader ne dessine qu'un impact. Un impact neuf vaut 47,4 % de π/7 (S222) : des naissances
simultanées feraient refuser le budget conjoint.
**Critères, déclarés avant construction.** Hauteur GPU contre cœur ≤ 3 mm (tolérance S201) sur la
scène multi-sources, intérieurs de grille sous leur borne ; composition du cœur sans refus aux
instants vérifiés, refus d'une variante dense publiés et non masqués ; retour dans le champ :
coefficients publiés et grille **identiques au bit** à un passage continu, sinon écart publié,
expliqué et confronté aux 3 mm ; coût avec et sans visibilité, pose visible et hors champ.
**Arrêt.** Scène vérifiée et mesurée. Si le budget ne tient pas sur la combinaison, le dire selon
ADR-131 D1 avec les techniques absentes, sans réduire la scène pour passer.

### Plan

- [x] **P1** — amorce, jeton, plan seuls.
- [x] **P2** — lectures ciblées (profil radial, `mixed_compose`, `Timeline` multi-sources,
  S222/S223) ; déclarer la scène (positions, naissances, trajectoires) et prédire son admission
  par le cœur avant de construire.
- [x] **P3** — impacts multiples : profils et centres en tableau au GPU, références CPU et
  composition du cœur à N impacts ; `--verify` vert.
- [ ] **P4** — sillages multiples dans un journal : grille, bornes, `verify_lattice` sur la scène.
  **Amendement P2 (23:27)** : aux instants refusés par le budget, séparer pente **réelle** et
  **majorants** (A208) — maximum réel des perturbations sur l'union des domaines, contre le
  plancher ; c'est le déclencheur écrit d'A255/A261.
- [ ] **P5** — visibilité : emprise et disques contre le cône de vue, cuisson et préparation CPU
  sautées hors champ ; retour dans le champ comparé à un passage continu.
- [ ] **P6** — coût : banc et cadence, scène mono et multi, visible et hors champ, alimentation
  publiée ; document de validation.
- [ ] **P7** — rituel §6, file, feuille de route, jeton ; copies à synchroniser.

### Notes de reprise

Base : S234 — grille du sillage 0,426 ms (960×540, secteur), cadence 384 Hz, 9 tests de l'hôte ;
`code/` 413 réussis / 5 ignorés (S233, non modifié depuis).

P2 : scène déclarée dans `scene.rs` (`WAKE_OFFSETS` 4/−26/34 m, `IMPACTS` huit positions,
naissances 0 à 28 s par 4 s ; variante dense à 1 s). `wake(recipe)` = `wake_at(recipe, 0)`,
identité et trajectoire S212 inchangées. `--scene-admission` : `mixed::slope_floor` sur trois
sillages (journal commun) et impacts, tous les 0,25 s sur 40 s.
**Premier essai faux, et instructif** : journal chargé des huit impacts dès 0 s → plancher
1,133 (2,52 π/7) à 0 s, car `slope_max_at` rend le maximum de naissance **avant** la naissance
(ADR-133, choix conservateur documenté). Un hôte inscrit un impact à sa naissance : journal
reconstruit à chaque instant avec les seuls impacts nés.
**Résultat** : scène **refusée sur 49 instants / 161**, premier refus 8 s (3e naissance),
pire 1,251 π/7 à 28,25 s. À chaque naissance : pression 0,175–0,202 (39–45 %) + impact neuf
0,213 (47 %) + anciens. Entre naissances, retour sous π/7 (0,78–1,07). Au-delà de 32 s : 0,73–
0,86. Variante dense : 40 refus, premier à 3 s, pire 1,420 à 7 s. S222 (« huit impacts
passent ») valait pour des impacts âgés, pas pour des naissances renouvelées.
Décision : scène conservée (critère « sans réduire la scène pour passer ») ; rendu cosmétique
qui annonce ; classification réelle/majorant en P4.

P3 : `ImpactSlot` (champ du cœur, naissance, actif) ; une table radiale, profils concaténés à
l'âge de chaque impact ; tampon GPU `impacts` (groupe 0, liaison 4 : centre relatif, actif) ;
`p.impact.x` = longueur de profil, `p.info.w` = nombre d'impacts. `--multi` : trois sillages
dans le journal commun + huit impacts ; `verify_multi` (23 âges déclarés, deux chemins,
`verify_lattice` à 6 âges, capture `captures/s235/scene.ppm`, bancs 3 s et 29 s).
**Mono identique au bit à S234** : toutes les lignes `VERIFY` et `LOD_INTERIEUR` (log
`verify_s235.log`, secteur). **Multi** : grille max η **0,368 mm** (12 s, 4 impacts), direct
≤0,091 mm ; tous âges sous 3 mm. Intérieurs : grille − direct 0,212–0,392 mm, bornes 2,45–
2,98 mm (rapports 0,07–0,16) ; pas 1,125–1,1875 m, **10 810 nœuds au plus** (capacité 16 384) ;
sauts ≤7 µm. Ce passage couvre aussi la partie « vérification » de P4 (sillages multiples).
Bancs multi (secteur, non encore alternés avec un témoin) : 960×540 grille **0,441 ms à 3 s,
0,459 à 29 s** (mono 0,427) ; direct 4,40 / 4,28 (mono 4,32 / 4,29). CPU sillage **3,12 ms à
3 s** (forçage de trois sillages ; mono 1,3), 0,45 ms à 29 s ; un pic isolé 53 ms au premier
banc 640×360 (mise en régime probable, non attribué).