# Flux du fond aux frontières — S270

## Critères déclarés avant correction

Consommateur : `Volume::step_perturbation_mobile`, ADR-165. Arrêt du lot : défaut
stationnaire reproduit puis corrigé dans ce pas, flux signés et bilan reçus,
refus/reprise et absence d'allocation vérifiés. La houle progressive complète
reste une réception séparée ; ne pas appeler ce cas constant une onde progressive.

1. Courant uniforme U=±0,5 m/s et niveau de fond a=±0,125 m, profondeur 2 m,
   domaine 4 m, dx=0,25 puis 0,125, dt=1 ms, 20 pas. Fond exact stationnaire,
   pression dynamique ρga constante : η'=v=0. Erreur de hauteur <=4 ulp du repos,
   critère d'arrondi ; témoin ancien premier pas ±dt Ua/dx, défaut >100 ulp.
2. Flux de bande signé contre intégrale indépendante de courant constant, y
   compris bande coupée par le fond solide ; bilan discret global = différence
   des deux flux, tolérance d'arrondi f32. Flux calculés avant modification de η'.
3. Refus d'élévation incohérente sur une frontière, état publié inchangé.
4. Un pas avec flux de bord non nul : chaque expiration restaure l'état ; reprise
   identique au bit et zéro allocation, via compteur réel de l'intégration.
5. Régressions du workspace release, dont identité du fond nul S253 et harmoniques
   stationnaires. Aucun seuil physique relevé après mesure.
