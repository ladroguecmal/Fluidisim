# S166 — Entrée prescrite et sortie autonome

## 1. Protocole avant mesure

Suite S165-1/A221. Véhicule mouillé Saint-Venant 1D, lit plat, Rusanov/RK2 S165,
fenêtre [30,90] m. Aucun oracle global dans la fermeture autonome.
À partir de h_t+u h_x+h u_x=0 et u_t+u u_x+g h_x=0, poser c=sqrt(gh) :
R±=u±2c vérifient (∂t+(u±c)∂x)R±=0 par substitution.
En régime subcritique |u|<c, un invariant entre à chaque bord, l'autre sort.

- Bord gauche : R+ prescrit par Q extérieur ; R- lu dans la cellule intérieure.
- Bord droit : R- prescrit par Q extérieur ; R+ lu dans la cellule intérieure.
- Reconstruction : u=(R++R-)/2, c=(R+-R-)/4, h=c²/g, q=hu.

Refuser explicitement les états secs, non finis, supercritiques ou c reconstruit <=0.
Recalculer les fantômes à **chaque étage** avec le total intérieur de cet étage.
Comparer quatre méthodes : invariants ci-dessus, extrapolation du total intérieur,
fond seul, et témoin analytique total aux deux étages. Ce dernier n'est pas une identité
discrète comme l'oracle S165 : il donne les états d'une solution continue exacte.

## 2. Référence dérivée et cas

Onde simple avant choc : h0(y)=1+a exp(-(y/8)²), u0=s·2(sqrt(g h0)-sqrt(g)), s=±1.
Un invariant est constant. Pour x'=s(x-x_c), résoudre
x'=y+[3 sqrt(g h0(y))-2 sqrt(g)]t, puis h=h0(y), u=s·2(sqrt(gh)-sqrt(g)).
Le transport de l'autre invariant reçoit directement cette formule.
L'inversion est unique si t·(3 sqrt(g)/2)·a·sqrt(2/e)/8<1, borne dérivée de
max |d exp(-(y/8)²)/dy|=sqrt(2/e)/8 et h>=1.

Durée 24 s, amplitudes 0,02 et 0,05 m, N global équivalent 120/240/480 (dx=1/0,5/0,25 m),
pas 0,2 dx/sqrt(g) et contrôle au demi-pas N240. Paramètres d'instrument, pas profils.
Sortie : centre initial 60 m, Q au repos. Entrée : centre 10 m (s=+1) ou 110 m (s=-1),
Q égal à l'onde simple connue ; d initial nul. Tester les deux directions.
L'extérieur résiduel inconnu de S165 n'est pas remplacé par une information inventée.

Mesurer max erreur hauteur/débit contre l'analytique, max écart au témoin analytique
discret (pour distinguer fermeture et erreur intérieure), erreur dans le cœur [40,80],
bilan de flux ouvert, Courant. Normaliser par a et a sqrt(g).
Le témoin doit converger en espace ; aucune identité à l'arrondi exigée contre le continu.
Témoins : repos, invariants de l'onde simple, refus supercritique, entrée omise par
extrapolation. Le seuil 1e-10 S165 garde seulement bilan et identités algébriques.

## 3. Mesures et réception

À produire en P3/P4. Aucun seuil perceptuel ou absorbeur choisi.
