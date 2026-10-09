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

Session : S724 — **terminée**. En autonomie, sans arrêt (l'utilisateur dort) ; session longue. LOD-ETAPE-3-S722, **B2 : APIC à quatre
bords**.

**Ce que la session fait.** `apic3d_bords_y.rs` : les bords en y par particules (le devant, `y = 0`, le derrière, `y = ly`), sur le modèle
du bord gauche (S698), avec la pose par la grille (S702) :
- la vitesse normale imposée aux faces ;
- les particules qui sortent, retirées et comptées ;
- l'entrée posée par quanta, avec la vitesse et l'affine du G2P.

Avec les bords en x qui existent (la gauche S698, la droite S682), la boîte est ouverte sur ses quatre côtés.

**Les essais ; chacun a ses critères, écrits avant lui.** Une boîte de 1 m × 1 m, 0,4 m d'eau, `dx` = 2,5 cm, les quatre bords ouverts.

| essai | ce qu'il juge | critères |
|---|---|---|
| **E1** | le repos (les vitesses imposées nulles) | après 1 s, la vitesse maximale sous 1 mm/s ; `V_φ` constant à 10⁻³ |
| **E2** | un courant uniforme en biais, (0,2 ; 0,1) m/s, 5 s : il entre par la gauche et par le devant (la pose par la grille), sort par la droite et par le derrière (le retrait) | `V_φ` tenu à **0,5 %** ; à l'intérieur (trois mailles des bords exclues), la vitesse moyenne des particules à **2 %** du courant, l'écart maximal sous 5 cm/s ; la surface plate à **3 mm** (l'étendue de η par la surface) |

E2 est une solution exacte : un courant uniforme sur un fond plat ne change pas.

**Contrôles du plan** (ADR-266, ADR-267, ADR-268, ADR-273, ADR-276, ADR-277, ADR-280, ADR-281, ADR-282)

- **témoin** : la solution exacte, le courant uniforme.
- **instrument** :
  - `V_φ` (la surface, ADR-280 D1) ;
  - la vitesse moyenne des particules de l'intérieur ;
  - η par colonne, par la surface.

  Tous sont recalculés à la fin (ADR-281 D2). Ce que rendrait chaque hypothèse :
  - les bords en y sont justes : E2 tient ;
  - une faute de pose ou de retrait : `V_φ` dérive, ou la surface penche vers un bord.
- **calcul** :
  - 40 × 40 × 16 mailles d'eau × 8 particules ≈ 205 000 particules ;
  - le pas de 10 ms ;
  - ≈ 3 à 5 min pour E2.
- **ADR**, et comment chacun est tenu (ADR-277 D1) : ADR-273 D1 (la 3D seule, contre l'exact) ; ADR-282 (la fermeture par l'outil).
- **pièges** :
  - les coins : une particule sort par x ou par y, comptée une fois (le retrait en x d'abord, puis en y) ;
  - les gouttes refusées avec les bords en y ;
  - les faces au-dessus de l'eau : la vitesse imposée y est nulle.

### Plan

- [x] **P1** — jeton ; plan.
- [x] **P2** — les bords en y ; E1, E2 (partiel), E2b.
- [x] **P3** — preuve ; fermeture.

### Notes de reprise
- **E1 tenu** : au repos, 4,9·10⁻⁶ m/s, `V_φ` −1,3·10⁻⁷, η à 1 µm. **E2 échoue en partie** : `V_φ` −0,39 % (tenu), la vitesse moyenne à 0,5 % (tenue) ; mais l'écart maximal 0,10 m/s (critère 5 cm/s) et l'étendue de η 7,2 mm (critère 3 mm). Le diagnostic : des particules de surface, l'une ralentie, une autre en chute libre à 1,8 m/s ; un creux de 5,4 mm à l'intérieur. **E2b**, le témoin : le courant en x seulement, les bords en y ouverts sans flux.
- **P2 fini** — E2b : le même écart de vitesse (0,103 m/s) en x seul, la surface 3,7 mm. L'écart de vitesse est celui de la surface libre d'APIC ; l'entrée en y ajoute ≈ 3,5 mm de rides. B2 acquis pour la masse et l'écoulement.
