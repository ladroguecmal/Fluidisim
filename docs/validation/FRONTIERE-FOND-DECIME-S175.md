# Frontière autonome et fond décimé — S175

## Protocole déclaré

Suite S174-1/A50. Mode --assembly de fond_mobile, durée12 s au lieu de6 : la crête
part de55m et avance à3√(g(1+a))-2√g, donc dépasse90m pour a0,05 et0,06. L’inversion
analytique vérifie encore l’absence de croisement de caractéristiques. Même fenêtre
[30,90], même total initial, même fond Q amplitude0,05 et sources S173–S174.

Comparer simultanément deux fermetures aux deux étages :

- Analytic : fantômes égaux aux moyennes analytiques du total ; témoin S174.
- Anchored : écart d’invariant sortant calculé depuis T intérieur et Q intérieur,
  réancré sur Q fantôme, selon support open_boundary/S169. Aucun résidu entrant
  inconnu n’est fourni. Les moyennes Q viennent du même interpolant spatial/temporel.

L’écart de champ entre ces deux calculs est mesuré point par point àchaque pas,
puis maximisé ; ce n’est pas une différence de maxima d’erreur.
Le témoin Discrete doit retrouver un solveur total avec la même fermeture. Un témoin
total autonome est donc avancé pour chaque fond Q, avec exactement le même calcul de
frontière àchaque étage ; comparer seulement au total àfrontière analytique serait faux.

Les deux budgets de S174 restent calculés avec le flux numérique effectivement utilisé
par chaque fermeture, et les flux Q interpolés ou analytiques. Les prédictions signées
ne changent pas : changer le bord total ne change pas la différence des flux de Q.
Cela ne prouve pas que le champ ni les échanges totaux des deux fermetures soient égaux.

H0 (spatial exact)/H8m phase0,5 ; cadence continue/1/2s, amplitudes0,05/0,06 ; N120/240
et demi-pas àN240.288 évolutions résiduelles (4 sources ×2 frontières) et54 témoins
totaux (un analytique et deux autonomes par configuration). Mesurer préservation,
transport au-delà de la sortie, écart apparié des bords, identité discrète et deux budgets.
Instantané suivant de Q connu dans le véhicule analytique seulement ; aucun événement
inconnu anticipé. Aucun coût3D, réception perceptive ou seuil is_smooth_at annoncé.
