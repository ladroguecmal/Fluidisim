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

Session : S148 — en cours
Agent : Codex (GPT-6 ; fichiers, git et cargo disponibles)
Objectif : S147-1, construire et recevoir un fond spectral explicite selon ADR-100.

### Plan

- [x] **P1** — état réel, lectures de reprise, jeton et plan committé seul.
- [x] **P2** — arrêter le contrat de recette et de cuisson bornée/reproductible ; écrire le constructeur explicite sans migrer la voie historique ; tests des coefficients et refus.
- [>] **P3** — réception indépendante contre S147, moments et pics, phases/dérivées, B+W et rejeu ; témoins historiques et debug/release.
- [ ] **P4** — suite complète, rapport, angles/leçons/actions, journal, index/décomptes et passation ; rituel de fin, jeton rendu.

### Notes de reprise

Départ master99cb421, toutes les copies propres et en retard. Aucune copie créée.
S147 :277 tests/cinq ignorés,100 ADR,212 angles,230 leçons,18 invariants,6 SPEC,23 cas.
ADR-100 décide un candidat JONSWAP normalisé dans une bande explicite ; fixture [0,5fp;4fp],
gamma1/3,3/7,N32/64/128/256. Référence S147 f64/libm uniquement pour tests.
L'exp f32 bornée existe déjà dans gaussian_spectrum (S97) ; préférer la partager sans changer
ses opérations. Ni recette numérique reçue localement ni hash debug/release ne prouvent I-03
sur plusieurs plateformes. Le fond historique et les scénarios restent inchangés.
BILAN-S145 : B1 exécuté S146, S63-1 close S147 ; W4/sillage puis B2 restent la trajectoire.
P2 : cuisson V1 construite, quatre tests ciblés debug reçus. Gravité portée par B et vérifiée dans les trois compositions.
