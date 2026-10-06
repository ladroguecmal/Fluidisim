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

Session : S504 — **en cours**. En autonomie, **6.4, la coque qui perce la surface en mouvement sur la carte** (seconde part de S503).
La référence, pour une coque qui perce le couvercle : (a) la pression d'un couvercle en partie couvert retire le dépôt du pas (le flux de
paroi l'emporte pendant ce pas) ; (b) quand l'ouverture du couvercle d'une colonne se referme, l'eau de surface de la part recouverte
passe aux voisines de la couche du haut, au prorata des faces partagées et de leurs couvercles (S334).

**Ce que la session fait.** Le cœur expose les poids du transfert (le rapport de fermeture de chaque colonne et ses quatre parts) ;
`set_motion` les reçoit ; deux noyaux sur la carte (rassembler ce que chaque colonne reçoit de ses voisines, puis appliquer en somme
compensée) ; `lid_partial` retire le dépôt du pas ; le dépôt remis à zéro au pas suivant sans mouvement. Un banc : la coque de la porte D
en pilonnement imposé (5 cm, 3,5 rad/s), puis en roulis (0,05 rad), carte contre référence.

**Ordre de grandeur, écrit avant.** Le terme concurrent : l'élévation que la coque rayonne, ≈ 1 cm (S336 : la force de δ ≈ 5 kN pour 5 cm
de pilonnement) ; S503 tenait 2·10⁻⁶ m pour un solide immergé. Le transfert ajoute un rassemblement en flottant au lieu d'une somme
exacte : un écart de l'ordre de l'arrondi f32 de la surface (10⁻⁷ m), sans effet visible.

**Critères, écrits avant.** (1) les bancs de S358 et de S493 (cloison) inchangés ; (2) la coque en pilonnement puis en roulis : la carte à
10⁻⁴ m de la référence, l'élévation ≥ 10 × l'écart ; (3) les volumes à 10⁻⁶ m³ ; (4) le coût, publié.

### Plan

- [x] **P1** — jeton, plan seul.
- [ ] **P2** — les poids du transfert (cœur) ; dépôt et transfert sur la carte ; (1).
- [ ] **P3** — le banc de la coque ; (2)–(4).
- [ ] **P4** — preuve ; liste 6.4 ; rituel.

### Notes de reprise
