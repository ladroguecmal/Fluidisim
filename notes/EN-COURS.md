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

Session : S267 — terminée
Agent : Codex, GPT-6 ; fichiers, git, cargo, Python et GPU local.
Entrée : « continue ». Copie unique master propre c476631, jeton libre.
Objectif : réduire le coût de cuisson GPU du sillage en préservant le rendu accepté et I-09.

### Plan

- [x] **P1** — amorce, jeton et plan seuls.
- [x] **P2** — lectures, diagnostic du calcul du sillage et contrat de réception avant code ; comparer avec J2.
- [x] **P3** — construire une optimisation bornée avec témoin conservé et vérifications ciblées.
- [x] **P4** — recevoir précision, images et coût ; retenir ou rejeter sur les critères déclarés.
- [x] **P5** — rituel §6, file entière, journal, index et jeton.

### Notes de reprise

S266 : GPU eau 2,24–2,26 ms dont sillage 1,06–1,08 ms. Ne pas réduire la cadence
sans preuve d'erreur et I-09. Aspect R7 accepté, optimisation doit le préserver.

P3 : huit accumulateurs nommés, témoin --sillage-cuisson-directe, bancs intégrés.
Tests hôte 19/1/0. Vingt comparaisons de grilles au bit et retours temporels reçus.
Premier coût : cuisson 0,585/0,574 ms contre 1,083/1,064 ; eau 1,744/1,737 ms.
Pointe référence 2,962 ms à conserver ; images et réception spectrale encore à faire.

P4 : sept images au bit S266, 36 cas spectraux <=0,340 mm. Deux passages de coût
reçus (46 % cuisson, 22–23 % total). Premier maximum 2,962 ms conservé ; second
maximum 1,836 ms. CPU toujours ~4,1 ms. Preuve CUISSON-SILLAGE-S267 complète.

P5 : rituel terminé ; suite S268 bords ouverts J2. Coût CPU et pointes gardés dans
la file, critères et limites dans CUISSON-SILLAGE-S267. Copie unique.
