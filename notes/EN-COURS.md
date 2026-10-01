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

Session : S423 — **en cours**. Demande de l'utilisateur (2026-10-01) : *« Continue »* — la suite déclarée : **C7e**
([preuve](../docs/validation/APIC-CARTE-S416.md) §14 : B10 en bande étroite 3,9 ms par pas + 1,9 de bascule ; restent la bascule, les
dispatchs du cycle, la surface).

**Ce que la session fait, dans l'ordre du gain attendu.** (1) **La bascule en groupe** : `switch_apply` et `floor_move` parcourent
les colonnes et les faces sur un fil (≈ 1 ms quand rien ne bascule) ; leurs boucles sont indépendantes par colonne ou par face, leurs
sommes sont des entiers (l'ordre n'y change rien) — un groupe de 256 fils, les gestes ordonnés (retraits par la visite de la
référence, ensemencements en ordre de colonnes) gardés au fil 0 et sautés quand il n'y en a pas. (2) **Les dispatchs du cycle** :
le niveau 1 dans le groupe des niveaux grossiers (quatre dispatchs de moins par cycle) ; mesurer aussi un lissage V(3,3), moins
d'itérations contre plus de lissages. (3) **La surface** : mesurer ce qui coûte dans `reconstruct` (les images aux parois, les
mailles loin de toute particule) avant d'y toucher. (4) **La multigrille par défaut** si toutes les issues tiennent.

**Critères, écrits avant.** (1) Issues inchangées : B10 en bande étroite au pincement de la référence, gestes et `n` comme S422,
volume exact ; bascules forcées (`--apic3d-carte-decision`, `INITIAL`, `PENTE`, `MAINTIEN`) identiques à la référence ; raccord, bande,
ballottement dans leurs tolérances. (2) Le coût publié par sous-étage avant et après ; **visé : bascule ≤ 0,8 ms, projection ≤ 1 ms,
pas + bascule ≤ 4 ms** sur B10 en bande étroite ; δ ≤ 2 ms reste l'objectif de C7e, la session dit ce qui en reste. (3) Suite, zéro
avertissement.

### Plan

- [x] **P1** — jeton, plan seul.
- [x] **P2** — `switch_apply` en groupe ; bascules forcées identiques ; mesure.
- [x] **P3** — *réordonné après la mesure des médianes (S423)* : au pas ordinaire, projection 1,38 ms, surface 0,68 + décision 0,81
  (dont la surface rafraîchie), séparation 0,32 ; application 0,10 et fond 0,17 (0,60 et 0,42 au 99ᵉ centile : les pas qui
  basculent). D'abord **la multigrille** : le niveau 1 dans le groupe, V(3,3) mesuré ; symétrie, issues, mesure.
- [x] **P4** — **la surface** (`reconstruct`, deux fois par pas) : mesurer ce qui coûte, réduction exacte si elle se trouve.
- [ ] **P5** — les pas qui basculent : le retrait parallèle à forme close et l'ensemencement par préfixe ; la multigrille par défaut.
- [ ] **P6** — non-régression, suite ; preuve §15 ; registres.
- [ ] **P7** — rituel.

### Notes de reprise
- **P2** — `switch_apply_group` (256 fils : capacité, colonnes converties, décalage, faces qui cessent d'être frontière, masque, en
  parallèle ; réductions entières dans le groupe ; retraits et ensemencements au fil 0 ; aucune barrière sous condition — FXC).
  Bascules forcées (`INITIAL`, `PENTE`, `MAINTIEN`) **identiques à la référence** (positions, fonds, réserves, volume à 0 quantum) ;
  B10 en bande étroite inchangé. **Mais** l'application ne passe que de 0,63 à 0,60 ms : essai — sans le noyau, 0,095 ms (la liste) ;
  le noyau, ≈ 0,5 ms au 99ᵉ centile, vient des pas où des colonnes basculent : le fil 0 y retire des centaines de particules une à une.
  **Réorientation de P3** (déclarée avant) : le retrait parallèle à forme close — la visite de la référence (échange avec la dernière)
  met dans la k-ième plus petite place retirée sous le nouveau `n` la k-ième plus grande particule gardée au-delà ; pour la bascule et
  le fond, le traitement d'une retirée n'est qu'un compte (par colonne pour le fond) — puis l'ensemencement par préfixe sur les colonnes.
- **Médianes** (B10 en bande étroite, multigrille) : transfert 0,17, surface 0,68, projection 1,38, séparation 0,32, absorption 0,09 + 0,19, échange 0,16 + 0,32 ; bascule : décision 0,81, application 0,10, fond 0,17. Le banc les imprime (`cout_median_ms`).
- **P3** — la multigrille. **Essai 1, le niveau 1 dans le groupe** (sept dispatchs par cycle au lieu de onze) : **plus lent**, projection médiane 1,38 → 1,89 ms — un seul groupe traite 2 688 mailles moins vite que quatre dispatchs ; revenu en arrière. **Mesure** (`SANS_GROSSIERS=1`) : sans le groupe des niveaux ≥ 2, 38,8 itérations à ≈ 34 µs ; avec, 13,9 à ≈ 74 µs — le groupe coûte ≈ 40 µs par cycle, surtout en phases à barrière vides. **Retenu** : le nombre de niveaux ≥ 2 en **constante de pipeline** (`override MG_NC`, fixée à la création — une constante pour FXC, sans phases vides) ; plafond adaptatif avec la multigrille : pire des huit derniers + 2, repli à 40. Symétrie inchangée (≤ 1,95·10⁻⁷), issues de B10 identiques. **Projection médiane 1,28 ms, p99 1,35** (1,38 et 1,45). Reste ≈ 75 µs par itération × 16 : fusionner des noyaux (mise à jour + premier lissage, restriction + premier lissage du niveau 1), ou un autre cycle — non fait.
- **P4** — la surface. **Réemploi** de la surface de la décision au pas suivant, maille par maille (aucune colonne à portée convertie, ensemencée ou au fond déplacé — `swb[SW_KEEP]` marqué par la bascule — et le même corps, confirmé par l'hôte à 10⁻⁶ maille, `TOL_CORPS=`) : exact, mais **sans gain mesurable sur B10** (le fond suit la cavité presque à chaque pas ; et la position recalculée du corps dérive de ≈ 10⁻⁶ m de sa position intégrée) ; gardé. **Essai** : sans la reconstruction, le passage tombe à 0,014 ms — c'est elle (0,6 ms), dont le temps est celui du fil le plus long (≈ 5 000 mailles de la bande, chacune 125 mailles voisines et 25 colonnes virtuelles). **`reconstruct_coop`** : 32 fils par maille, voisines et colonnes réparties, sommes réduites — l'ordre des sommes change, à l'arrondi ; étages : `φ` 1,6·10⁻⁶ m (ballottement), 1,1·10⁻⁶ (bande), 4,6·10⁻⁶ (B10), étiquettes identiques. B10 en bande étroite : **surface 0,69 → 0,09 ms, décision 0,81 → 0,22** (médianes) ; pincement identique, volume exact ; `φ` à l'interface max 6,8 mm (t = 0,869 √(D/g)), dans l'enveloppe des témoins (S420). **Pas p99 3,20 ms + bascule 1,25** ; médianes ≈ 2,7 + 0,5.
