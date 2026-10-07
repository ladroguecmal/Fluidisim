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

Session : S624 — **terminée**. En autonomie (ADR-247 : la physique des partiels). **11.3 — la chaîne entière** : le tsunami de S614
(`H/d` = 0,0185, d'amplitude finie) entre par le bord caractéristique de S622 et remonte la plage — S622 n'avait éprouvé le bord qu'à 2 mm.

**Ce que la session fait.** Aucun code nouveau dans le cœur : un essai qui assemble `SaintVenant2D` d'ordre deux (S620), `pas_avec_bord`
(S622) et l'onde solitaire de `grand_evenement` (S614) ; le domaine commence au bord, à 500 m du pied de la plage 1:19,85, au repos ;
l'onde entre de 500 m au large. Contre le même domaine prolongé de 1 100 m au large, l'onde posée dedans. Ne fait pas : le niveau lu dans
le tsunami macroscopique lui-même (sa forme est une impulsion polynomiale, la loi de Synolakis veut une onde solitaire), le déferlement.

**Références, calculées avant** (`s624_ref.py`, numpy ; 170 s — une première mesure à 130 s arrêtait l'onde avant le haut de sa course).
Remontée **forcée**, maille 1, ½, ¼ m : **0.806045340, 0.869017632, 0.900503778 m** ; **étendue** : 0.856423174, 0.919395466, 0.950881612 m. L'écart
étendu − forcé vaut **0.050377834 m aux trois mailles** : il ne dépend pas de la maille — c'est le raidissement de l'onde, que Saint-Venant
non dispersif accumule sur les 500 m de plus de l'étendu (une onde solitaire réelle, dispersive, garde sa forme), non un artefact du bord.
Synolakis : 0,861419 m ; le forcé à ¼ m en est à 0.0391 m.

**Quantum** : la cote d'une maille, `dx/cot β` (0.012594 m à ¼ m). **Critères, écrits avant.** (1) les trois remontées forcées et
les deux étendues à 1 et ½ m égales aux références (au bit : des cotes de mailles) ; (2) l'écart étendu − forcé le même aux mailles 1 et ½ m
(à 10⁻⁹ m) ; (3) le forcé croissant avec la maille et, à ¼ m, à moins de dix quanta de Synolakis ; (4) `h ≥ 0`.

### Plan

- [x] **P1** — jeton ; plan.
- [x] **P2** — l'essai ; (1)–(4).
- [x] **P3** — preuve ; liste 11.3 ; rituel.

### Notes de reprise
- **P2 fini** — (1)–(4) tenus du premier essai, au bit. Suite : 811 essais listés.
- **P3** — preuve CHAINE-TSUNAMI-S624 ; ligne 11.3 ; index ; journal.
