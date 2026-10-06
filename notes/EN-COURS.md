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

Session : S565 — **en cours**. En autonomie, **5.8 — le réseau fermé sous pression** (absent ; « reporté en v2 » par ADR-010 §4 — la v1
est atteinte et la liste entière est l'objectif, ADR-190). Première pièce : **la solution d'un réseau de conduites en charge** — les
charges aux jonctions et les débits, les réservoirs (les nœuds de V à surface libre) imposant leurs charges.

**Ce que la session fait.** `hydro_charge.rs` : des conduites `h_a − h_b = R·Q·|Q|` (une résistance quadratique, Darcy–Weisbach en régime
turbulent rugueux) entre des sommets fixes (charges données) et des jonctions (inconnues, une demande chacune) ; Newton sur les charges des
jonctions, la matrice jacobienne (un laplacien pondéré) résolue par élimination de Gauss à pivot partiel dans un tampon de l'appelant
(I-06), un pas amorti si le résidu croît ; sous 1 µm de perte, une conduite est linéarisée (la dérivée de la racine y est infinie). Une
jonction sans chemin vers une charge fixe est refusée. Le couplage au pas de V (les réservoirs qui se vident par le réseau) viendra
ensuite.

**Références, calculées avant, par des méthodes indépendantes** (ADR-239 D1). (1) *Trois réservoirs* (100, 80, 50 m ; `R` = 2 000, 3 000,
1 500 s²/m⁵) reliés à une jonction — par bissection sur la continuité : **`h_j` = 77,455794994 m**, débits 0,106170158 ; 0,029121613 ;
−0,135291771 m³/s. (2) *Une maille* — un réservoir à 60 m relié à J0, la maille J0–J1–J2–J3, demandes 0,06 ; 0,08 ; 0,04 m³/s — **par
Hardy Cross** (une autre méthode : corrections de débit dans la maille) : charges J0 43,800000000, J1 34,964659639, J2 33,231017516, J3
34,924075772 m ; débits J0→J1 0,093996491217, J0→J3 0,086003508783 m³/s.

**Quantum** (ADR-236 D1) : les références publiées à 10⁻⁹ m. **Critères, écrits avant.** (1), (2) les charges à 10⁻⁸ m (rapport 10), les
débits à 10⁻⁹ m³/s ; la continuité à chaque jonction sous 10⁻¹² m³/s ; moins de 30 itérations. (3) Refus : une jonction isolée des
charges fixes (`Domain`), une résistance non positive (`Domain`), un tampon trop court (`Capacity`).

### Plan

- [x] **P1** — jeton ; plan.
- [ ] **P2** — `hydro_charge.rs` et ses essais ; (1)–(3).
- [ ] **P3** — preuve ; liste 5.8 ; rituel.

### Notes de reprise
