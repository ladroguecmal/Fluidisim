# ADR-059 — Le support périodique ne devient pas régional en raccourcissant sa durée

- **Statut : ACTÉE**, S76, 2026-09-08, délégation technique.
- **Complète** ADR-058 ; conserve ses mesures, refuse son adoption comme impact régional.
- **Preuve** : [TRANSPORT-RADIAL-S76](../validation/TRANSPORT-RADIAL-S76.md).

## Décision

Le champ ImpactField à 40 modes reste un support de comparaison physique. Il ne doit pas
être raccordé tel quel comme un impact isolé à l’interrogation régionale B+W. Il contient une
copie exacte à 16 m dès t=0 dans le scénario ; réduire ttl ne supprime pas cette copie.
Son rayon d’énergie croît de 2,217 à 7,040 m entre 0 et 8 s, puis revient à 6,293 m à 12 s.
L’énergie totale tient à 5e-8 relatif sur la référence f64 construite, sans certifier l’isolement.

Le prochain développement est un **candidat radial non périodique dans sa définition continue**,
avec quadrature spectrale de Hankel en eau profonde. On garde un événement isotrope et un milieu
uniforme. Ce choix vise à supprimer la répétition imposée par le carré, pas à sélectionner W
pour B2. La quadrature finie aura son propre domaine de validité : elle doit le déclarer.

## Contrat de la prochaine tranche S77

Convention envisagée : eta(r,t) = intégrale A(k) J0(kr) cos(omega(k)t) k dk,
omega²=gk. Le potentiel de surface porte -A(k) omega/k J0(kr) sin(omega t).
À naissance, le spectre fixe une déformation initiale à vitesse nulle, pas un contact solide.
Par l’identité de Parseval de cette convention radiale, l’énergie initiale vaut
pi*rho*g intégrale A(k)² k dk. La dérivation et la quadrature doivent être vérifiées dans
le code, sans reprendre la normalisation du carré ni une amplitude issue du volume audio.

Le spectre, ses bornes, la résolution et le rayon/temps recevables seront des paramètres
explicites ; leur choix de qualité reste à calibrer B2. Les valeurs de J0/J1 et la sommation
doivent avoir un chemin déterministe avant toute autorité gameplay. Une bibliothèque
transcendante non spécifiée ne reçoit pas I-03 par la seule présence d’un hash.

La réception de cette tranche demande : référence indépendante aux petits arguments et aux
zéros de Bessel, convergence au raffinement spectral sur un domaine déclaré, énergie et volume,
et refus au-delà du domaine testé. Il faut aussi vérifier que l’énergie s’éloigne sans recopier
la source à une distance fixe. Une quadrature finie n’autorise pas une promesse de validité
à rayon et temps infinis. Le test de vitesse de groupe exige un paquet suffisamment étroit ;
le déplacement moyen du spectre large S75 ne le remplace pas.

## Ce que la mesure a corrigé

La densité rho/2*(g eta² + psi*deta_dt) employée en S75 était valable **sous intégrale**.
Sa partie cinétique n’est pas une densité locale positive. Le diagnostic radial S76 emploie
rho/2 intégrale |grad phi|² dz, calculée par produits de modes et 1/(k_i+k_j).
Aucun résultat total S75 n’est rétracté. Le changement de question exigeait un changement
de mesure ; la formule correcte pour le total ne localisait pas l’énergie.

S75-1 est close par mesure et décision de support. W3 reste partielle. S72-2 reste ouverte :
le nouveau candidat doit éclairer rétention et erreur de troncature, sans inventer une mort TTL.
Invariants relus I-03, I-08 et I-14, inchangés. Aucun code de propagation régional ajouté en S76.
