# ADR-130 — La version interactive J1 rend l'eau sur GPU, par un hôte séparé

- **Statut : actée**, S207, 2026-09-13, **arbitrage de l'utilisateur** (réponse à la question posée
  en fin de S206, option recommandée).
- **Tranche** l'arbitrage « chemin de rendu et hôte de J1 » de la
  [feuille de route](../FEUILLE-DE-ROUTE.md) §4, qui fusionnait l'hôte interactif et A247.
- **Applique** ADR-020 (« le renderer, ses ressources GPU : l'hôte fournit un `IGpuBackend`
  minimal, ou aucun »), ADR-003 et I-08 (« seules des phases repliées passent au GPU »), et le
  `gpu_sim_ms` qu'ADR-012 déclarait sans qu'aucun hôte l'exerce (A250).
- Mesures qui l'ont motivée : [COUT-IMAGE-S206](../validation/COUT-IMAGE-S206.md).

## La question, et la réponse

Sur la scène représentative de J1, B évalué par sommet sur CPU coûte 42 ms par image à la
densité qui montre un impact (36 ms sur seize fils), contre 2 ms visés ; W est déjà ramené à
27 µs par impact (ADR-129). Quatre options étaient posées : (A) GPU par un hôte séparé, avec
dépendances ; (B) CPU seul, en comptant les 2 ms en temps mur sur tous les cœurs ; (C) changer le
profil ADR-125 ; (D) une densité qui perd l'impact visible.

**L'utilisateur a retenu (A) : GPU, hôte séparé.**

## Décision

1. **La version interactive J1 évalue B et W sur GPU**, dans un **hôte d'affichage séparé**. Les
   options (B), (C) et (D) ne sont pas retenues ; ADR-125 (60 images/s, eau 2 ms) est inchangé.
2. **`water-core` reste sans dépendance** (ADR-020) et reste l'autorité de ce qui s'évalue : il
   **publie** ce que le GPU consomme — recettes cuites de B, phases repliées en entiers (I-08),
   tables d'impact d'ADR-129 — et ne connaît aucune API graphique.
3. **L'hôte d'affichage vit hors du workspace actuel**, qui garde sa règle : aucun membre avec
   dépendance, construction hors réseau. Ce qui a des dépendances est un espace de travail à part,
   de sorte que le cœur et le harnais se construisent et se testent toujours sans réseau.
4. **Chemin cosmétique** : ce que le GPU calcule sert à l'image. Aucune grandeur de jeu n'en
   provient (I-04, I-15) ; les requêtes de gameplay restent servies par la bibliothèque.
5. **Aucune dépendance n'est téléchargée sans une autorisation nommée** : bibliothèques, versions,
   taille et source seront soumises à l'utilisateur au début du lot de l'hôte, pas avant.

## Ce qui reste ouvert, et qui tranchera

- **La pile exacte** (API graphique, fenêtrage) : choix technique du lot de l'hôte, soumis avec
  la demande de téléchargement.
- **Le budget GPU de l'eau.** ADR-125 fixe « eau 2 ms par image » sans dire où l'eau s'évalue ;
  ADR-012 déclarait `gpu_sim_ms = 2,5`. Aucune valeur n'est inventée ici : la première mesure de
  l'hôte GPU sur la scène représentative dira s'il y a une incompatibilité, et elle s'arbitrera
  explicitement (ADR-127 D7).
- **Le déterminisme du chemin GPU** n'est pas requis (chemin cosmétique) ; celui de B et W
  répliqués reste celui de la bibliothèque (I-03).

## Conséquences sur la trajectoire

J1 sort quand l'hôte GPU rend la scène représentative en temps réel, composée B+W, et que son
coût est mesuré face au profil. ADR-129 reste utile tel quel : la matrice de Bessel est la donnée
qu'un impact envoie au GPU. La réversibilité appartient à l'utilisateur : revenir à (B), (C) ou
(D) demande sa décision.
