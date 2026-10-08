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

Session : S697 — **en cours**. En autonomie, sans arrêt. S695 : le raccord du large fait se retourner la vague trop tôt. Il est jugé seul ici
(ADR-273 D1, ADR-276 D2 : une seule cause à la fois).

**Ce que la session fait.** Le même montage que S693–S695 (le repère de S650 : l'eau à 0,5 m, le fond plat à z = 0, l'escalier, l'air
balistique, le relais au rivage), **sans raccord au large** : APIC 3D depuis 0 m, un mur à gauche (`x_r = 0`). On a ainsi trois
différences, chacune d'une seule cause :

| écart | la cause qui seule change |
|---|---|
| sans raccord (0 m) − S690 (2,624 s) | le repère (0,5 m et z = 0, contre 0,55 m et 0,05 m) |
| raccord à 1,0 m (2,582 s) − sans raccord | la zone de colonnes derrière l'onde, que l'onde ne traverse pas |
| raccord à 5,0 m (2,524 s) − raccord à 1,0 m | la part de l'onde qui traverse le raccord |

**Contrôles du plan** (ADR-266, ADR-267, ADR-268, ADR-273, ADR-276)

- **témoin** : le montage sans raccord, construit par la même fonction (`deux_raccords_porteur`, ADR-276 D1), qui ne change que la
  présence du raccord.
- **instrument** : le premier retournement. Ce que rendrait chaque hypothèse :
  - si le repère est neutre, ≈ 2,624 s (S690), et l'écart de 0,04 s est la zone de colonnes ;
  - si le repère compte, ≈ 2,58 s, et la zone de colonnes est neutre ;
  - entre les deux, chacune pour sa part.
- **calcul** : le coût, ≈ 14 min (≈ 237 000 particules, comme S690 : ADR-274 D1).
- **ADR** : ADR-273, ADR-276.
- **pièges** :
  - sans raccord, ni zone de colonnes, ni bord ouvert à gauche, ni volume entré ;
  - la même onde (x₁ = 3,4 m) ;
  - le même `lz` (1,0 m).

**Critères, écrits avant.**

1. Les trois écarts mesurés, et chacun attribué à sa seule cause.
2. La masse à 10⁻¹² près, la dette sous un quantum.

### Plan

- [x] **P1** — jeton ; plan.
- [ ] **P2** — le montage sans raccord ; l'essai ; (1)–(2).
- [ ] **P3** — preuve ; rituel.

### Notes de reprise
