# ADR-192 — Le rendu de l'eau dans Godot 4

- **Statut : actée**, S356, 2026-09-25, **décision de l'utilisateur** : *« Peut être godot ou unreal serait envisageable
  car rendu toujours pas convaincant »*, puis, entre Godot 4, Unreal Engine 5, les deux, ou notre propre rendu :
  **« Godot 4 »** — *« déjà sur ton PC ; le cœur Rust s'y branche directement. Premier pas : la mer de B rendue dans
  Godot, que tu juges sur images avant tout portage. »*
- **Remplace** [ADR-191](ADR-191-le-rendu-realiste-un-module-du-moteur.md) D2 **en ce qu'il désignait un moteur maison
  à construire** et l'afficheur comme point de départ du module de rendu.
- **Laisse entiers** ADR-191 D1 (l'architecture de l'eau ne change pas), D3 (une session de rendu, une de physique) et
  D4 (le rendu se juge à l'œil de l'utilisateur) ; [ADR-001](ADR-001-decomposition-en-couches.md),
  [ADR-127](ADR-127-ambition-complete-construction-progressive.md), I-13, [ADR-124](ADR-124-image-budget-et-effets-bornes.md)
  (images locales, aucune page publiée), [ADR-130](ADR-130-rendu-j1-sur-gpu-par-un-hote-separe.md) (l'hôte GPU séparé, qui reste le
  banc), et la règle des dépendances : **aucune bibliothèque téléchargée sans accord nommé**.

## 1. Ce que la session a constaté

- **Godot 4.4.1 est présent sur le poste** (et 4.2, 4.4) ; **Unreal Engine ne l'est pas** — le lanceur d'Epic seul.
- Godot 4 : libre (MIT) ; rendu Forward+ sur Vulkan ou Direct3D 12, reflets à l'écran, illumination globale (SDFGI,
  VoxelGI), brouillard volumique, halo, courbes de tonalité (AgX, ACES), matériaux de ciel ; nuanceurs dans son
  langage, proche de GLSL ; calcul par `RenderingDevice` ; code natif par GDExtension, que godot-rust ouvre à notre
  cœur Rust sans couche C++.
- Unreal Engine 5 a le meilleur rendu d'emblée, mais demandait une installation d'environ 50 Go sous compte Epic, un
  greffon C++ autour du cœur, et la réécriture en HLSL du calcul de δ sur la carte.

## 2. Décisions

**D1 — Le rendu final de l'eau se fait dans Godot 4.** Le système d'eau — le cœur Rust, B, W, δ, V — ne change pas
(ADR-191 D1). Godot rend : sa lumière, son ciel, ses reflets, son post-traitement, et **nos nuanceurs d'eau écrits dans
son langage**.

**D2 — Premier pas : un prototype sans intégration native.** La mer de B — ses composantes, exportées du cœur — rendue
dans Godot par un nuanceur d'eau qui porte le CWM de la bande et les crêtes de S356 (écume à la couverture de Monahan,
lumière des crêtes), dans l'environnement de Godot ; images jugées par l'utilisateur **avant tout portage**.

**D3 — Ensuite, l'intégration native** : le cœur chargé par GDExtension, les données publiées vers des textures ; δ sur
la carte — calculé dans Godot ou partagé depuis notre hôte — se décide sur mesures. Chaque bibliothèque à télécharger
se demande nommément : nom, version, taille, source.

**D4 — L'afficheur reste le banc.** Il porte les mesures et les réceptions physiques (ADR-130, ADR-175 D4) ; ses
modules de rendu — `water_cretes.wgsl` le premier — sont des **références à porter**, jugées dans Godot.

## 3. Ce que la décision ne tranche pas

- **Le jeu entier dans Godot** : le système d'eau n'en décide pas ; cette décision porte sur le rendu de l'eau.
- **La version** : 4.4.1 est présente ; une plus récente demanderait l'accord de téléchargement.
- **Le chemin de δ vers l'image** dans Godot — calcul par `RenderingDevice` ou partage avec notre hôte — : sur mesures.

## 4. Ce qui devient faux si cette décision est mal lue

**« Godot » ne veut pas dire que la physique change** (ADR-191 D1), **ni que les réceptions se font dans Godot** : la
référence physique reste le cœur et le banc. **Ni qu'Unreal est écarté pour toujours** : si Godot plafonne sur images,
la question se rouvre — à la demande de l'utilisateur.
