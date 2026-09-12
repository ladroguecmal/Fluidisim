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

Session : S204 — en cours
Agent : Claude Code (Opus 5 ; fichiers, git et cargo disponibles)
Objectif : porter la **clarification de l'utilisateur** du 2026-09-13 — ambition finale
complète, construction progressive par versions de plus en plus capables — dans une
**décision corrective d'ADR-124** (nouvel ADR, ancien non réécrit), puis la propager à la
feuille de route, à REPRISE et à la file active. Aucun code. Aucun travail antérieur supprimé.

### État réel constaté à l'amorce

master = 5a4ba1e (S203 P7), status propre ; trois copies isolées à 5a4ba1e, propres ;
branche archivée claude/reprise-projet-5134cd inchangée. Jeton libre depuis S203 (même
agent, deux minutes plus tôt) ; aucun travail en cours, aucune session concurrente.
Codex a reconnu la correction en conversation sans l'appliquer : **aucun fichier ne la
porte**. Lecture restrictive recensée dans 18 fichiers (58 occurrences), dont ADR-124
(« δ effet borné », « V attend un besoin gameplay nommé »), ADR-125 §35-36, REPRISE,
file active (4 lignes), 00_INDEX, README, PLAN-BENCHMARK B3, BILAN-B4-S176, IMAGE-B-S201,
BUDGET-IMAGE-S202, IMPACT-W-S203 et le journal — **y compris mes propres textes S203**.
**Pas de fichier « roadmap »** : la trajectoire vit dans ADR-053 §3 (historique), ADR-054 §3
(lots W), ADR-124 « Décision et ordre » et l'en-tête de REPRISE.

### Plan

- [x] **P1** — état réel, jeton, plan seul.
- [x] **P2** — ADR-127, décision corrective d'ADR-124 : ambition finale complète (δ général,
 interactions volumiques, inondations complexes, V, grande échelle) ; ordre progressif en
 cinq versions ; V placée par dépendances, pas en fin ; responsabilités/interfaces/autorité
 (I-04, I-10, I-11, I-15, I-17)/conservation préservées ; bancs exécutés quand ils tranchent ;
 60 Hz/2 ms et 2 % acquis, budget = cible à confronter, incompatibilité ⇒ arbitrage explicite.
 Note de renvoi datée en tête d'ADR-124 ; note corrective datée ADR-125 (§35-36).
- [ ] **P3** — `docs/FEUILLE-DE-ROUTE.md`, unique porteur de la trajectoire : cinq versions,
 dépendances, ce qui existe, déclencheurs de bancs, arbitrages ouverts (hôte interactif,
 A247). REPRISE, index et file active y renvoient sans la recopier (L137).
- [ ] **P4** — propagation : REPRISE (paragraphe de trajectoire, file active, §4 et marqueurs
 datés sur S201–S203), QUESTIONS-OUVERTES (direction, lignes δ/A244/S199-2/V/S202-1),
 AGENTS (« Ce que tu ne décides pas »), PLAN-BENCHMARK B3, BILAN-B4-S176, 00_INDEX, README,
 notes datées en fin d'IMAGE-B-S201, BUDGET-IMAGE-S202, IMPACT-W-S203.
- [ ] **P5** — rituel §6 : journal, angle mort, leçon, décomptes (127 ADR attendus), file
 active entière, invariants cités relus, compteur, jeton libre, copies avancées.

### Notes de reprise

Formulation de référence de l'utilisateur, à citer telle quelle : *« ambition finale
complète, construction progressive par versions de plus en plus capables »*. Trajectoire
utilisateur : (1) version visible et interactive B/W ; (2) domaines volumiques bornés comme
cas de construction et de validation du système général ; (3) extension des phénomènes,
interactions entre domaines, frontières mobiles ; (4) V et inondations complexes, puis
articulation avec la représentation volumétrique ; (5) ambitions initiales complètes.
« Leur ordre détaillé doit suivre les dépendances, sans repousser V artificiellement à la
toute fin. » ADR-054 §1 : V constructible indépendamment de δ (C12 cas V seul).
ADR-001 : δ solveur 3D à surface libre, jamais autoritaire ; V graphe entier, serveur
autoritaire, peut exposer une surface et déclencher un domaine δ local.
