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
Session          : S30
État             : en cours
Battement        : 2026-09-06
Objectif         : Réécrire les cinq assertions fautives — et leur donner une provenance
```

### Plan

Action **S29-1**. L'audit S29 a recensé six assertions fautives sur cinq cas et dit, pour chacune,
**ce qu'il faudrait mesurer à la place**. Il n'a pas dit **avec quel seuil** — et c'est là que se
trouve le travail : un seuil sans provenance viole I-14, et le corpus en compte déjà trois qui
traînent depuis leur écriture (A106).

**Ce que la session ne doit pas faire.** Remplacer « aucune divergence » par « divergence < 5 % »
serait un progrès de forme et une régression de fond : on aurait échangé un symptôme contre un
nombre inventé. **Chaque seuil doit sortir d'une formule du corpus ou être marqué « à calibrer »
avec le banc qui le fixera** — c'est I-14, et c'est la seule sortie honnête.

*Thèse déclarée : les cinq cas ne se ressemblent pas, et deux d'entre eux n'ont pas besoin d'un
seuil du tout.* « Nettement supérieure » (C07) et « sensiblement plus longue » (C10) décrivent des
**rapports** dont la théorie donne la valeur — il n'y a pas de seuil à choisir, il y a une formule à
retrouver. Les vacuités (C15, C18) demandent un **témoin**, pas un seuil. Seul C11 demande vraiment
un nombre neuf, et c'est celui qu'il faudra peut-être marquer « à calibrer ».

- [x] **P1** — plan, jeton.
- [x] **P2** — **C10**, masse ajoutée : `T_avec/T_sans = √(1 + m_a/m)`. Référence fermée, et le cas
      est partiellement exécuté depuis S21 — donc vérifiable, pas seulement réécrit.
- [ ] **P3** — **C07**, sillage transcritique : le facteur de résonance `1/√|1 − Fr_h²|` d'ADR-011
      §4. Attention, il **diverge à `Fr_h = 1`** : la mesure ne peut pas se faire au point critique,
      et l'énoncé actuel y place pourtant son assertion.
- [ ] **P4** — **C15** et **C18**, les deux vacuités : un témoin chacune, et la grandeur qu'il doit
      faire bouger.
- [ ] **P5** — **C11**, le seul qui demande un nombre neuf. Chercher d'abord s'il existe une
      grandeur dont le seuil se dérive ; à défaut, « à calibrer » avec son banc, ce qui est un
      statut légitime et non un échec.
- [ ] **P6** — notes correctives datées dans `CAS-CANONIQUES` pour les cinq, et mise à jour du
      registre S29.
- [ ] **P7** — répercussions : index, angles morts, actions, décomptes.
- [ ] **P8** — rituel de fin (`REPRISE.md` §6).

### Notes de reprise

**Ce que S29 laisse et qui commande cette session.**

- **Les trois classes** — recevable · symptôme · vacuité — s'appliquent à toute assertion nouvelle.
  Celles écrites ici doivent y être soumises **avant** d'être publiées.
- **Le corollaire d'atteignabilité (A133)** : vérifier qu'un montage peut atteindre le régime où
  l'assertion échoue. C'est directement en jeu pour C07, dont le point critique est singulier.
- **`avec_cfl` borne à 2,0**, délibérément. Ne pas resserrer.
- **C04 en échec, `C01-jet` rouge, C08 sans verdict** : trois décisions.

**Branche.** `claude/s22-suite`. `master` s'arrête à S17 (A107).

#### P2 — C10 : le rapport ne dépend de rien, et c'est ce qui le rend assertable

`T = 2π·√((m + m_a)/(ρ_eau·g·A))` donne `T_avec/T_sans = √(1 + m_a/m)`.

**A26** pose que la masse ajoutée d'une coque vaut **environ la masse déplacée**. Et un corps qui
flotte déplace, par Archimède, **exactement sa propre masse**. Donc `m_a ≈ m`, et le rapport vaut
**√2 ≈ 1,414** — sans dépendre de la taille du cube, de sa densité ni de la profondeur : tout
s'annule dans le quotient.

> **Il n'y avait pas de seuil à choisir. Il y avait une formule à retrouver.**

**Assertion : `T_avec/T_sans = 1,414 ± 15 %`**, la tolérance encodant le mot « environ » d'A26 —
`m_a/m ∈ [0,5 ; 1,5]` donne `[1,225 ; 1,581]`, soit −13 % à +12 %. **Contrôle indépendant** : le
disque équivalent de même aire, `m_a = (8/3)ρR³` avec `R = a/√π`, donne **1,399**. Les deux voies
concordent à 1 % sans rien partager.
