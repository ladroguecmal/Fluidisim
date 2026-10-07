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

Session : S680 — **en cours**. En autonomie vers la v2 ; K3, 4.14 ; le relais au rivage
([conception](../docs/registres/RELAIS-RIVAGE-S679.md), ADR-271).

**L'interface, précisée.** La conception prévoyait un flux HLL calculé à part et imposé aux deux côtés. Plus simple et aussi exact :

- Saint-Venant 2D garde son bord caractéristique (S622, S628), nourri par l'état de la dernière colonne 3D ;
- il **rend le flux de masse qu'il a réellement fait passer** pendant le pas, par rangée : la moyenne de ses deux étages de Heun,
  exactement ce qui change son volume ;
- APIC retire ou pose des particules pour ce même volume, et un réservoir garde le reste d'un quantum de particule.

**Ce que la session fait.** La première brique : `SaintVenant2D` garde, à chaque pas forcé, le flux de masse de ses bords gauche et
droit par rangée (`flux_des_bords`).

**Contrôles du plan** (ADR-266, ADR-267, ADR-268)

- **témoin** : les essais de S613–S628, aux mêmes sorties (le relevé ne touche pas l'arithmétique).
- **instrument** : le bilan de volume. Ce qui départagerait : un flux relevé juste rend
  `ΔV = dt·dx·Σ_j flux_j`, pas après pas, à l'arrondi près (10⁻¹² en relatif du volume) ; un étage de Heun oublié, ou le flux d'un seul
  étage, s'en écarte de l'ordre de la variation du flux pendant le pas.
- **calcul** : aucun nombre nouveau ; le plancher, l'arrondi `f64` d'une somme de quelques milliers de mailles (≈ 10⁻¹³ en relatif).
- **ADR** : ADR-271.
- **pièges** :
  - Heun : `U ← U + ½·k·(L(U) + L(U¹))`, donc le flux du pas est la demi-somme des deux étages ;
  - le signe : entrant positif à gauche, sortant positif à droite ;
  - le frottement après le pas ne change pas le volume.

**Critères, écrits avant.**

1. Une houle entrée par le bord gauche (S622) et un bord droit forcé : à chaque pas, `ΔV` égal à `dt·dx·Σ(flux gauche − flux droit)` à
   10⁻¹² près en relatif.
2. Les essais de S613–S628 aux mêmes sorties.

### Plan

- [x] **P1** — jeton ; plan.
- [ ] **P2** — `flux_des_bords` ; l'essai ; (1)–(2).
- [ ] **P3** — preuve ; la conception, note ; rituel.

### Notes de reprise
