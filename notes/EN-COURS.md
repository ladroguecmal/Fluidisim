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

Session : S480 — **terminée**. **Décision de l'utilisateur** (« Ok ») sur la proposition faite à ses questions — reprendre depuis une
autre conversation, les défauts de méthode, la structure de gestion (*« avoir en contexte les choses essentielles nouvelles mais
aussi les intentions initiales […] pour ne pas travailler dans le flou »*). K2-2 passe en S481.

**Ce que la session fait.** (1) **`BOUSSOLE.md`** — deux pages lues en premier : pourquoi (l'eau de DyingStar, une surprise), les
intentions d'origine, les décisions en vigueur en une ligne chacune, ce que l'utilisateur juge et ce qui se tranche ici, ce qui
attend l'utilisateur. (2) **Trois registres générés** (un outil chacun, un `--check` tenu par `etat_projet.py`) : le **tableau de
bord vers 100 %** (les points par campagne, l'historique du décompte), les **décisions en vigueur** (les ADR, leur statut, qui en
remplace ou en précise qui), les **anomalies ouvertes** (celles dont l'en-tête porte un statut). (3) **Le journal découpé** : les
entrées avant S470 archivées. (4) **Les calculs longs** : `outils/calcul.py` les lance hors de la conversation, sortie dans
`calculs/` (non versionné), registre `notes/CALCULS.md` versionné. (5) **Le rituel outillé** : `outils/rituel.py debut|fin`.
(6) **REPRISE** : l'ordre de lecture et l'état renvoient à la boussole et au tableau de bord (son §4 décrivait encore S351) ; la
règle « le plan déclare ses entrées et comment il les vérifie ».

**Critères, écrits avant.** (1) une session froide trouve en deux pages le pourquoi, les décisions en vigueur et la suite ; (2) les
trois registres régénérés à l'identique par leur outil, `--check` à 0 ; (3) le journal archivé sans perte (le nombre d'entrées
avant = après) ; (4) un calcul lancé par `calcul.py` survit à la conversation et se retrouve par le registre ; (5) le rituel de
fin de S480 fait par `rituel.py`.

### Plan

- [x] **P1** — jeton, plan seul.
- [x] **P2** — la boussole ; REPRISE et AGENTS.
- [x] **P3** — les trois registres générés, tenus par `--check`.
- [x] **P4** — le journal archivé (P4a) ; `calcul.py` (P4b) ; `rituel.py` (P4c) — trois commits.
- [x] **P5** — preuve ; rituel (par `rituel.py`).

### Notes de reprise
- **P2** — `BOUSSOLE.md` (le pourquoi, DyingStar et la surprise, la fin à 100 %, l'architecture en quatre lignes, quinze décisions
  en vigueur, ce que l'utilisateur décide et ce qui attend de lui, la méthode, les pièges) ; AGENTS : la boussole d'abord ; REPRISE :
  §2 la règle des entrées vérifiées et des calculs longs, §3 l'ordre de lecture (boussole, tableau de bord, plan de complétion,
  décisions et anomalies générées), §4 renvoie au tableau de bord (il décrivait encore S351).
- **P3** — reprise à chaud à 16:37 (battement 12:03) : P2 complétée (diff cohérent, coché), committée. Trois outils, un registre
  chacun, `ecarts` appelé par `etat_projet.py --check` : `tableau_de_bord.py` (120 points, 3 validés, 74 partiels, 43 absents ; 0 hors
  campagne), `decisions.py` (220 ADR lus en tête : 169 actées, 49 proposées, 2 rétractées en partie ; colonnes « nomme » et « nommé
  par » ; 38 Ko), `anomalies.py` (39 entrées à statut d'ANGLES-MORTS, 24 ouvertes). **Trouvé** : la levée d'A322 (S442) n'avait
  jamais été écrite au registre source — note datée ajoutée. Pièges de lecture levés : titre dans un second gras, une note sur A324
  écrite sous A320 (une note qui nomme une autre anomalie ne tranche plus), A303 donné deux fois (« A303 bis »).
- **P4a** — le journal : 1,36 Mo → 7 Ko (S470–S479) ; S01–S469 dans `notes/journal/`, cinq archives par centaine (99 + 100 + 100
  + 100 + 70 + 10 vivantes = 479 entrées, avant comme après ; texte identique à l'espacement près) ; liens relatifs recalés d'un
  niveau, 0 lien mort dans les archives.
- **P4b** — `outils/calcul.py` (lancer, etat, garder) ; `calculs/` ignoré par git, `notes/CALCULS.md` versionné. Essai : un calcul
  de 45 s lancé, le shell qui l'a lancé fermé aussitôt, retrouvé « terminé » par le registre ; la sortie d'un processus détaché
  n'arrivait pas au journal (pas de console) — poignées explicites. **Les sorties de S479 mises à l'abri** (`calculs/…-s479-b10-bulle`) :
  `b10_p24.txt` est **vide** (le calcul est mort avec sa conversation) ; `bulle_b.txt` : la grande cuve (80×80×68, dx 2 cm, R 8 cm)
  oscille à **47,84 Hz contre 42,22** de Minnaert (rapport 1,133, six périodes, volume moyen 2,03·10⁻³ contre 2,14 pour la sphère) —
  à consigner dans POCHES-AIR-S479 par K2-2 ; B10 à `D/dx` = 24 est à relancer, par `calcul.py`.
- **P4c** — `outils/rituel.py` : `debut` (l'amorce, le jeton et son avis, le plan, le tableau, les calculs), `fin` (vérifie cases et
  journal, régénère les trois registres, libère le jeton, coche, `etat_projet --check`) ; essai du refus : P4 non cochée et pas
  d'entrée S480 → deux MANQUE, rien d'écrit. `calcul.py` : `VAR=valeur` avant `--`, inscrits avec la commande. **B10 à `D/dx` = 24
  relancé** par l'outil à 16:52 (`calculs/20261004-165215-b10-poches-p24`, ≈ 2 h 30) : K2-2 le consigne à son arrivée.
- **P5** — preuve STRUCTURE-S480 ; ADR-221 (la décision) ; le lot des registres (index : boussole, registres générés, plan de
  complétion, K2, POCHES-AIR-S479, STRUCTURE-S480, ADR-221, archives du journal ; file active : la décision du jour ; feuille de route :
  S478–S480 ; boussole : la ligne « structure ») ; journal ; rituel par `rituel.py fin --lot`.

