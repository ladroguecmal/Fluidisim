# ADR-191 — Le rendu réaliste de l'eau : un module du moteur maison, en alternance avec la physique

- **Statut : actée**, S355, 2026-09-25, **décision de l'utilisateur**, en cinq réponses, après sa question :
  *« Est ce que tu penses que l'architecture du projet/code/etc... doit changé ? Car actuellement le rendu est pas
  comme celui que j'imaginais pas assez réaliste »*, avec une référence : FluidNinja LIVE-2, sur Fab.
- **Remplace** [ADR-178](ADR-178-strategie-en-trois-systemes-physiques.md) D2 **en ce qu'il arrêtait le
  perfectionnement visuel** (« aucun lot de perfectionnement visuel ne s'ouvre sur A tant que… », « le rendu actuel
  devient un instrument de diagnostic »). Le reste de D2 — A suffisant pour servir B et C — demeure.
- **Laisse entiers** [ADR-001](ADR-001-decomposition-en-couches.md) (les couches),
  [ADR-127](ADR-127-ambition-complete-construction-progressive.md) (l'ambition),
  [ADR-174](ADR-174-arbitrages-du-2026-09-19.md) (machine, profil, v1), ADR-178 D1, D3 et D4,
  [ADR-190](ADR-190-apres-la-v1-la-liste-entiere.md) (la liste entière), I-13 (le rendu ne pilote pas la physique) et
  [ADR-124](ADR-124-image-budget-et-effets-bornes.md) (images de banc locales, aucune page publiée).

## 1. Ce que l'utilisateur a répondu

| question posée par la session | réponse |
|---|---|
| quel moteur portera le jeu ? | **un moteur maison** |
| où en est-il ? | **à construire** |
| où se fait le rendu final de l'eau ? | **dans le moteur** |
| qui écrit le rendu réaliste de l'eau ? | **nous, en module** : le système d'eau publie ses données et fournit les shaders de l'eau, branchables dans le moteur ; l'afficheur sert de vitrine et de banc |
| que fait-on maintenant ? | **alterner avec la physique** |

## 2. Ce que la session a constaté

**La référence** est une simulation **2D** attachée au joueur, qui pilote un rendu riche — textures de détail et
d'écume entraînées par ses vitesses, particules, volumétrie — dans le moteur d'Unreal, sur une scène dessinée ; au
loin, des motifs passifs. Son réalisme vient surtout du rendu et de la scène, peu de la finesse du solveur.

**Notre architecture a la même forme** — δ près du joueur, dans un domaine qui le suit (S349) ; B, analytique, au
loin — et va plus loin : la 3D, la flottaison, les volumes. **L'écart est dans le rendu.** L'afficheur est un
instrument : ciel calé sur une photographie (S308), Fresnel, reflet solaire, queue de spectre en pentes (S256) ; ni
écume, ni réfraction, ni absorption selon la profondeur, ni lumière sous la surface, ni embruns, ni décor.
ADR-178 D2 l'avait voulu ainsi.

## 3. Décisions

**D1 — L'architecture de l'eau ne change pas.** B, W, δ et V (ADR-001), l'ordonnanceur, l'autorité (I-04, I-15)
restent ce qu'ils sont. Le rendu est en aval : il lit ce que le système publie et ne le pilote jamais (I-13).

**D2 — Le rendu final est celui du moteur du jeu, un moteur maison à construire, et nous en écrivons la part de
l'eau comme un module** : les données que le système d'eau publie — hauteurs, déplacements, vitesses, écume, et ce
que chaque élément de rendu demandera — et les shaders qui les rendent. **Le point de départ est l'afficheur**
(Rust, wgpu, WGSL) : chaque élément de rendu s'y écrit **séparable** — son module de shader, ses entrées publiées,
son coût mesuré — pour être extrait quand le moteur existera. L'afficheur en est la vitrine et le banc.

**D3 — Les sessions alternent** : une session de rendu, une session de physique. La physique garde sa propre suite
— la v1 en scène vivante et le lot 5 en alternance (ADR-184 D1, ADR-190 D4).

**D4 — Le rendu se juge à l'œil de l'utilisateur**, sur des références ([REVUE-VISUELLE](../validation/REVUE-VISUELLE.md)) ;
FluidNinja LIVE-2 en est une, de style. Chaque élément garde un coût mesuré, dans le profil de l'eau d'ADR-174 D3
(ADR-131 : un dépassement qualifie l'implémentation).

## 4. Ce que la décision ne tranche pas

- **Le langage et l'API du moteur** : il n'existe pas ; le module part de wgpu, et le reste tant que rien ne le
  demande.
- **L'ordre des éléments de rendu** : proposé par les sessions sur la [liste](../LISTE-PROJET-FINI.md) — 8.4 écume et
  embruns, 8.5 transparence et réfraction, 8.6 vue sous-marine, 8.9 détails —, jugé par l'utilisateur.
- **Les références visuelles** de chaque élément : demandées à l'utilisateur au moment de la revue.

## 5. Ce qui devient faux si cette décision est mal lue

**« Rendu réaliste » ne veut pas dire « l'afficheur devient le jeu »** : c'est un module pour un moteur. **Ni que la
physique s'arrête** : elle alterne. **Ni qu'un effet de rendu remplace une physique** du périmètre (ADR-127) : une
écume dessinée n'est pas un déferlement reçu, et la liste ne coche pas l'un pour l'autre.
