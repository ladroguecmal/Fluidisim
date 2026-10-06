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

Session : S517 — **terminée**. En autonomie, **4.13, la coque en marche et sa vague d'étrave** : la liste dit « Manquent la coque en marche
et sa vague d'étrave, la gerbe (4.16), la résolution près de la coque (± 43–49 % à 25 cm) et la production GPU ». La coque mobile tourne
sur la carte depuis S503–S509 ; son sillage dans δ n'a jamais été jugé.

**Ce que la session fait.** Un banc sur la carte (`--lineaire-sillage`) : la coque de la porte D menée à 3 m/s (départ en rampe d'1 s) dans
un δ de 48 × 24 × 4 m (192 × 96 × 16 mailles de 25 cm), éponges aux bords ; le cœur ne sert que de découpeur (recoupage en boîte, S508).
Après 10 s, le sillage : pour chaque distance derrière la coque, la position latérale de la plus forte élévation hors de l'axe ; la droite
de ces points donne le demi-angle.

**Ordre de grandeur, calculé.** Profondeur 4 m : Froude de profondeur `U/√(gh)` = 0,48 (sous-critique : l'angle de Kelvin des eaux
profondes, 19,47°) ; onde transverse `λ = 2πU²/g` = 5,76 m (`kh` = 4,4 : eau profonde) ; à 20 m derrière, la ligne des cuspides passe à
7,1 m de l'axe — dans le domaine (12 m de demi-largeur). 295 000 mailles : la carte seule le permet.

**Critères, écrits avant.** (1) le demi-angle du sillage à 2° de 19,47° ; (2) aucune instabilité (l'élévation bornée, le volume tenu à
10⁻⁶ m³ hors éponge) ; (3) le coût par pas publié. 4.13 : la coque en marche et sa vague dans δ, sur la carte.

### Plan

- [x] **P1** — jeton, plan seul.
- [x] **P2** — le banc ; (1)–(3).
- [x] **P3** — preuve ; liste 4.13 ; rituel.

### Notes de reprise
- **P2 (en cours)** — premier instrument (maximum latéral) : 2° — il prend le champ proche. Second (le bord du coin à un seuil) : domaine
  64 × 24 m saturé (le coin touche les éponges, les ondes du démarrage remplissent le domaine) → 64 × 48 m, rampe de 3 s : **24,1 / 21,2 /
  14,6° aux seuils 0,2 / 0,3 / 0,4** — l'instrument ne tranche pas (retenir 0,3 serait choisir après coup). **Troisième instrument, déclaré
  avant de le lancer** : la moyenne de |η| le long des rayons issus de l'étrave, de 10 à 24 m, pour chaque angle de 5 à 35° (pas de 0,5°) ;
  l'angle du maximum contre 19,47° à 2°. L'élévation maximale, 0,72 m, est sur l'étrave (la stagnation, `U²/2g` = 0,46 m, amplifiée par le
  couvercle partiel). Recoupage CPU 25 ms par pas sur 786 000 mailles (les boucles entières du cœur et de la géométrie dominent).
- **P2 fini** — instrument 3 (déclaré avant) : maximum à 6,5° (transverses), local à 17°, amplitude ÷2 entre 21 et 25° → **(1) manqué**,
  pas de quatrième instrument. (2) tenu. (3) 25,5 ms : **A329** ouverte (les boucles entières du recoupage sur un grand domaine ; 6.4 à
  0,82 ms ne vaut que pour un petit domaine — la liste le dit).
- **P3** — preuve SILLAGE-S517 ; listes 4.13 et 6.4 (limite) ; A329 ; index ; journal.

