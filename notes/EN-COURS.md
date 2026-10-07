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

Session : S598 — **terminée**. En autonomie (ADR-247), **4.4 — la profondeur adaptative, un domaine qui suit un objet qui coule** (absent).
Première pièce : **le plan**, pas encore l'exécution dans δ.

**Ce que la session fait.** `coule.rs` : (a) **la descente prévue** d'une sphère plus dense que l'eau — la masse ajoutée `½ρ_w·V`, la
traînée de Schiller–Naumann jusqu'à `Re` = 1 000 puis de Newton (`C_d` = 0,44) —, RK4 à pas fixe, f64 ; (b) **l'enveloppe verticale** du
domaine : son bas reste sous l'objet **prévu `τ_a` plus tard** (le temps que δ grandisse), avec une marge, arrondi au quantum vers le bas,
**jamais remonté** pendant la chute ; chaque descente du bas est un agrandissement (un changement de niveau, ADR-210), compté. Ne fait
pas : l'exécution dans δ (le transfert d'état vers le domaine agrandi), l'objet qui remonte, les objets non sphériques.

**Références, calculées avant par ce script, avec son propre code.** Une boule d'acier de 0,1 m de rayon (7 800 kg/m³) dans l'eau de mer :
la vitesse terminale de Newton `√(8·r·(ρ_s − ρ_w)·g/(3·ρ_w·C_d))` = **6.268812 m/s** (`Re` = 1.29e+06) ; la descente depuis le repos (RK4,
dt = 10⁻⁴ s) : à 1, 2, 5, 10 s, **z = 3.23120 ; 9.16062 ; 27.93725 ; 59.28130 m**, `v` = 6.26881 m/s à 10 s.
L'enveloppe : anticipation 2 s, marge 0,5 m, quantum 1 m, un domaine initial de 2 m.

**Quantum** (ADR-236, ADR-249) : f64 ; le pas du code (10⁻³ s) contre 10⁻⁴ s — sous 10⁻⁶ relatif. **Critères, écrits avant.** (1) la vitesse
terminale à 10⁻⁶ relatif ; (2) la profondeur aux quatre instants à 10⁻⁴ relatif ; (3) l'enveloppe, à chaque pas de 10 s : l'objet (son
centre ± son rayon) dedans avec au moins la marge, le bas jamais remonté, multiple du quantum ; les agrandissements comptés et leur nombre
égal à celui que le script compte avec sa propre descente ; (4) refus : rayon, densité non positifs, une sphère moins dense que l'eau.

### Plan

- [x] **P1** — jeton ; plan.
- [x] **P2** — `coule.rs` et ses essais ; (1)–(4).
- [x] **P3** — preuve ; liste 4.4 ; rituel.

### Notes de reprise
- **Avant l'essai, un manque du plan comblé** (ADR-244 D1) : le plan annonçait le nombre d'agrandissements « compté par le script », sans
  l'avoir compté. Compté maintenant avec la descente du script (RK4 à 10⁻⁴ s) : l'enveloppe mise à jour toutes les 0,1 s de 0 à 10 s, le bas
  `⌈(z(t + 2 s) + r + 0,5)/1⌉·1` s'il descend, depuis 2 m : **64 agrandissements**, le bas final à **73 m**.
- **P2 fini** — terminale, descente, enveloppe (64, 73 m) tenues ; refus. Constat : 64 agrandissements en 10 s, trop pour δ. Suite 773.
- **P3** — preuve COULE-S598 ; liste 4.4 (absent → partiel) et décompte ; index ; journal.

