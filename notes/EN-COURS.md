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

Session : S417 — **terminée**. Demande de l'utilisateur (2026-10-01) : *« Continue »* — la suite déclarée : **C7b puis C7c**
([conception](../docs/validation/APIC-CARTE-S416.md) §1). Agent : Claude Code (Opus 5.5), au poste ; RTX 5070 Laptop.

**Ce que la session fait.** **C7b** : le corps cinématique de S393 sur la carte — mailles solides, faces imposées, images radiales
dans la reconstruction, particules repoussées — puis **B10 nu** (APIC seul, sans bande) sur la carte contre la référence. Ensuite
la **conception de C7c** (la zone des colonnes et le fond : ≈ 1 800 lignes du cœur, découpées), et son premier morceau si le temps
le permet.

**Critères, écrits avant.** (1) Les étages avec le corps, sur un état de B10 chauffé : étiquettes identiques (solides compris),
`φ` à 10⁻⁵ m, vitesses de grille à 10⁻⁴ m/s après projection et après imposition, positions à 10⁻⁵ m après le corps. (2) **B10
nu**, Fr = 2, D/dx = 8, quart de domaine : pincement **au même pas** que la référence (ou à un pas, publié), `φ` à **3 mm** de la
référence dans la bande de l'interface (|φ| < dx) jusqu'au pincement, air enfermé et cavité au pincement publiés des deux côtés.
(3) Coût par étage publié. (4) Zéro avertissement ; suite inchangée. **Arrêt** : un écart au-delà de l'arrondi se publie et
s'explique avant C7c.

### Plan

- [x] **P1** — jeton, plan seul.
- [x] **P2** — la carte : le corps (paramètres, étiquettes solides, faces imposées, images radiales, particules repoussées) ; banc des étages sur B10 ; critère 1.
- [x] **P3** — B10 nu sur la carte contre la référence : pincement, air enfermé, cavité, `φ` à l'interface, coût ; critères 2 et 3.
- [x] **P4** — conception de C7c : lecture de `apic3d_columns.rs`, découpage, critères (preuve §8).
- [x] **P5** — *ajoutée après P4, déclarée avant d'y toucher* : **C7c-1**, la zone sans échange. Tampons de la zone (masque,
  `η` et son reste, table de lecture de S400) ; `columns_label` et les particules virtuelles dans la reconstruction ; banc des
  étages sur une cuve **mixte** (moitié colonnes, moitié particules : jusqu'à l'advection, l'échange n'intervient pas).
- [x] **P6** — `columns_begin` et `columns_advect` (la vitesse du pas précédent au pied de la caractéristique) ; banc.
- [x] **P7** — `columns_transport` : débits mouillés par face, une fois, appliqués aux deux colonnes avec des signes opposés ;
  `η` avancé en double flottant (`η` + reste, comme la référence) ; banc.
- [x] **P8** — la cuve **tout en colonnes** (`APIC3D_COLONNES`, S398) : ballottement à 3 mm de la référence sur 10 s ; volume.
- [x] **P9** — suite entière, zéro avertissement ; preuve ; liste, file, feuille de route, index.
- [x] **P10** — rituel.

**Critères de C7c-1, écrits avant** (preuve §8.2) : étages à l'arrondi sur la cuve mixte — `φ` à 10⁻⁵ m et étiquettes identiques,
vitesses advectées à 10⁻⁵ m/s, `η` à 10⁻⁶ m après transport ; la cuve tout en colonnes : surface à **3 mm** sur 10 s, période à
0,5 %, dérive de volume publiée (la conservation se fait en double flottant, pas au bit : la masse exacte vient avec l'échange,
C7c-2, en entiers).

