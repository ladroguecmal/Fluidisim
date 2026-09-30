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

Session : S418 — **en cours**. Demande de l'utilisateur (2026-10-01) : *« Continue »* — la suite déclarée : **C7c-2**, l'échange à
la frontière sur la carte ([conception](../docs/validation/APIC-CARTE-S416.md) §8). Agent : Claude Code (Opus 5.5), au poste.

**Ce que la référence fait** (`columns_exchange`, S399–S407), et comment la carte le fait. (1) **L'absorption** : une particule
entrée dans une colonne de la zone paie le solde de la face-maille de frontière la plus proche (ou `η`, loin de toute bande) et rend
sa quantité de mouvement aux faces de la zone, `f += w·(v − f)/8` — **un mélange qui dépend de l'ordre** (deux particules dans
l'ordre inverse : écart `a·b·(v₂ − v₁)`, jusqu'à 1/64 de l'écart des vitesses, pas un arrondi). La référence visite les particules
en montant et retire par échange avec la dernière : l'ordre de visite se **reconstruit** exactement à partir de la liste triée des
absorbées (deux pointeurs), puis un fil applique les gestes dans cet ordre. (2) **Le retrait** d'un solde dû, (3) **la pose** d'un
solde reçu : séquentiels et rares (une ligne de faces-mailles) — un fil, dans l'ordre de la référence. Les soldes : **quanta
entiers** (une particule = 2²⁴), chargés par le transport aux faces de frontière. `n` **résident** sur la carte, lu par les noyaux ;
dispatch à la capacité (l'indirect est C7e) ; **compactage stable** des particules retirées. La séparation tenue côté bande (S400).

**Critères, écrits avant.** (1) Étages de la cuve mixte à l'arrondi jusqu'à l'advection, **soldes** après le transport au quantum
près de la référence (au plus un quantum par rangée d'écart d'arrondi). (2) **Un pas entier** sur la cuve mixte : mêmes nombres
d'absorbées, retirées, posées ; même `n` ; positions triées à 10⁻⁵ m ; soldes et `η` à l'arrondi ; volume total de la carte
**constant exactement** en quanta. (3) **Le raccord** (`apic3d_raccord`, S399–S407), 5 cm, 30 s : surface à **3 mm** de la
référence (pire colonne, `η` dans la zone, `φ` dans la bande), période et niveaux publiés, écart ≤ celui du témoin si la
reconstruction décroche ; volume exact ; coût de l'échange publié. (4) Zéro avertissement ; suite inchangée ; ballottement, B10 et
cuve tout en colonnes inchangés. **Arrêt** : un écart qui n'est pas d'arrondi se publie et s'explique avant P6.

### Plan

- [x] **P1** — jeton, plan seul.
- [x] **P2** — cœur : accès de banc aux soldes. Carte : `n` résident (noyaux par particule sur la capacité), soldes en quanta chargés
  par le transport aux faces de frontière ; banc : soldes après le transport ; tout l'existant inchangé.
- [x] **P3** — le compactage stable des particules marquées (préfixe par blocs) ; la séparation tenue côté bande ; banc.
- [x] **P4** — l'absorption : marques en parallèle, liste triée, ordre de visite reconstruit, gestes sur un fil ; banc : nombres.
- [x] **P5** — retrait et pose sur un fil ; le pas entier sur la cuve mixte ; critère 2.
- [x] **P6** — le raccord sur la carte, 30 s ; critère 3.
- [x] **P7** — suite entière ; preuve §10 ; liste, file, feuille de route, index.
- [ ] **P8** — rituel.

### Notes de reprise
- **P2** — cœur : `columns_soldes`, `particle_capacity`. Carte : `isolde` (soldes en quanta), `pcount` (`n` résident, compteurs),
  `plist` ; noyaux par particule lancés sur la capacité, `n` lu sur la carte ; le transport charge `solde −= vers la zone` par rangée
  de frontière, le même entier que le débit. Cuve mixte : **soldes à 58 quanta** au pire (5,4·10⁻¹¹ m³ sur 4·10⁻⁶, 44 faces-mailles
  non nulles) — critère 1 « au quantum près » **manqué tel qu'écrit, et mal posé** : le solde d'un pas est `u·dx²·dt`, et `u` diffère
  déjà de 2,5·10⁻⁶ m/s après la projection (écart admis), soit une borne de 1,3·10⁻¹⁰ m³ ; l'observé est dessous. Tout l'existant
  inchangé (ballottement au caractère près ; `η` 2,4·10⁻⁷ et 2,7·10⁻⁷ m).
- **P3** — marque `x.w ≠ 0` (le tri l'ignore) ; compactage stable (compte par groupe de 256, préfixe sur un fil, rangement par préfixe dans le groupe, recopie, `n`) ; séparation tenue côté bande (S400). Banc : une particule sur sept marquée — **ordre et valeurs identiques** au filtre attendu (10 971 et 5 541) ; l'existant inchangé.
- **P4** — **décision** : la carte garde ses tableaux **indice pour indice** avec la référence — les retraits se font par échange avec la dernière, sur un fil, comme `remove_particle` (un compactage stable changerait les indices, donc l'ordre de visite suivant, donc le mélange). `exchange_begin`, `absorb_mark` (liste par atomiques), `absorb_serial` (tri de la liste, visite à deux pointeurs, `absorb_one` : mélange aux faces de la zone, solde de la face la plus proche ou volume de la colonne, échange avec la dernière). Cuve mixte, un pas : **16 absorbées des deux côtés**.
- **P5** — `exchange_serial` (un fil) : chaque face-maille de frontière, dans l'ordre de la référence — retrait (la plus proche de
  la face, profondeur par profondeur), pose (à `dx/16` de la face, sous-réseau le plus libre, vitesse et `C` de la grille), puis
  les marquées retirées du plus grand indice au plus petit, par échange avec la dernière ; la réserve (S408) est nulle sans
  bascule. Cuve mixte, un pas après 2 à 90 pas de chauffe : **absorbées, retirées, posées, `n` identiques** à chaque fois
  (retraits 32 et 12, poses 8, 4, 4, absorptions 16, 20, 4), **positions à 6·10⁻⁸ m indice pour indice** (3·10⁻⁷ au pire),
  vitesses ≤ 1,4·10⁻⁵ m/s, soldes ≤ 1,1·10⁻¹⁰ m³, `η` ≤ 3·10⁻⁷ m. Critère 2 tenu (le volume exact : P6).
- **P6, 1 s** — `CAS=raccord --apic3d-carte-ballottement` ; `total_quanta` (particules × 2²⁴ + colonnes + soldes). 1 s : gestes
  identiques (399 absorbées, 97 retirées, 156 posées), **volume de la carte constant à 0 quantum**, surface à 0,001 mm. **L'ordre
  diverge au pas 12** — le premier pas de retraits — et l'ensemble diffère d'une particule à 25 mm : le retrait prend « la plus
  proche de la face », et sur le réseau d'ensemencement plusieurs particules en sont **à égale distance** ; un écart d'arrondi
  tranche l'égalité autrement. Choix discret entre candidates équivalentes, sans effet de masse. 30 s : au calcul.
- **P6** — 30 s, 1 500 pas : **volume de la carte constant à 0 quantum** (référence : 2·10⁻¹⁶ m³) ; gestes cumulés 5 204 / 5 256
  absorbées, 109 / 108 retirées, 5 303 / 5 356 posées, `n` 6 574 / 6 576 (carte / référence) ; période −0,036 % ; itérations 92,0 des
  deux. **Surface** (pire colonne ; `η` dans la zone, `φ` dans la bande), maximum courant : 0,34 mm (4 s), 2,22 (10 s), 2,70 (14 s),
  3,23 (22 s), **4,41 mm (30 s)** — critère 3 (3 mm) manqué. **Témoins** (la référence contre elle-même, vitesses initiales
  ±ε, `TEMOIN=`) : ε = 10⁻⁶ — 0,39, 2,23, 2,65, 3,11, **3,40 mm**, période −0,043 % ; ε = 10⁻⁴ — 2,01 (4 s), 2,89, 2,89, 3,98,
  **4,03 mm**, période −0,063 %. **La carte suit la courbe des témoins** et finit 10 % au-dessus du plus grand des deux : la
  comparaison « ≤ témoin », sur deux échantillons, est manquée de peu, dans la même dispersion. Coût p99 2,84 ms, dont séparation +
  échange 0,88 (les deux fils : ≈ 0,7 ms).
- **P7** — non-régression : B10 au caractère près (pas 54, 9,195 mm), tout en colonnes identique (0,003 mm, volume 0) ; suite **753**, zéro avertissement. Preuve §10 ; liste 4.19 (partiel), file, feuille de route, index. `--check` : 0.
