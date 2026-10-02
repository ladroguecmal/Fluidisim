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

Session : S446 — **en cours**. Demande de l'utilisateur (2026-10-02) : *« Continue »* — la suite déclarée : **c2, le raccord bande ↔
mer** ([ADR-214](../docs/adr/ADR-214-b-entre-dans-la-bande.md), note de S445).

**La conception.** Un domaine `Apic3` en eau totale, posé dans un domaine `Volume3` en δ relatif, sur la même grille (même maille,
mêmes couches, décalage entier en `x`). Le raccord passe par **la zone des colonnes** d'`Apic3` — une surface à hauteur de colonne,
comme le pas couplé —, la bande de particules restant à l'intérieur. (1) **Bords ouverts** (`Apic3::enable_open_boundaries`,
`set_open_boundaries`) : sur les faces latérales du domaine, la vitesse normale est **imposée** au lieu de nulle — la projection la
prend déjà comme donnée de Neumann ; le transport des colonnes compte son débit, mouillé à la hauteur de la colonne du bord. Sans
bords ouverts : au bit. (2) **Chaque pas** (banc `raccord_bande_mer`) : la mer avance ; la bande reçoit à ses bords `B + δ` aux faces ;
la bande avance ; la mer reçoit, à l'intérieur de la bande (à deux colonnes de ses bords), `δ = total − B` — hauteurs et vitesses.

**Critères, écrits avant.** **c2 reçu** si, sous B seul (houle de 5 cm, 4 m, 25 cm), sur **10 s** : (1) le δ créé dans la mer **hors** de
la bande reste sous **1 cm** ; (2) la masse : le volume de δ de la mer, sa dérive, sous 1 % du volume que la houle fait passer par une
face de la bande en une demi-période ; (3) sans bords ouverts, `Apic3` au bit (la suite) ; zéro avertissement. Bancs courts (D4).

### Plan

- [x] **P1** — jeton, plan seul.
- [x] **P2** — les bords ouverts d'`Apic3` ; la vitesse de grille imposable ; essais.
- [x] **P3** — le banc `raccord_bande_mer` ; mesures ; critères.
- [ ] **P4** — preuve ; rituel (allégé).

### Notes de reprise
- **P2** — `Apic3::enable_open_boundaries`, `set_open_boundaries` (les faces `u` des bords `i = 0` et `i = nx`, vitesse normale imposée
  par `walls`), le débit de bord dans `columns_transport` (mouillé à la hauteur de la colonne du bord), `set_grid_velocities`. Essai
  `open_boundaries_carry_their_flux_s446` : le volume change exactement de ce que les bords font passer (entrée seule : 2,5721·10⁻² m³
  pour 2,5721·10⁻² ; vitesses égales : 1,1766·10⁻³ pour 1,1766·10⁻³ — les hauteurs mouillées diffèrent).
- **P3** — banc `raccord_bande_mer` (mer `Volume3` 16 m relative, bande `Apic3` 4 m en eau totale, zone de colonnes seule, bords
  ouverts ; houle de 5 cm, 4 m ; 10 s, 14 s de calcul) : marge 2 — δ hors de la bande **9,86 mm** au plus (plafonne dès ≈ 5 s) ; la
  dérive du volume de δ de la mer **1,4·10⁻³ m³**, qui oscille, **4,4 %** du volume d'une demi-période (3,2·10⁻² m³) ; la bande à 5 mm de
  B en son milieu. Marge 0 : 10,8 cm, 35 % ; 1 : 12,5 mm, 7,5 % ; 4 : 9,1 mm, 7,2 %. **(1) tenu, (2) manqué** : le flux de l'interface
  n'est pas compté pareil des deux côtés (la mer le mouille à la moyenne de deux colonnes, la bande à sa colonne de bord ; et la
  marge recouvre). c2, première session. Suite **762**, zéro avertissement.

