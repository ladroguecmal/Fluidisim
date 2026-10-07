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

Session : S607 — **en cours**. En autonomie (ADR-247) : **1.5 — la grille 3D de référence stable : adressage, zones actives, échanges
client/serveur** (ADR-006 §2 ; absent, conçu). La seule structure que serveur et clients partagent ; elle ne contient aucune eau.

**Ce que la session fait.** Un module `hydro_grid.rs` : `CellId { frame: u32, niveau: u8, morton: u64 }` — cellule de base 64 m, trois
niveaux (64 / 512 / 4 096 m : le troisième est la région de rebasage d'ADR-002 §2.3, asserté), Morton 3D à 20 bits par axe (±33 554 km
au niveau 0) ; `cellule(frame, niveau, position)`, `coordonnees`, `parent` (`morton >> 9`), `enfants` (une plage contiguë de 512 clés),
`voisins` (26) ; `ZonesActives` — les cellules de niveau 0 qu'une boule d'intérêt touche, et leurs ancêtres ; `Echange` — l'écart entre
deux états (ajouts, retraits, triés), encodé en octets (8 + 12 par cellule) et décodé : le client reconstruit l'état du serveur. Ne fait pas :
la subdivision de publication par type de donnée (R07), le routage d'un événement W, l'index des volumes V, le transport réseau (10.1).

**Références, calculées avant** (ce script, par une implémentation Python indépendante). Clés : 0xe00000000000000, 0xa92492492492493, 0x624924924905a, 0x2a492492693, 0x5b6db6db6db6db6 (les
points [(0, (0.0, 0.0, 0.0)), (0, (100.0, -50.0, 7.0)), (1, (-5000.0, 2000.0, 0.0)), (2, (40000.0, -300.0, 5.0)), (0, (-33554432.0, 33554431.0, -1.0))]). Trois intérêts mobiles sur dix pas (une barque de 100 m de rayon à 30 m par pas ; un point fixe de 200 m ; un nageur de
64 m à 25 m par pas) : tailles [305, 309, 308, 307, 306, 308, 305, 308, 306, 308] ; ajouts [305, 10, 11, 7, 7, 13, 8, 14, 6, 11] ; retraits [0, 6, 12, 8, 8, 11, 11, 11, 8, 9] ; octets par message [3668, 200, 284, 188, 188, 296, 236, 308, 176, 248] (total
**5792**) ; au dernier pas, **14** parents de niveau 1 et **8** de niveau 2.

**Quantum** : des entiers (des clés). **Critères, écrits avant.** (1) les cinq clés au bit ; l'aller-retour `cellule` ↔ `coordonnees` sur
10⁵ points pseudo-aléatoires ; (2) le parent de la cellule d'un point est la cellule du point au niveau supérieur (les mêmes 10⁵ points,
niveaux 0 → 1 → 2) ; les 512 enfants d'une cellule de niveau 1 forment la plage `[p·512, (p+1)·512)` et ont ce parent ; (3) 26 voisins à
Chebyshev 1, tous distincts ; (4) les tailles, ajouts, retraits et octets des dix pas, égaux à la référence ; après chaque message, l'état du
client égal à celui du serveur, au bit ; les comptes de parents 14 et 8 ; (5) le niveau 2 vaut 4 096 m, le seuil de rebasage ;
(6) refus : niveau > 2, position hors de portée ou non finie, message tronqué.

### Plan

- [x] **P1** — jeton ; plan.
- [ ] **P2** — `hydro_grid.rs` et ses essais ; (1)–(6).
- [ ] **P3** — preuve ; liste 1.5 ; rituel.

### Notes de reprise
