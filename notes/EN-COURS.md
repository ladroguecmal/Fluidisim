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

Session : S462 — **terminée**. *« … puis continue »* — l'utilisateur a lancé l'afficheur (`--surface-direct` : 0,96 du temps réel,
16,8 ms au 99e centile chez lui) ; la commande de Godot donnée pour bash ne passait pas sous PowerShell (l'opérateur `&` manquait) —
corrigée dans la réponse. R39 sans verdict : la suite par défaut, **les caustiques**.

**Ce que la session fait.** Les caustiques sur le sable de `saut.tscn`, comme S361 les pose sous B : l'éclairement direct du fond
multiplié par la focalisation de la surface. **Dans le domaine** : l'afficheur calcule, à chaque image enregistrée, la hauteur des
colonnes sur `φ` (la première traversée depuis le haut), sa hessienne, et la focalisation `C = 1/|det(I + D·Hess η)|`,
`D = H·(1 − 1/n)` (la profondeur sous la surface, l'indice) ; une carte de 80 × 80 sur 8 bits, aux coordonnées de la surface ; le sable
la lit au point de surface d'où vient son soleil réfracté. **Au-delà** : la même formule sur B, analytique.

**Critères, écrits avant.** (1) la focalisation moyenne sur le domaine à 1 près de 5 % (l'énergie se conserve : une caustique
déplace la lumière, ne la crée pas) ; (2) le raccord domaine | mer de B sans saut visible des caustiques ; (3) toujours 60 images/s au
moins ; (4) les images montrées.

### Plan

- [x] **P1** — jeton, plan seul.
- [x] **P2** — la carte des caustiques à l'enregistrement ; sa lecture dans Godot ; mesures ; images.
- [x] **P3** — preuve ; rituel (allégé).

### Notes de reprise
- **P2** — l'export (`saut_caustiques.bin`, une carte de 80 × 80 sur 8 bits par image) ; dans Godot, `focalisation()` dans
  `saut_optique.gdshaderinc` (la carte au point de surface d'où vient le soleil réfracté ; au-delà, B analytique) et sa texture dans
  `saut.gd` (`CAUSTIQUES=0` : sans). **Mesuré** : la formule ponctuelle `1/|det(I + D·Hess η)|` donnait une focalisation moyenne de
  **1,10 à 1,96** — fausse : lue sur la surface, elle ne conserve pas l'énergie (là où les rayons se croisent) ; **le dépôt** (4 × 4
  échantillons par cellule, déposés en bilinéaire au point du fond) : **0,979 à 0,995** — (1) tenu (la perte : la lumière sortie par les
  bords) ; (2) au raccord, le motif change un peu (le domaine perd sa lumière de bord, B n'en dépose pas) — presque ; (3) **405
  images/s** — tenu ; (4) images envoyées.
- **P3** — C10-SCENES-S454 §11 ; journal ; jeton libre ; maillons 1 ; suivant : S463, la suite de C11 (la surface fine), ou ce que R39 désigne.
