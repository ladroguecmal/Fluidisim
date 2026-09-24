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

Session : S335 — **en cours**. **La vitesse de paroi au centroïde de la part couverte** — le remède nommé en
S332 —, puis le couvercle partiel d'A317 allumé par défaut ; chemin de la porte D
([ADR-189](../docs/adr/ADR-189-la-v1-d-abord.md)).
Agent : Claude Opus 5.5, application desktop ; fichiers, git, cargo, carte réelle, accès web.
Entrée : *« Continue »* (2026-09-24, 08:00), suite déclarée par S334. **Troisième session sur ce fil** —
A317 trouvé en S333, attribué en S334 — : justifiée au journal de S334, elle seule allume le couvercle
partiel, sans lequel une coque qui bouge dans δ rayonne selon sa maille (± 30–44 %). Verdict visuel attendu.

**Ce que la session doit rendre possible.** Une paroi en mouvement rigide qui pousse dans δ **exactement**
l'eau qu'elle balaie, face par face : le champ `V + Ω × r` est linéaire, son flux à travers la part couverte
d'une face plane vaut sa valeur **au centroïde** de cette part, fois son aire. δ la prend au centre de la face
(S332) : sur une petite part au coin d'une face, l'écart vaut `Ω·dx/2`, et le couvercle partiel l'amplifie de
`1/a` (S334 : 5,47 m/s). La découpe est déjà linéaire par triangle — quatre par face, autour du centre — : le
centroïde de la part négative de chaque triangle se calcule en forme close, comme son aire.

Critères, écrits avant le code :
1. **Géométrie** : le centroïde de la part couverte, exact sur des cas clos — demi-face, coin, face pleine —, et
   la même aire que `face_negative` à l'arrondi près.
2. **La sphère qui tourne** (S332, critère 1) : résidu à 12 mailles par rayon au plus la moitié de S332
   (0,73 % de `Ω·R` → ≤ 0,37 %), décroissance d'ordre ≥ 1 entre 6 et 12 mailles (S332 : 0,56).
3. **La contre-épreuve de S333 au couvercle partiel** : vitesse sur les faces ouvertes ≤ 1 m/s (S334 : 5,47).
4. **Si 3 tient, couvercle partiel par défaut** : essais du cœur verts ; valeurs S3xx changées publiées une à
   une — seuls la rotation et les couvercles en partie couverts les changent ; translation au bit.
5. **La scène de la porte D** au nouveau défaut : critères 3 à 5 de S333, flancs des placements 30/30 et 8/52,
   images refaites.

### Plan

- [x] **P1** — jeton, plan seul.
- [x] **P2** — le centroïde de la part couverte d'une face (`delta3d_cut.rs`) ; critère 1.
- [x] **P3** — la paroi au centroïde : stocké à la découpe, lu par la divergence ; critère 2.
- [ ] **P4** — la contre-épreuve de S333 au couvercle partiel ; critère 3 ; défaut allumé si tenu ; critère 4.
- [ ] **P5** — la scène de la porte D au nouveau défaut ; critère 5 ; images.
- [ ] **P6** — preuve (PORTE-D-S333 §7) ; A317 ; file, feuille de route, liste si un état change.
- [ ] **P7** — rituel.

### Notes de reprise
- **P2, critère 1 tenu.** `face_negative_centroid` : face pleine (0,5 ; 0,5), demi-face (0,25 ; 0,5), coin d'un
  champ linéaire (1/6 ; 1/6), part 1/8 ; aire identique à `face_negative` à 10⁻¹⁵ sur mille faces tirées.
- **P3, impasse — la vitesse de paroi au centroïde, éprouvée et écartée.** Exacte pour le polyèdre discret :
  sur les faces **pleinement ouvertes**, le résidu de la sphère qui tourne est divisé par deux (0,43 → 0,21 % de
  `Ω·R` à 6 mailles par rayon ; 0,29 → 0,12 % à 12 ; ordre 0,56 → 0,83). Mais sur les **faces presque fermées**
  (4–6 %), il double (6,5·10⁻³ → 1,25·10⁻² m/s) : les petites facettes du polyèdre pompent vraiment, et la
  lecture au centre des faces le compensait en partie. Critère 1 de S332 (max sur toutes les faces ≤ 1 % à 12)
  : 1,17 % — **manqué** ; critère 2 de S335 manqué. Et la contre-épreuve au couvercle partiel reste à
  **5,46 m/s** : le diagnostic de S334 était faux. **Code revenu à P2** : la vitesse au centre des faces ; la
  géométrie `face_negative_centroid` reste, éprouvée, inemployée.
- **La vraie source des pointes, mesurée** : un plancher d'ouverture du couvercle partiel — 0,1 → 0,55 m/s ;
  0,25 → 0,39 ; 0,5 → 0,36 (S332 : 0,32). Ce sont les colonnes ouvertes à moins de 10 %, où `1/a > 10` change
  le reste de découpe d'une coque qui tourne en pointe.
