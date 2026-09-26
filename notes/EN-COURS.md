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

Session : S389 — **en cours**. **C4b**, première part : **la surface d'APIC** ([APIC3D-S388](../docs/validation/APIC3D-S388.md) :
la lecture de la surface commande la période). Demande de l'utilisateur (2026-09-26) : *« continue »* — troisième session du
fil (C4) : la voie de la v1 a été proposée à l'utilisateur en fin de S388, qui a demandé la suite ; sa demande prime
(REPRISE §6). Agent : Claude Opus 5.5, session cloud Claude Code ; fichiers, git, cargo, Python ; ni carte graphique, ni Godot.

**Thèse.** Calculée sur un réseau en `f64` pour une surface qui parcourt **continûment** une maille (huit positions), la lecture
de S388 — noyau d'une maille — se trompe jusqu'à **9,9 %** de maille (S388 n'avait regardé que deux positions, ±6,1 %). Le
**rayon du noyau** commande cette erreur, pas le nombre de particules : 3,6 % à 1,5 maille, **2,5 % à 2 mailles** ; 27
particules par maille n'y changent presque rien (2,4 %). Retenir un noyau de deux mailles, rayon au repos minimax sur les
positions continues ; puis remesurer les ballottements, dont la période devrait suivre.

**Critères, écrits avant.** (1) Le modèle `f64` et la reconstruction 3D lisent la même hauteur à 0,1 % de maille près, sur les
huit positions. (2) Lecture ≤ 2,5 % de maille sur toute position (critère 2 de S388, 1 %, reste manqué ; ce qui change est
publié). (3) Repos : ≤ 1 cm/s à 5 cm, masse exacte. (4) Les critères de S388, **inchangés** : (1, 0) ≤ 1 % à 2,5 cm, (1, 1) ≤ 2 %.
(5) **L'énergie, sur une mesure qui la porte** : l'amplitude du moment ne croît jamais — amortissement par période ≥ 0 par
régression sur les extremums (S354) ; publié. (6) Attribution : noyau d'une maille contre deux, même cas. (7) Suite
inchangée, zéro avertissement.

### Plan

- [x] **P1** — jeton, plan seul.
- [ ] **P2** — le modèle de lecture en `f64` dans le cœur, positions continues, rayon du noyau en paramètre ; rayon minimax ;
  critère 1.
- [ ] **P3** — la reconstruction à deux mailles ; critères 2 et 3.
- [ ] **P4** — les ballottements remesurés, l'amortissement par régression, l'attribution ; critères 4 à 6.
- [ ] **P5** — critère 7 ; preuve (section datée d'APIC3D-S388) ; liste, file, feuille de route.
- [ ] **P6** — rituel.

### Notes de reprise

*(vide)*
