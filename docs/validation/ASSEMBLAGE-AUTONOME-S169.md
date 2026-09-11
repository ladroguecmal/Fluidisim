# S169 — Assemblage autonome, équilibré et conservatif

## 1. Dérivation avant mesure

Suite S168-1/A224. Résidu en moyennes S168, Q exact connu, S=0 ; RK2 sur d,
flux total = flux différentiel numérique + flux physique de Q intégré dans le temps.
La fermeture doit produire un fantôme égal à Q fantôme lorsque d intérieur est nul.

S166 transportait un invariant **total** depuis l'intérieur. Pour Q variable, ses valeurs
diffèrent entre cellule intérieure et fantôme. Le candidat transporte plutôt l'écart :
ΔR_sortant=R(T_intérieur)−R(Q_intérieur), avec R±=u±2sqrt(gh).
Au fantôme, R_sortant=R(Q_fantôme)+ΔR_sortant ; invariant entrant=R(Q_fantôme).
Gauche : sortant R− ; droite : sortant R+. Hypothèse : aucun résidu entrant prescrit.

Reconstruction sans annulation à d=0 : δu=ΔR/2, δc=−ΔR/4 à gauche, +ΔR/4 à droite ;
δh=(2c_Q δc+δc²)/g, δq=H_Q δu+u_Q δh+δh δu. Ajouter ces écarts à Q fantôme.
Cette écriture donne exactement Q pour ΔR=0 sans seuil artificiel. Recalculer aux deux
étages avec les totaux et Q intérieurs de l'étage. Refus secs/non finis/supercritiques S166.

Quatre voies : témoin T exact en moyennes, transfert total S166, transfert d'écart ci-dessus,
fond seul. Les fantômes exacts ne sont consultés que par le témoin et les diagnostics.

## 2. Protocole

Fenêtre[30,90] m, référence onde simple S166 en moyennes/quadratures S168, durée24 s,
avant choc ; largeur8 m. Cas :

- **Entrée pure** : Q=T a0,05, centre initial10 m vers la droite (110 m vers la gauche).
  d=0 doit rester nul pendant l'entrée puis l'approche de la sortie du fond variable.
- **Sortie sur repos** : Q=(1,0), T a0,05 centre60 m. Perturbation initialement intérieure.
- **Sortie sur fond variable** : Q a0,05, T a0,06, centre55 m vers la droite (65 vers la gauche).
  Les deux champs sont des solutions non linéaires exactes, pas une superposition.

Les gaussiennes ne sont pas à support compact : mesurer l'écart d'invariant entrant
analytique omis par la fermeture, y compris celui dû aux moyennes. Ne pas le déclarer nul.
La fermeture ne peut pas recevoir un résidu entrant inconnu par hypothèse.
N120/240/480 vers la droite ; contrôles gauche et demi-pas N240 :15 montages, quatre voies.

Mesurer max erreur hauteur/débit contre T moyen, max écart de champ au témoin aux mêmes
mailles, max résidu et bilan de volume sur les **flux effectivement utilisés**, pas sur
ceux du témoin. Normalisations0,05 m et0,05 sqrt(g) m²/s ; bilan relatif au volume initial.
Seuil instrumental1e-10 pour préservation, identités et bilan, aucun seuil physique.

Tests des supports S165–S168 à rejouer s'ils sont modifiés ; extraire les références communes
sans les dupliquer. Aucune API runtime adoptée ni performance de production reçue.

## 3. Résultats

À recevoir en P3/P4.
