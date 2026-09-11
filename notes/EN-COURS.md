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

Session : S170 — en cours
Agent : Codex (GPT-6 ; fichiers, git et cargo disponibles)
Objectif : S169-1/A50, source moyenne exacte, interpolée sur réseau décimé et omise.

### Plan

- [x] **P1** — état réel, jeton et plan seul.
- [x] **P2** — protocole et bilan prédit du défaut de source, copies synchronisées.
- [x] **P3** — construire la sonde, comparer résolutions et décalages du réseau source.
- [x] **P4** — réception, résultats, limites et suivi A50.
- [ ] **P5** — rituel de fin, journal/leçons/index/décomptes, reprise et copies.

### Notes de reprise

Master0cdfa31 propre, quatre copies alignées, S169 terminée. Corpus lu dans cette conversation.
112 ADR,224 angles,251 leçons,18 invariants,6 SPEC,23 cas. Aucun runtime adopté.
S169 quatre tests et24 tests S165–S168 reçus ; workspace299/cinq ignorés reçu S163.
SPEC-004 §6.2 et B4 demandent une variation du réseau source : pas de facteur4 universel.
BILAN-S145 porté par poursuite B4. Aucun seuil physique à inventer.
P2 : SOURCE-DECIMEE-S170 déclaré : H indépendant de dx, origine0/H/2 ; intégration
exacte de l'interpolant linéaire, défaut signé du volume prédit par l'erreur de source.

P3 : quatre tests propres reçus et48 évolutions release. Défaut de volume signé prédit
à<=2,20e-15 ; réseau16m décalé peut faire pire que l'omission. Aucun support modifié.
En-tête CSV H renommé source_spacing : PowerShell confondait H et h ; campagne relancée.

P4 : S169-1 réalisée sur véhicule, A50 partielle ; A225/S170-1 source par flux partagé.
Mesures publiées sans seuil physique. Pas d'ADR ni runtime modifié.
