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

Session : S128 — en cours
Agent : Codex (GPT-6 ; fichiers, git et cargo disponibles)
Objectif : S127-1 — exercer le montage N256/R80/48 s dans LiveWater B+W, renouvellement,
sauvegarde/restauration, identité avec construction directe et coût mesuré.

### Plan

- [x] **P1** — vérifier branches/copies/jeton, lire la suite, déclarer le plan seul.
- [x] **P2** — construire le scénario hôte : impact S127 unique, fond B réel, points jusqu'à80 m,
      horizon4→24→48 s, comparaison en bits des dix sorties aux champs directs et restaurés.
      Refus : temps, rayon tardif, renouvellement impossible, sauvegarde tronquée et N différent ;
      publication précédente et sorties conservées. TTL4 inchangé, sauvegarde en mémoire seulement.
- [ ] **P3** — recevoir le scénario, mesurer séparément update, requête64, save/restore et cycle
      complet après mise en régime ; bornes de mesure explicites, rapport et suite motivée.
      Corriger la bibliothèque seulement si un défaut est reproduit et documenté.
- [ ] **P4** — vérification adaptée, journal/angles/leçons/actions, index/README/passation,
      décomptes et invariants, jeton libre et commit final propre.

### Notes de reprise

Départ4c33261 sur master propre ; anciennes copies en retard et propres, aucune branche
avancée ni copie créée. Cargo1.97 disponible. Règles et REPRISE déjà lus dans cette conversation.
Points d'entrée : bench_live_water.rs (S88), prepared_water::{LiveWater, Prepared}, live_snapshot.rs.
Production actuelle : update(None) reconstruit le journal et les champs, même à horizon constant ;
current() ne vérifie pas l'instant demandé, c'est la requête qui refuse hors horizon.
Référence du cycle : même Background et composition ponctuelle, champ RadialImpact256 construit
directement depuis les faits ; identité d'acheminement seulement. Réception physique = S127.
Critère : identité des bits de chaque sortie, pas seulement hash ; refus avec témoins.
Aucun seuil de performance choisi ; résultats locaux, pas de budget cible certifié.

P2 : cycle_transported_water.rs construit et première exécution release reçue :1280 points-temps,
dix composantes identiques, sources détruites avant reprise, WLIV289 octets, TTL4 conservé.
Refus atomiques : horizon dépassé, rayon au dernier point, extension64 s impossible, now hors
horizon, sauvegarde tronquée et N128 cible. B seul diffère des64 résultats à48 s (W présent).
Mesure initiale requête64 ~971 µs, cycle3 requêtes ~3004 µs ; compléter en P3 par update+requête48.