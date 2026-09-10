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

Session : S163 — terminée
Agent : Codex (GPT-6 ; fichiers, git et cargo disponibles)
Objectif : S162-1 / A218, intégrer réellement le résidu couplé en Saint-Venant et le
recevoir contre l'évolution totale. Pas de δ 3D ni de seuil de bascule dans ce lot.

### Plan

- [x] **P1** — état réel, jeton et plan committé seul.
- [x] **P2** — préciser variables conservatives, flux croisés, pas et frontières ; choisir
      un instrument minimal compatible avec le solveur total existant. Déclarer les réceptions
      et la contre-épreuve avant le calcul. Synchroniser les copies propres au jeton occupé.
- [x] **P3** — construire l'intégration du fond et du résidu, sans soustraction a posteriori
      du total ; recevoir contre la référence et retirer volontairement un terme de couplage.
- [x] **P4** — campagne de raffinement et cas limites, limites de portée, verdict et suivi
      A218/A50. ADR seulement si décision nouvelle. Tests adaptés et bilan numérique.
- [x] **P5** — rituel de fin REPRISE §6 : journal, angles/leçons, index, décomptes, prochaine
      action, jeton libéré et copies synchronisées. Aucun worktree créé.

### Notes de reprise

Départ master 8902f5a, arbre propre ; trois autres copies au même commit, propres et libres.
Cargo 1.97.0 disponible. REPRISE et corpus lus en S162 dans cette conversation ; états et
nouvelle entrée relus, aucune modification intermédiaire. 299 tests/cinq ignorés, plus le test
analytique S162. 112 ADR,218 angles,244 leçons,18 invariants,6 SPEC,23 cas.

ADR-112 impose de tester l'équation résiduelle, pas la superposition indépendante.
SPEC-004 §6.1 contient les termes croisés et le résidu du fond. A218 est prioritaire ;
A217 reste partielle, A216 reportée. BILAN-S145 suivi par la poursuite B4.
P2 : RESIDU-COUPLE-S163 déclare flux physique résiduel, correction de viscosité Rusanov,
deux fonds (évolué/figé), sources, RK2, critères et trois contre-épreuves avant calcul.
Référence Shallow1D configurée en Rusanov, ordre un espace, RK2 ; copie conservée sans modification.

P3 : intégration Q,d distincte construite dans examples/support/residu_shallow.rs ;
aucun appel à Shallow1D dans le véhicule. À N240, accord h<=5,93e-15 normalisé,
contre-épreuves physiques/numériques/source toutes refusées. 2 nouveaux tests propres
à l'exemple reçus en debug (+3 tests importés du host) ; campagne release reçue.

P4 : cas limites et raffinements reçus. N960 : max h reconstruction 1,61352e-13 ;
écart spatial de la référence 9,07266e-3 L2 normalisé. Aucun ordre spatial certifié.
5 tests propres à l'exemple +3 host importés passent ; campagne release complète reçue.
Suite workspace 299 tests réussis/cinq ignorés. S162-1 réalisée dans le périmètre 1D,
A218 traitée ici, A50 partielle. Pas d'ADR nouveau ; A219/S163-1 pour le fond prescrit en temps.
P5 : rituel effectué, journal, L245, A219, index/README/REPRISE et suivi ADR-112.
Décomptes 112 ADR/219 angles/245 leçons vérifiés ; 18 invariants/6 SPEC/23 cas inchangés.
BILAN-S145 porté via S163-1, aucune décision humaine requise. Jeton rendu.
Copies propres à avancer au commit de clôture avant réponse finale, sans retrait.