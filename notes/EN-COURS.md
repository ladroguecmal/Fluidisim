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

Session : S540 — **terminée**. En autonomie, **C13 — la remontée d'une bulle** (non exécuté) : des bulles de 0,1, 1 et 5 mm lâchées à 3 m
de profondeur ; vitesse terminale à ± 15 % de SPEC-002 §2 (5,5 mm/s ; 0,12–0,25 m/s ; ≈ 0,25 m/s) ; trajectoire selon `−g_eff`, pas selon
`+Z`. Les grosses bulles d'APIC existent (S479–S485) ; les petites, qui portent les microbulles (7.3, absent) et l'aération, non.

**Ce que la session fait.** `bulle.rs` : une bulle ponctuelle — poussée `(ρ − ρ_air)·V·(−g_eff)`, masse ajoutée ½ρV, traînée de Tomiyama
pour bulles contaminées (`C_D = max(24/Re·(1 + 0,15 Re^0,687), (8/3)·Eo/(Eo + 4))` : Schiller–Naumann, puis le régime où la bulle se
déforme), la vitesse relative à l'eau ; `vitesse_terminale(d, g, milieu)` par point fixe. Eau douce de SPEC-002 : ρ = 1 000, μ = 10⁻³ Pa·s,
σ = 0,072 N/m.

**Ordre de grandeur, calculé (la loi d'abord, au plan).** 0,1 mm : **4,99 mm/s** (Stokes 5,45 : −8 %, `Re` 0,5) ; 1 mm : **0,112 m/s**
(table 0,12–0,25 : −6 % sous le bas, `Re` 112) ; 5 mm : **0,231 m/s** (table 0,25 : −8 %, `Eo` 3,4). Remontée de 3 m : 602 / 27 / 13 s.

**Critères, écrits avant.** (1) La vitesse atteinte par l'intégration (lâchée au repos, à 3 m) à 10⁻⁶ de `vitesse_terminale` (le quantum :
l'arrondi f64, rapport > 10⁶). (2) **C13** : à ± 15 % de la table (la fourchette du 1 mm élargie de 15 % de part et d'autre). (3) Sous un
`g_eff` incliné de 20°, la trajectoire selon `−g_eff` à 10⁻⁹ rad.

### Plan

- [x] **P1** — jeton, plan seul.
- [x] **P2** — la bulle, les essais ; (1)–(3).
- [x] **P3** — preuve ; listes 7.4, 13.2 ; C13 ; rituel.

### Notes de reprise
- **P2 fini** — `bulle.rs` ; essais `s540` : 4,98 mm/s, 0,1124, 0,2309 m/s, intégrées à 10⁻⁶ ; selon −g_eff. Suite 702. C13 est rangé sous
  7.4 dans la liste (pas 7.3, les microbulles visuelles, qui restent absentes).
- **P3** — preuve C13-BULLES-S540 ; listes 7.4, 13.2 ; C13 ; index ; journal.

