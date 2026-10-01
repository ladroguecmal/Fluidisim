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

Session : S425 — **en cours**. Demande de l'utilisateur (2026-10-01) : *« Continue »* — la suite déclarée : **C7e**
([preuve](../docs/validation/APIC-CARTE-S416.md) §16 : B10 en bande étroite, pas + bascule 3,25 ms au p99 ; la projection à 0,885 ;
restent les fils de l'échange et de l'absorption, 0,62 + 0,40, la séparation, 0,41).

**Ce que la session fait.** (1) **Profiler la fin du pas** comme la projection en S424 : chaque noyau de la séparation, de
l'absorption et de l'échange, répété dans un passage horodaté ; et le nombre de gestes par pas (absorbées, retirées, posées), pour
savoir ce que coûtent les fils séquentiels par geste. Soupçon à vérifier : la séparation et le tri lancent leurs noyaux sur la
**capacité** (≈ 130 000 fils) quand la bande n'a que ≈ 4 000 particules vivantes. (2) **Réduire ce que le profil désigne**, dans
l'ordre du gain, à sémantique exacte — pour les lancements à la capacité, un dispatch indirect taillé sur `n` (arguments écrits par
un noyau, copiés entre deux passages) ; pour les fils, ce qui garde l'ordre de la référence là où il fait le résultat. (3) Mesurer ;
ce qui reste dit.

**Critères, écrits avant.** (1) Issues inchangées : étages à l'arrondi, gestes et `n` comme S424, B10 en bande étroite au pincement de
la référence, volume exact, bascules forcées identiques, ballottement, raccord, bande dans leurs témoins. (2) Le coût par sous-étage
publié avant et après ; **visé : pas + bascule ≤ 2,5 ms au p99** sur B10 en bande étroite (3,25) ; δ ≤ 2 ms reste l'objectif de C7e.
(3) Suite, zéro avertissement.

### Plan

- [x] **P1** — jeton, plan seul.
- [x] **P2** — le profil de la fin du pas et les gestes par pas.
- [x] **P3** — la première réduction que le profil désigne ; issues, mesure.
- [x] **P4** — la seconde ; issues, mesure.
- [x] **P4b** — *ajouté en cours de session (après P4)* : le fil de l'échange — ne visiter que les colonnes à fond dont le solde dépasse une particule.
- [x] **P5** — non-régression, suite ; preuve §17 ; registres.
- [ ] **P6** — rituel.

### Notes de reprise
- **P2** — le profil (`PROFIL=1`, banc B10 en bande étroite). **Les fils, par pas** (moindres carrés sur les pas) : absorption **6 + 2,0 µs par absorbée** (médiane 98 par pas, 210 au plus) ; échange **66 + 2,05 µs par geste** (retirées + posées : médiane 139, 266 au plus ; faces-mailles actives 46, 127 au plus). **La fin du pas, noyau par noyau** (chaque préfixe de la suite répété 50 fois, différences — un noyau du tri répété seul corrompt les tranches et a fait perdre la carte au premier essai) : **`compact_scan` 80 µs** (le préfixe des groupes sur un fil, ≈ 500 groupes à la capacité) — il sert à chaque liste ordonnée : absorbées, faces-mailles actives, bascule, fond, soit ≈ 4 × 80 µs par pas ; **`scan_blocks` 13,5 µs** (le même motif dans le tri, 84 blocs), trois à quatre tris par pas ; **`separate_shift` 101 µs**, **`bin_sort` 64 µs** ; le reste 0,3 à 6 µs. **Réordonné** (P3, P4 du plan : « ce que le profil désigne ») : **P3 — les préfixes en groupe** (entiers : exact) ; **P4 — la séparation et le tri par maille** ; les fils (parallélisme par vagues de non-conflit, à faces disjointes) ensuite si le temps reste.
- **P3** — les préfixes en groupe (`compact_scan`, `scan_blocks` : 256 fils, morceaux contigus, préfixe des sommes de morceaux ; entiers, même résultat ; dispatch inchangé, un groupe) : **80 → 2,2 µs** et **13,5 → 1,2 µs**. Étages (tri, compactage, échange) identiques ; B10 en bande étroite : pincement identique, volume exact, même premier écart de gestes qu'avant (pas 34). Sous-étages médians : absorption (la liste) 0,091 → 0,013 ms, échange 0,166 → 0,070, application 0,099 → 0,022, fond 0,107 → 0,029. **Pas p99 2,75 → 2,56 ms ; bascule p99 0,48 → 0,31** ; pas + bascule 2,87.
- **P4** — **l'occupation** (nouvelle ligne du profil) : quelques mailles portent 30 à 50 particules (8 nominales) ; le tri par insertion sur un fil et la séparation (27 mailles voisines, un fil par particule) y passaient leur temps. **Tri par rang** (`bin_rank`, `bin_place` : chaque particule compte les indices plus petits de sa tranche et s'écrit à ce rang dans `pscratch`, libre hors du compactage) : même ordre, **64 → 4,7 µs**. **Séparation élaguée** : une maille voisine n'est lue que si la particule est à moins de `dmin` (+ 10⁻⁴ maille) de leur frontière ; les autres ne contribuent rien — même somme, même ordre : `separate_shift` **102 → 50 µs** au profil. Étages identiques (tri, séparation 1,19·10⁻⁷ et 2,38·10⁻⁷ m comme avant) ; B10 en bande étroite : pincement identique, volume exact, même premier écart de gestes. Médianes : séparation 0,327 → **0,106 ms**, transfert 0,165 → 0,103, décision 0,218 → 0,147. **Pas p99 2,29 ms, bascule p99 0,25** — 2,54 pour 2,5 visés. Restent les fils (0,61 + 0,41 au p99) et la projection (0,88).
- **P4b** — ajouté au plan après P4 (le profil des fils : 66 µs fixes au fil de l'échange, même sans geste). Le solde vertical parcourait les 256 colonnes une à une, trois diffusions chacune ; une colonne dont le solde ne dépasse une particule dans aucun sens ne fait rien, sans effet de bord : par morceaux de 64 colonnes, chaque fil teste la sienne, un préfixe range les colonnes dues dans l'ordre, seules elles sont visitées. Étage du fond et bande 30 s **identiques** (gestes cumulés compris : 5 478 / 354 / 5 795, 1,318 mm) ; B10 en bande étroite : pincement identique, volume exact. Fil de l'échange **66 + 2,05 → 30 + 1,90 µs par geste** ; médiane 0,33 → 0,28 ms. **Pas p99 2,21 ms + bascule 0,23 = 2,45 ms : la cible de 2,5 est tenue.**
- **P5** — non-régression **identique à S424 au chiffre près** : étages (tri, compactage, séparation, échange, projection), cycle, bascules forcées (10 instants), B10 nu (pas 54), ballottement 0,447 (diagonale 0,454), colonnes 0,002, raccord 4,015 mm (gestes 5 289 / 109 / 5 385), bande 1,318 (P4b). Suite du cœur 753 / 19 / 0 avertissement. Preuve §17 ; liste 4.19, feuille de route, index, file.
