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

Session : S483 — **terminée**. Sur « Continue », la suite du jeton : **ADR-222 D2 et D3** — la référence CPU parallèle, le banc de
non-régression du rituel. Puis, si la session le permet, la relance de B10 à 24 mailles.

**Ce que la session fait.** (1) **Mesurer d'abord** : un relevé du temps par étage dans `Apic3::step_upto` (la bulle de S479, B10),
pour paralléliser là où le temps est. (2) **La référence parallèle** : `Apic3` reçoit un système de tâches de l'hôte
(`set_jobs`, `ScopedJobs` aux bancs) ; les boucles chaudes deviennent des **écritures disjointes** (`parallel_fill_f32`, S243 : le
résultat ne dépend ni du grain ni du nombre de fils) ; les sommes gardent leur ordre séquentiel. (3) **Le banc de non-régression** :
`outils/non_regression.py` — la bulle (référence, quelques pas) et `--v1` court sur la carte : masse, particules, une empreinte des
champs au bit contre `docs/validation/EMPREINTES.md` (versionné) ; le coût contre un seuil ; appelé par `rituel.py fin`.

**Entrées, et comment elles se vérifient.** La séquentielle est la référence : chaque boucle parallélisée se compare **au bit** à
elle (les 43 essais d'APIC 3D, et une empreinte de la bulle après N pas, `ScopedJobs::with_workers(1)` contre 16).

**Critères, écrits avant.** (1) au bit : la bulle après 20 pas, mêmes bits avec 1 et 16 fils ; les essais d'APIC 3D passent ; (2) la
bulle de S479 (0,15 s) **au moins 4 fois plus vite** qu'en séquentiel ; (3) le banc de non-régression tourne en moins de 3 min, échoue
sur une empreinte modifiée, passe sur l'état présent ; `rituel.py fin` l'appelle. Ce qui ne tient pas est dit.

### Plan

- [x] **P1** — jeton, plan seul.
- [x] **P2** — le temps par étage de la référence ; où il est.
- [x] **P3** — la référence parallèle, au bit ; (1), (2).
- [x] **P4** — le banc de non-régression ; (3).
- [x] **P5** — preuve ; rituel.

### Notes de reprise
- **P2** — `Apic3::step_marked` (un repère nommé par étage ; le cœur ne lit aucune horloge, l'exemple oui ; `step_upto` l'appelle avec
  un repère vide). La bulle (`PROFIL=1`, 20 pas) : reconstruction **41 %**, séparation + corps + échange 19 %, projection 15 %, p2g 10 %,
  advection 10 %, g2p 5 %.
- **P3a** — `set_jobs` ; la reconstruction en `parallel_fill_f32` (`reconstruct_cell`) : **14,1 → 2,8 s**, empreinte identique avec 0, 1
  et 16 fils (`b67cab1db66f94f5`) ; les essais d'APIC 3D passent.
- **P3b** — en écritures disjointes (`parallel_fill_f32`, `as_flattened_mut` pour les triplets, sans `unsafe`) : l'advection, le transfert
  vers les particules, le produit `A·d` (avec et sans poches) ; **en collecte**, dans l'ordre de la carte : la séparation (chaque particule
  somme ses voisines, `separate_shift`) et le transfert vers la grille (chaque face somme les particules des mailles qui la touchent,
  `p2g` ; le tri se fait là et la reconstruction le reprend, `bin_fresh`). Empreinte de la bulle **identique** avec 0, 1, 4, 8, 12, 16 fils ;
  43 essais d'APIC 3D ; le banc carte | référence inchangé. **Vitesse** (20 pas de la bulle) : 34,6 s → **11,9 s** sur 16 fils (×2,9) ;
  B10 à 16 mailles, 10 pas : 81 → 25 s (×3,2). **Critère (2) manqué** (×4) : la projection reste séquentielle pour l'essentiel (les
  produits scalaires ordonnés, les mises à jour ; `ScopedJobs` recrée ses fils à chaque appel, 150 fois par pas). Le séquentiel ralentit
  (34,6 → 47,7 s) : les collectes font plus de travail que les dispersions — le prix d'un résultat indépendant du nombre de fils.
- **P4** — `outils/non_regression.py` (la bulle au bit et 1 = 16 fils ; la carte contre la référence sur les poches ; `--v1` 5 s : masse,
  trajectoire, pas médian sous 1,3 fois l'inscrit) ; empreintes inscrites dans `docs/validation/EMPREINTES.md` ; 124 s ; passe sur l'état
  présent, échoue sur une empreinte modifiée (`0000…` : « dc06f28c8a909e04 au lieu de 0000000000000000 ») ; `rituel.py fin` le lance
  (`--sans-banc "raison"` pour le sauter). Critère (3) tenu. Une vérification lancée ce matin est restée suspendue 12 h — la machine en
  veille, vraisemblablement ; elle a fini d'elle-même à la reprise, avec le bon résultat.
- **P5** — preuve REFERENCE-PARALLELE-S483 ; index ; journal.
