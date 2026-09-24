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

Session : S353 — **en cours**. **La v1 en scène vivante, 1 : δ à 30 Hz dans la fenêtre, interpolé au rendu**
(ADR-012 §7) — liste 4.19 et 8.7, feuille de route §3 ter.
Agent : Claude Opus 5.5, application desktop ; fichiers, git, cargo, carte réelle, accès web.
Entrée — S352 : l'ordre après la v1. S348 avait reçu la porte C au banc et laissé ceci : « l'interpolation du rendu
d'ADR-012 §7 manque — δ change à 30 Hz dans une image à 60 ». La fenêtre fait aujourd'hui un pas entier par image.

**La construction.** `Step3` garde la surface publiée du pas précédent (`published_prev`) : copie sur la carte avant
chaque publication, et dans `set_state` et `resize`. Le rendu lie les deux tampons et mélange
`courant·(1 − β) + précédent·β`, β dans l'uniforme de δ ; β = 0 passe par une branche qui rend la lecture de S302
telle quelle. La fenêtre, avec `--pas-delta=33333`, fait une part du pas par image (S348) : l'image de la partie 1
voit l'état suivant publié et montre le milieu (β = 0,5) ; celle de la partie 0 montre l'état publié (β = 0). Aucune
latence ajoutée : la part est soumise avant l'image (S302 D7).

Critères, écrits avant le code :
1. **Identité** : sans interpolation, les captures de l'anneau (R16) gardent leurs empreintes publiées
   (`s339a_proche_avec_1.0s` `0x5a79f01d64083ef3`…) ; les empreintes du pas de S343 et l'identité de S350 tiennent.
2. **Le mélange, au bit** : une image rendue de (précédent, courant, β = 0,5) est identique au bit à celle du seul
   tampon (précédent + courant)/2 — la multiplication par ½ est exacte.
3. **La saccade, mesurée** sur 240 images à 30 Hz : sans interpolation, une image sur deux ne bouge pas ; avec,
   aucune, et chaque variation d'image reste entre la moitié et le double de leur médiane.
4. **En direct avec le rendu** (`--cadence`, sans synchronisation verticale) : intervalle médian et p95 — B seul,
   B + δ à 60 Hz, B + δ à 30 Hz interpolé ; la part de δ par image lue par différence ; alimentation relevée.
5. Preuve (COUT-DELTA3D-S341 §12), liste 4.19 et 8.7, file, feuille de route ; **revue R18 préparée** — la fenêtre
   vivante, interpolée ou non — : le verdict revient à l'utilisateur.

### Plan

- [x] **P1** — jeton, plan seul.
- [x] **P2** — `Step3` : la surface précédente, copiée avant chaque publication ; empreintes S343, identité S350.
- [x] **P3** — le rendu : la liaison du précédent, β, la branche ; captures de l'anneau au bit (critère 1).
- [x] **P4** — la fenêtre : deux parts à `--pas-delta=33333`, β alterné, touche d'interpolation ; banc du mélange au
  bit (critère 2).
- [ ] **P5** — la saccade mesurée (critère 3).
- [ ] **P6** — la cadence en direct (critère 4).
- [ ] **P7** — preuve, liste, file, feuille de route ; R18 préparée (critère 5).
- [ ] **P8** — rituel.

### Notes de reprise
- **P2.** `published_prev` : copie de `published` sur la carte avant la passe qui publie (pas entier et partie 1),
  dans `set_state`, `set_full_state` et `resize` (la précédente prend la courante réécrite : pas de mélange jusqu'à
  la publication suivante). Au pas 60, précédente = publiée d'avant, **0 différence** sur 13 440 colonnes ; empreintes
  S343 inchangées ; redimensionnement S350 au bit, allocateur avant = après, **89 849 856 octets, 28 allocations** —
  un tampon de plus que S350 (27).
- **P3, critère 1 tenu.** `water.wgsl` : liaison 3 (`delta3d_prev`), β dans `d3.size.w`, branche à β = 0 ; `View::blend` ;
  `attach_delta3d(courante, précédente)`. Captures de l'anneau rejouées (`INSTANTS=0,60,120 … --meilleur
  --eau-physique=2 --delta3d --anneau --captures`) : `s339a_proche_avec_1.0s` 0x5a79f01d64083ef3, `haute_avec_1.0s`
  0xd4e6d1e43e57c1cb, `proche_avec_2.0s` 0x2e37efc6f485f73d, `rasante_avec_2.0s` 0x142774b45a5ca313 — **les quatre
  identiques** aux empreintes publiées (SCENE-DELTA3D-S302 §8).
- **P4, critère 2 tenu.** `Live` : à `--pas-delta=33333`, une part par image (`K_DEUX_PARTS` = 7, S348) ; β = ½ après
  la part 1, 0 après la part 0 ; touche `I`. Banc `--melange` (anneau, 61 pas, pose proche) : écart précédente/courante
  1,84 cm ; image β = ½ contre image du tampon (p + c)/2 calculé sur CPU : **0 octet différent** ; témoin courante
  seule : 550 530 octets différents. Fenêtre `--smoke` à 30 Hz en deux parts : 120 images, sans erreur.
