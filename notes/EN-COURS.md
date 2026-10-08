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

Session : S707 — **en cours**. En autonomie, sans arrêt. **Première session longue** (ADR-279 D3) : la naissance de la 3D, pièces N1 et N2
de [LOD-ETAPE-2-S705](../docs/registres/LOD-ETAPE-2-S705.md).

**Ce que la session fait.** `birth_from_columns` (`apic3d_naissance.rs`) fait naître la 3D d'un état 2D :
- elle retire toutes les particules ;
- par colonne, elle pose `round(volume / quantum)` particules aux places du réseau, la plus basse d'abord. La hauteur est ainsi tenue au
  quantum près (3 mm à 2,5 cm), non à la couche (1,25 cm) ;
- chaque particule prend la vitesse et l'affine du G2P, sur une grille remplie par l'appelant ;
- l'écart de volume, donné − posé, revient à l'appelant.

**Les essais, dans l'ordre ; chacun a ses critères, écrits avant lui.**

| essai | ce qu'il juge | la seule cause qui change | critères |
|---|---|---|---|
| **E1 (N1)** | l'eau au repos, fond plat, 4 m, `h` = 0,49 m (une couche partielle), 1 s | la naissance, contre le semis du réseau (le témoin) | posé + écart = donné à 10⁻¹² ; la vitesse maximale après 1 s sous 1 mm/s, et au plus trois fois celle du témoin |
| **E2 (N2)** | l'onde de S704 sur un fond plat de 10 m, 1,6 s ; à 0,4 s, la 3D est réduite à `(h, ū)` par colonne, puis renaît par le profil vertical de SGN | la renaissance (la perte de la structure verticale), entre deux copies de la 3D (ADR-273 D1) | la crête aux plans 5, 6 et 7 m à moins de **3 mm** de la 3D ininterrompue ; l'instant de la crête au plan de 7 m à moins de **0,02 s** ; le nombre de particules tenu |
| **E3** | la même naissance à 0,4 s, mais depuis l'état de SGN, qui a porté l'onde depuis le départ | le porteur avant la naissance : SGN au lieu de la 3D | rapporté et attribué ; aucun critère de passage (S704 : SGN 5 % plus haut) |

E3 ne se lance que si E2 tient. Si E2 échoue, la session cherche la cause de E2 et s'arrête quand elle est nommée.

**Contrôles du plan** (ADR-266, ADR-267, ADR-268, ADR-273, ADR-276, ADR-277, ADR-279)

- **témoin** : E1, le semis du réseau ; E2, la 3D ininterrompue, par la même fonction ; E3, E2.
- **instrument** :
  - la crête, lue par le volume d'une tranche de 10 cm (S704 : résolution 0,2 mm, bruit ≈ ±2 mm) ;
  - la vitesse maximale des particules.

  Ce que rendrait chaque hypothèse en E2 :
  - le profil de SGN suffit à refaire la structure verticale : les crêtes à ±2 mm, le bruit de lecture ;
  - il ne suffit pas : un écart de crête, croissant vers l'aval.
- **calcul** : les tolérances respectent ADR-279 D1. À 2,5 cm, le juge diffère de sa version à 1,25 cm d'environ 6 mm de crête. 3 mm
  vaut la moitié de cette convergence, et un peu plus que le bruit de lecture. Le coût : ≈ 1 min par repos, ≈ 3 min par onde.
- **ADR**, et comment chacun est tenu (ADR-277 D1) :
  - ADR-273 D1, par E2 : la naissance est jugée entre deux copies de la 3D, avant d'être nourrie par SGN en E3 ;
  - ADR-276 D2 : chaque essai ne change qu'une cause, la colonne de la table ;
  - ADR-278 D2 : sans objet ici (aucun déferlement).
- **pièges** :
  - la réduction compte les particules par colonne : la masse reste exacte, et l'écart de E2 vaut 0 ;
  - `ū_x` et `ū_xx` par différences le long de x, nuls aux murs ;
  - au-dessus de l'eau, le profil est pris à la surface ;
  - l'instant de réduction doit tomber exactement à 0,4 s : le pas est borné.

**Critères de la session.** E1 et E2 tenus, ou leur échec nommé ; E3 rapporté.

### Plan

- [x] **P1** — jeton ; plan ; la note du déclencheur.
- [ ] **P2** — E1.
- [ ] **P3** — E2.
- [ ] **P4** — E3.
- [ ] **P5** — preuve ; rituel.

### Notes de reprise
