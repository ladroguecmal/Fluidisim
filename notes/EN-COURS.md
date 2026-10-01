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

Session : S428 — **en cours**. Demande de l'utilisateur (2026-10-01) : *« Continue »* — la suite déclarée : **C7e**
([preuve](../docs/validation/APIC-CARTE-S416.md) §19 : B10 en bande étroite, pas + bascule 2,16 ms au p99 ; la projection 0,886 —
le groupe des niveaux grossiers, 18 µs par itération ; la part fixe du fil de l'échange, 53 µs).

**Avant ce plan — un écart à la procédure, déclaré.** Un essai a été écrit et mesuré avant que le plan ne soit posé : **le départ
chaud** de la projection (partir de la pression du pas précédent, `r = b − A·p`, même critère d'arrêt que la référence). Mesuré sur
B10 en bande étroite : 13,9 → 13,4 itérations, projection médiane 0,844 → 0,831 ms, p99 inchangé — la pression change trop d'un pas
à l'autre ; il rapproche la carte de la référence (premier écart de gestes au pas 50 au lieu de 34, `φ` max 4,8 mm au lieu de 6,8)
mais n'apporte rien au coût. **Retiré** (rien n'en est commité) ; consigné ici et dans la preuve.

**Ce que la session fait.** (1) **Les barrières à vide du groupe des niveaux grossiers** : la boucle de remontée a des bornes
constantes (FXC) et, au dernier tour (vers le niveau 1), deux phases de lissage sans travail mais avec leurs barrières ; le
chargement des natures et celui de `r₂` sont deux phases qui peuvent n'en faire qu'une — même arithmétique, moins de barrières.
(2) **La part fixe du fil de l'échange** : par face-maille active, une seule décision diffusée (retirer, poser, rien), et seulement la
partie qui sert — aujourd'hui `xg_remove_all`, la boucle de retrait d'origine et la réduction des poses s'exécutent toutes, chacune avec
ses diffusions. (3) Mesurer ; vérifier le déterminisme (`DUMP_B10`, deux exécutions).

**Critères, écrits avant.** (1) Issues identiques à S427 (gestes, étages, B10 en bande étroite et nu, bascules forcées, raccord,
bande) ; sinon l'écart isolé au bit. (2) Déterministe. (3) **Visé : pas + bascule ≤ 2 ms au p99** (2,16). (4) Suite, zéro
avertissement.

### Plan

- [x] **P1** — jeton, plan seul (et l'essai du départ chaud, déclaré).
- [ ] **P2** — les barrières à vide du groupe des niveaux grossiers ; identité, mesure.
- [ ] **P3** — la part fixe de l'échange ; identité, déterminisme, mesure.
- [ ] **P4** — non-régression, suite ; preuve §20 ; registres.
- [ ] **P5** — rituel.

### Notes de reprise
