# ADR-070 — Champ de référence d'une pression gaussienne mobile

- **Statut : ACTÉE**, S90, 2026-09-08, délégation technique.
- Complète ADR-069. Instrument de référence f64 ; aucun raccordement au runtime autoritaire.

## Modèle et construction

Première pression localisée : `p(x,t)=P0 exp(-|x-X(t)|²/(2σ²))`, avec translation rectiligne
pendant un segment de durée finie. σ est un écart type spatial en mètres, P0 un pic en Pa.
Ce profil est un candidat prescrit, pas une pression de coque calibrée ni un volume déplacé.
La formulation profonde linéaire et ses références restent celles d'ADR-069.

Convention Fourier et transformée, dérivées pour ce profil :

```
p(x) = 1/(2π)² ∫ P_hat(k) exp(i k·x) d²k
P_hat(k) = 2π σ² P0 exp(-σ² |k|²/2)
d²k = k dk dθ.
```

`GaussianPressure` construit les directions et nombres d'onde par quadrature des milieux
sur `[0,k_max] × [0,2π]`, avec poids `k Δk Δθ/(2π)²`. Chaque mode réutilise la réponse
analytique S89, avec l'amplitude de transformée ci-dessus. Le même noyau linéaire accepte
cette amplitude dimensionnée en Pa·m² et produit une transformée d'élévation, à intégrer.
Le mode k=0 n'est pas échantillonné et n'est pas remplacé par une onde artificielle.

`field` prépare à l'instant demandé les réponses complexes puis `sample` reconstruit hauteur
et vitesse verticale aux points. `unit_pressure` reçoit séparément la transformée inverse
du profil. Le prototype utilise Vec, libm et f64, comme instrument hors I-03/I-06/I-08 runtime,
sans modifier ces invariants. Nombres radiaux 1–512 et directions paires 4–512 sont des limites
de travail de l'instrument, **pas une réception de toutes ces configurations**.

## Énergie globale et travail

Parseval appliqué au champ continu donne les intégrales :

```
E = ρ/[2(2π)²] ∫ [g |q_hat|² + |q_dot_hat|²/k] d²k       [J]
Power = -1/(2π)² ∫ Re(P_hat conj(q_dot_hat)) d²k          [W].
```

Les fonctions modales S89 portent une moyenne de cosinus réel par unité d'aire. Le facteur
de conversion est ici **2 × poids de quadrature** pour l'énergie et la puissance. Confondre
la moyenne modale et la transformée continue donnerait un facteur deux erroné. Une intégration
spatiale indépendante de `-p eta_dot` contrôle ce raccordement.

`energy_j` est une estimation par quadrature de l'énergie du champ continu sous-jacent.
Ce n'est pas l'intégrale sur le plan infini de la somme finie d'ondes utilisée pour échantillonner :
cette somme ne possède pas une décroissance lointaine certifiée. Aucun domaine carré périodique
n'est introduit, mais les erreurs lointaines de quadrature ne sont pas supprimées pour autant.
Il faut encore recevoir une emprise spatio-temporelle avant emploi régional.

## Résultats S90

Fixture : σ=1 m, P0=10 Pa, v=(2,0) m/s, émission 0–4 s, g=9,81, ρ=1025,
eau profonde idéale. Les chiffres suivants ne valent que pour ce montage.

| Contrôle | Résultat |
|---|---|
| Profil unitaire, quatre points de rayon 0 à 5 m | erreur absolue 64² ≤3,701e-4 ; 128² ≤9,180e-5 |
| Énergie 1 s, 128² / 256² | 0,06274106813456 / 0,06274105985157 J |
| Énergie 4 s, 64² / 128² / 256² | 0,08833796909674 / 0,08830547110295 / 0,08830337350684 J |
| Énergie 8 s après extinction | identique à 4 s à l'arrondi près |
| Travail temporel 48×64, 400 milieux de pas | 0,08840987296891 J ; énergie 0,08840879144878 J |
| Puissance à 2 s, spectral / spatial | 0,02301678222474 / 0,02301678224234 W |

Le premier essai 64²→128² **a échoué** au seuil 1e-5 J : écart 3,25e-5 J.
Le seuil est conservé ; réception du raffinement 128²→256², écart 2,10e-6 J (~0,0024 %).
La normalisation du profil montre aussi une convergence proche d'un facteur quatre.
Les points de champ testés sont (-4,0), (0,0), (4,2), (12,0), aux instants 1/4/8 s :
écart fin/finer <1e-6 m sur hauteur et <1e-5 m/s sur vitesse ; symétrie y→-y vérifiée.

Deux autres contrôles séparent les axes d'erreur : directions 64→128 à radial fixé 128,
et k_max 6→9 rad/m avec Δk constant (128→192 nœuds). Les seuils de régression sont
respectivement 1e-8 m sur les points angulaires testés, 1e-9 m sur le point de coupure et
1e-12 J pour sa différence d'énergie. Ces seuils restent **à calibrer pour un usage élargi**.
La coupure de pression à 6/σ omet exactement exp(-18) de son intégrale au centre ; cette
borne ne devient pas une borne générale de réponse dynamique.

Le travail spatial utilise la gaussienne exacte, un carré de ±6σ centré sur la source et
48×48 milieux de cellules. L'écart avec la puissance spectrale vaut 1,76e-11 W.
Le travail temporel concorde à 1,08e-6 J avec l'énergie du **même** maillage 48×64 ; ceci
ne reçoit pas sa précision spectrale, inférieure à celle de 128²/256². Leçon L204.

## Ce qui reste ouvert

S89-1 réalisée sur un segment et ce profil de référence. W4 et S88-1 restent partiels :
pas de réception de l'angle de Kelvin, de courbe coque/pression, de non-linéarité, de fond
variable, d'intégration LiveWater ou de conformité interplateforme. Aucun nouveau budget cible.
**S90-1, prochaine session S91 :** trajectoire à plusieurs segments et travail total sur le
champ superposé, avec test de découpage invariant et changement de direction. Ne pas additionner
les énergies des segments (L203). Puis fixer l'emprise de réception et construire le chemin runtime.
Aucun nouvel angle numéroté ; I-01, I-03, I-06, I-08 et I-11 inchangés.
