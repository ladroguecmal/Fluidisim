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

Session : S395 — **en cours**. **C5a, deuxième part** ([ADR-207](../docs/adr/ADR-207-la-campagne-du-solveur-volumique-3d.md) D5 ;
[B10-APIC-S320](../docs/validation/B10-APIC-S320.md) §14) : A316 en 2D — **où l'échange comprime**, puis un échange qui ne
comprime pas. Demande de l'utilisateur (2026-09-26) : *« Continue »*. Agent : Claude Opus 5.5, session cloud Claude Code ;
fichiers, git, cargo, Python ; ni carte graphique, ni Godot ; articles bloqués par le réseau. Sert 4.12, 4.16, A316.

**Thèse.** L'échange du montage paroi passe l'eau **profondeur par profondeur** : ce qui sort des colonnes à la profondeur `k`
devient une particule posée à la profondeur `k`, dans une eau déjà pleine — rien n'est poussé au-dessus, la densité monte,
la surface ne monte pas, et la pression, qui voit la surface, ne s'y oppose pas ; un retrait à la profondeur `k` creuse sans
que la surface baisse. Côté colonnes, la même eau change `h`, donc la surface. Un fluide incompressible, lui, pousse ce qui
est au-dessus : **l'eau échangée à la profondeur `k` équivaut, en volume, à de l'eau ajoutée ou ôtée à la surface.**
**Prédiction** (P2) : l'excès de densité de S354 se loge sous la surface, aux profondeurs où l'on insère ; le bilan par
profondeur de la dernière colonne libre le montre. **Remède (C)** : insérer et retirer **au sommet** de la dernière colonne
libre — la particule la plus haute part ; la nouvelle se pose sur la rangée du haut —, le solde de l'échange tenu en un seul
compte ; le reste du montage paroi inchangé.

**Critères, écrits avant** (paroi, 30 s, 5 et 2,5 cm ; ceux de S394). (1) Sans variable, au bit. (2) Masse exacte. (3) Masse à
gauche de la frontière à ±0,002 m² d'APIC seul par tranche de 10 s. (4) Densité 4 ± 0,2 en `i_b − 1`. (5) Saut < 0,5 maille ;
période aux zéros et amortissement (régression) à 1 point d'APIC seul. (6) Repos à 5 cm < 1 cm/s. Publié : (C) avec la
correction (B) de S394 — son énergie ajoutée, qui devrait tomber près de zéro si (C) ne comprime plus.

### Plan

- [>] **P1** — jeton, plan seul.
- [ ] **P2** — l'instrument : bilan par profondeur de la dernière colonne libre (insertions, retraits, densité), 30 s, contre
  APIC seul ; la prédiction.
- [ ] **P3** — (C), l'échange au sommet ; critères 1 à 6 ; avec (B), publié.
- [ ] **P4** — preuve (§15 de B10-APIC-S320) ; A316, file, liste.
- [ ] **P5** — rituel.

### Notes de reprise