### Notes de reprise
- **P2** — le corps sur la carte : `Params` à 160 octets (centre, vitesse, centre avancé), étiquettes solides et image radiale dans `reconstruct`, `impose_body` (après gravité et après extrapolation), `move_body` (fin du pas) ; `B10` (Fr, D/dx, quart) dans le banc ; `CAS=b10`. **B10, 15 pas de chauffe** (131 072 particules, corps à −3,96 m/s) : transfert 4,8·10⁻⁶ m/s (max 5,9), `φ` 4,7·10⁻⁶ m, étiquettes identiques (67 solides), **211 itérations des deux côtés**, vitesses 5,2·10⁻⁶ m/s, faces non nulles 49 248 contre 49 246 (deux faces à zéro d'un côté, dans l'écart), positions **2,4·10⁻⁷ m** après le corps. Ballottement inchangé. Critère 1 tenu. Le banc re-chauffe la référence à chaque étage : 2 min.
- **P3** — `--apic3d-carte-b10` (FR, ND, ITERATIONS, TEMOIN) : Fr 2, D/dx 8, 131 072 particules, 71 pas. **Pincement
  identique au chiffre près** (pas 54, t = 2,1308 √(R/g), profondeur 1,312 D, air 0,0781 D³, cavité 1,937 D, couronne 0,205 D),
  itérations 207,4 / 207,5, aucune non convergée ; coût p99 6,42 ms, 49 ns/particule (projection 4,18 au plafond 600). **Mais**
  l'écart de `φ` dans la bande |φ| < dx monte à 0,005 mm (pas 20), 0,35 (pas 40), **9,2 mm** à t = 1,454 √(D/g), juste avant le
  pincement : critère 2 (3 mm) manqué sur `φ`. Hypothèse, à éprouver avant de conclure : `φ` est discontinu là où le noyau ne voit
  presque plus de particule (x̄ porté par une seule, ou `φ = dx` sans voisine) — au col de la cavité. Témoin : une seconde
  référence aux vitesses initiales perturbées (`TEMOIN=1e-6`, `1e-4`), au calcul.
- **P3** — série par pas (déterministe : 9,195 deux fois) : ≤ 0,32 mm jusqu'au pas 44, puis 1,08 (48), 1,70 (49), 9,20 (52), 2,58,
  0,66 (54). **Témoins** : référence perturbée de ±10⁻⁶ m/s — **22,59 mm** (pas 49), 8,77 (52) ; ±10⁻⁴ — 21,22 (49), 7,87 (52) ;
  pincement identique partout. Critère 2 manqué tel qu'écrit, et intenable : la référence ne le tient pas contre elle-même ;
  remplacé pour C7c par « écart de la carte ≤ celui du témoin à 10⁻⁶, pas à pas ». Preuve §7.
- **P5** — cœur : `columns_state` (masque, reste de `η`, table S400, bande). Carte : tampons `cols` (`η`, reste, table, débits) et `cmask`, `load` charge aussi les faces du pas précédent et la zone ; `reconstruct` : maille de colonne `φ = z − η lu`, particules virtuelles (`round` en `floor(x + 0,5)` : WGSL arrondit au pair). Banc `CAS=raccord` (`raccord_state`) : transfert 4,5·10⁻⁸ m/s, **`φ` 5,9·10⁻⁷ m, étiquettes identiques** ; projection à 0,11 m/s — attendu, l'advection de la zone manque (P6).
- **P6** — `columns_begin` (vitesse du début du pas dans la copie des faces), `columns_advect` (une face `u`/`v` est de la zone si l'une de ses colonnes l'est — S406 ; `w` si sa colonne l'est) ; cuve mixte : **advection 7,8·10⁻⁸ m/s**, projection 94/94 itérations, vitesses 2,5·10⁻⁶ m/s, positions 1,2·10⁻⁷ m après advection. L'étage final diffère (0,13 m) : l'échange et la séparation tenue côté bande — C7c-2, attendu.
- **P7** — `columns_flux` (un débit par face de colonnes, rangée par rangée, arrondi au quantum `dx³/8·2⁻²⁴`) et `columns_update` (le volume de la colonne en entiers sur deux mots, `η` relu en flottant) — **les volumes en entiers dès C7c-1** : FXC ne garantit pas le `mad` fusionné qu'exige un double flottant sans erreur ; en entiers, la conservation est exacte par construction. `η` à **2,4·10⁻⁷ m** (mixte) et **2,7·10⁻⁷ m** (tout en colonnes, 88/88 itérations). Critère d'étage tenu.
- **P8** — `CAS=colonnes --apic3d-carte-ballottement` : 500 pas, **surface (`η`) à 0,003 mm** de la référence, période 1,9768 s des deux côtés (10 passages), itérations 89,1/89,1 ; **volume de la carte constant exactement** (dérive 0 en quanta ; la référence −3,3·10⁻¹⁶ m³) ; coût p99 1,46 ms, dont projection 1,39. Ballottement en particules inchangé (0,456 mm, +0,003 %) ; sa surface reconstruite mesurée 0,68 ms ce passage (0,41 avant) — variance de la carte, à surveiller en C7e.
- **P9** — suite du cœur **753 réussis**, 19 ignorés, zéro avertissement (cœur et afficheur). Preuve §7 (C7b), §8 (conception de C7c), §9 (C7c-1) ; liste 4.19 (partiel, inchangé), file, feuille de route, index. `--check` : 0.
- **P10** — journal ; jeton libre ; maillons 5 (justifiés : S406) ; suivant : S418, C7c-2.
