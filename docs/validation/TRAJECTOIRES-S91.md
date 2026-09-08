# S91 — Trajectoires segmentées et travail total

2026-09-08. Réalisation de S90-1, conformément à ADR-069/070 ; aucun nouvel ADR physique.
`GaussianPressure::trajectory` reçoit une suite non vide de segments temporellement contigus
et spatialement raccordés. La vitesse et la pression peuvent changer entre segments ; le profil
gaussien reste commun. Les intervalles actifs sont semi-ouverts : au raccord, seule la nouvelle
pression travaille. Les ondes des segments terminés continuent de contribuer au champ.

Chaque réponse complexe est calculée avec sa naissance et son origine puis sommée avant le
calcul de l'énergie. La puissance utilise la pression active contre la vitesse **totale**.
`field` délègue désormais au même chemin pour un segment. Les trous temporels, chevauchements,
durées invalides, valeurs non finies et sauts de position sont refusés avant construction.

Le raccord spatial exige ici l'égalité f64 de `origine + vitesse × durée` avec l'origine suivante.
C'est un contrat d'instrument : une trajectoire hôte doit construire ses raccords avec la même
opération. Il ne reçoit ni les données bruitées ni une tolérance géométrique de production.
Un virage instantané est un forçage prescrit, pas un modèle d'accélération réaliste de coque.
Un arrêt d'émission suivi d'un redémarrage doit être représenté par un segment de pression nulle,
sans trou dans la chronologie ; cette représentation n'ajoute aucune onde pendant l'arrêt.

## Mesures et vérification

Trois tests S91 en release puis suite debug. Instrument f64 avec allocations, hors runtime.

- Une translation 0–4 s scindée à 2 s reproduit hauteur, vitesse, énergie et puissance du
  segment entier à l'arrondi près. Instants 0/1/2/2,000001/4/8 s ; trois points spatiaux.
- Virage de 90° à 2 s : v=(2,0) puis (0,2) m/s, σ=1 m, pic 10 Pa, g=9,81, ρ=1025.
  Quadrature 32×48, coupure 6 rad/m. Travail intégré sur 400 milieux de pas de 0,01 s :
  **0,1267090729839 J** ; énergie totale à 4 s : **0,1267085948949 J** ; écart **4,781e-7 J**.
  Somme des énergies isolées : **0,2160453260870 J**, volontairement différente.
- À 3 s, puissance spectrale **0,01208323529831 W** ; intégration spatiale indépendante
  de la gaussienne exacte contre la vitesse totale : **0,01208323530044 W** (40² cellules,
  carré ±6σ autour de la pression). Énergie après extinction à 8 s conservée.
- Rejets exercés pour trou et chevauchement d'une microseconde, saut de position et NaN futur,
  y compris lorsque l'instant demandé précède le segment invalide.

Seuils de régression : 1e-13 J/W pour découpage, 1e-14 m ou m/s sur ses échantillons,
2e-5 J pour travail temporel, 1e-8 W pour travail spatial. Ils reçoivent ces identités et
fixtures, **pas une précision de production**. Comme L204 le rappelle, fermer le bilan ne reçoit
pas la finesse spectrale 32×48 du virage. Aucun angle de Kelvin ni emprise générale déduit.

## Suite

S90-1 réalisée sur le candidat de référence. **S91-1, prochaine session S92 :** fixer une
emprise spatiale et une fenêtre temporelle reçues par raffinement indépendant des directions,
rayons et coupure, notamment pour le virage. Exposer les refus hors de cette emprise avant le
portage runtime. Coque/pression, accélération lissée, déterminisme et intégration restent ouverts.
Aucun invariant ou angle numéroté ajouté. L203/L204 appliquées ; aucune nouvelle leçon distincte.
