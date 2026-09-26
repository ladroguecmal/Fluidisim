# ADR-204 — La pluie, une arête de V : surface d'ouverture, exposition commandée, intensité en entrée du pas

- **Statut : actée**, S378, 2026-09-26, par le projet (décision technique interne, ADR-028), au service de décisions de
  l'utilisateur ([ADR-202](ADR-202-niveau-de-detail-des-contenants.md) D3, [ADR-203](ADR-203-reponses-aux-zones-d-ombre-d-adr-202.md) D2).
- **Précise** [ADR-010](ADR-010-reseau-hydraulique-volumes-finis.md) §5 (*« Pluie : Q = intensité · surface_libre ·
  sky_exposure »*), qui n'est pas réécrit ; s'appuie sur [ADR-199](ADR-199-vannes-et-pompes-dans-v.md) (la commande d'arête)
  et [ADR-140](ADR-140-restauration-du-graphe-V.md) (WVST v2). Liste **5.5**.

## 1. Décisions

**D1 — La pluie est une arête de V**, `Flow::Rain { catchment_mm2 }`, **du ciel vers un nœud** : son `from` et son `to`
désignent tous deux le nœud qui la reçoit. Elle ne vide aucun nœud : la normalisation par nœud amont l'ignore, et
l'application n'ajoute qu'à l'aval. Son débit passe par la même quantification à report de reste, et par la même borne de
place libre, que les autres arêtes : un contenant plein ne reçoit plus rien par elle — un trop-plein ou un déversoir
(autres arêtes) évacue le surplus.

**D2 — Surface d'ouverture, et non surface libre.** ADR-010 §5 écrivait la surface libre. Un contenant ouvert reçoit la
pluie qui tombe **dans son ouverture** : celle qui frappe ses parois intérieures au-dessus de l'eau y ruisselle et finit
dans l'eau. La surface d'ouverture, horizontale, est une donnée de l'auteur (mm²), fixe ; elle vaut la surface libre pour
un prisme. Un contenant qui collecte au-delà de son ouverture (gouttières, toit) le dit par sa surface.

**D3 — L'exposition est la commande de l'arête** (ADR-199 D1) : 0 à 1 000, la part de l'ouverture que le ciel atteint.
L'exposition du décor, précalculée, en est la valeur d'auteur ; une **bâche entière** la met à 0, une **demi-bâche** à 500,
posées et retirées à tout moment (ADR-203 D2) : c'est un état répliqué, sauvegardé par WVST v2, sans format nouveau. Qui
calcule la part couverte à partir des objets posés (lancer de rayons, occultants mobiles) est l'affaire de l'hôte et de la
prévision ; V ne reçoit que le nombre.

**D4 — L'intensité est une entrée du pas**, `Meteo { pluie_mm_h }`, par `step_meteo` : une valeur pour tout le réseau
(un réseau tient dans une région ; une intensité par nœud viendra avec la météo, si elle le demande). Elle est fournie
**à l'identique à tous les participants** (I-03) ; qui la calcule relève de la météo (ADR-203 D1). `step` reste le pas
**sans pluie**, au bit.

## 2. Ce qui n'est pas fait

L'absorption par le sol (ADR-010 §5, l'autre moitié de 5.5) ; la neige, la grêle ; l'évaporation (ADR-203 D6) ; la
création de flaques par la pluie hors contenant (ADR-010 §5) ; le rendu des rides de pluie (ADR-202 D3, un effet factice).
