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

Session : S678 — **terminée**. En autonomie vers la v2 ; K3, 4.14 (« manquent une surface fiable en eau mince au rivage au repos »).
Sur le fond lisse (S640), l'eau au repos avec un rivage court à **0,26 m/s**. En deçà de la profondeur du noyau, la surface de Zhu et
Bridson se trompe de un à deux centimètres dans le film.

**Ce que la session fait.** **La surface du film par sa dernière couche** (`film_smooth`, après l'extension de `φ` sous le fond).

- Dans une colonne dont l'eau est moins profonde que trois mailles, la surface est la plus haute particule plus `dx/4` (la demi-
  distance entre couches), corrigée de l'écart de lecture du noyau à cette place dans la maille (`lattice_read_error`, une table de
  64 décalages cuite avec le fond).
- `φ = z − surface` y est mêlé à `φ` du noyau, avec un poids 1 sous deux mailles de profondeur, 0 au-delà de trois.
- Une colonne éclaboussée (moins de la moitié des particules qu'elle aurait pleine) garde `φ` du noyau.

Au repos, le film et l'eau profonde lisent ainsi la même surface.

**Contrôles du plan** (ADR-266, ADR-267, ADR-268)

- **témoin** : la pente immergée de S640 (l'eau partout plus profonde que trois mailles : rien ne change, au bit) ; la même plage sans
  la correction de lecture (`φ` du film sans l'écart du noyau).
- **instrument** : l'essai de repos de S640 sur la pente, avec rivage, la vitesse maximale sur 2 s, à 5 cm et 2,5 cm. Ce que
  rendrait chaque hypothèse :
  - si la surface du film est la cause et que la correction est juste, **moins de 1 cm/s** aux deux mailles ;
  - si la correction de lecture manque, **quelques cm/s** (le calcul ci-dessous) ;
  - si le film n'est pas la cause, ou que la colonne est mal lue, **0,26 m/s** comme avant.
- **calcul** (scratchpad `s678_calc.py`, ce script qui asserte) : la lecture du noyau varie de −1,18 à +1,73 mm sur une maille de 5 cm ;
  sans correction, la marche entre film et eau profonde entretient jusqu'à **3.8 cm/s** dans un film de 2 cm (asserté
  au-dessus du critère).
- **ADR** : ADR-259 D1 (le témoin), ADR-268 D2 (un remède jugé sous une seule cause), ADR-254 D2.
- **pièges** :
  - la place de la surface dans la maille, `frac(η/dx − ½)` ;
  - les gouttes (`is_droplet`), hors du film ;
  - les mailles sous le fond, qui gardent `SOLID` par `label_smooth` ;
  - S644 au fond lisse (le témoin de la remontée, ignoré, 6 min) changera : rapporté, non jugé.

**Critères, écrits avant.**

1. Au repos sur la pente avec rivage : au plus **1 cm/s** sur 2 s, à 5 cm et à 2,5 cm.
2. La pente immergée : au bit d'avant (8·10⁻⁶ m/s).
3. Le témoin sans correction rapporté, au-dessus de 1 cm/s comme le calcul l'annonce.
4. Les essais de S639 à S658 (le fond en escalier) inchangés.

### Plan

- [x] **P1** — jeton ; plan.
- [x] **P2** — `film_smooth` ; les essais ; (1)–(4).
- [x] **P3** — preuve ; liste 4.14 ; rituel.

### Notes de reprise
- **P2 fini** — (1) **manqué** : le film seul 0,33 / 0,22 m/s. Trouvé en route (aux notes, ADR-244) : les faces à peine ouvertes, seconde cause ; film + faces extrapolées (< 40 %) : 2,5 / 3,1 mm/s sur la plage du plan, 4 plages sur 6 ; manque sur 1:10 à 5 cm (0,39) et 1:3 à 0,31 m à 2,5 cm (0,15). Éteints par défaut, au bit d'avant. (2) tenu ; (3) 1,66 cm/s ; (4) par construction.
