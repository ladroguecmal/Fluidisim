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

Session : S442 — **en cours**. Demande de l'utilisateur (2026-10-02) : *« J'accepte et continue »* — **A322 levée en mode relatif**,
l'écart accepté ([preuve](../docs/validation/MULTIGRILLE-3D-S385.md) §8).

**Ce que la session fait.** (1) **La bande sous Lax-Wendroff devient le défaut du mode relatif** — `Volume3` (`band_lax_wendroff`
vrai) et `Step3` (`band_lw` vrai) ; sans effet hors du mode relatif ; `BANDE_LW=0`, `MER_BANDE_LW=0` rendent l'ancienne pour
comparaison. (2) **Son effet sur A320** : le germe de 1 mm sous la houle de 4 m à 25 cm — 7,5 cm, repos sur une face et au centre ;
6 cm sur une face — contre S437 (0,106 · 0,036 · 0,077 s⁻¹). La bande croisée retirée ne freinait A320 que de 20 % (S369) ; mais le
FTCS est de ceux qui dépendent de la place de la surface.

**Critères, écrits avant.** (1) **La bascule** : la production (sans `RELATIF`) au bit ; le témoin relatif nul au bit (carte et
référence) ; la suite du cœur, zéro avertissement. (2) **A320** : mesuré et écrit ; **C7d-3a reçu** si, avec la bande sous
Lax-Wendroff, le taux de la bande de Benjamin-Feir tient le critère de S437 (au plus 1,5 fois `ω(ak)²/2` sous 6 et 7,5 cm, toute place
du repos : 0,026 et 0,041 s⁻¹) ; autrement, ce que le remède change d'A320 s'écrit.

### Plan

- [x] **P1** — jeton, plan seul ; l'arbitrage inscrit.
- [x] **P2** — Lax-Wendroff par défaut ; production au bit, témoin, suite.
- [x] **P3** — A320 sous Lax-Wendroff.
- [ ] **P4** — preuve ; registres ; rituel.

### Notes de reprise
- **P2** — `Volume3::band_lax_wendroff` et `Step3::band_lw` vrais par défaut ; `MER_BANDE_LW=0`, `BANDE_LW=0` rendent la bande centrée.
  **Production au bit** (la sortie du banc de trajectoire identique à la ligne de base de S439) ; **témoin relatif nul au bit** sur
  400 pas, carte et référence ; trajectoire relative à 1,44·10⁻⁵ m de sa référence ; suite **759**, zéro avertissement — (1) tenu.
- **P3** — 25 cm, houle de 4 m, germe de 1 mm, taux de la bande 35–59 s, la bande sous Lax-Wendroff (S437 entre parenthèses) :
  7,5 cm — sur une face **0,1057** (0,1059), à ¼ **0,0676** (0,0680), au centre **0,0345** (0,036) ; 6 cm — sur une face **0,0768**
  (0,0769), au centre **0,0197** (0,018). Le témoin nul au bit partout. **A320 inchangée** : à 25 cm, `C` est trop petit pour que le
  FTCS de la bande y pèse. **C7d-3a non reçu.**

