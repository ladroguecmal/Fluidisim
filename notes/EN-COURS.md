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

Session : S147 — terminée
Agent : Codex (GPT-6 ; fichiers, git et cargo disponibles)
Objectif : clore S63-1 sur preuves existantes, puis instruire A212 (forme du spectre de B).

### Plan

- [x] **P1** — vérifier copies, branches, historique et outils ; prendre le jeton et committer ce plan seul.
- [x] **P2** — lectures de reprise ; clore S63-1/S145-2 avec le périmètre exact de la dispersion construite et les dépendances restantes de B2.
- [x] **P3** — examiner A212 dans les spécifications et le code, vérifier les références physiques et arrêter une décision de spectre avec son protocole de réception ; construire le lot que cette décision permet dans la session.
- [x] **P4** — vérifier le lot, écrire journal et passation, contrôler décomptes et renvois, exécuter le rituel de fin et rendre le jeton.

### Notes de reprise

Départ : master 66cd765, identique à 886155 ; autres copies propres et historiques, aucune copie créée.
S146 terminée ; 275 tests réussis/cinq ignorés annoncés, 99 ADR, 212 angles, 18 invariants,
6 spécifications, 23 cas. Cargo 1.97.0 disponible. B1 partiel : volets perceptuels et LOD ouverts.
La recommandation BILAN-S145 est portée : B1 exécuté S146, S63-1 prise en premier ici.
P3 : ADR-100, SPEC-001 §1 bis, instrument spectral cfg(test), deux tests release reçus.
A212 partielle : décision et instrument réalisés, constructeur spectral S147-1 à construire.
P4 : suite workspace 277 réussis/cinq ignorés ; deux essais S147 aussi en release.
Rituel exécuté : journal, A212 partielle, L230, suivi S147-1, index/README/REPRISE actualisés.
100 ADR,212 angles (formats de titres et de tables pris en compte),18 invariants,6 SPEC,23 cas.
S148 : construire le candidat spectral explicite, sans migrer implicitement le fond historique.