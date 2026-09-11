# S168 — Moyennes de cellules et flux intégrés

## 1. Dérivation et protocole avant mesure

Suite S167-1/A223. Pour une solution exacte Q de Q_t+∂xF(Q)=0,
Qbar_i(t)=1/dx ∫cellule Q(x,t)dx et I_face=∫t..t+dt F(Q(face,s))ds vérifient
dx(Qbar_i+−Qbar_i0)=I_gauche−I_droite. Recevoir cette identité **avec deux intégrations
indépendantes**, sans reconstruire I depuis la variation de Qbar ni réciproquement.

Le résidu S167 garde son flux numérique différentiel et son RK2. Pour Q exact, Sbar=0.
La variation totale sur la fenêtre est alors l'intégrale RK2 du flux différentiel net
plus I_gauche−I_droite. Pour un Q figé, Sbar_i=[F(Q_gauche)−F(Q_droite)]/dx exactement
en espace : sa somme télescope vers les flux physiques du bord, non vers une somme
de dérivées ponctuelles. Aucune retouche a posteriori de l'état ou du bilan.

Référence onde simple S166, centre **55 m** au lieu de60 pour casser la symétrie,
largeur8 m, direction droite, fenêtre[30,90], durée6 s. Trois cas S167 :
Q=T onde exacte a0,05 ; Q a0,05 et T a0,06 (perturbation non nulle) ; Q figé a0,05,
T onde évolutive a0,05. Fantômes T en moyennes de cellules aux deux étages, fournis
par la référence : le bord autonome reste hors réception de cette session.

N global équivalent120/240/480, pas nominal0,2 dx/sqrt(g) et demi-pas N240.
Comparer trois représentations : centres + flux RK2 (témoin S167), moyennes + flux RK2
(isole le temps), moyennes + flux intégrés (candidat). La deuxième et la troisième ont
le même état évolué : seul le diagnostic d'intégration du flux prescrit change.

Quadrature adaptative de Simpson : intégration du polynôme interpolant les extrémités
et le milieu, poids1/6,4/6,1/6. Comparer un panneau à ses deux moitiés ; le terme dominant
en pas^4 est divisé par16, d'où estimateur différence/15 et extrapolation. Tolérance
absolue1e-12 sur chaque composante, contrôle1e-13, profondeur maximale déclarée ; refuser
la non-convergence. Tolérances d'instrument, pas seuils physiques. La quadrature spatiale
évalue Q, la temporelle évalue son flux : aucun flux n'est inféré de l'état avancé.

## 2. Réceptions attendues

Fond exact d=0 ; bilan local de Qbar et bilan total de la fenêtre <1e-10 relatif au
volume initial (seuil S165 conservé). Flux net non nul constaté, y compris pour Q figé.
Perturbation et correction du Q figé comparées aux moyennes de T exact au raffinement.
Comparer aussi au continu sans confondre conservation et transport. Rejouer S167 si
son support change ; aucune bibliothèque ni dépendance nouvelle.

## 3. Résultats

À recevoir en P3/P4.
