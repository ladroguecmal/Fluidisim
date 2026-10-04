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

Session : S479 — **en cours**. En autonomie, K2-1 ([conception](../docs/registres/CAMPAGNE-K2-S478.md), ADR-220 D1) : **l'air
enfermé dans la référence APIC** (`code/water-core/src/apic3d.rs`).

**Ce que la session fait.** `Apic3::enable_air_pockets` (mémoire réservée à la configuration, I-06) : à chaque pas, après les
étiquettes, les **composantes d'air enfermé** — les mailles d'air que l'air libre (la rangée du haut) n'atteint pas, par remplissage —,
suivies d'un pas à l'autre par recouvrement (fusion : leurs airs s'ajoutent ; scission : l'air se partage au volume) ; leur volume
sur la surface reconstruite (fraction d'air `0,5 + φ/dx`) ; à la naissance, la pression de l'eau qui les borde. **L'invariant de
chaque poche** : `a = V·P^(1/γ)`, γ = 1,4 (adiabatique). **Dans la projection, chaque poche est une inconnue** : la loi linéarisée
`P^(n+1) − Pⁿ = −(γPⁿ/Vⁿ)·ΔV`, `ΔV` le flux de ses faces après correction, donne une ligne `(s + Σ1/θ)·p_b − Σ p_c/θ = s·p_bⁿ −
(ρ·dx/dt)·Σ u*_sortant`, `s = ρ·Vⁿ/(γPⁿ·dt²·dx)`, symétrique avec les lignes de l'eau voisine : le système reste défini positif, le
même gradient conjugué le résout — implicite, donc stable quel que soit le pas devant la raideur de la poche. Sans
`enable_air_pockets`, le pas au bit.

**Critères, écrits avant.** (1) sans poches, au bit (les tests d'APIC 3D, dont S393 et S389) ; (2) une bulle immobile en eau calme
garde son volume à 1 % sur une seconde et remonte (elle ne s'effondre plus) ; masse d'eau exacte ; (3) **Minnaert** : une bulle
lâchée en surpression oscille à `f = (1/2πR)·√(3γP/ρ)` à 15 % ; (4) **A311** : B10 3D (`apic3d_b10`) va au bout à `D/dx` = 16 avec
les poches, et la bulle pincée vit (son volume après le pincement, tracé) ; `D/dx` = 24 si le temps le permet (dit sinon).

### Plan

- [x] **P1** — jeton, plan seul.
- [x] **P2** — les poches dans `apic3d.rs` ; les essais (1) à (3).
- [ ] **P3** — B10 avec poches (4) ; mesures.
- [ ] **P4** — preuve ; rituel (allégé).

### Notes de reprise
- **P2** — `code/water-core/src/apic3d_poches.rs` : `enable_air_pockets`, la détection (remplissage), le suivi (recouvrement : fusion,
  scission), la naissance à la pression de l'eau voisine, l'invariant `V·P^(1/γ)`, la poche comme inconnue de la projection
  (`project_with_pockets`, une seconde projection : le chemin sans poches au bit, par construction). **Trouvé en chemin** : le volume
  géométrique seul saute de 1 % quand des mailles changent d'étiquette (trois mailles d'un coup) — chaque saut frappe la poche ;
  la première bulle oscillait à 56 Hz (× 1,34). **Remède** : le volume suivi par le flux de la projection, rappelé vers la
  géométrie en 0,1 s. **Mesuré** : (1) les 43 tests d'APIC 3D passent (le nouveau `air_pocket_holds_a_bubble_s479` compris) ;
  (2) la bulle (R = 0,08 m, R/dx = 4, à 0,3 m de fond) remonte (son centre de 0,258 à 0,300 m en 0,15 s), son volume moyen
  comprimé de 1,6 % (l'adiabatique sous la charge en attend 2,0 %), oscillation de ± 1,3 %, masse exacte ; (3) **42,5 Hz contre
  41,6 de Minnaert (× 1,021)** — la cuve, par la méthode des images, attendrait 38,4 (× 1,107) : dans les 15 % des deux.

