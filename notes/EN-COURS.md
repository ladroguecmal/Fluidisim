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

---

## Session en cours

Session : S220 — en cours
Agent : Claude Code, Opus 5 (fichiers, git et cargo disponibles)
Entrée : « Reprends le projet » ; master et trois copies propres à 8ef0c64, jeton libre,
maillons 0. AGENTS, REPRISE (jeton, file active, §4 S219–S217, §5–§9), EN-COURS, journal
S218–S219, PARTITION-S219, ADR-135, code local_bound/partition/phase lus. Copie principale.

### Thèse et plan

A259 : la borne ADR-135 somme des **modules** de variation, `c_k·min(2,D_k)` ; aux mailles
moyennes cette somme ne voit aucune annulation entre modes. Développer chaque mode à l'ordre
deux autour de la phase **exécutée** au centre : `a sin(ψ+δ) = a sin ψ + a cos ψ·δ + R`,
`|R| ≤ a·δ²/2`. Le terme `a cos ψ` est exactement `η_k(c)` : la Hessienne de la pente vaut
`M = −Σ w_k ⊗ (2π t_k) η_k(c)`, **signée**, et `|S(c) + M u|` est convexe en `u`, donc son
maximum sur le rectangle est à un coin. Écart exécuté/linéaire `e_k` (arrondi des produits
`t·x`, fraction, Q32, arrondi des coins) majoré par `E_k` et payé `c_k E_k`. Choix par mode :
dans `M` si `E_k + D_k²/2 < min(2, D_k)`, sinon terme ADR-135. Retenir le minimum des branches
ordre un, ordre deux et globale, chacune avec sa réserve : jamais pire qu'ADR-135.
Au maximum de `|S|`, `S·Mu = 0` : l'excès devient quadratique, ce qu'ADR-135 ne peut pas.
Critères déclarés avant mesure : sondes sous la borne (pic manqué, multidirectionnel, 4000 m) ;
borne ordre deux ≤ ordre un sur tout rectangle ; gain strict près d'un maximum ; S219 inchangée
en ordre un au bit. Gain de partition non promis. Aucune admission migrée, A258 reste ouverte.

- [x] **P1** — jeton et plan seuls.
- [ ] **P2** — ADR-136 : dérivation, écart de phase quantifiée, choix par mode, réserve, limites.
- [ ] **P3** — construire la branche ordre deux (Field, Prepared) sans changer ADR-135 ni S219.
- [ ] **P4** — tests : couverture, domination, gain près du maximum, 4000 m, refus, identité S219.
- [ ] **P5** — exemple S220 : même rectangles S218 et partition S219 par ordre ; campagne isolée.
- [ ] **P6** — publier la réception S220 et ses relevés bruts.
- [ ] **P7** — rituel §6, journal, registres, index, file active, jeton et copies.

### Notes de reprise

Suite S219 : partition adaptative disponible, A259 (plateau des grandes mailles) ouverte.
