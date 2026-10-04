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
- [ ] **P2** — l'état des poches exporté par la référence ; `apic3d_poches.wgsl`, ses tampons ; la détection ; essai (2).
- [ ] **P3** — le bilan par poche : listes, réductions, héritage, naissance, rappel, résorption ; contre la référence sur un même état.
- [ ] **P4** — la projection avec poches ; la bulle, essai (3).
- [ ] **P5** — B10 (4) ; `--v1` (5) par `calcul.py`.
- [ ] **P6** — preuve ; rituel (par `rituel.py`).

### Notes de reprise
