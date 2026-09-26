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
- **Ce fichier ne porte que la session en cours** ([ADR-187](../docs/adr/ADR-187-methode-refondue-s321.md)
  D3). À la clôture, ce qui doit survivre des notes va à la preuve ou au journal ; la session
  suivante remplace ensuite toute la section. Aucune section d'archive, 300 lignes au plus :
  `outils/etat_projet.py --check` le vérifie. Notes de S301 à S320 : `git show 78622a19:notes/EN-COURS.md`.

---

## Session en cours

Session : S378 — **terminée**. *« Continue »* ; par l'alternance, la physique ; à deux maillons (S376, S377), un point qui
change d'état : **5.5, la pluie selon l'exposition au ciel**, absente, décidée hier (ADR-203 D2 : bâche entière ou demi
posée et retirée en temps réel). Ne construit pas la météo (à la fin, ADR-197 D5, ADR-203 D5) : l'entrée que V en
recevra.
Agent : Claude Opus 5.5, application desktop ; fichiers, git, cargo, carte réelle, accès web ; Godot 4.4.1 local.

**Thèse.** Une **arête de pluie** (`Flow::Rain { catchment_mm2 }`), du ciel vers un nœud : débit `intensité × surface
d'ouverture × exposition`, quantifié avec report de reste comme les autres, borné par la place libre. **L'exposition est la
commande de l'arête** (ADR-199 : 0 à 1 000, état répliqué, sauvegardé en WVST v2) — une bâche entière la met à 0, une
demi-bâche à 500. **L'intensité est une entrée météo** du pas (`step_meteo`, mm/h), fournie à l'identique à tous les
participants ; `step` reste le pas sans pluie, **identique au bit**. **Surface d'ouverture** plutôt que surface libre
(ADR-010 §5) : la pluie qui tombe dans l'ouverture d'un contenant finit dans son eau, parois intérieures comprises.

**Critères, écrits avant.** (1) Sans pluie, les trajectoires existantes **inchangées au bit** (l'empreinte de S372). (2)
10 mm/h sur 32 m², une heure : **320 000 ml ± 1** ; demi-bâche : 160 000 ± 1 ; bâche : **0** ; bâche posée à 30 min :
240 000 ± 1. (3) La piscine à débordement sous la pluie, pompe arrêtée : le déversoir débite la pluie, charge sur le seuil
`(Q/k)^⅔` **à ±1 %**. (4) Bilan exact (pluie entrée = volume gagné + rejeté), refus atomiques, la commande sauvegardée.

### Plan

- [x] **P1** — jeton, plan seul.
- [x] **P2** — ADR-204 : la pluie, arête de V (surface d'ouverture, exposition = commande, intensité = entrée du pas).
- [x] **P3** — `Flow::Rain`, `step_meteo`, `Meteo` ; critères 1, 2, 4.
- [x] **P4** — la piscine à débordement sous la pluie ; critère 3.
- [x] **P5** — preuve `PLUIE-V-S378`, liste 5.5, registre, file, feuille de route, index, ADR-010 note datée.
- [x] **P6** — rituel.

### Notes de reprise

**P3 — fait.** `Flow::Rain { catchment_mm2 }` (du ciel vers le nœud que `from` et `to` désignent), `Meteo`, `step_meteo` ;
`step` = `step_meteo(Meteo::SEC)`. La pluie calculée en tête de boucle, ignorée par la normalisation amont, jamais
retirée ; étiquette 3 dans l'empreinte de la base. **Critère 1** : empreinte de S372 inchangée, 45 essais V. **Critère
2** : 319 999 / 159 999 / 0 / 159 999 / 239 999 ml pour 320 000 / 160 000 / 0 / 160 000 / 240 000 (troncature, ≤ 1 ml).
**Erreur du critère écrit** : « bâche posée à 30 min : 240 000 » — c'est la valeur d'une demi-bâche ; bâche entière :
160 000 ; les deux éprouvés, dit dans l'essai. **Critère 4** : contenant plein fermé à la pluie ; bilan exact à chaque
pas (entré 62 221 ml, sorti 528 983) ; refus `Domain` (−1, NaN, ∞) et `Capacity` (`to ≠ from`) ; demi-bâche sauvegardée.

**P4 — fait.** La piscine à débordement sous 20 mm/h, pompe arrêtée, deux heures depuis le seuil : déversoir
**1,7778·10⁻⁴ m³/s = la pluie** ; charge **0,8560 mm pour 0,8569** (−0,11 %). Suite du cœur : 543 + 20 + 2 + 1, 0 avertissement.
