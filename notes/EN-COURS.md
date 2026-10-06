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

Session : S510 — **terminée**. En autonomie, **9.5, le consommateur d'un événement prédit, confirmé ou rétracté** : le journal des impacts
(ADR-056) tient les causes, prédites puis confirmées ou rejetées, et rend `Change::Retract` — rien ne le consomme : la composition ne lit que
les confirmés. Et le lot des registres (dû).

**Ce que la session fait.** `wave_consumer` (cœur, sans allocation) : par cause, l'impact affiché et son poids ; une prédiction s'affiche
dès son admission ; une confirmation au même effet visible la garde telle quelle ; une confirmation corrigée fond enchaîné de l'une à
l'autre sur `τ` ; un rejet l'éteint en fondu sur `τ` ; chaque impact garde son âge (aucun retour du temps). Le chemin d'image seul : le jeu
ne lit que les confirmés (I-04).

**Ordre de grandeur, écrit avant.** Un fondu en `smoothstep` sur `τ` = 0,5 s a une pente maximale `1,5/τ` ; à 60 images/s, le saut d'une
image dû au fondu vaut au plus `1,5·(1/60)/0,5` = 5 % de l'amplitude de l'impact — contre 100 % pour un retrait sec (le témoin).

**Critères, écrits avant.** (1) confirmation au même effet : l'image identique au bit avant et après, et identique à celle du seul
confirmé ; (2) rejet : le saut d'une image dû au retrait ≤ 5 % de l'amplitude, nul après `τ` (le témoin sec : ≈ 100 %) ; (3) confirmation
corrigée (0,5 m plus loin) : saut ≤ 5 %, et après `τ` l'image du seul confirmé, au bit ; (4) l'âge de chaque impact suit l'horloge ; capacité
bornée, refus sans écriture. Si (1)–(4), 9.5 validée.

### Plan

- [x] **P1** — jeton, plan seul.
- [x] **P2** — `wave_consumer` ; essais (1)–(4).
- [x] **P3** — preuve ; liste 9.5 ; lot des registres ; rituel.

### Notes de reprise
- **P2** — `wave_consumer` ; critères 1 (au bit, 150 images), 2 (1,56 % ; témoin sec 8,6 % — la prévision « ≈ 100 % » supposait une
  crête), 3 (2,28 % ; au bit après le fondu), 4. Un essai mal compté (30 images = 499 980 µs < 500 000) corrigé. 669 essais.
- **P3** — preuve CONSOMMATEUR-S510 ; **9.5 validée** (9 / 120) ; lot : feuille de route (9 / 68 / 43, S507–S510), index ; journal.

