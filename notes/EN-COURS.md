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

Session : S443 — **en cours**. Demande de l'utilisateur (2026-10-02) : *« J'accepte ta proposition »* (la bascule sans attendre C7d-3a),
puis, sur les quatre propositions pour accélérer, *« Ok go »* — [ADR-213](../docs/adr/ADR-213-accelerer-tolerance-plafond-rituel-bancs.md)
(tolérance de 5 %, plafond de deux sessions, registres par lots de trois, bancs courts).

**Ce que la session fait.** **La bascule des défauts** (C7d-3b, seconde moitié) : `Volume3` naît en mode relatif (`RELATIVE_ALL`,
fantôme latéral d'A324, bande sous Lax-Wendroff) ; `Step3` aussi (pipelines relatifs compilés à la création). Le pas de S297 reste
atteignable (`set_relative_background(0)`, `set_relative(false)`, `RELATIF=0` aux bancs). Les essais du cœur qui mesurent le pas de S297
l'épinglent explicitement.

**Critères, écrits avant.** (1) La suite du cœur passe, zéro avertissement ; chaque essai épinglé au pas de S297 est nommé. (2) Le pas
de S297 reste **au bit** : `RELATIF=0 --delta3d-trajectoire` identique à la ligne de base de S439. (3) Sous les nouveaux défauts : la
trajectoire carte–référence tient le critère de S439 (écart avant l'horizon ≤ 1,53·10⁻⁴ m) ; le témoin nul au bit ; `--delta3d-cas2`
(porte B, critère 2 : la carte suit la référence sous 3 mm) tenu. (4) **Le coût** : le pas relatif de la scène de revue à 25 cm au plus
5 % plus cher que celui de S297 (ADR-213 D1 en sus). (5) La scène de revue à 25 cm et 30 Hz tient 120 s. Bancs courts (D4).

### Plan

- [x] **P1** — ADR-213 ; jeton, plan seul.
- [x] **P2** — la bascule : cœur et carte ; suite.
- [ ] **P3** — les rejeux ; le coût.
- [ ] **P4** — preuve ; rituel (allégé, ADR-213 D3).

### Notes de reprise
- **P2** — `Volume3` naît en `RELATIVE_ALL` ; `Step3` appelle `set_relative(true)` à sa création. Épinglés au pas de S297 (ils
  le mesurent) : `coupled_geometry_zero_and_oblique_ghosts_s297` (un fantôme non nul sous la seule élévation de B) et
  `coupled_transverse_invariance_and_rotation_s297` (δ qui croît depuis zéro sous B seul — nul en mode relatif). Bancs : `RELATIF=0`
  rend le pas de S297 (`--delta3d-trajectoire`, `--delta3d-a321`, `--delta3d-cout-scene`). Suite **759**, zéro avertissement.

