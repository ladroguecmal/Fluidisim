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

---

## Session en cours

```
Session          : S29
État             : en cours
Battement        : 2026-09-06
Objectif         : Audit des assertions — ce que chaque cas peut voir
```

### Plan

Action **S28-3**, née d'**A129**. C23 a mesuré une borne fausse — `u_max` sous-estimé ×5,5, Courant
réalisé à 2,48 — **sans que le solveur casse**. Un cas dont l'assertion aurait été « le solveur
diverge » serait donc passé, et aurait certifié l'absence d'un défaut présent.

La question de cette session : **combien de cas du corpus sont écrits sous cette forme ?**

**Le critère de tri, et il demande d'être posé avant l'inventaire.** Toutes les assertions négatives
ne se valent pas :

- « **zéro allocation après `seal()`** » (C18) porte sur un **compteur** — la grandeur existe, elle
  vaut zéro, et elle vaudrait autre chose si le défaut était là. **Légitime.**
- « **aucune divergence sur 120 s** » (C11) porte sur l'**absence d'un symptôme**. Elle ne peut
  échouer que sur une catastrophe, et reste verte sur tout défaut qui n'en produit pas.
  **Disqualifiée.**

> **Critère retenu : une assertion est recevable s'il existe une grandeur continue dont elle est le
> seuil.** Sinon, elle ne mesure que la survenue d'un accident.

*Thèse déclarée : le harnais en contient autant que le corpus, et j'en ai écrit moi-même.* La mesure
de stabilité de S27 classe les exécutions en `Stable / Diverge / NonFini` et a répondu « OK partout »
de `ν = 0,45` à `0,99` — **c'est exactement la forme disqualifiée**, et j'en ai tiré une conclusion.
Si la thèse est juste, l'audit doit commencer par mes propres mesures avant de juger le corpus.

- [ ] **P1** — plan, jeton.
- [ ] **P2** — le critère, posé et éprouvé sur trois cas connus avant d'être appliqué en série.
      Un critère qui classe mal un cas évident classera mal les autres en silence.
- [ ] **P3** — inventaire des **23 cas** de `CAS-CANONIQUES` : forme d'assertion, verdict, et pour
      les cas disqualifiés, **la grandeur qu'il aurait fallu assertir**.
- [ ] **P4** — inventaire des assertions **du code** — `physics.rs`, `delta.rs`, `main.rs`. C'est
      là qu'elles s'exécutent, et un énoncé correct implémenté en « ça n'a pas cassé » ne vaut pas
      mieux qu'un énoncé fautif.
- [ ] **P5** — corriger ce qui peut l'être dans le code, et **mesurer** que la correction change
      quelque chose : une assertion durcie qui reste verte sans qu'on sache pourquoi n'a rien
      prouvé.
- [ ] **P6** — registre `AUDIT-ASSERTIONS-S29`, et note datée sur les cas dont l'énoncé change.
- [ ] **P7** — répercussions : index, angles morts, actions, décomptes.
- [ ] **P8** — rituel de fin (`REPRISE.md` §6).

### Notes de reprise

**Ce que S28 laisse et qui commande cette session.**

- **A129** : la forme de l'assertion décide de ce que le cas peut voir. C'est la trouvaille à
  exploiter, et elle est de sévérité 1.
- **`ν = 0,70` est débloqué pour le solveur du projet, pas pour le véhicule** — la constante de
  `delta.rs` reste à 0,45 délibérément (L97). Ne pas la « corriger ».
- **C04 en échec, `C01-jet` rouge, C08 sans verdict** : trois décisions.

**Branche.** `claude/s22-suite`. `master` s'arrête à S17 (A107).
