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

Session : S687 — **en cours**. En autonomie vers la v2 ; K3, 4.14 ; le relais au rivage. ADR-273 D1 : le raccord se juge d'abord entre
deux copies du même solveur.

**Ce que la session fait.**

- **Le bord droit de Saint-Venant à flux imposé** (`pas_avec_flux_droit`) : la face droite fait passer le flux de masse donné par
  rangée, avec le flux de quantité de mouvement `F·u + ½·g·h²` de la maille de bord. C'est l'équivalent du bord droit d'APIC, qui
  reçoit `F/h`.
- **Le raccord seul** : le domaine du large, un Saint-Venant, porte l'onde jusqu'à 5,35 m. Le raccord est le schéma de
  `RelaisRivage` : l'état du bord du large nourrit le bord caractéristique du rivage ; le flux rendu revient au large comme flux imposé.

L'onde de S644 et le tout-Saint-Venant de S685, le même état initial.

**Contrôles du plan** (ADR-266, ADR-267, ADR-268, ADR-273)

- **témoin** : le tout-Saint-Venant de S685 (0,2190 et 0,2398 m) ; sans flux imposé, Saint-Venant au bit (S613–S680).
- **instrument** : la remontée et la masse. Ce que rendrait chaque hypothèse :
  - si le schéma de raccord est juste, la remontée du tout-Saint-Venant à l'écart du bord caractéristique près (S622), et donc **l'écart
    de S685 vient de l'onde portée par APIC** ;
  - si le schéma est en faute, le même déficit que S685 (−15 %) ;
  - si le flux est mal rendu, une masse qui dérive.
- **calcul** (ce script) : 18 mailles par largeur d'onde à 5 cm, 36 à 2,5 cm ; l'écart attendu ≈ 3,2 % et ≈ 1,6 % (S622 : 2,9 % à 20,
  1,3 % à 40). La borne, **6 %**.
- **ADR** : ADR-271, ADR-273, ADR-268 D1.
- **pièges** :
  - les deux domaines lisent l'état avant leur pas, comme le relais ;
  - la quantité de mouvement du flux imposé, à la vitesse de la maille de bord ;
  - le bord gauche du rivage et le flux imposé du large, au même pas.

**Critères, écrits avant.**

1. Saint-Venant sans flux imposé, au bit.
2. La remontée du raccord seul à moins de **6 %** du tout-Saint-Venant, à 5 et 2,5 cm.
3. La masse des deux domaines constante à 10⁻¹² près.

### Plan

- [x] **P1** — jeton ; plan.
- [ ] **P2** — le flux imposé ; le raccord seul ; (1)–(3).
- [ ] **P3** — preuve ; rituel.

### Notes de reprise
