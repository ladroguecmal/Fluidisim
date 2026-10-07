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

Session : S639 — **terminée**. En autonomie (ADR-247), sur la demande de l'utilisateur (2026-10-07 : « reprend avec 1 », le plan du **rouleau
3D** en cinq étapes, accepté : « Ok très bien »). **Étape 1 — le fond en pente dans APIC 3D, au repos** (4.14, 4.16).

**Ce que la session fait.** `Apic3::set_seabed(fond)` : une hauteur de fond par colonne, mise en **escalier** (les mailles dont le centre est
sous le fond deviennent solides : `SOLID`, comme la sphère de S393) ; à chaque pas, les faces qui touchent le fond sont des parois immobiles,
les particules qui y entrent sont repoussées au-dessus (sans vitesse descendante), et la reconstruction de la surface **reflète** les
particules sous le fond, comme les parois (S389) et la sphère (S393). Le réglage alloue, le pas non. Ne fait pas : les faces coupées (un
fond lisse), la vague (étape 2).

**Références, calculées avant** (ce script). Un canal de 48 × 4 × 16 mailles de 5 cm (2,4 × 0,2 × 0,8 m) ; fond plat à 5 cm jusqu'à 0,805 m puis
pente 1:3 ; eau au repos à 0,4 m — le rivage à **1.855 m**. **852 mailles solides** ; **6048 particules** posées (au-dessus de
l'escalier, sous le niveau). **Amendement, avant toute mesure de repos** : la pente partait de 0,8 m, où `(k + ½)·dx = fond(x)`
exactement pour `i = 3k + 14` — f32 et f64 tranchaient ces égalités différemment (6 016 particules contre 5 984) ; à 0,805 m, aucune
égalité (asserté, ADR-257 D1). Le témoin : le même canal à fond plat (5 cm), 10752 particules. **Bornes du montage** (ADR-257 D1, assertées) :
le rivage dans le domaine, le fond sous le couvercle.

**Quantum** : la maille (5 cm) ; la vitesse en f32. **Critères, écrits avant** — ceux du repos de S388 et S393. (1) 852 mailles solides,
6048 particules, aucune perdue en 2 s, aucune sous le fond de sa colonne ; (2) la vitesse parasite ≤ 1 cm/s sur 2 s ; (3) le témoin à fond
plat, ≤ 1 cm/s ; (4) refus : une longueur fausse, une valeur non finie ou hors de [0, hauteur du domaine].

### Plan

- [x] **P1** — jeton ; plan.
- [x] **P2** — `set_seabed` et ses essais ; (1)–(4).
- [x] **P3** — preuve ; listes 4.14, 4.16 ; rituel.

### Notes de reprise
- **P2 fini** — (1), (3), (4) tenus ; **(2) manqué** : 1,51 cm/s au rivage d'une maille. Localisé : sans contremarches 4,7 cm/s ; avec,
  1,5 ; l'image de coin aggravait (retirée) ; à 2,5 cm, 1,18 — la géométrie en escalier. Remède : les faces coupées (prochaine étape de la
  campagne). Les 47 essais d'APIC 3D passent. Suite : 823 essais listés.
- **P3** — preuve FOND-APIC3D-S639 ; lignes 4.14 ; index ; journal.
