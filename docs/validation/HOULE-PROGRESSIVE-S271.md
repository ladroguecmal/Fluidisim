# Houle progressive — premier contrôle du résidu, S271

## Contrat avant mesure

Suite S270 : un courant uniforme ne reçoit pas une houle progressive. Avant une
campagne de durée, contrôler le résidu cinématique initial contre la dérivée
analytique indépendante d'ADR-152. Un fond linéaire n'est pas solution non linéaire :
η' ne doit pas rester nul. Le comparer à zéro masquerait la physique recherchée.

Fixture : h=1 m, longueur 4 m, k=π/2, a=0,125 m, g=9,81, ρ=1025, phase initiale
0,37 rad ; onde vers +x. θ=kx−ωt+phase, ω²=gk tanh(kh), q=agk/ω.
U=q cosh(k(z+h))/cosh(kh) cosθ ; W=q sinh(k(z+h))/cosh(kh) sinθ.
ζ=a cosθ, P=ρga cosh(k(z+h))/cosh(kh) cosθ. Dérivées exactes.

À v=η'=0, la cinématique perturbative continue exige
`η'_t = W(ζ) − W(0) − U(ζ) ζ_x`.
Cette expression locale sert d'oracle, sans réutiliser la quadrature des flux.
Contrôler d'abord incompressibilité, momentum linéaire, W(-h)=0 et ζ_t=W(0),
puis transport seul aux mailles 0,125, 0,0625, 0,03125 m. Norme L2 sur tout le
domaine, bords inclus, erreur relative <2 % à la maille fine et décroissante.
Bilan discret : télescopage vers les deux flux externes. Témoin supprimant ces
flux : doit manquer le seuil de 2 %, sinon la métrique ne distingue pas le défaut.

Pas réel : démarrage à v=η'=0, mêmes champs progressifs, dx=0,125, durées 1000 et
500 µs. Vérifier que le taux initial de hauteur tend vers le transport initial
lorsque dt baisse (la vitesse nouvellement projetée contribue à l'ordre dt).
Ce contrôle est une réception du démarrage, **pas une réception temporelle de la
houle traversante**. Arrêter au verdict de cet oracle avant de choisir la durée
et la référence de la campagne suivante. Aucun seuil modifié après mesure.
