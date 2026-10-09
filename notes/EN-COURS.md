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

Session : S731 — **en cours**. La cinquantième revue de méthode (ADR-222 D4), sur S726–S730, et le lot S729–S731 (ADR-213 D3).

**Ce que la session fait.**
- Relire les frictions de S726 à S730, et décider (ADR-285). Les trois relevées :
  1. **le témoin tout-3D de la plage portait un raccord** (Saint-Venant au rivage, à 10,775 m) dont personne n'avait mesuré le saut. De
     S717 à S730, ses remontées et la conclusion de S718–S719 (« le large SGN en cause ») en dépendaient. C'est l'utilisateur qui l'a vu,
     sur une image (R43) ;
  2. **le filtre d'`essai.py`** est une sous-chaîne : E1 de S730 a lancé E2 aussi, et dans l'autre ordre ;
  3. **un ADR créé sans sa ligne d'index** a fait refuser la fermeture de S730 ; l'arbre est resté modifié, à restaurer à la main.
- **La demande de l'utilisateur** (2026-10-09, après S730) : le système qui choisit le type de simulation (SGN, la 3D, Saint-Venant) doit être
  la pièce la plus peaufinée et la plus solide. La revue en fait la campagne suivante, avec ses juges.
- `essai.py` : un nom qui en désigne plusieurs est refusé, sauf s'il en désigne un exactement (alors seul lui tourne).
- Le lot : FEUILLE-DE-ROUTE et ROADMAP-VIVANTE (S729–S731).

**Critère** : `essai.py` ne lance que l'essai nommé quand un autre nom le contient. Vérifié sur `the_shore_relay_beyond_the_jet_s730`, par
la liste des essais seulement (sans rien lancer de long : `--liste`).

### Plan

- [x] **P1** — jeton ; plan.
- [ ] **P2** — `essai.py` ; ADR-285 ; METHODE ; le lot.
- [ ] **P3** — fermeture.

### Notes de reprise
