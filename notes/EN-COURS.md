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

Session : S397 — **en cours**. **C5a, troisième part** ([ADR-207](../docs/adr/ADR-207-la-campagne-du-solveur-volumique-3d.md) D5 ;
[B10-APIC-S320](../docs/validation/B10-APIC-S320.md) §15) : trancher la source de la circulation permanente à la frontière du
raccord. Demande de l'utilisateur (2026-09-26) : *« Continue, l'objectif est de peaufiner et terminer le solveur »* — le
raccord (C5) est le verrou de la campagne : il porte C6 (la bascule), puis C7 et C10. Agent : Claude Opus 5.5, session cloud
Claude Code ; fichiers, git, cargo, Python ; ni carte graphique, ni Godot. Sert 4.12, 4.16, A316.

**Thèse (H1, de S395).** Dans la zone des colonnes, les particules sont réensemencées à chaque pas sur des points fixes, à la
vitesse que la grille a **en ces points** : la quantité de mouvement n'y est jamais transportée — le terme `(u·∇)u` existe du
côté des particules, pas de l'autre. Pour une onde stationnaire, sa moyenne (une contrainte de Reynolds, `ρ⟨u²⟩`) n'est
équilibrée que d'un côté de la frontière : le reste y entretient une circulation moyenne. **Épreuve directe** : réensemencer
les colonnes avec la vitesse de la grille **au pied de la caractéristique** — au point `x − dt·u(x)` —, une advection
semi-lagrangienne de la quantité de mouvement (`RACCORD_ADVECTION=1`). **Prédiction** : la circulation tombe au niveau d'APIC
seul.

**Critères, écrits avant** (paroi, 30 s ; ceux de S394–S395). (1) Sans variable, au bit. (2) **H1** : avec l'advection, 5 cm, la
vitesse moyenne sur la face, |ū| ≤ **5 mm/s** à toute profondeur (S395 : 54 ; APIC seul : 1,9) — sinon H1 est **réfutée**, et
publiée comme telle. (3) Si H1 tient : masse à gauche ±0,002 m² d'APIC seul par tranche, densité 4 ± 0,2, saut < 0,5 maille,
période et amortissement à 1 point d'APIC seul, à 5 et 2,5 cm ; repos < 1 cm/s.

### Plan

- [>] **P1** — jeton, plan seul ; la décision de l'utilisateur consignée.
- [ ] **P2** — l'advection des colonnes (`RACCORD_ADVECTION=1`) ; le profil de la face ; critères 1 et 2.
- [ ] **P3** — les critères de S394 avec l'advection, 5 et 2,5 cm, repos ; critère 3 ; attribution si manqué.
- [ ] **P4** — preuve (§16 de B10-APIC-S320) ; A316, file, liste.
- [ ] **P5** — rituel.

### Notes de reprise

