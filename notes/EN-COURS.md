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

Session : S390 — **en cours**. **C3**, première part : **la multigrille sur la carte** ([conception](../docs/registres/CAMPAGNE-SOLVEUR-3D-S384.md)
§5, [ADR-207](../docs/adr/ADR-207-la-campagne-du-solveur-volumique-3d.md)). Demande de l'utilisateur (2026-09-26) : *« Reprends le
projet »*, puis, entre la pluie (pièce 5), C3 et les gerbes, **C3**. Agent : Claude Opus 5.5, Claude Code (application de
bureau) au poste — fichiers, git, cargo, Python, RTX 5070 Laptop, Godot 4.6.3.

**Maillons à 3** : ce fil (la campagne) n'a pas encore changé l'état d'un point ; C3 vise le coût de δ (4.19) et la maille fine
de la haute mer. Choisi par l'utilisateur ; la colonne graduée sur la carte n'en fait pas partie (S387 : elle sert l'eau calme,
pas la haute mer) — reportée à l'usage qui la consomme, les contenants.

**Thèse.** La production (afficheur, `Step3`) résout la pression par **32 cycles** de gradient conjugué préconditionné par
Jacobi : 2,06 ms des 3,68 du pas, 0,062 ms par cycle (S341), et des pas **déclarés dégradés** (S302). La référence a reçu en S385
un cycle en V qui fait 9 à 11 itérations quelle que soit la maille. **Le porter tel quel sur la carte** — Jacobi amorti ω = 6/7,
deux lissages avant, deux après, huit au plus grossier, restriction par moyenne, prolongation par injection, niveaux grossiers
rediscrétisés depuis la surface à chaque projection — comme préconditionneur du même gradient conjugué, à travail fixe.
**Prédiction** (*estimée*, avant mesure) : sur la scène de la porte B (120 × 112 × 28, deux niveaux grossiers), un cycle
multigrille coûte 4 à 6 cycles de Jacobi (six passes fines contre une, plus les niveaux grossiers, où domine le coût
d'appel) ; il atteint le résidu de 32 cycles de Jacobi en 3 à 6 cycles. Le gain à 25 cm serait donc modeste (projection
≈ 1,2 à 1,6 ms) ; il grandit avec la maille, là où Jacobi rampe — la maille de 10 cm est C3b.

**Critères, écrits avant.**
1. **Éteinte par défaut** : surface publiée identique au bit (empreintes de `--delta3d-empreinte`), dispatchs inchangés.
2. **L'instrument** : sur la géométrie de la scène après chauffe, le cycle en V de la carte contre une réplique CPU en `f64`,
   même résidu d'entrée — écart ≤ 10⁻⁵ du maximum ; symétrie `⟨u, M⁻¹v⟩` contre `⟨M⁻¹u, v⟩` ≤ 10⁻⁵ relatif, positivité ;
   l'essai **vu échouer** sur un cycle rendu asymétrique (un lissage après au lieu de deux).
3. **La convergence** sur la scène de la porte B : vrai résidu relatif et divergence franche contre les cycles, Jacobi 8 à 64,
   multigrille 1 à 8 ; **la multigrille atteint le résidu de 32 cycles de Jacobi en 8 cycles au plus**.
4. **Le coût** : cycle multigrille publié ; à résidu égal, pas entier ≤ celui de Jacobi à 32 cycles ; la porte C tient (deux
   parts ≤ 2 ms au 99ᵉ centile).
5. **La référence** : production multigrille à **3 mm** de la référence sur les trois cas de cuve ; divergence publiée.
6. Suite inchangée, zéro avertissement. **Non visé ici** : 10 cm (C3b), A298 remesurée (C3b, sur le pas retenu), activation
   par défaut dans la scène vivante (décidée en P6 sur les chiffres, empreintes nouvelles expliquées si oui).

### Plan

- [x] **P1** — jeton, plan seul.
- [ ] **P2** — les noyaux WGSL : lissages fins et grossiers, résidu restreint, prolongation, géométrie des niveaux ; les
  variantes du gradient conjugué (mise à jour sans `z`, produit `r·z`, première direction).
- [ ] **P3** — le branchement dans `Step3` : tampons réservés à la configuration (I-06), multigrille éteinte par défaut, nombre de
  dispatchs du profil ; critère 1.
- [ ] **P4** — l'instrument : réplique `f64`, symétrie, positivité, vu échouer ; critère 2.
- [ ] **P5** — convergence et coût sur la scène de la porte B, deux parts à 30 Hz ; critères 3 et 4.
- [ ] **P6** — les trois cas de cuve avec la multigrille ; critère 5 ; activation par défaut décidée.
- [ ] **P7** — critère 6 ; preuve (section datée de MULTIGRILLE-3D-S385) ; file, feuille de route, liste.
- [ ] **P8** — rituel.

### Notes de reprise
