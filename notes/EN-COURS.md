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

Session : S394 — **en cours**. **C5a** de la campagne ([ADR-207](../docs/adr/ADR-207-la-campagne-du-solveur-volumique-3d.md) D5) :
**A316 d'abord en 2D** — tenir la densité des particules à la frontière du raccord particules ↔ colonnes. Demande de
l'utilisateur (2026-09-26) : *« Continue »*. Agent : Claude Opus 5.5, session cloud Claude Code ; fichiers, git, cargo,
Python ; ni carte graphique, ni Godot ; articles bloqués par le réseau. Sert **4.16**, **4.12**, A316.

**Pourquoi en 2D, et ce qui n'a pas été lu.** C5 demandait de lire d'abord Chentanez, Müller et Kim (2014) : tous ses sites
sont bloqués ; le résumé seul est connu — la surface suivie par un **champ de densité**, somme de celle des particules et de
celle de la grille. La campagne le prévoit : *une session qui ne peut pas les lire le dit et avance sur ce qui n'en dépend
pas*. Le défaut à lever, A316, est connu en 2D (S354 : la frontière convertit un débit en particules à densité nominale,
les particules s'y tassent — 5 par maille au lieu de 4 — et la masse migre, +12 mm en 30 s à 5 cm). Le banc 2D rejoue 30 s
en 16 s ; le même essai en 3D coûterait des heures. **Le mécanisme se règle en 2D, puis se porte en 3D (C5b).**

**Thèse (A), la suite que S354 a déclarée.** Une **bande** : à chaque pas, la dernière colonne de mailles du côté des
particules est **réensemencée depuis sa hauteur géométrique** (rangées continues, comme les colonnes de S327), la différence
de masse versée à la première colonne. La densité y est remise au nominal à chaque pas, la masse suit la géométrie. Si (A)
manque, l'attribuer avant tout autre essai ; la voie du champ de densité (une cible de divergence tirée de la densité, en
position) viendrait ensuite.

**Critères, écrits avant** (montage paroi de S327, 30 s, 5 et 2,5 cm). (1) **Sans la bande, au bit** : S354 rejoué (masse
0,50098 / 0,50734 / 0,51211 m², 4,017 / 4,846 / 4,960 particules par maille, saut 0,3893, période +9,96 %) — **fait avant le
plan**. (2) Masse exacte. (3) **Masse à gauche de la frontière** à ±0,002 m² d'APIC seul sur chaque tranche de 10 s (S354 :
+0,0113). (4) **Densité** : 4 ± 0,2 particules par maille occupée dans la dernière colonne **libre** (non réensemencée), sur
chaque tranche. (5) Les critères de S327 sur 30 s : saut < 0,5 maille ; période aux zéros à 1 point d'APIC seul ;
amortissement (régression) à 1 point d'APIC seul. (6) Repos à 5 cm : vitesse < 1 cm/s.

### Plan

- [>] **P1** — jeton, plan seul.
- [ ] **P2** — la bande (`RACCORD_BANDE=1`) dans `lot5_comparaison.rs`, la densité lue sur la dernière colonne libre ;
  critères 1 et 2.
- [ ] **P3** — 30 s à 5 et 2,5 cm, repos ; critères 3 à 6 ; attribution si manqué.
- [ ] **P4** — preuve (section datée de B10-APIC-S320, §14) ; A316, file, liste.
- [ ] **P5** — rituel.

### Notes de reprise

