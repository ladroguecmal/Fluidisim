# Budget coopératif du candidat δ — S230, 2026-09-14

## Contrat et critère de réception déclarés avant construction

Lot J2/S200-1/A244, application de SPEC-004 §4.1 et ADR-007/012, sans modifier I-05.
`step_budgeted` reçoit un budget en millisecondes et une horloge monotone injectée. Il publie
soit un pas entier avec son rapport numérique, soit **zéro temps avancé et tout dt restant**.
Un arrêt faute de temps conserve u/w/p bit à bit ; le prochain appel recommence le pas depuis
cet état. Les calculs interrompus ne sont ni sérialisés ni conservés comme progression acquise.

Contrôles dans préparation, advection, second membre, pression, correction, diagnostics et
validation. Tranches d'au plus 64 éléments entre contrôles pour les parcours ; une réduction
d'hôte porte au plus 64 cellules, avec les mêmes groupes et le même ordre de fusion qu'avant.
Le retour arrière échange des buffers préalloués en temps constant ; aucune recopie de domaine
après expiration. Les buffers sortis temporairement sont rendus même lors d'un refus.

Le pas à plafond d'itérations et son enveloppe de mesure restent disponibles pour les bancs.
Le chemin non limité doit garder ses bits et ses tests. La pression reste expérimentale f64 ;
aucune exception à I-08 pour δ ni admission B3 n'est déduite de ce lot.

**Garantie coopérative, pas préemption.** L'hôte doit fournir une horloge non bloquante et des
réductions à durée bornée. L'expiration est observée au prochain contrôle ; le dépassement peut
comprendre une tranche, un appel d'hôte, le contrôle et le retour. Une désallocation massive ou
un rollback linéaire après expiration seraient des défauts, pas des marges admissibles.
Le maximum local observé n'est pas une borne de pire cas système : I-05 complet reste à recevoir
par admission des blocs et marges sur le matériel/hôte visés. Une horloge reculant est refusée.

Réception prévue : expiration à chaque phase et après plusieurs itérations, état non nul
intact puis continuation identique à un témoin ; zéro budget, horloge reculant, non-fini et
absence d'allocation avec témoin positif. Mesurer chemin complet, coût des contrôles et retard
observé au retour sur grilles/charges déclarées, sans ajuster le seuil après mesure.
