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

Session : S393 — **en cours**. D'abord le **verdict R33** ; puis **C4b**, seconde part : **B10 en 3D**, une sphère qui entre dans
l'eau, sur APIC 3D ([ADR-207](../docs/adr/ADR-207-la-campagne-du-solveur-volumique-3d.md) D5 ; [APIC3D-S388](../docs/validation/APIC3D-S388.md)).
Demande de l'utilisateur (2026-09-26) : *« Reprends le projet, pour R33 je valide actuellement mais pour plus tard des
sessions de peaufinage »*. Le travail du poste (S390–S392, branche `poste`) rejoint cette branche en avance rapide.
Agent : Claude Opus 5.5, session cloud Claude Code ; fichiers, git, cargo, Python ; ni carte graphique, ni Godot ; articles
illisibles (réseau), recherche seulement. Sert **4.16** (surface non graphe) et 4.12.

**Thèse.** Le banc 2D de S320 faisait entrer un **cylindre** ; le cas 3D est une **sphère**, axisymétrique, que la littérature
mesure : le pincement profond (*deep seal*) tombe à `t_p = β·√(R/g)`, presque indépendant de la vitesse, `β` de **1,72 à
2,29** selon les auteurs (Glasheen et McMahon 1996 ; Duclaux et al. 2007 ; Aristoff et Bush 2009 — plage lue par la
recherche, les textes sont bloqués par le réseau : dit, non vérifié ligne à ligne). Porter le corps **cinématique** du banc 2D
dans `apic3d.rs` — mailles solides, faces à la vitesse du corps, paroi mobile pour la pression, particules repoussées —, et
mesurer le pincement sur un **quart de domaine** (l'axe au coin : deux parois = deux plans de symétrie, les images de S389
comprises), ce qui divise le coût par quatre.

**Mesures** (comme S320, en 3D) : air enfermé = mailles sans particule, hors du corps, sous le repos, au-dessus du corps, à moins
d'un diamètre de l'axe, qu'un remplissage depuis le haut n'atteint pas ; **pincement** = premier pas où il dépasse `D³/32`
(domaine entier) ; lu à chaque pas.

**Critères, écrits avant.** (1) **Sans corps, rien ne change** : `apic3d_ballottement 10 0.05` imprime les chiffres de S389 P4.
(2) **Corps au repos** à demi immergé : vitesse parasite ≤ 1 cm/s, masse exacte (essai). (3) **Quart contre domaine entier**
(`D/dx` = 8, `Fr` = 2) : même pincement à un pas près. (4) **Convergence** : `Fr` = 2, `D/dx` = 8, 12, 16 — le pincement à
12 et 16 à **5 %** l'un de l'autre. (5) **Mesure publiée** : à la maille la plus fine, `t_p/√(R/g)` dans **[1,72 ; 2,29]**.
(6) Masse exacte partout ; suite, zéro avertissement. **Publiés** : `Fr` = 4 (8 et 12), la distance des parois, le coût.

### Plan

- [x] **P1** — jeton, plan seul.
- [x] **P2** — verdict R33 consigné (revue, preuve, décisions, file).
- [x] **P3** — le corps cinématique dans `apic3d.rs` ; essais (repos à demi immergé, masse) ; critères 1 et 2.
- [>] **P4** — l'exemple `apic3d_b10` (quart ou entier, mesures) ; critère 3.
- [ ] **P5** — la convergence, la plage publiée, `Fr` = 4, les parois ; critères 4 et 5.
- [ ] **P6** — critère 6 ; preuve `B10-APIC3D-S393` ; liste, file, feuille de route, index.
- [ ] **P7** — rituel.

### Notes de reprise
**P2** : R33 consigné — revue §38, preuve S392 (§ Verdict), décisions, file (rendu), feuille de route ; liste : 8.4 disait
encore « R32 posée » (périmé depuis S385), corrigé ; 8.10 nomme S392. Aucun défaut nommé : rien à expliquer.

**Référence du critère 1** (avant P3, binaire gardé) : `apic3d_ballottement 10 0.05` → période 1,9964 s, **+1,01 %**, énergie
+7,04 %, amortissement **+0,21 %**, 9 pics, 93,5 itérations moyennes, 500 pas.

**P3 — le corps dans `apic3d.rs`** : `Sphere3`, `set_body`, mailles `SOLID` (centre dans la sphère), faces à la vitesse du corps
après chaque écriture, paroi mobile pour la pression (ni coefficient ni correction vers un solide), particules repoussées à
`R + 0,05·dx` avec une vitesse normale au moins celle du corps ; le corps avance de `v·dt`. **Trouvé en chemin** : au repos, à
demi immergée, **9,4 cm/s** au premier passage, au ras de l'eau contre la sphère — le biais de paroi de S389, contre le corps :
le noyau ne voit des particules que d'un côté, la surface y paraît plus basse, l'eau monte. **Remède, le même** : le corps
reflète les particules dans la reconstruction (image radiale `c + (2R − d)·n`, près du corps seulement). **Critère 2 tenu, de
justesse** : **9,6 mm/s** au pire (2 s, 100 pas), 0,77 mm/s à la fin ; masse exacte ; **vu échouer** sans mailles solides
(27,6 cm/s). **Critère 1 tenu** : `apic3d_ballottement 10 0.05` imprime la même ligne qu'avant, au chiffre près (hors durée).
Essais : refus (rayon nul, NaN), entrée de vingt pas (masse, aucune particule dans le corps).
