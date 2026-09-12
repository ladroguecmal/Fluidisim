# ADR-124 — Image, budget, puis effets volumiques bornés

- **Statut : actée**, S201,2026-09-13, direction explicite de l'utilisateur.
- Modifie la priorité de construction portée par S198–S200 et restreint le périmètre
  de livraison de δ envisagé par ADR-001/ADR-007 ; ne réécrit aucun ADR antérieur.
- Autorisation supplémentaire explicite : exemple CPU produisant une image locale
  PPM, sans dépendance ni publication. Elle a été demandée puis accordée.

## Décision et ordre

1. **Rendre B visible**, depuis le champ du projet : caméra, rayons et image de banc.
   S201 le réalise. Les images locales dérivées et leur preview sont permises ;
   Markdown demeure le format de conception, aucune page HTML ni publication requise.
2. **Fixer le budget d'image et brancher le coût par bloc**, sur cible/charge identifiées.
   Coût mesuré et budget choisi sont deux données distinctes. Les mesures S183–S185
   peuvent informer les coûts des composants qu'elles couvrent, pas remplacer une
   mesure du rendu complet ou d'un bloc δ absent de ces campagnes.
3. **δ est un effet borné par son cas d'usage**, par exemple impact ou représentation
   visuelle de compartiment inondé. Un solveur volumétrique général ne conditionne plus
   l'affichage de B/W ni leurs réceptions propres. **V attend un besoin gameplay nommé**.
   Les domaines, interfaces et budgets de ces effets seront spécifiés après le budget
   d'image. Le noyau S199–S200 est conservé comme candidat, sans promesse d'intégration.

La décomposition B/W demeure. δ ne reçoit aucune autorité gameplay (I-04), aucune
sérialisation (I-17) ; reporter V ne transfère pas ses responsabilités à δ. Les cinq
arbitrages d'ADR-027 ne sont pas rouverts. Ni coût de production ni choix de famille
de solveur ne sont déduits du simple fait de limiter son périmètre.

## Conséquences sur les reçus et la file

**2 % est déjà décidé**, ADR-120. Les mentions historiques de critère numérique
manquant ne justifient pas de nouvelles campagnes d'attente. Une image permet
l'observation humaine ; elle ne fournit pas à elle seule un critère perceptuel,
un double aveugle, une force sur coque ou la validation numérique d'ADR-123.

B4 conserve ses exigences, mais ses volets se reçoivent sur les cas effectivement
livrés ; aucune réception globale n'est revendiquée par contournement du solveur.
A50 reste partielle. A98/multijoueur, physique de B et superposition restent distincts.
S200-1 (précision/budget δ) et S199-2 (faces coupées) sont **reportées derrière cette
direction** ; les défauts doivent être résolus si un effet retenu emploie ce noyau.

La caméra donne une distance pour explorer le LOD, pas un LOD implémenté. Mousse,
spray, audio et vue sous-marine ont désormais un premier support d'observation à
étendre, pas une réception ni une implémentation acquise.

## Vérification et réversibilité

[IMAGE-B-S201](../validation/IMAGE-B-S201.md) porte images, paramètres et limites.
La prochaine session traite le budget d'image ; le lot suivant dimensionne les effets
δ/V. Revenir à un δ général demande une nouvelle décision explicitant ses cas utiles
et ses moyens, pas la reprise automatique d'un reliquat de banc.
