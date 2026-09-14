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

Session : S231 — en cours
Agent : Codex, GPT-6 (fichiers, git, cargo et Python disponibles)
Entrée : « Continue », quatre copies à88b98fe, master propre, jeton libre.

**Objectif.** Construire et qualifier la pression f32 de δ sur le candidat réel, en comparant au témoin f64 avant modification. Conserver abandon atomique et absence d'allocation.
**Critère d'arrêt.** Réception numérique (repos, projection, raffinement plat/coupé et continuation), comparaison du coût/stockage, limites et nouveaux bits explicites. Ne pas masquer le défaut spatial connu ni déclarer B3/I-05 reçus. Si f32 échoue, conserver la preuve et décider explicitement du contrat requis.

### Plan

- [x] **P1** — amorce, jeton et plan seuls.
- [x] **P2** — lectures ciblées, critères et référence f64 avant modification.
- [>] **P3** — construire la pression f32 et ses contrôles numériques sur le chemin réel.
- [ ] **P4** — réception comparée, coûts, suite et preuve des limites.
- [ ] **P5** — rituel §6, priorité comparée, journal, file, jeton et copies synchronisées.

Étapes sous quinze minutes ; découpage déclaré si nécessaire.

### Notes de reprise

Base :407 tests réussis,5 ignorés. S230 reçoit l'arrêt coopératif, pas I-05 complet. Empreinte filtre f64 :0x0ad3f695685ca27a, ordre plat1,947, lisse0,898, marche0,895. Aucun nouveau test général à l'amorce.

P2 : référence f64 exécutée avant modification, code/target/s231-f64.txt (dix cas et champs complets) ; protocole PRESSION-F32-S231. Stockage32×16=44032octets,128×64=692224octets. Divergence premier pas f64 entre2,27e-6 et8,07e-6 ; continuation100pas reçue. Horloge de clôture21:33, décalage mural depuis amorce21:01 : étape trop longue au regard du quart d’heure, prochaines modifications découpées si nécessaire. Aucun changement numérique encore fait.
