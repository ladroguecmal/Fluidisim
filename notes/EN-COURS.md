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

Session : S336 — **en cours**. **La coque qui cesse de pilonner** : δ mesure hors ligne la masse ajoutée et
l'amortissement par rayonnement de la coque de la porte D, que le corps du jeu reçoit comme des constantes de
son archétype — ADR-008 §2, I-04 intact ; chemin de la porte D ([ADR-189](../docs/adr/ADR-189-la-v1-d-abord.md)).
Agent : Claude Opus 5.5, application desktop ; fichiers, git, cargo, carte réelle, accès web.
Entrée : *« Continue »* ; suite déclarée par S335. Verdict visuel de la porte D toujours attendu (R15).

**Ce que la session doit rendre possible.** Aujourd'hui la coque du jeu, lâchée de 10 cm, pilonne sans fin —
± 12 cm pendant les 8 s de la scène — pendant que δ emporte l'énergie de ses anneaux : le jeu et l'eau se
contredisent, et l'œil le voit. **La physique linéaire le dit** : en pilonnement imposé `z = Z·sin ωt`, la force
de δ sur la coque vaut `A·ω²Z·sin ωt − B·ωZ·cos ωt` — `A` la masse ajoutée, `B` l'amortissement de rayonnement.
δ les mesure, le jeu les reçoit à la pulsation propre qu'ils déterminent, `ω² = K/(m + A)`.

Critères, écrits avant le code :
1. **`A(ω)` et `B(ω)`** de la coque 4 × 1,6 × 1 m en pilonnement imposé, δ à 25 cm, placement 30/30, quatre
   pulsations 3 à 4,5 rad/s ; la force se décompose à 5 % près — résidu de l'ajustement sous 5 % de son
   amplitude. À 12,5 cm, deux pulsations : l'écart à 25 cm publié, comme la résolution d'A317 le demande.
2. **Le corps du jeu amorti** : `radiation_damping`, linéaire en la vitesse relative à l'eau ; lâché en eau
   calme, il pilonne à `ω'` et s'amortit au taux `ζ = B/(2√(K(m + A)))`, les deux à ± 2 % ; valeurs par défaut
   nulles, C10 et S331–S333 inchangés au bit.
3. **La scène de la porte D** avec ces constantes : critères 3 à 5 de S333 ; **bilan** — l'énergie que la
   coque perd par son amortissement et celle que δ reçoit de sa paroi, à ± 25 % l'une de l'autre sur la scène.
4. **Images** refaites.

### Plan

- [x] **P1** — jeton, plan seul.
- [ ] **P2** — banc `rayonnement_coque` : pilonnement imposé, force de δ, `A` et `B` ; critère 1.
- [ ] **P3** — le corps du jeu amorti, constantes de l'archétype ; critère 2.
- [ ] **P4** — la scène de la porte D amortie ; critère 3 ; images (critère 4).
- [ ] **P5** — preuve `docs/validation/RAYONNEMENT-COQUE-S336.md` ; liste, file, feuille de route.
- [ ] **P6** — rituel.

### Notes de reprise

