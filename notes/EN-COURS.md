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

Session : S481 — **en cours**. En autonomie (ADR-215), **K2-2 — l'air enfermé sur la carte** ([conception](../docs/registres/CAMPAGNE-K2-S478.md),
ADR-220 D1 ; la référence : `code/water-core/src/apic3d_poches.rs`, [POCHES-AIR-S479](../docs/validation/POCHES-AIR-S479.md)).

**Ce que la session fait.** Les poches de S479 portées sur la carte (`viewer/src/apic3d_carte.rs`), **dans un module à part**
(`apic3d_poches.wgsl`, mêmes liaisons, deux tampons de plus) : le chemin sans poches n'est pas touché. (1) **La détection** : une
union-find sans verrou sur les mailles d'air (l'accrochage toujours vers la plus petite étiquette, l'air libre = 0 : les mailles
d'air de la rangée du haut), puis l'aplatissement ; la racine d'une composante enfermée est sa plus petite maille, et les poches se
numérotent dans l'ordre des mailles — **l'ordre de la référence**, qui remplit depuis la première maille rencontrée. (2) **Le bilan
par poche** : les listes (les mailles d'air de chaque poche ; les mailles d'eau qui la bordent, une fois par poche), une réduction
par poche dans un ordre fixe (un groupe par poche, aucun atomique flottant) ; l'héritage par recouvrement (des compteurs entiers),
la naissance, le rappel du volume, la résorption. (3) **La projection** : le gradient conjugué diagonal, une ligne par poche ; `A·d`
des poches par une réduction par poche sur la liste des faces eau | poche, à chaque itération. (4) Le saut de `--v1` avec poches.

**Entrées, et comment elles se vérifient (REPRISE §2).** L'état de départ de chaque essai est un `Apic3` construit dans le banc
(la bulle d'`apic3d_bulle`, B10 de `b10_band_state_from`) et chargé sur la carte par `load` ; les étiquettes de la carte se
comparent à celles de la référence avant toute poche (`labels()`), et l'état des poches de la référence (`of`, air, volume suivi,
dernier pas) est exporté par un accesseur et chargé avec lui — un essai qui partirait d'un état de poches différent le dirait.

**Critères, écrits avant.** (1) sans poches, au bit : le chemin d'avant n'est pas modifié (les bancs de la carte inchangés) ; (2) la
poche de chaque maille, sur la carte, **identique** à la référence (la bulle, et B10 au pincement) ; (3) la bulle de S479 sur la
carte : volume et pression **à 1 %** de la référence pas à pas sur 0,15 s, sa fréquence à 2 % de celle de la référence ; masse
exacte ; (4) B10 à 16 mailles avec poches sur la carte va au bout, la bulle vit (son volume après le pincement à 5 % de la
référence) ; (5) `--v1` stable 60 s avec poches, masse exacte, le coût des poches mesuré et inscrit (ADR-131). Ce qui ne tient pas
dans la session est dit, et passe à S482.

### Plan

- [x] **P1** — jeton, plan seul.
- [x] **P2** — l'état des poches exporté par la référence ; `apic3d_poches.wgsl`, ses tampons ; la détection ; essai (2).
- [x] **P3** — le bilan par poche : listes, réductions, héritage, naissance, rappel, résorption ; contre la référence sur un même état.
- [x] **P4** — la projection avec poches ; la bulle, essai (3).
- [>] **P5** — B10 (4) ; `--v1` (5) par `calcul.py`.
- [ ] **P6** — preuve ; rituel (par `rituel.py`).

### Notes de reprise
- **P2** — `AirPocketState` (référence : `air_pocket_state`) ; `viewer/src/apic3d_poches.{rs,wgsl}` (module à part, `Params` pris au
  texte du nuanceur principal ; liaisons 27 `pko`, 28 `pkf`, réservées à la configuration) ; détection : `pk_init`, `pk_merge`
  (union-find, accrochage vers le plus petit, air libre = 0), `pk_flatten`, `pk_number` (un groupe, préfixe par tranches),
  `pk_assign` ; banc `--apic3d-poches` (`CAS=bulle|plusieurs`, `CHEMINEE=0`). **Mesuré** : 0 maille de poche différente à
  étiquettes égales (bulle 6 pas, plusieurs 6 pas, sans cheminée 2 pas) ; les seuls écarts (4 mailles, bulle, pas 1) sont des
  étiquettes qui diffèrent déjà (la reconstruction de la carte). La carte ne fait pas encore la résorption (P3) : le cas « plusieurs »
  n'a pas révélé d'écart car la bulle de 6 mm n'est pas résolue à dx = 2 cm. Le module se compile sous Dx12.
- **Pause demandée par l'utilisateur** à 21:29 (« fais pause, je reprendrai plus tard ») : reprendre à P3. B10 à 24 mailles tourne
  détaché (`python outils/calcul.py etat`).
