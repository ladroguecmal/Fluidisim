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

Session : S717 — **en cours**. En autonomie, sans arrêt (l'utilisateur dort) ; session longue. LOD-ETAPE-2-S705, **M1 : la mort de la 3D
vers Saint-Venant**, par la surface (S715, ADR-280 D1).

**Ce que la session fait.** `mort_vers_sv` : la 3D rend à Saint-Venant, par colonne et par rangée :
- la hauteur, lue par sa surface reconstruite (φ) au-dessus de l'escalier, puis rapportée au fond continu de Saint-Venant (le niveau de
  la surface est gardé) ;
- la quantité de mouvement `h·ū`, `ū` la moyenne des vitesses des particules de la colonne.

**Les essais, dans l'ordre ; chacun a ses critères, écrits avant lui.**

| essai | ce qu'il juge | critères |
|---|---|---|
| **E1** | l'eau au repos (fond plat, 4 m, `h` = 0,49 m) : la 3D au repos meurt, Saint-Venant reprend 1 s | le volume rendu à 10⁻³ du volume de la surface ; après 1 s, la vitesse de Saint-Venant sous 1 mm/s et `|η|` sous 2 mm |
| **E2** | la vague de S690 après le déferlement : à **t = 3,2 s**, toute la 3D meurt ; Saint-Venant reprend la plage entière (la 3D et le rivage réunis) jusqu'à 5 s | contre le tout-3D qui continue jusqu'à 5 s (`Large::AucunJusqua5`, le même montage, ADR-276 D1) : la remontée maximale à **1,25 cm** près (0,15 m sur la pente de 1:12, ADR-278 D2), son instant à **0,1 s** ; le volume rendu à **0,5 %** du volume de la surface et du rivage |

**Contrôles du plan** (ADR-266, ADR-267, ADR-268, ADR-273, ADR-276, ADR-277, ADR-278, ADR-280, ADR-281)

- **témoin** : E1, le repos lui-même ; E2, le tout-3D jusqu'à 5 s, par la même fonction (`deux_raccords_porteur`). Seule la mort change
  (`Large::AucunMort`, un mode nommé, ADR-277 D2).
- **instrument** : la remontée sous la maille de S688 (le niveau de la plus haute maille mouillée), sur le Saint-Venant du rivage pour le
  témoin, sur le Saint-Venant réuni après la mort. Elle est recalculée à chaque pas, donc elle peut échouer (ADR-281 D2). Ce que rendrait
  chaque hypothèse :
  - la mort rend à Saint-Venant un état juste : la remontée à 1,25 cm ;
  - elle perd du volume ou de la quantité de mouvement : une remontée plus basse, ou plus tardive.
- **calcul** :
  - le volume de la surface est étalonné en S708 (3·10⁻⁴) ;
  - la différence entre l'escalier et le fond continu se moyenne sur les marches (le niveau est gardé ; le volume en dépend de
    ≈ dx/2 par marche) : c'est la tolérance de 0,5 % ;
  - le coût : ≈ 20 min pour le témoin, ≈ 12 min pour la mort.
- **ADR**, et comment chacun est tenu (ADR-277 D1) :
  - ADR-273 D1 : le juge est le même solveur continué ;
  - ADR-280 D1 : la surface ;
  - ADR-281 D1 : chaque résultat montré dès qu'il est mesuré.
- **pièges** :
  - les dispositions des tableaux : Saint-Venant `i·ny + j`, APIC `(k·ny + j)·nx + i` ;
  - le pas de Saint-Venant seul, par sa propre condition de Courant (0,4) ;
  - la remontée du témoin se lit sur le Saint-Venant du rivage.

**Critères de la session.** E1 et E2 tenus, ou leur échec nommé.

### Plan

- [x] **P1** — jeton ; plan.
- [x] **P2** — E1.
- [ ] **P3** — E2.
- [ ] **P4** — preuve ; rituel.

### Notes de reprise
- **P2 fini (E1)** — **tenu** : le volume rendu à 1,1·10⁻¹³ de la surface ; après 1 s, la vitesse de Saint-Venant 5,7·10⁻⁷ m/s, |η| 1,6·10⁻⁷ m.
