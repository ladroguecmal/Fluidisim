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

Session : S529 — **terminée**. En autonomie, **A330 localisée avant tout remède** (ADR-226 D1) : le sillage de la coque dans δ est 3,5 fois
moins ample que la théorie d'une pression `ρ g d` sur son empreinte (S520). Deux causes possibles : δ sous-résolu près de la coque
(A317 : ± 43–49 % à 25 cm) ou un modèle de référence inadapté (une pression n'est pas un corps qui perce la surface). Une convergence en
maille les sépare.

**Ce que la session fait.** Le banc du sillage (`--lineaire-sillage`) reçoit `SILLAGE_DX`, `SILLAGE_NZ`, `DT_US` ; trois mailles — 50, 25,
12,5 cm — sur 56 × 40 × 3 m (53 760 / 430 080 / 3 440 640 mailles : sous la limite de 4,19 M des noyaux de mailles), 15 s à 3 m/s, le pas
de temps proportionnel à la maille (20 / 10 / 5 ms). La surface exportée ; `reference_sillage.py convergence` : les écarts entre mailles
sur un réseau commun (tous les 50 cm, de 4 à 22 m derrière la coque, `|y|` ≤ 12 m), et le profil des rayons de 10 à 20 m.

**Ordre de grandeur, calculé.** λ₀ = 5,76 m (11,5 / 23 / 46 mailles par longueur d'onde) ; `kh` = 3,3 par 3 m de fond (profond) ; la coque
au bout de 15 s à 46,6 m, l'éponge à 53 m ; les rayons à 35° et 24 m passent à 13,8 m de l'axe (l'éponge à 17 m).

**Critères, écrits avant.** (1) **La convergence** : `‖η₂₅ − η₁₂,₅‖ < ‖η₅₀ − η₂₅‖`, d'ordre observé `log₂` du rapport ≥ 0,5. (2) **L'attribution,
règle déclarée avant** : si le maximum du profil des rayons à 12,5 cm dépasse celui à 25 cm de plus de 30 %, A330 revient à la résolution de
δ (A317) ; à moins de 10 %, δ est convergé et l'écart à la théorie revient au modèle de référence ; entre les deux, non tranché.

### Plan

- [x] **P1** — jeton, plan seul.
- [x] **P2** — les paramètres du banc, trois calculs ; (1), (2).
- [x] **P3** — preuve ; A330 ; rituel.

### Notes de reprise
- **P2 fini** — 12,5 cm d'abord refusé (une liaison de 230 Mo > 128 Mo) → la carte demande les limites de l'adaptateur ; S518 au bit.
  rms 0,0504 / 0,0617 / 0,0612 m ; maximum du profil 0,091 / 0,102 / 0,093 ; écarts 0,064 puis 0,073 (ordre −0,18) → **(1) manqué**,
  **(2) : le modèle**. Pic d'étrave 0,30 / 0,72 / 1,47 m : divergent.
- **P3** — preuve A330-CONVERGENCE-S529 ; notes A330, A317 ; liste 4.13 ; index ; journal.

