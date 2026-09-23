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

Session : S326 — **en cours**. **Lot 3 : Jacobi sur le chemin coupé** (A315), alternance
d'[ADR-188](../docs/adr/ADR-188-lot-3-a-la-place-du-lot-2-bloque.md).
Agent : Claude Opus 5.5, application desktop ; fichiers, git, cargo, carte réelle, accès web.
Entrée : *« continue »*, après S325 ; suite déclarée : Jacobi sur le chemin coupé.

**Ce que la session doit rendre possible.** Une trajectoire à maille fine sur fond coupé : à 128, la
bosse de S324 coûtait **16 029 itérations et 708 s** pour un pas, contre 347 et 4 s sans elle — la
tolérance physique d'ADR-144 ne se refermait que lentement dans les petites cellules, le mode linéaire
n'ayant aucun préconditionneur. Consommateur : la frontière mobile du lot 3, puis les corps du lot 4.

**Le remède.** Le Jacobi du mode mobile 3D (`prime_mobile3`, `dot_prec3`), porté au chemin coupé du mode
linéaire : diagonale de la ligne pondérée — ouvertures des faces vers une maille fluide, deux fois
l'ouverture du couvercle. **Seulement quand `ny > 1`** : à `ny = 1`, la 3D reste la 2D au bit, et la 2D
résout sans préconditionneur. Le fond plat n'est pas touché.

Critères, écrits avant le code :
1. Identité 2D à `ny = 1` et fond plat : tous les essais antérieurs **au bit** ; suite complète verte.
2. Bosse à 128 : **au plus 1 000 itérations** (16 029 avant).
3. Débits à 32, 64 et 128 égaux à ceux de S324 à **10⁻⁵ près** en relatif — deux solutions convergées à
   la même tolérance ; ordre toujours ≥ 1,8.
4. Aucune allocation dans le pas : le tampon `prec` existe.

### Plan

- [x] **P1** — jeton, plan seul.
- [x] **P2** — Jacobi sur le chemin coupé, commutable pour la mesure ; essais.
- [x] **P3** — le banc de S324 rejoué, avec et sans Jacobi.
- [x] **P4** — preuve : section datée de [FACES-COUPEES-3D-S324](../docs/validation/FACES-COUPEES-3D-S324.md),
  avec « Reproduire » ; A315, file.
- [>] **P5** — S320 P5b : §5 bis au retour du calcul lancé le 22 à 20:11 — asynchrone. **Redéclarée à 08:55**,
  sur question de l'utilisateur (*« Il n'y a pas de problème avec P5b ? »*) : annoncé pour 1 à 2 h, le calcul
  tourne depuis 12 h 38 (12 h 17 de CPU) sans rien écrire. Le pas d'APIC n'a pas de plancher et le gradient
  conjugué plafonne à 4 000 itérations : il peut ramper ou s'être emballé. **P5a** — trace de progression
  par variable d'environnement (sortie par défaut inchangée), le même calcul rejoué en parallèle ;
  **P5b** — verdict : attendre, ou arrêter et dire pourquoi.
- [ ] **P6** — rituel.

### Notes de reprise

**P2 + P3 (07:32), fusion déclarée : le banc ne change pas, il se rejoue.** `prime_mobile3` et `dot_prec3`
rendus `pub(super)` ; diagonale du chemin coupé calculée une fois à la configuration ; Jacobi quand
`ny > 1`, commutable (`set_precondition_cut`). Essai à 64 : 645 → **220** itérations, débit à 2,4·10⁻⁶.
Suite : **590 réussis**, 0 échec ; les essais de S324 — identité 2D à `ny` = 1 — passent. Banc : bosse à
128, **16 029 → 425 itérations, 708 → 5,7 s** ; débits à ≤ 2,4·10⁻⁶ de S324 ; ordre 1,947. Témoin :
347 → 422 itérations — Jacobi y coûte un peu, ses petites cellules ne gênaient pas. Banc entier : 12 min
→ 12 s.
