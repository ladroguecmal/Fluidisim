# Résidu progressif temporel — S272

## Référence déclarée avant mesure

Prolonger S271 à une durée de 2 s. Domaine x∈[0,4], profondeur h=1 m,
k=π/2, phase 0,37, g=9,81, rho=1025 ; a=0,01 puis 0,005 m pour contrôler
la troncature d'ordre deux. dt=2 ms, dx=0,125 puis 0,0625 et 0,03125 si nécessaire.
Pas d'éponge : référence avec v normal nul aux bords, identique au contrat réel.
Il ne s'agit donc pas d'une réception de transparence avec éponge.

Écrire φ=φ1+φ2, η=η1+η2, où φ1=(ag/ω) C(z) sinθ, θ=kx−ωt+phase,
C=cosh(k(z+h))/cosh(kh), ω²=gk tanh(kh). Développement des conditions exactes
à z=η, en gardant l'ordre a² :

- η2_t = G φ2 + K ; K=−U1 η1_x + η1 W1_z = a q k sin(2θ), q=agk/ω.
- φ2_t = −g η2 + D ; D=−η1 φ1_tz −(U1²+W1²)/2 à z=0.
- Avec T=tanh(kh), D=D0+Dc cos(2θ), D0=a²gkT/2−q²(1+T²)/4,
  Dc=a²gkT/2−q²(1−T²)/4.

G est l'opérateur harmonique de profondeur finie à murs de Neumann. Dans la
base cos(nπx/L), λ_n=k_n tanh(k_n h). Chaque mode vérifie
η_n'=λ_n φ_n+K_n, φ_n'=−gη_n+D_n, η_n(0)=φ_n(0)=0.
Donc η_n''+gλ_nη_n=λ_nD_n+K_n', η_n'(0)=K_n(0).
Résoudre exactement cette ODE forcée sinusoïdale ; projections spatiales
analytiques, sans solveur MAC ni interpolation de ses résultats.

## Critères et arrêt

- Contrôler les projections par quadrature indépendante et l'ODE par intégration
  RK4 à pas divisé par deux ; erreur relative oracle <1e-6.
- Oracle tronqué à 128 puis 256 modes : écart L2 espace-temps <0,1 % sur les
  mêmes points de mesure (x centres, t de 0,05 à 2 s).
- Comparer **η' évolué**, pas B+η' dominé par le fond. Erreur L2 espace-temps
  <=2 % (seuil B4) et décroissante en raffinant dx ; diviser dt par deux doit
  changer la solution de <=0,5 % de la norme de l'oracle. Tous les pas reçus.
- Amplitude divisée par deux : comparer les champs normalisés par a², écart
  <=1 % à résolution reçue, sinon troncature non qualifiée.
- Si refus numérique ou seuil manqué : conserver la mesure, isoler une cause
  avant correction ; ne pas remplacer la métrique par l'erreur du champ total.
- Arrêt au reçu borné ou diagnostic chiffré avec prochain correctif concret.

La comparaison reçoit le résidu dans un domaine de perturbation fermé traversé
par un fond prescrit. Une houle incidente entièrement transparente, d'autres
spectres, la 3D, le budget et les forces restent hors de ce banc.
