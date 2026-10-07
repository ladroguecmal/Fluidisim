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

Session : S604 — **terminée**. En autonomie (ADR-247) : **12.2 — l'éditeur de rivières : dessin, validation bloquante, gravure**
(SPEC-005 §5 ; absent). Le cœur de l'éditeur, sans son interface : ce que l'outil affiche en continu, ce qu'il refuse, ce qu'il grave.

**Ce que la session fait.** Un module `riviere.rs` : un réseau de biefs (une ligne d'eau tracée par sommets `(x, y, z)`, une largeur, un
débit, un `n` de Manning ; des nœuds source, confluence, lac, mer) ; `profil` — par segment, la pente de la ligne d'eau, la hauteur
normale de Manning (section rectangulaire), `v = Q/A`, `Fr = v/√(g·h)`, et les ressauts (`Fr` qui passe de plus de 1 à moins de 1) ;
`valider` — les règles bloquantes de SPEC-005 §5.3 : la ligne d'eau descend strictement (tolérance nulle, aussi à travers un nœud),
`ΣQ` entrant = sortant à chaque confluence, `v` dans une plage plausible (**0,1–3 m/s** par défaut, réglable : un torrent la relève),
un lac a un exutoire ; `graver` — le fond `z_eau − h` au milieu de chaque segment, et les conflits où il creuse le terrain de plus d'un
seuil. Ne fait pas : l'interface, la spline (des sommets ici), `largeur(s)` et `section_type(s)`, `debit(t)`, la cinquième règle (les
régions de niveau marin pavent la planète), la gravure dans une carte de hauteurs.

**Références, calculées avant** (ce script, par une bissection indépendante). Bief A (30 m³/s, 20 m, n = 0,035, pente 5·10⁻⁴) : `h` =
**1.781932256 m**, `v` = 0.841783 m/s, `Fr` = 0.201335. **La faute de saisie** (le premier sommet tapé 69,5 au lieu de 20,0 : une chute de
50 m, ×100) : `v` = **3.5191 m/s** > 3 — bloquée ; la faute inverse (÷100) : `v` = 0.1765 m/s — **elle passe** (`v ∝ S^0,3` : la règle
de vitesse n'attrape qu'un sens ; à noter dans la preuve). Bief E (5 m³/s, 8 m, n = 0,03) : raide (6 m sur 300 m) `Fr` = **1.1765**,
doux (0,1 m sur 400 m) `Fr` = **0.1457** — un ressaut entre les deux. Gravure du bief C (40 m³/s, 25 m, pente 5·10⁻⁴ : `h` =
**1.832244795 m**) dans un terrain plat à 19,5 m : creusements **2.582244795** et **3.082244795 m** — deux conflits au seuil de 1 m ;
à 17,7 m : 0.782244795 et 1.282244795 m — un conflit (le second segment).

**Quantum** : f64. **Critères, écrits avant.** (1) `h` du bief A à 10⁻⁹ m ; `v` de la faute à 10⁻⁶ ; (2) le réseau juste (A, B → confluence →
C → lac → D → mer) : aucun défaut ; (3) chaque faute isolément, un défaut et un seul, du bon genre, au bon endroit : la chute ×100
(vitesse ; A), un sommet qui remonte (le milieu de A à 20,1), un sommet plat (le milieu de A à 20,0 : tolérance nulle), un aval de
nœud plus haut que l'amont (C parti de 19,1), B à 11 m³/s (confluence, écart
1 m³/s), le lac sans exutoire ; la faute ÷100 : aucun défaut ; (4) le ressaut du bief E entre ses deux segments, `Fr` à 10⁻⁶ ; (5) la
gravure : les creusements à 10⁻⁹ m et les conflits ({0, 1} à 19,5 m ; {1} à 17,7 m) ; (6) refus : largeur, débit, `n` non positifs,
moins de deux sommets, un nœud hors du réseau.

### Plan

- [x] **P1** — jeton ; plan.
- [x] **P2** — `riviere.rs` et ses essais ; (1)–(6).
- [x] **P3** — preuve ; liste 12.2 ; rituel.

### Notes de reprise
- **P2 fini** — (1)–(6) tenus du premier essai ; chaque faute un seul défaut ; la faute ÷100 passe (attendu). Suite 795.
- **P3** — preuve RIVIERE-S604 ; liste 12.2 (absent → partiel) et décompte ; index ; journal.
