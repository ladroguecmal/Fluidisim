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
Session          : S42
État             : en cours
Battement        : 2026-09-07
Objectif         : L'essai à zéro de C03 — un montage sans seiche ne doit pas rendre de demi-vie
```

### Plan

Action **S41-4**, angle mort **A167** : *tout montage de mesure doit venir avec un essai dont le
résultat attendu est zéro.* L'inventaire de S41 a trouvé **un seul** essai de ce genre dans tout le
harnais — `B-S27-garde` — et c'est celui que la lignée B a apporté.

**C03 est le plus exposé des montages qui n'en ont pas.** Sa demi-vie d'amplitude ne se lit nulle
part : elle vient d'une **régression linéaire de `ln(pic)` sur le temps**, sur une enveloppe
reconstruite par recherche de maxima sur des demi-périodes. C'est la mesure la plus indirecte du
corpus, et A167 dit exactement ce qui arrive à ces mesures-là : *`R` valait 0,18 à 0,32 dans les trois
montages, faux comme juste.*

> **L'essai à zéro de C03 s'écrit tout seul** : le même montage, **sans excitation** (`eta_bord = 0`).
> Le bassin est plat et au repos ; la jauge ne doit rien voir, et **aucune demi-vie ne doit pouvoir
> être calculée**.

*Thèse déclarée : `demi_vie_seiche` rend un nombre fini et plausible sur un bassin au repos.* Elle
régresse `ln(pic)` sans jamais demander si les pics sont autre chose que de l'arrondi ; une pente
négative dans le bruit rend `ln2/(−τ)/T`, un nombre qui n'a aucune raison d'être absurde.

**Si la thèse est vraie, le défaut est du même ordre que celui qu'A167 décrit** : la mesure ne
distingue pas un signal d'un bruit, et **le corpus publie 161,14 périodes** sur cette base. Si elle
est fausse — si la fonction refuse déjà — alors le garde-fou existe sans avoir jamais été vu
refuser, ce qui est le sujet de **L118** et se corrige par le même test.

**Un point de comparaison existe et il est instructif** : le véhicule d'accueil a déjà un garde-fou
d'amplitude — **G5**, *`mesurer_seiche` refuse une amplitude qu'elle ne peut pas voir*, audité en
S34. **Le véhicule importé n'en a aucun.** Deux implémentations de la même mesure, une protégée,
l'autre non — encore une divergence que l'oracle croisé n'a pas cherchée parce qu'elle est dans le
**harnais**, pas dans les solveurs.

- [>] **P1** — plan, jeton.
- [ ] **P2** — **l'essai à zéro** : `demi_vie_seiche` sur un bassin au repos. Mesurer ce qu'elle
      rend, sans rien corriger encore.
- [ ] **P3** — selon le résultat : écrire le refus, **avec son témoin** — le cas qu'il doit
      refuser *et* le cas qu'il ne doit pas refuser (**L119**).
- [ ] **P4** — vérifier que le refus ne change **aucun** chiffre publié : les quatre demi-vies du
      tableau d'`ADR-040` §5 doivent se reproduire à l'identique.
- [ ] **P5** — **C06 et C08** : ont-ils un essai à zéro possible, et lequel ? Écrire au moins le
      plus court des deux.
- [ ] **P6** — répercussions : `CAS-CANONIQUES` — l'essai à zéro devient une condition de mesure ;
      **A167** relu ; ce que l'inventaire des témoins doit devenir.
- [ ] **P7** — rituel de fin (`REPRISE.md` §6).

### Notes de reprise

**Ce qui commande cette session.**

- **`demi_vie_seiche` est exerçable seule depuis S36** (extraite de la fermeture de `c03_seiche`,
  précisément pour qu'on puisse la tester). C'est ce qui rend cette session courte.
- **La signature** : `demi_vie_seiche(m, pas_m, h0, eta_bord, periodes, sc)`. `eta_bord = 0` donne le
  bassin plat au repos — l'essai à zéro, sans une ligne de montage nouvelle.
- **Elle rend `f64::INFINITY`** si la pente de régression est positive. C'est le seul refus existant,
  et il ne couvre pas le cas d'un bruit **décroissant**.
- **Les chiffres à ne pas casser** : 6,01 · 44,36 · 43,12 · 161,14 périodes (`ADR-040` §5), rejoués
  en S36 à **0,00 %**. Un refus mal placé les ferait disparaître.
- **G5 existe côté accueil** et a été vu refuser en S34. Le comparer plutôt que le réinventer.
- **État de départ** : `cargo test` = **88 tests** (38 cœur + 50 harnais, deux `ignore`), `check` = 0
  échec, hashs `0x3e2c06a7b00e73e3` et `0x1a8b0629a9f51b6e`.
