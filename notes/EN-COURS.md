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

Session : S329 — **en cours**. **La v1 d'abord** (ADR-189, à écrire en P2) ; **lot 3 : un obstacle qui
n'est pas un fond** — essai 2 de la piscine, la boule immergée.
Agent : Claude Opus 5.5, application desktop ; fichiers, git, cargo, carte réelle, accès web.
Entrée : *« continue, jusqu'à la v1 »*, après S328.

**Le chemin de la v1, lu dans le dépôt.** v1 = porte D franchie (ADR-174 D4, ADR-178 D7) : lots 3 et 4.
Lot 3 : faces coupées 3D « fixes d'abord, puis à frontière mobile » — essais 2 (boule immergée : volume
déplacé, flottabilité) et 3 (mouvement imposé : forces, vagues). Lot 4 : corps rigide, forces rendues,
masse ajoutée — une boule libre à son tirant et à la période de C10 (cube 0,5 m à 500 kg/m³ : tirant
0,25 m ± 1 %, période 1,00 s ± 5 %, rapport √2 ± 15 % avec la masse ajoutée). Porte D : un bateau flotte
et perturbe l'eau qui le porte, **sans autorité de δ sur le jeu** (I-04, ADR-008) — le jeu le fait
flotter sur B + W ; δ le voit comme une paroi mobile et ne lui rend qu'un décalage visuel borné.

**Ce que la session doit rendre possible.** La découpe de δ ne connaît qu'un fond en hauteur. Il lui
faut un **solide quelconque** : une forme donnée par sa distance signée aux nœuds de la grille, coupée
exactement pour un champ linéaire par morceaux — faces en quatre triangles autour de leur centre,
mailles en vingt-quatre tétraèdres qui s'appuient sur eux, formules closes sans soustraction instable.
Fixe dans cette session ; la frontière mobile suivra sur les mêmes tampons.

Critères, écrits avant le code :
1. **Plan** : une distance signée plane donne fractions et ouvertures exactes à 10⁻⁶ ; un solide absent
   laisse la géométrie du fond **au bit**.
2. **Sphère immergée** : volume déplacé `Σ(1 − fraction)·dx³` d'**ordre ≥ 1,8** vers `4πR³/3` ; ouvertures
   et fractions symétriques par réflexion.
3. **Cohérence** : pour chaque maille, `Σ ouvertures·normales` des faces et paroi solide se ferment
   (théorème de la divergence discret) ; la poussée hydrostatique intégrée sur la paroi discrète tend
   vers `ρgV` à l'ordre ≥ 1,8.
4. **Les pas** : lac au repos exact autour de la sphère, modes linéaire et mobile ; la sphère qui touche
   le fond ou approche la surface est refusée (`Domain`).
5. **Ordre** du débit ouvert au premier pas autour de la sphère, mode linéaire : **≥ 1,8**.

### Plan

- [x] **P1** — jeton, plan seul.
- [x] **P2** — ADR-189 : la v1 d'abord ; index, file.
- [ ] **P3** — géométrie : solide par distance signée aux nœuds, faces et mailles exactes pour le champ
  linéaire ; critères 1 à 3 en essais.
- [ ] **P4** — le solide dans les pas linéaire et mobile ; critère 4.
- [ ] **P5** — banc : débit autour de la sphère, trois mailles, ordre ; poussée.
- [ ] **P6** — preuve : section datée de [FACES-COUPEES-3D-S324](../docs/validation/FACES-COUPEES-3D-S324.md),
  avec « Reproduire » ; file, liste 4.15.
- [ ] **P7** — rituel.

### Notes de reprise

