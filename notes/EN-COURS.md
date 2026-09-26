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

- [x] **P1** — jeton, plan seul.
- [x] **P2** — l'instrument : bilan par profondeur de la dernière colonne libre (insertions, retraits, densité), 30 s, contre
  APIC seul ; la prédiction.
- [x] **P3** — (C), l'échange au sommet ; critères 1 à 6 ; avec (B), publié.
- [>] **P4** — preuve (§15 de B10-APIC-S320) ; A316, file, liste.
- [ ] **P5** — rituel.

### Notes de reprise
**P2 — le bilan par profondeur** (`RACCORD_BILAN=1`, paroi, 5 cm, 30 s ; sans la variable, au bit) :

| rangée (y) | 0 (2,5 cm) | 1 | 2 | 3 | 4 | 5 | 6 | 7 | 8 | 9 (47,5 cm) |
|---|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|
| insertions | 24 | 22 | 27 | 27 | 34 | 46 | 65 | 92 | 119 | 129 |
| retraits | 76 | 73 | 77 | 71 | 67 | 60 | 50 | 42 | 30 | 23 |
| particules par maille (APIC seul) | 4,44 (4,15) | 4,83 (4,15) | 4,40 (4,13) | **5,02** (3,99) | **5,47** (3,99) | **5,44** (3,99) | **5,13** (4,03) | 4,49 (4,00) | 3,99 (3,96) | 3,10 (3,96) |

**La prédiction est contredite** : l'excès n'est pas là où l'on insère — on insère surtout en haut (rangées 7–9), où la densité
est normale ou basse ; il est au milieu (rangées 3–6, 5,0 à 5,5). **Ce que le bilan montre** : les totaux s'équilibrent (581
insertions, 569 retraits), mais l'échange fait tourner une **recirculation** à la frontière — l'eau passe aux colonnes par le
bas, revient aux particules par le haut. (C) reste le candidat déclaré : au sommet, l'échange n'a plus de structure par
profondeur du côté des particules, recirculation comprise.

**P3 — (C) manqué, attribué.** 5 cm, 30 s : masse à gauche +0,0131 / +0,0229 / +0,0241 m² (S354 : +0,0113), densité 5,41 /
5,26 / 4,53, saut 1,27, amortissement 14 %/période, vitesse 1,75 m/s ; 2,5 cm : diverge (arrêté après 10 min) ; repos :
0,65 cm/s (tenu) ; avec (B), la correction ajoute **457 J/m** (102 sans (C)) — (C) comprime davantage. **Attribution** (bilan) :
l'échange a lieu au sommet, mais le **fond s'entasse** — 7,76 / 6,53 / 5,91 particules par maille aux rangées 0–2 (APIC seul
4,13) : les particules que l'écoulement pousse vers la frontière en profondeur y sont arrêtées et ne sont plus retirées là où
elles arrivent. **L'échange doit retirer où les particules arrivent.**

**La source : une circulation permanente à travers la frontière.** Vitesse horizontale moyenne sur la face, 30 s (mm/s),
rangées 0 → 9 : hybride **+21,4 +21,2 +20,5 +18,2 +13,6 +5,7 −6,3 −21,1 −37,4 −54,0** ; APIC seul, même face, de +0,7 à −1,9.
L'eau entre dans les colonnes par le bas et en ressort par le haut, en permanence, à une vitesse de l'ordre de la moitié de
celle de l'onde. **Témoins** : sans paroi (échange eulérien) +22,6 / −68,8 ; avec la mémoire de vitesse des colonnes
+25,6 / −60,5 ; frontière aux trois quarts +20,1 / −43,1 — ni la paroi, ni l'aller-retour de vitesse, ni la place de la
frontière : **elle naît des colonnes**. Hypothèse, non tranchée : réensemencées à chaque pas sur des points fixes, les
colonnes n'advectent pas la quantité de mouvement ; le terme non linéaire manque d'un côté, pas de l'autre. Épreuve par
l'amplitude (`LOT5_AMPLITUDE`) : à 1 cm, rien ne bouge (l'onde est sous l'espacement des particules à 5 cm — témoin
dégénéré) ; à 4 cm, ×2,65 en bas, ×1,3 en haut — ni A ni A² : **non tranché**.
