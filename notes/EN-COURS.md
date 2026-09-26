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

Session : S384 — **en cours**. **Décision de l'utilisateur** (2026-09-26, S384, session cloud sans carte graphique ni
Godot) : *« Solveur 3D ici »* — la campagne du solveur volumique 3D commence maintenant ; la pluie, pièce 5, et le verdict
R32 se feront depuis le poste. Agent : Claude Opus 5.5, session cloud Claude Code ; fichiers, git, cargo, Python ; ni
carte graphique, ni Godot.

**Thèse.** La campagne du solveur volumique 3D temps réel (FEUILLE-DE-ROUTE §3 ter ; S379, R30) commence par sa
**conception** : ce que le dépôt a déjà (δ 3D en colonnes, référence CPU et production GPU ; APIC sur banc 2D ; faces
coupées ; ordonnanceur), l'état de l'art du volumique temps réel, l'architecture qui les réunit (domaines, niveaux de
détail d'ADR-202, prévision d'ADR-013, représentations), des cibles chiffrées tirées des usages, et un découpage en
sessions dont chacune a son critère « reçu si ». Livrables : `docs/registres/CAMPAGNE-SOLVEUR-3D-S384.md` et un ADR qui
acte ce qui relève de l'autonomie technique (S71) et nomme ce qui demande l'utilisateur.

**Critères, écrits avant.** (1) Chaque capacité existante citée avec sa preuve (lien) et son chiffre mesuré ; aucune
valeur sans provenance (I-14). (2) Chaque méthode de l'état de l'art avec une source identifiée (auteurs, année, lieu) ;
ses chiffres marqués **publié**, **estimé** ou **non vérifié**. (3) Chaque cible chiffrée rattachée à un usage (point de
la liste, tolérance d'image de 3 mm, δ ≤ 2 ms GPU d'ADR-174 D3, cadence d'ADR-012 §7). (4) Chaque session du découpage
nomme sa porte ou son point de liste, son critère « reçu si », et **où** elle peut se faire (session cloud sans carte, ou
poste avec carte et Godot). (5) `etat_projet.py --check` sans erreur. Aucun code du cœur changé : suite Rust inchangée
(664 réussis, 18 ignorés, mesuré à l'ouverture sur `daf67e0c`).

### Plan

- [x] **P1** — jeton, plan seul.
- [x] **P2** — inventaire : δ 3D (référence, production, coût, limites A297, A298, A316, A320), APIC (B10, raccord),
  faces coupées, ordonnanceur, piscine en δ ; lu dans les preuves, chiffres et liens.
- [x] **P3** — état de l'art (1) : grilles hybrides temps réel (colonnes hautes, fonction hauteur + 3D + particules),
  pression sur la carte (multigrille), grilles éparses.
- [x] **P4** — état de l'art (2) : particules sur grille (FLIP, APIC, MPM) sur la carte ; SPH et PBF ; Boltzmann sur
  réseau à surface libre ; coûts et qualités publiés.
- [ ] **P5** — cibles chiffrées : les usages volumiques de la liste, taille des domaines, mailles, cadence, budget.
- [ ] **P6** — l'architecture proposée, les alternatives écartées et leurs raisons.
- [ ] **P7** — le découpage en sessions : critères « reçu si », lieu (cloud ou poste).
- [ ] **P8** — ADR de la campagne ; liste, file, feuille de route, index.
- [ ] **P9** — rituel.

### Notes de reprise

*(vide)*
