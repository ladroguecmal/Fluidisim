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

Session : S208 — en cours
Agent : Claude Code (Opus 5 ; fichiers, git et cargo disponibles)
Objectif : (1) **construire ADR-129** dans `water-core` — table radiale à matrice de Bessel
précalculée pour le chemin d'image de W, avec la réception écrite dans l'ADR ; (2) **recommander
la pile de l'hôte GPU** (ADR-130), à la demande de l'utilisateur, **sans rien télécharger**.

### État réel à l'amorce

master = copies = 79bfda6 (S207 P4), propres ; jeton libre depuis 08:53. Message utilisateur :
« pour la décision de l'hôte je pensais à l'option 1 mais à voir via ta recommandation » —
l'option 1 de S204 (application séparée avec dépendances) est celle qu'ADR-130 acte.

### Conception, fixée avant le code

Sous-module `radial_table.rs` de `radial_impact` (accès aux nœuds privés, pas de copie de la
formule). `RadialImpact::table_len(step)` ; `bake_table(step, &mut [[f32;2]])` remplit
`(J0, J1)(k_n r_i)`, `r_i = i·step`, i ∈ 0..M, M = ⌊R/step⌋ + 2 (un nœud au-delà de R pour
l'Hermite du dernier intervalle, dans la portée de Bessel déjà contrôlée) ; `RadialTable::profile
(time, &mut [(f32, f32)])` rend η(r_i) et pente radiale par image, **même ordre de sommation que
`sample`** ; `RadialTable::eval(profile, frame, cell, point)` : refus hors emprise comme `admits`,
Hermite cubique pour η et sa dérivée pour la pente. Aucune allocation ; stockage et profil fournis
par l'hôte (I-06). Chemin cosmétique (ADR-129 §3).

**Critères déclarés avant mesure** : (a) aux nœuds, pour une source en position 0, η et pente
égaux **au bit** à `sample` ; (b) à λ/8 sur le champ S203, max|Δη| ≤ 0,09 mm et max|Δpente| ≤ 2 %
de `slope_max`, sur toute l'emprise et l'horizon (âges 0–56 s) ; (c) zéro allocation dans
`profile` et `eval` (essai d'intégration à compteur global) ; (d) refus `Time`, `Domain` et
stockage insuffisant nommés ; (e) image S205 +3 s par la table contre le chemin direct : zéro
rayon non résolu, écart maximal par canal publié, zéro pixel différent hors emprise.

### Plan

- [x] **P1** — état réel, conception et critères, plan seul.
- [x] **P2** — `radial_table.rs` : `table_len`, `bake_table`, `profile`, `eval` ; compilation.
- [x] **P3** — essais unitaires (a), (b), (d) ; essai d'intégration (c).
- [x] **P4** — banc : noyau réel dans `frame_cost` (coût par image, construction, mémoire) ; image
 S205 par la table dans `render_impact` contre le chemin direct (e).
- [ ] **P5** — recommandation de pile GPU : critères (ADR-020/130, hors réseau pour le cœur,
 portabilité, licences, maintenance), versions vérifiées en ligne **sans téléchargement** ;
 demande d'autorisation nommée formulée pour le lot suivant. Document `HOTE-GPU-S208`.
- [ ] **P6** — rituel §6 : journal, angles, leçons, notes ADR-129, feuille de route, file active,
 index/README/REPRISE, décomptes, velocite, compteur, jeton libre, copies.

### Notes de reprise

P2 : `radial_table.rs`, sous-module de `radial_impact` (accès aux nœuds, pas de copie de la
formule). Type d'erreur propre `TableError { Step, Storage, Profile, Field(Error) }` plutôt qu'un
variant ajouté à l'erreur partagée des champs (ADR-082). `profile` garde les opérations de
`sample` (`coef·J0·ct`, `coef·k·J1·ct`) pour l'égalité au bit aux nœuds. Compilation sans
avertissement.

P3 premier passage : (a) égalité au bit aux nœuds **passe** ; (c) zéro allocation **passe**
(témoin > 0) ; (d) refus nommés **passe**. **(b) ÉCHOUE** : à λ/8, max|Δη| = **0,1820 mm à l'âge
0 s** > 0,09 mm déclarés ; max|Δpente| = 0,001357 ≤ 0,004252 (2 % de slope_max) passe. Le seuil
0,09 mm venait de S206, mesuré à +3 s seulement : la naissance (pic central compact) est plus
exigeante. **Non déplacé.** Mesure ajoutée *après* l'échec, dite comme telle : erreur par âge à
λ/8 et à λ/16 ; décision de pas à corriger par note datée d'ADR-129, pas par l'essai.
P3 après échec (mesure ajoutée, release) : **λ/16 : max|Δη| 0,0127 mm** (âge 0), pente 0,000190 ;
0,0043 mm à 2 s → 0,0008 mm à 56 s. **λ/8 : 0,1820 mm à 0 s**, 0,0689 à 2 s, 0,0731 à 4 s → 0,0123 à
56 s ; pente 0,001357. Rapport 14,3 ≈ h⁴ = 16. Essai réécrit : λ/16 exigé ≤ 0,09 mm ; λ/8 gardé
sous 3 mm (tolérance de marche) et sous 2 % en pente, **sans le présenter comme la réception**.
ADR-129 §2 (« pas ≤ λ/8 ») à corriger par note datée : réception à 0,09 mm = λ/16. Trois essais
unitaires + un d'intégration (zéro allocation) verts.
P4 (release, un fil). `frame_cost table` — chemin construit, table réelle :
| pas | M | mémoire | cuisson | grille | B | profil | éval (dans R) | image méd/max | ÷ 2 ms | direct S206 |
| λ/16 | 250 | 500 Ko | 1,299 ms | 4 px | 11,441 | 0,0462 | 0,329 | 11,476 / 12,745 | 5,7 | 72,4 |
| λ/16 | 250 | 500 Ko | 1,299 ms | 2 px | 41,741 | 0,0386 | 0,908 | 43,021 / 52,453 | 21,5 | 280 |
| λ/8 | 126 | 252 Ko | 0,830 ms | 4 px | 11,761 | 0,0273 | 0,343 | 11,791 / 16,103 | 5,9 | 72,4 |
| λ/8 | 126 | 252 Ko | 0,830 ms | 2 px | 41,429 | 0,0270 | 0,834 | 42,637 / 44,811 | 21,3 | 280 |
Éval ≈ 43 ns/sommet dans R (prédicat d'emprise, racine, pente incluse ; S206 : 8 ns Hermite nu).
W par image : ~0,4–0,9 ms à 2 px, dont profil 0,04 ms ; **B = 97 % de l'image**.
`render-table ../captures 3 1.5 16` — critère (e) : 289 462 eau, **0 non résolu, 0 refus**,
15 841 760 évals (direct 15 841 764), **18 523 ms** (direct S205 102 218 ms), FNV 0xf78a4221c46add31
contre direct 0x0b13a4c1e39a2a3e : **27 pixels différents, 0 hors emprise, écart max 1 niveau,
0 pixel > 2 niveaux**. Reçu.
