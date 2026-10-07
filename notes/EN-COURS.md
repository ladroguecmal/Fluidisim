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

Session : S595 — **en cours**. En autonomie (ADR-247), **7.2 — spray, embruns, gouttelettes** (absent). Première pièce : **la goutte**.

**Ce que la session fait.** `goutte.rs` : une goutte sphérique de rayon `r` dans l'air au repos — la poussée d'Archimède comprise, la
traînée d'une sphère rigide par Schiller–Naumann, `C_d = 24/Re·(1 + 0,15·Re^0,687)` (de Stokes au régime quadratique) ; la vitesse
terminale (bissection sur l'équilibre) ; le vol, RK4 à pas fixe en f64 (déterministe), jusqu'au retour à la surface (l'instant et le point
de chute, interpolés). Ne fait pas : l'émission (le déferlement, la gerbe — d'où naissent les gouttes), le vent, l'évaporation, la
déformation des grosses gouttes (au-delà de ~1 mm, `C_d` d'une sphère rigide surestime la vitesse terminale), le rendu.

**Références, calculées avant par ce script, avec son propre code** (air 1,2 kg/m³, μ = 1,8·10⁻⁵ Pa·s). Vitesse terminale : `r` = 10 µm,
**0.011992 m/s** (Stokes pur : 0.012097) ; `r` = 1 mm, **6.9556 m/s**. Une goutte de 0,5 mm lancée à 10 m/s à 45° : retombe en
**0.90367 s** à **2.3030 m** (dans le vide : 10.1937 m).

**Quantum** (ADR-236 D1, ADR-249 D1) : f64 ; la bissection à 10⁻¹² ; le pas du RK4 du code (10⁻⁴ s) contre celui du script (10⁻⁵ s) —
erreur estimée sous 10⁻⁶ relatif. **Critères, écrits avant.** (1) les vitesses terminales à 10⁻³ relatif ; (2) le temps de vol et la portée
à 10⁻³ relatif ; (3) une goutte très lourde (la masse volumique de l'air nulle) retrouve la portée du vide `v²/g` à 10⁻⁶ ; (4) refus :
rayon, vitesse ou pas non positifs.

### Plan

- [x] **P1** — jeton ; plan.
- [ ] **P2** — `goutte.rs` et ses essais ; (1)–(4).
- [ ] **P3** — preuve ; liste 7.2 ; rituel.

### Notes de reprise
