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

Session : S562 — **en cours**. En autonomie, **5.7** : deux manques de S560 — le déversoir par couches, non éprouvé, et l'instantané de la
composition (I-17 : un état qu'on ne sauve pas n'est pas un état).

**Ce que la session fait.** (a) **L'écrémeur** : une cuve de 1 m², 1,0 m³ d'eau sous 0,4 m³ d'huile (surface à 1,4 m), un déversoir de
0,1 m (`C_d` = 0,62) dont la crête est à 1,2 m — au-dessus de l'interface : seule l'huile passe. (b) **Le bloc `WVLQ`** : la composition
entière, nœuds × liquides en `i64`, l'empreinte de la table (FNV-1a des densités), une somme de contrôle ; la restauration refuse une
longueur, une version, une table, une intégrité, une ligne qui ne somme pas au volume du nœud restauré — atomique. À côté de WVST
(ADR-140), pas dedans : un réseau sans liquides n'a pas de bloc, et WVST V2 reste lisible.

**Références, calculées avant.** Écrémeur : `dH/dt = −(2/3)·C_d·b·√(2g)·H^{3/2}/A` → `H(t) = 1/(1/√0,2 + 0,0915·t)²` ; à 600 s,
`H` = 3,06·10⁻⁴ m → il reste **200 306 ml** d'huile, l'eau intacte (1 000 000 ml). Constante (ADR-240 D2) : la charge tombe à 1 % de
0,2 m vers 190 s ; l'essai dure 600 s. Instantané : la continuation au bit — un pas de 1 000 à 6 000 du manomètre de S560, d'une traite
contre restauré au pas 1 000.

**Quantum** (ADR-236 D1) : 1 ml. **Critères, écrits avant.** (1) Écrémeur : l'eau intacte au millilitre à chaque pas ; l'huile restante
à 100 ml de 200 306 ml (rapport 100). (2) Instantané : la suite restaurée identique au bit (composition, nœuds) ; (3) les refus : un octet
changé (`Integrity`), une autre table (`Configuration`), des nœuds aux volumes différents (`Record`), une longueur (`Length`) ; rien
d'écrit sur refus.

### Plan

- [x] **P1** — jeton ; plan.
- [ ] **P2** — le bloc `WVLQ` et les essais ; (1)–(3).
- [ ] **P3** — preuve ; liste 5.7 ; rituel.

### Notes de reprise
