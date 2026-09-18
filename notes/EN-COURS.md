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

Session : S284 — en cours
Agent : Codex GPT-6, application desktop ; fichiers, git, cargo, outils locaux.
Entrée : continuer, préparation progressive demandée après S283.
Objectif : amortir la couronne avant réduction et éprouver le passage gardé, sans prétendre
recevoir la réduction automatique ni I-12 perceptif.

### Plan

- [x] **P1** — état Git, jeton et plan seuls.
- [x] **P2** — construire un amortissement de préparation sans allocation, consommé après
  les vrais pas de Live ; réception exponentielle, centre conservé, refus atomiques.
- [x] **P3** — demande progressive dans Layer, mesure contre réduction brutale et témoin,
  coût complet, garde S283 inchangé, tests et limites ; découper avant quinze minutes.
- [>] **P4** — rituel §6 : preuves, journal, file/feuille de route/index, jeton libre.

### Notes de reprise

S283 clôturée 48d2ab7 ; copie principale unique, propre. Transfert brut à 1,024 s refusé :
33,447 mm contre 3 mm. Garde CPU de hauteur seulement, pas la pente ni la suite temporelle.
Critère : une demande tardive ne permute jamais au-dessus du garde 3 mm ; suivre la variation
supplémentaire de hauteur par pas de préparation, le centre et le coût complet. Si le fond
réalimente trop les bords, conserver le refus et mesurer la limite, ne pas relâcher le garde.
P2 : noyau prepare_shrink construit et reçu, décroissance répétée indépendante, intérieur au
bit, refus atomiques et zéro allocation. Branchement Live en P3 en cours, non encore reçu.
P3 : plutôt qu'un taux inventé, limiter le changement nodal à 1,5 mm par vrai pas (moitié de
3 mm S201, marge d'interpolation restante à mesurer). Tentative gardée tous les 16 pas : choix
de coût de banc, à qualifier ; aucune diminution de la tolérance de publication S283.
P3 mesure finale : demande 1,024 s, permutation 1,792 s ; correction nodale maximale 1,503 mm
(arrondi à 96 m, ulp 2^-17 m) ; écart centre 66,994 mm jusqu'à 5,120 s, fidélité non reçue.
321 updates mesurés : zéro allocation. 256 après demande : médiane 12,3857 ms, p99 42,0321,
max 43,1006, préparation et garde compris. Coût réinjecté élargi à ce travail, sans reprendre
le coût du domaine large après permutation. Essai release sur secteur, garde inchangé.
Suites release : 507 cœur/harnais, 34 viewer réussis ; 19 ignorés, aucun échec.
