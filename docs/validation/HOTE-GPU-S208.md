# S208 — Recommandation de pile pour l'hôte GPU de J1

2026-09-13. Demande de l'utilisateur : « pour la décision de l'hôte je pensais à l'option 1 mais à
voir via ta recommandation ». L'option 1 de S204 — **une application d'affichage séparée, avec
dépendances** — est celle qu'[ADR-130](../adr/ADR-130-rendu-j1-sur-gpu-par-un-hote-separe.md) a
actée. Ce document recommande **la pile** de cet hôte. **Rien n'a été téléchargé** : versions et
licences lues sur l'API publique de crates.io le 2026-09-13.

## 1. Ce que l'hôte doit faire, et ce qu'il ne doit pas toucher

- ouvrir une fenêtre, piloter une caméra, rendre en temps réel la scène représentative de J1 :
  mer S201, impacts, sillages ;
- évaluer **B et W sur GPU** (ADR-130), avec les données que `water-core` publie : recettes cuites
  et phases temporelles repliées en entiers (I-08), profils d'impact par table de Bessel (ADR-129,
  construits en S208) ;
- mesurer son **temps GPU d'eau** par image, pour confronter enfin le profil ADR-125 et le
  `gpu_sim_ms` d'ADR-012 à une mesure ;
- **ne rien imposer à `water-core`** : aucune dépendance, aucune API graphique dans le cœur ; le
  workspace `code/` reste constructible et testable **hors réseau** (ADR-020).

## 2. Options pesées

| pile | portabilité | ce qu'elle apporte | ce qu'elle coûte | verdict |
|---|---|---|---|---|
| **wgpu + winit** | Windows (D3D12, Vulkan), Linux (Vulkan), macOS (Metal), web | API GPU moderne en Rust, WGSL, **calcul GPU** et requêtes d'horodatage ; fenêtrage de référence de l'écosystème | arbre de dépendances important (wgpu-core, wgpu-hal, naga, liaisons système) ; API qui change entre versions majeures | **recommandée** |
| D3D12 brut (`windows`) | Windows seul | contrôle total, outils PIX | verbeux, non portable, dépendance tout de même | écartée : ferme la portabilité pour un gain que J1 ne demande pas |
| Vulkan brut (`ash`) + fenêtrage | large, pas Metal nativement | contrôle total | très verbeux, gestion mémoire à écrire | écartée pour J1 ; reste possible plus tard derrière la même frontière |
| OpenGL (`glow`, `glutin`) | large | simple | calcul GPU inégal selon les plateformes, API en fin de vie | écartée |
| moteur (Bevy, macroquad…) | large | fenêtre, caméra, matériaux prêts | le moteur impose son architecture ; ADR-020 fait du moteur **un hôte parmi d'autres**, pas le cadre du banc | écartée pour l'hôte de validation |

## 3. Recommandation

**wgpu + winit**, et `pollster` pour attendre l'initialisation asynchrone de wgpu sans exécuteur :

| bibliothèque | version stable au 2026-09-13 | licence | Rust minimal | archive de la version |
|---|---|---|---|---|
| `wgpu` | **30.0.1** (2026-08-22) | MIT OR Apache-2.0 | 1.87 | 231 Ko |
| `winit` | **0.30.13** (2026-09-04) | Apache-2.0 | 1.86 | 434 Ko |
| `pollster` | **1.0.1** (2026-07-10) | Apache-2.0 / MIT | — | 10 Ko |

Rust local : 1.97 (S202), au-dessus des deux minimums. `wgpu` 30.0.1 déclare 28 dépendances
directes, dont 19 non optionnelles (dont `wgpu-core`, `wgpu-hal`, `naga`, `bytemuck`,
`raw-window-handle`) ; **l'arbre transitif complet et sa taille ne sont pas connus sans résolution**
et ne sont pas inventés ici.

**Implantation** : un espace de travail à part, `viewer/` à la racine du dépôt, qui dépend de
`code/water-core` par chemin. `code/Cargo.toml` n'est pas modifié ; sa règle « aucun membre avec
dépendance, construction sans réseau » reste vraie. Le `Cargo.lock` de `viewer/` est versionné pour
figer l'arbre.

**Chemin de données**, dans l'esprit d'I-08 :
- **B** : par image, le CPU publie pour chaque composante amplitude, vecteur d'onde et **phase
  temporelle repliée** calculée en entiers depuis `SimTime` ; le GPU ajoute la phase spatiale en
  coordonnées **relatives à la caméra**, pour que la précision `f32` porte là où l'on regarde ;
- **W** : pour chaque impact, le CPU calcule le profil de l'instant (`RadialTable::profile`,
  0,03–0,05 ms mesurés) et l'envoie comme tampon de 250 valeurs ; le GPU fait l'Hermite par sommet ;
- **maillage** : grille projetée à la densité que l'observateur exige (S206 : ≤ 2,75 px près de
  l'impact) — 36 000 sommets sont une charge triviale pour un GPU, pas pour un cœur de CPU ;
- **chemin cosmétique** : aucune grandeur de jeu ne sort du GPU (I-04, I-15).

## 4. Ce que je demanderai avant de télécharger

Aucune dépendance sans autorisation nommée (ADR-130 §5). La demande se fera **en deux temps**, au
début du lot de l'hôte :

1. **Résolution seule** — générer le `Cargo.lock` de `viewer/` pour `wgpu 30.0.1`, `winit 0.30.13`,
   `pollster 1.0.1` : lecture de l'index de crates.io, sans les sources. Elle donne la liste exacte
   des bibliothèques transitives et leurs versions.
2. **Téléchargement des sources** — sur présentation de cette liste, avec licences et taille totale,
   depuis crates.io uniquement.

Question ouverte à trancher à ce moment, par l'utilisateur : **conserver ensuite les sources dans le
dépôt** (`cargo vendor`, construction de `viewer/` redevenue hors réseau, plusieurs dizaines de Mo
probables) ou **se contenter du `Cargo.lock`** (réseau requis à chaque machine neuve).

## 5. Ce que la recommandation ne dit pas

Elle ne fixe pas le budget GPU de l'eau (ADR-130 : la première mesure le dira), ni la forme finale
du maillage (grille projetée, clipmap ou adaptatif), ni la technologie de rendu de δ, qui viendra
avec J2. Elle ne garantit pas le comportement de wgpu sur le matériel cible de livraison, qui
n'est toujours pas nommé.
