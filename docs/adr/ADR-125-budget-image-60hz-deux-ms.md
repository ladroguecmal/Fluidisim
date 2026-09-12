# ADR-125 — Profil initial : 60 images/s, eau2 ms par image

- **Statut : actée**, S202,2026-09-13, réponse explicite de l'utilisateur.
- Applique l'étape2 d'ADR-124, précise le budget de travail d'ADR-007/ADR-012.
- Ne remplace ni I-05 (respect effectif du budget), ni le seuil physique2 % ADR-120.

## Décision

Le premier profil vise **60 images/s** et **2 ms pour l'eau par image**. L'eau couvre
B/W/δ et le rendu associé sur le chemin critique. Les2 ms ne se répètent pas par
couche ou par bloc. Sans pipeline parallèle mesuré, on utilise la somme conservatrice
des coûts nécessaires à l'image ; aucun gain d'ordonnancement supposé.

Le matériel de livraison reste à nommer. Les relevés CPU de S202 décrivent la machine
locale, pas une qualification GPU ni une promesse de portabilité. Le renderer CPU
S201 reste la référence visuelle hors ligne, pas le chemin de production à60 Hz.

`cost_per_block_ms` est alimenté par une **horloge injectée** autour du pas complet.
Une mesure inconnue reste absente. S202 définit explicitement son unité : **un domaine
Volume = un bloc de banc**, avec dimensions/charge/plafond publiés. Un coût dégradé
porte son report de qualité. Aucune division par des blocs qui n'existent pas.

Une configuration dépassant déjà2 ms est incompatible avec ce profil sur la cible
mesurée. Une configuration sous2 ms est seulement compatible en coût isolé : il
reste la qualité, le coût B/W/rendu, les autres domaines et la variabilité. Aucun
ordonnanceur ni mécanisme d'arrêt temporel n'est déclaré reçu par cette télémétrie.

## Conséquences et vérification

[BUDGET-IMAGE-S202](../validation/BUDGET-IMAGE-S202.md) publie les mesures :32×16
converge à≈0,59 ms médiane ;64×32 à≈4,79 ms. On ne choisit pas32×16 comme solveur
de production : les filtres géométriques et les cas d'usage restent à recevoir.

La prochaine construction rend visible un **impact porté par W**, avec emprise et
observateur explicites, puis identifie la part exigeant éventuellement δ. V attend
un besoin gameplay. S200-1 et S199-2 restent des défauts à résoudre si le noyau est
retenu pour un effet, pas des préalables à tout affichage de l'eau.

Changer ce profil de travail demandera une décision nommant la nouvelle fréquence,
le temps eau et la cible ; aucun seuil ne sera changé pour faire passer une mesure.