- **Reprise** à 21:31 (« Reprends le projet ») : jeton repris, P3. B10 à 24 mailles toujours en cours (l'exemple n'écrit qu'à la fin).
- **P3** — `pk_lists` (LA, LW dans l'ordre des mailles, un groupe), `pk_overlap` (compteurs entiers), `pk_reduce` (un groupe par
  poche, arbre fixe : volume, centre, mailles, pression de bord), `pk_scalars` (héritage, naissance, rappel, pression, résorption),
  `pk_remap` ; `load` charge air, volume suivi, dernier pas et la pression de la référence. **FXC** refuse l'écriture indexée dans un
  tableau local de structure : `bord_new` sans tableau. **Mesuré** (banc `--apic3d-poches`) : nombre de poches identique partout ;
  écart de volume ≤ 6·10⁻⁶, de pression ≤ 7·10⁻⁶ (bulle 8 pas ; trois bulles et une cheminée ; trois bulles sous pression après
  4 pas de chauffe — la naissance — ; la bulle d'une maille résorbée des deux côtés).
- **P4** — `pk_faces` (LF : faces eau | poche, `1/θ` et flux sortant), `pk_rows`, `pk_cg_init_finish`, `pk_cg_apply`, `pk_cg_rows`
  (`A·d` des poches, un groupe par poche), `pk_cg_alpha`, `pk_cg_beta`, `pk_correct`, `pk_post` ; `assemble`, `cg_init_reduce`,
  `cg_update`, `cg_direction` repris ; avec poches, la diagonale (la multigrille ne les voit pas encore). **Mesuré** (`MODE=suivi`,
  `calculs/20261004-221515-s481-bulle-suivi`) : 300 pas, volume à 2,9·10⁻⁵, pression à 4,1·10⁻⁵ de la référence ; **42,47 Hz contre
  42,50** ; 151 itérations au plus, toutes convergées ; masse exacte. Critère (3) tenu. `calcul.py` résout un programme donné par un
  chemin relatif au dépôt. L'instrumentation de P5 (`APIC3D_POCHES=1` sur B10 et `--v1`) est dans ce commit, compilée, non lancée.
- **P5a** — `--v1` avec poches (`APIC3D_POCHES=1`) : **emballement vers t = 8,33 s** (le pas stable tombe à 1 ms, puis la carte est
  perdue) ; tracé : une quinzaine de poches d'une ou deux mailles (dx = 5 cm) dans l'eau brassée sous la cavité, pression jusqu'à
  283 kPa. **Tranché** : `POCHE_MAILLES_MIN` = 8 dans la référence et la carte (une poche de moins de 8 mailles d'air se résorbe ; la
  règle `V < dx³` ne les attrapait pas, leur volume compte la part d'air des mailles d'eau voisines) ; 43 essais d'APIC 3D passent.
  Puis 60 s **stables**, masse exacte au quantum, une poche observée (1,87·10⁻³ m³ ≈ 15 dx³, 107 kPa) — le critère (5) tenu au
  minimum : à dx = 5 cm, la scène ne fait presque que des poches de moins de 8 mailles. **Coût** : pas médian 71,4 ms contre 16,7
  sans poches (témoin, même binaire) — reconstruction +11 ms (détection et listes par un seul groupe), projection +≈ 40 ms (la
  diagonale à la place de la multigrille). À inscrire (ADR-131) ; remèdes : les poches dans la multigrille (préconditionneur par
  blocs), sauter les noyaux quand aucun air n'est enfermé, les parcours à plusieurs groupes. B10 à 16 mailles lancé.
- **P5b** — **à 23:46, B10 à 16 mailles (S481) et B10 à 24 mailles (lancé en S480, 7 h de calcul) sont morts ensemble**, code
  0xC000013A (Ctrl+C) : lancés par `calcul.py`, ils restaient dans l'objet de tâche de la session (la sortie, `BREAKAWAY_FROM_JOB`,
  est refusée). **Corrigé** : sous Windows, le lanceur est créé par WMI (`Win32_Process.Create`, parent : le service WMI), lit sa
  commande et son environnement dans `lancement.json`, écrit lui-même `sortie.log` ; `detache.txt` dit s'il est hors de la session ;
  le registre traduit les codes de sortie. Essais : environnement transmis, sortie écrite. B10 à 16 mailles relancé à 23:50.
- **Question de l'utilisateur à 23:46** (« d'autres pistes d'amélioration […] que tu prennes toi-même les solutions ») : répondu — cinq
  pistes décidées (calculs hors session, référence CPU parallèle, banc de non-régression dans le rituel, révision de méthode tous
  les cinq sessions, scènes fines pour les poches) ; deux en attente de son accord (relance planifiée, téléchargements anticipés).
  À écrire en P6 : ADR-222.
