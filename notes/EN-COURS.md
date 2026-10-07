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

Session : S685 — **terminée**. En autonomie vers la v2 ; K3, 4.14 ; le relais au rivage, l'étape 2 de la conception.

**Ce que la session fait.** L'onde solitaire de S644 (`H/d` = 0,2, pente 1:3, non déferlante) part dans APIC 3D. Elle traverse le
raccord à trois mailles de fond, et monte la plage dans Saint-Venant 2D. Le relais est jugé contre :

- le tout-Saint-Venant, depuis le même état initial ;
- Synolakis (0.2295 m).

L'état initial, dans les deux montages : l'onde et sa vitesse au large du pied, l'eau au repos au-delà.

**Contrôles du plan** (ADR-266, ADR-267, ADR-268, ADR-272)

- **témoin** : le tout-Saint-Venant, le même état initial, la même lecture de la remontée (la plus haute maille mouillée, `h` > 1 mm,
  moyennée sur les rangées).
- **instrument** : la remontée maximale sur 3 s, et la masse. Ce que rendrait chaque hypothèse :
  - un flux bien transmis rend la remontée du tout-Saint-Venant, à l'écart près de l'onde portée par APIC au large (≈ 4 %, le calcul) ;
  - un raccord qui réfléchit ou retient l'eau rend une remontée tronquée de plusieurs dizaines de % ;
  - une dette qui grandit trahit un échange mal compté.
- **calcul** (ce script) :
  - le raccord **à 5,35 m aux deux mailles**, à 16,4 cm de fond (3,3 et 6,6 mailles). À 2,5 cm, la règle des trois mailles le
    mettrait à 5,575 m, où l'onde levée par Green dépasse McCowan (`H/h` = 1,25, refusé par le script). ADR-271 D1 le veut au-delà du
    déferlement ;
  - l'onde y fait ≈ 8,6 cm, `H/h` = 0,55 (asserté sous 0,78) ;
  - l'écart attendu au tout-Saint-Venant : ≈ 4.2 % (la crête d'APIC au pied, 3 % au-dessus, S644). La borne, **10 %**.
- **ADR** : ADR-271, ADR-272 (sans objet : une onde, non un repos).
- **pièges** :
  - l'état initial du relais : Saint-Venant au repos au-delà du raccord, APIC sans l'onde au-delà du pied ; le tout-Saint-Venant, de même ;
  - le pas commun : la CFL de Saint-Venant (`√(g·h)` + `u`) ;
  - le bord de Saint-Venant prend la moyenne des rangées (la côte est uniforme).

**Critères, écrits avant.**

1. La remontée du relais à moins de **10 %** de celle du tout-Saint-Venant, à 5 et 2,5 cm.
2. La masse : Saint-Venant + particules × quantum + réservoir − dette, constante à 10⁻¹² près en relatif.
3. Rapportés : la remontée contre Synolakis, la dette, la crête au raccord.

### Plan

- [x] **P1** — jeton ; plan.
- [x] **P2** — l'essai ; (1)–(3).
- [x] **P3** — preuve ; rituel.

### Notes de reprise
- **P2 fini** — (1) **manqué** : −15,2 % et −13,9 % du tout-Saint-Venant. Localisé : le volume transmis égal (+1,6 %), `η` et `u` du bord 3D lus juste ; l'onde portée par APIC diffère de celle de Saint-Venant (dispersion). Le calcul du plan ne regardait que la crête. (2) tenu.
