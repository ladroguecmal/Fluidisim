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

Session : S447 — **terminée**. Demande de l'utilisateur (2026-10-02) : *« Continue »* — la suite déclarée : **c2, le raccord
conservatif**, seconde et dernière session ([preuve](../docs/validation/APIC-CARTE-S416.md) §23.3).

**La conception.** **La mer est la seule comptable de la masse de δ** : son pas avance toutes ses colonnes, intérieur de la bande
compris, par ses propres débits — conservatifs. À l'intérieur de la bande, elle ne reçoit plus la hauteur de la bande telle quelle,
mais sa **forme** : `δη = (η_bande − η_B) + c`, `c` uniforme, choisi pour que le volume de δ de l'intérieur reste celui que la mer vient de
calculer ; la bande reçoit le même `c` sur les mêmes colonnes, pour que les deux ne divergent pas. Les vitesses, comme en S446.
`RACCORD_CONSERVATIF=0` rend le raccord de S446.

**Critères, écrits avant.** Ceux de S446 : sous B seul (5 cm, 4 m), 10 s — (1) δ hors de la bande sous **1 cm** ; (2) la dérive du
volume de δ de la mer sous **1 %** du volume d'une demi-période (3,2·10⁻² m³) ; la suite, zéro avertissement. Reçu : c2 ; sinon, c2 plafonné
(ADR-213 D2), la limite écrite, et c3 commence quand même avec le raccord mesuré.

### Plan

- [x] **P1** — jeton, plan seul.
- [x] **P2** — le raccord conservatif au banc ; mesures ; critères.
- [x] **P3** — preuve ; rituel (allégé).

### Notes de reprise
- **P2** — `raccord_bande_mer`, raccord conservatif (le défaut ; `RACCORD_CONSERVATIF=0`, celui de S446), marge 2, 10 s : δ hors de la
  bande **9,42 mm** — (1) tenu ; dérive brute du volume de δ **1,13·10⁻³ m³, 3,5 %** d'une demi-période — (2) **manqué tel qu'écrit** ;
  mais le bilan du pas de la mer l'explique : l'éponge et la bande de B aux faces extérieures, **8,5·10⁻⁴ m³** ; ce qui reste au raccord,
  **3,9·10⁻⁶ m³ — 0,01 %** (S446 : 2,2·10⁻³, 6,8 %). Le critère (2) mêlait l'éponge — qui absorbe les ondes de δ sorties de la bande, son
  rôle — au raccord ; la mesure qui isole le raccord le tient cent fois. Marges 1 et 4 : 12,6 et 8,1 mm, 3,1 et 4,7 % bruts. Suite **762**.
  **c2 : non reçu tel qu'écrit ; plafonné (ADR-213 D2)** — le raccord conservatif est retenu, c3 commence avec lui.
- **P3** — APIC-CARTE §23.4 ; journal ; jeton libre ; maillons 4 (justifiés : S406) ; suivant : S448, c3 et le lot des registres.
