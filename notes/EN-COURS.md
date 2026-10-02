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

Session : S441 — **terminée**. Demande de l'utilisateur (2026-10-02) : *« Continue »* — la suite déclarée : **A322**, les bouffées du
mode relatif à 10 cm ([preuve](../docs/validation/MULTIGRILLE-3D-S385.md) §7).

**Ce que la session trouve en entrant.** Scène de revue à 10 cm (80 × 80 × 70, repos 3,5 m — la face `k` = 35), 30 Hz, mg 8 : en mode
relatif, elle tient 120 s ; mais la part de l'échelle de la maille de `w` dépasse 0,05 pendant 65 s (0,62 au plus), sur des faces `w`
entre `k` = 38 et 46, au-dessus du repos — sous les crêtes de la mer. Pas de référence CPU pour cette scène (448 000 mailles).

**Ce que la session fait.** D'abord quatre discriminants, sur la carte, en mode relatif, 120 s : (a) **60 Hz** (`PAS_US=16667`) — une
limite de pas ? (b) **sans le paquet** (`PAQUET=0`) — la mer seule ? (c) **24 cycles** de projection — une projection mal convergée ?
(d) **la scène à 25 cm** (sans `MAILLE`), où la surface franchit aussi des centres — propre à 10 cm ? Puis, selon eux, un instrument
sur la face d'une bouffée (étage du pas, fantômes, mouillure), et la référence CPU sur une fenêtre réduite si la question le demande.

**Critères, écrits avant.** La localisation est **faite** quand une variante éteint les bouffées (part de maille de `w` sous 0,05 sur
120 s) et qu'un instrument les attache à un terme ou un étage du pas. **A322 levée** si un remède fait tenir la scène relative à 10 cm
et 30 Hz sur 120 s, part de maille sous 0,05, sans changer la production (sans `RELATIF`) au bit. Autrement, ce qui est écarté s'écrit.

### Plan

- [x] **P1** — jeton, plan seul.
- [x] **P2** — les quatre discriminants.
- [x] **P3** — l'instrument, selon eux ; un remède s'il se montre.
- [x] **P4** — preuve ; A322 ; registres ; rituel.

### Notes de reprise
- **P2** — mode relatif, 120 s, part de maille de `w` (secondes au-dessus de 0,05 · maximum) : témoin S440 (10 cm, 30 Hz, mg 8)
  **65 · 0,62** ; (a) **60 Hz** : **3 · 0,15** (aucune au-dessus de 0,2) ; (b) **sans le paquet** : δ **nul au bit** sur 120 s — la scène
  entière tient le point fixe ; (c) **24 cycles** : 65 · 0,61 (divergence 1,5·10⁻⁶) — pas la projection ; (d) **25 cm** : 0 · 0,022.
  **Les bouffées viennent de la dynamique de δ au pas long, à 10 cm** — une limite de pas, ni la projection, ni le point fixe.
  Pour attribuer le terme : les commutateurs de banc de S391 en mode relatif (`relative_bench`, compilés quand `COMMUTATEURS` est
  posé ; `extra_switched` retire le résidu en mode relatif).
- **P3** — attribution en mode relatif (commutateurs de S391, 10 cm, 30 Hz, 120 s ; secondes > 0,05 · maximum) : rien d'éteint
  (pipelines de banc) 65 · 0,61 ; sans `u′·∇u′` 66 ; sans `U·∇u′` 36 ; sans `u′·∇U` 70 ; sans le terme d'ADR-209 : explose à 7 s ;
  **sans la bande relative (16) : 0 · 0,033** — **la localisation est faite** : la bande relative transporte `η′` à la vitesse de B,
  hauteur de face centrée, pas explicite — **FTCS**, croissance `C²/2` par pas (`C = U·dt/dx` ≈ 0,3 à 10 cm et 30 Hz ; deux à trois
  fois moins à 60 Hz ou à 25 cm). **Remède** : `Volume3::set_relative_band_lax_wendroff` (référence) et `Step3::set_relative_band_lax_wendroff`
  (carte, `override BAND_LW`), éteints par défaut — la perturbation de face sous Lax-Wendroff ; à δ nul, au bit la même ; essai
  `zero_delta_stays_zero_with_the_lax_wendroff_band_s441`. **Mesures** : la scène à 10 cm, 30 Hz : tient 120 s, **10 s sur 120** au-dessus
  de 0,05 (0,115 au plus) — mais δ y est presque nul (1 à 2 mm, `max_u` 3 à 5 cm/s, contre 0,6 m/s dans les bouffées d'avant) : le
  critère, une **part** relative, fluctue quand le champ s'éteint ; **manqué tel qu'écrit**. La production **au bit** ; le témoin relatif
  sous Lax-Wendroff **nul au bit** (carte et référence) ; la trajectoire relative suit sa référence à **1,4·10⁻⁵ m** sur 400 pas (sans
  Lax-Wendroff : 1,5·10⁻⁴, l'horizon du millimètre au pas 260 ; avec : jamais). Suite **759**, zéro avertissement.
- **P4** — MULTIGRILLE-3D-S385 §8 ; A322 annotée ; registres ; journal ; jeton libre ; maillons 29 (justifiés : S406) ; suivant : S442, A322 — le critère en amplitude absolue.
