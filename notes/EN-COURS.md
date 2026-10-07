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

Session : S613 — **terminée**. En autonomie (ADR-247) : **4.14 — la plage : rouleau 3D, mouillage et séchage (C04)** (absent : C04 ne
tournait que sur un véhicule 1D, la rupture de barrage sur lit sec). Cette session fait **le mouillage et le séchage en 2D**, jugés sur la
solution analytique de Thacker (une nappe plane qui tourne dans une cuvette paraboloïde ; le rivage se déplace).

**Ce que la session fait.** Un module `saint_venant_2d.rs` : Saint-Venant 2D en volumes finis d'ordre un, **reconstruction hydrostatique**
d'Audusse (positive, équilibrée), flux de Rusanov, murs aux bords, la vitesse **désingularisée à la Kurganov–Petrova** (`u = √2·h·q/√(h⁴ + max(h⁴, ε))`, `ε` = (1 mm)⁴) ; `pas(dt)` refuse un nombre de Courant
au-delà de ½. Ne fait pas : le rouleau 3D, l'ordre deux, le frottement, la houle incidente sur une plage réelle, le branchement à δ.

**Références, calculées avant** (`s613_ref.py`, numpy indépendant). Thacker (SWASHES) : `a` = 1 m, `h₀` = 0,1 m, domaine de 4 m, `η` = 0,5 ;
période `T` = 4,485701 s ; `10·N` pas par période. Écart L1 de `h` après une période : **0.427088494** (50²), **0.242184670** (100²),
**0.128641914** (200²) — rapports 1.7635 et 1.8826 : **l'ordre un, qui converge**. Masse : variation sous 10⁻¹³ aux trois ; `h` jamais
négatif. Le centre de masse à 100² : T·250/1000 : (2.017083014, 2.475469549) ; T·500/1000 : (1.552226477, 2.058725469) ; T·1000/1000 : (2.395102837, 1.902849265) (l'exact : (2 ; 2,5), (1,5 ; 2), (2,5 ; 2) — l'oscillation amortie par la diffusion de l'ordre
un). **Le lac au repos** (cote −0,05 m, bords secs), 500 pas : vitesse max **1.852e-16 m/s**.

**Amendement, avant la mesure du code** (ADR-244 D1) : la première référence (vitesse `q/h` au-dessus de 10⁻⁶ m, nulle
sous) atteignait **Courant 0,60** dans une maille presque sèche (`h` = 1,2·10⁻⁶ m) — le refus du critère (5) l'aurait arrêtée à 200² ; la
vitesse désingularisée, Courant ne dépasse pas **0.1896** (asserté), et les références ci-dessus sont les nouvelles.

**Quantum** : f64. **Critères, écrits avant.** (1) à 100², l'écart L1 et les trois centres de masse égaux aux références à 10⁻⁹ ; (2) les écarts
L1 à 50² et 200² à 10⁻⁹ ; le rapport 100 → 200 au moins 1,8 ; (3) la masse à 10⁻¹³ relatif, `h ≥ 0` à chaque pas, aux trois résolutions ;
(4) le lac au repos immobile sous 10⁻¹⁴ m/s ; (5) refus : moins de deux mailles, `dx` ou `dt` non positifs, un pas au-delà de Courant ½.

### Plan

- [x] **P1** — jeton ; plan.
- [x] **P2** — `saint_venant_2d.rs` et ses essais ; (1)–(5).
- [x] **P3** — preuve ; liste 4.14 ; rituel.

### Notes de reprise
- **P2 fini** — (1)–(5) tenus, à 10⁻¹⁵ de numpy, après l'amendement (vitesse désingularisée). Suite : 802 essais listés.
- **P3** — preuve THACKER-S613 ; liste 4.14 (absent → partiel) et décompte ; index ; journal.
