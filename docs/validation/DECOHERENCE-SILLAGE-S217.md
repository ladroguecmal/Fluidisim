# S217 — Décohérence du sillage : similitude et histoire du forçage

## Protocole déclaré avant campagne

**Objet** : A255, part dynamique ; savoir si une table dépendant seulement du temps réduit
depuis extinction pourrait remplacer le majorant directionnel d'ADR-134.

**Techniques présentes** : source gaussienne du cœur, demi-spectre, préparation exacte du
journal, coefficients d'image rebasés, majorant directionnel ADR-134. Recherche hors ligne
par récurrence trigonométrique f64 sur grille puis raffinement local, vérifiée contre le cœur.
**Absentes** : GPU, LOD, visibilité, mutualisation, solveur δ. Aucun coût d'image mesuré.
**Domaine** : une trajectoire droite à charge constante en milieu uniforme profond ; g=9,81,
rho=1025. Les variantes sont des fixtures de banc, pas des paramètres de production.
**Rang de passage** : sans objet, aucun temps d'exécution publié.

### Dérivation depuis le modèle exécuté

La dispersion du noyau est `omega² = g k`. Avec `q = sigma k`, `T = sqrt(sigma/g)`,
`u = v/sqrt(g sigma)`, `d = D/T` et `tau = (t-D)/T`, la réponse à une source mobile
contient à la fois `sqrt(q)` et le Doppler `q_x u`. L'intégration entre naissance et
extinction conserve `d` dans les amplitudes et phases modales. La charge multiplie
linéairement tout le champ : elle s'élimine dans le rapport majorant/maximum, à l'arrondi près.

Une homothétie `sigma -> a sigma`, `kmax -> kmax/a`, `x -> a x`,
`v -> sqrt(a) v`, `D -> sqrt(a) D`, `t -> sqrt(a) t`, `F -> a³ F`
(g et rho fixes) conserve les pentes et leurs rapports. Cette **similitude conditionnelle**
ne supprime ni `u`, ni `d`, ni `sigma*kmax`, ni la forme du trajet. Elle ne prouve aucune
loi à une variable pour la famille générale. Source : équations exécutées dans
`modal_pressure`, cuisson `gaussian_spectrum`, force/aire dans `wake_source` ; SPEC-001 §1.

### Montages et critères

Base : sigma=2 m, cutoff=3 rad/m, v=3 m/s, F=19 620 N, D=8 s, quatre tronçons,
origine (-12,0), emprise [-64,64]×[-48,48] m. Grille 1 m puis recherche locale à
pas 0,05 m ; contrôle grille 0,5 m avec raffinement à 0,025 m. Recette 64×128,
puis 128×256 et 256×512 sur les cas discriminants. Maxima toujours minorants échantillonnés.

- Homothéties a=0,5 et 2 : même temps réduit ; écarts relatifs de rapport attendus <0,1 %.
- F/2 et découpage en huit tronçons : témoins de linéarité et de représentation (<0,1 %).
- Vitesse 1,5 et 6 m/s, durée 4 et 16 s variées séparément ; après-extinction à
  tau=0,4,12,24. Un écart de rapport >5 % persistant sous raffinement **réfute**
  la courbe universelle en tau sur ces fixtures. Ce seuil est un discriminant de banc,
  pas une tolérance physique ni une garde de production.
- Forçage séparé : t/D=0,25 et 0,75, sans les ranger dans tau positif.
- Vérifier la pente reconstruite contre `Prepared::sample_batch` à l'argmax et en
  plusieurs points fixes ; écart absolu <1e-5, seuil de banc déclaré. Zéro refus masqué.
- Pour interpréter les cas discriminants, variation du maximum <1 % entre les deux
  dernières quadratures et les deux grilles. Sinon publier « non convergé ».

La durée d'image annoncée par ADR-132 accompagne chaque recette. Une mesure au-delà est
diagnostique et ne reçoit pas une image. Aucune enveloppe universelle sûre ne se déduit
du succès de quelques maxima : pas de table de production sans justification supplémentaire.
