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

Session : S608 — **terminée**. En autonomie (ADR-247) : **le lot** (dû ; feuille de route S605–S607), puis **1.6 — cellules, domaines et
solveurs distincts, niveaux d'activité des cellules** (ADR-006 ; absent, conçu). La source (`architecture_globale` §3.3) nomme cinq
niveaux : inactive, simplifiée, partiellement active, simulation active, niveau de détail supérieur — et dit que le niveau spatial et la
précision physique ne sont pas équivalents.

**Ce que la session fait.** Un module `activite.rs` : `Activite` (les cinq niveaux, ordonnés) ; `DomaineMeta` — ce que le serveur sait
d'un domaine (référentiel, origine, `dx` parmi les six niveaux d'ADR-006 §3.2, blocs de 8³ mailles) — ; `couverture` — le volume de chaque
cellule de 64 m de la HydroGrid couvert par les blocs d'un domaine, **non alignés** sur elle ; `niveaux` — par cellule : détail supérieur si
un domaine de `dx ≤ dx_detail` la touche (la précision, non la subdivision), active si un domaine la couvre entière, partielle s'il la
touche, simplifiée si W la marque, inactive sinon. Ne fait pas : l'hystérésis des niveaux (celle des domaines existe, ADR-006 §4,
`scheduler::ON/OFF`), la forme réduite d'une perturbation qui disparaît (§3.4), la publication des niveaux au réseau.

**Références, calculées avant** (ce script, en rationnels exacts). Domaine A (`dx` 0,5 m, origine (3,3 ; −1,7 ; −65,7), 40 × 40 × 17
blocs) : **36 cellules** touchées, **2** pleines ([(1, 0, -1), (1, 1, -1)]), volume 1740800 m³ (= blocs × 4³). Domaine B
(`dx` 0,05 m, 50 × 50 × 5 blocs posé dans une cellule pleine de A, il dépasse dans celle du dessus) : 2 cellules. W : neuf cellules d'un sillage. Avec `dx_detail` =
0,10 m, les niveaux (inactive, simplifiée, partielle, active, détail) : **[0, 6, 33, 1, 2]**. Fractions couvertes par A : (0, 0, -2) : 0.025192871093750 ; (1, 0, -1) : 1.000000000000000 ; (2, 2, -1) : 0.261130371093750 ; (1, 0, 0) : 0.035937500000000.

**Quantum** : f64 ; la couverture est un volume en m³, exacte en rationnels, en f64 à l'arrondi (10⁻⁹ relatif). **Critères, écrits
avant.** (1) le nombre de cellules touchées et pleines de A, et les quatre fractions à 10⁻¹² ; (2) la somme des volumes couverts égale au
volume des blocs, à 10⁻⁹ relatif — pour A et pour B ; (3) le compte des cinq niveaux [0, 6, 33, 1, 2] ; les 2 cellules de B ([(1, 0, -1), (1, 0, 0)]) au niveau détail, dont
une que A couvre entière ; (4) refus : un `dx` hors des six niveaux, un domaine d'un autre référentiel ne couvre rien.

### Plan

- [x] **P1** — jeton ; le lot ; plan.
- [x] **P2** — `activite.rs` et ses essais ; (1)–(4).
- [x] **P3** — preuve ; liste 1.6 ; rituel (`--lot`).

### Notes de reprise
- **P2 fini** — (1)–(4) tenus du premier essai ; les fractions à 10⁻¹⁴ des rationnels. Suite : 798 essais listés (mesurée).
- **P3** — preuve ACTIVITE-S608 ; liste 1.6 (absent → partiel) et décompte ; index ; journal ; le lot.
