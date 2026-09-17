# Coût des reflets validés — S266

Contrat : [ADR-162](../adr/ADR-162-ciel-precalcule-des-reflets.md).
R7 acceptée par l'utilisateur le 2026-09-17 : « Très bien continue ».

## Protocole avant construction

1. Images témoins R7 directes : sept poses à 5 m/s au bit ; conserver les fichiers S265.
2. Cache contre fonction GPU directe sur des directions couvrant la sphère, les coutures,
   pôles, horizon et soleil. Valeurs finies ; publier erreur moyenne, p99 et maximale.
   L'erreur radiométrique ponctuelle est un diagnostic, l'acceptation se fait sur l'image.
3. Les sept poses aux mêmes réglages : erreur RGB sRGB moyenne ≤0,25 niveau/255,
   p99 ≤2 niveaux et maximum ≤16 niveaux, **par image**. Seuils d'approximation numérique
   déclarés pour conserver R7, pas seuils physiques ni vérité perceptive universelle.
4. Coût eau : baisse d'au moins 10 % de la médiane contre le calcul direct dans les deux
   poses référence/rasante, 1280×720, 120 images après chauffe, secteur ; le budget 2 ms
   reste distinct. Rapporter p95, coût initial de cuisson, mémoire et allocations d'update.
5. Cache réutilisé après mouvement caméra/temps ; invalidé après changement clair/brume,
   retour clair identique. Pas de cuisson répétée par image en régime.
6. Tests de l'hôte et `--reflets-verify`, sans nouveau passage du cœur inchangé.

Arrêt : optimisation intégrée si les critères sont tenus ; sinon conserver la mesure du rejet
et le témoin. L'erreur de quadrature 3/5 et le budget CPU du sillage restent des limites distinctes.
