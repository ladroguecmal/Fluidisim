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

Session : S644 — **terminée**. En autonomie ; la campagne du rouleau 3D (acceptée le 2026-10-07). **Étape 2 — une vague non déferlante qui
monte la pente dans APIC 3D**, sur le fond en escalier de S639 (au rivage, 1,5 cm/s de frémissement ; les faces coupées de S640 y sont pires).

**Ce que la session fait.** Une onde solitaire posée dans APIC 3D (l'élévation `H·sech²(γ(x − x₁))`, la vitesse `c·η/(d + η)` sur toute
la colonne), sur un canal de largeur quatre mailles : fond plat à 0.05 m puis pente 1:3 depuis le pied ; la remontée lue comme
la plus haute marche dont la première maille au-dessus est d'eau (`labels`), la rangée médiane, comme `Plage::cote_mouillee` de S614.
Jugée contre la loi de Synolakis (1987) et contre Saint-Venant 2D (S613, ordre deux de S620) sur la même plage, à la même maille.

**Références, calculées avant** (ce script). `d` = 0.35 m, `H` = 0.07 m (`H/d` = 0.2), pente 1:3 : **R = 0.2295 m** (Synolakis) ;
`γ` = 1.1066 m⁻¹ ; l'onde à la distance canonique `arccosh(√20)/γ` = 1.968 m du pied, centrée à 2.8 m (pied à 4.768 m) ;
`c` = 2.030 m/s. **Non déferlante** : `H/d` < 0,818·cot^(−10/9) = 0.241 (asserté). **Bornes** (ADR-257 D1, assertées) : la queue
de l'onde au mur gauche < 1 % de `H` (5.7e-04) ; la remontée sous le couvercle et dans le domaine (6,6 m × 0,8 m) ; aucune égalité
entre un centre de maille et le fond, aux deux mailles.

**Quantum** : la marche, `dx/3` en hauteur (0,83 cm à 2,5 cm). **Critères, écrits avant.** (1) les particules gardées, aucune sous le fond ;
(2) **à 2,5 cm, la remontée d'APIC à 20 % de Synolakis**, et à 20 % de Saint-Venant 2D à la même maille ; (3) les deux mailles (5 et
2,5 cm) rapportées — l'écart attribué seulement après elles (ADR-256 D2) ; un écart à la maille fine qui ne baisse pas se localise par un
témoin qui supprime une cause (ADR-259 D1) ; (4) le rouleau (étape 3) n'est pas demandé : la vague ne doit pas déferler — vérifié à l'œil
sur la surface (aucune particule détachée au-dessus du front de plus d'une maille).

### Plan

- [x] **P1** — jeton ; plan.
- [x] **P2** — l'essai (5 cm dans la suite, 2,5 cm `#[ignore]`) ; (1)–(4).
- [ ] **P3** — preuve ; liste 4.14 ; rituel.

### Notes de reprise
- **P2 fini** — (1) tenu ; **(2) manqué** : par les étiquettes 0,150 m aux deux mailles (le plan se trompait : la marche fait une maille, la
  lecture ne voit pas le film) ; par les particules 0,188/0,187 m — 82 % de Synolakis, 78 % de Saint-Venant à 2,5 cm ; (3) sans
  convergence ; (4) tenu. Témoins : la crête intacte au pied ; le fond lisse, 0,190 à 2,5 cm (les contremarches ne sont pas la cause) ;
  Saint-Venant sur l'escalier, non freiné. A333 ouverte. L'utilisateur : « Continue en autonomie jusqu'à la v2 ou vers v2 » (consigné).
