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
Session          : S44
État             : en cours
Battement        : 2026-09-07
Agent            : Claude Code
Objectif         : Inventorier les valeurs de repli — que devient un refus qui les traverse ?
```

### Plan

Action **S43-1**, qui reprend **S42-2**. **Deux sessions de suite ont trouvé par hasard un repli de
sévérité 1**, chacune en cherchant autre chose :

| | repli | ce qu'il rendait | pourquoi c'était grave |
|---|---|---|---|
| **S42** | `NaN.min(10⁶)` | `10⁶` périodes | le **meilleur score** face à un minorant de 15 |
| **S43** | `else { 1.0 }` | l'ordre `1,0` | **l'ordre nominal du schéma**, dans les bornes de G10 |

**La troisième ne doit pas être trouvée par hasard.** La recherche est mécanique et son critère
tient en une question : *que devient un refus qui passe là-dedans ?*

**Le recensement préalable donne l'ordre de grandeur** — 25 `unwrap_or` et 24 `min`/`max` dans le
harnais :

| repli | nombre | première lecture |
|---|---|---|
| `unwrap_or(f64::NAN)` et variantes | **13** | le refus **survit** — a priori sains |
| **`unwrap_or(0.0)`** | **8** | **suspects** |
| `unwrap_or(4.0)`, `unwrap_or(64)`, `unwrap_or(0)`, `unwrap_or(false)` | 4 | à regarder un par un |

> **Les huit `unwrap_or(0.0)` sont le cœur de la session.** Pour un **écart**, une **erreur**, une
> **dérive** ou une **vitesse parasite**, **zéro est la meilleure valeur possible** — pas une valeur
> neutre. C'est **A171** porté à son extrême : un repli qui ne vaut pas seulement le nominal, mais
> le **parfait**.

*Thèse déclarée : au moins un `unwrap_or(0.0)` se trouve sur le chemin d'une grandeur publiée, et y
transforme un refus en résultat parfait.*

**Si elle est fausse** — si les huit sont sur des chemins de diagnostic ou d'affichage — c'est un
résultat aussi, et le premier depuis trois sessions qui dirait que ce motif est sous contrôle. Mais
il faudra le **montrer**, pas le supposer : c'est ce que les deux sessions précédentes n'ont pas pu
faire, faute d'inventaire.

- [x] **P1** — plan, jeton.
- [x] **P2** — **l'inventaire complet**, un tableau : chaque repli, ce qu'il rend, et **ce que
      devient la valeur** — assertion publiée, diagnostic imprimé, ou calcul interne.
- [x] **P3** — classer : *sain* · *inoffensif ici* · **fautif**. Un repli n'est fautif que s'il est
      **sur le chemin d'une grandeur lue**, et le distinguer demande de suivre chaque valeur.
- [x] **P4** — corriger les fautifs, **avec leur témoin** (**L119**), et vérifier qu'aucun chiffre
      publié ne bouge.
- [x] **P5** — le registre `AUDIT-REPLIS-S44`, et la règle qui évite le prochain.
- [x] **P6a** — rituel : journal, leçons L165-L166, actions S44-1 et S44-2.
- [ ] **P6b** — rituel : index, décomptes, jeton libéré.

### Notes de reprise

**Ce qui commande cette session.**

- **Le critère n'est pas « le repli est-il correct »** mais *que devient un refus qui le traverse*.
  Un `unwrap_or(0.0)` sur une somme vide est juste ; le même sur une mesure qui a refusé est un
  mensonge. **Seul le chemin de la valeur les distingue.**
- **Les 13 `unwrap_or(NaN)` ne sont pas automatiquement sains** : `NaN` survit aux comparaisons, mais
  **pas à `min`/`max`** — c'est exactement ce qui a mordu en S42. Il faut vérifier ce qui suit.
- **Ne pas corriger un repli sans savoir ce qu'il portait.** Retirer un filet sans savoir ce qu'il
  retient est le geste qui transforme un défaut visible en défaut invisible (note de S38).
- **Chiffres à ne pas casser** : `C08-p = 0,999745`, les demi-vies 6,01 / 44,36 / 43,12 / 161,14, le
  front à 0,7365 %, les ordres 0,654 / 0,621 / 1,000, et les deux hashs.
- **État de départ** : `cargo test` = **95 tests** (38 cœur + 57 harnais, deux `ignore`), `check` = 0
  échec, hashs `0x3e2c06a7b00e73e3` et `0x1a8b0629a9f51b6e`.
- **Hors session, juste avant** : l'amorce a été déplacée dans `AGENTS.md` et le jeton a reçu une
  ligne `Agent`, sur demande de l'utilisateur — voir `FORK-S22-S26` §9.5. Cette session est la
  première à renseigner cette ligne.
