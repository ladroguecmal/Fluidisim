# Premier raccordement volumique B/W→δ — S250

## Protocole avant construction

Contrat : [ADR-149](../adr/ADR-149-premier-raccordement-volumique.md).
Capacité visée : `Volume` consomme les échantillons différentiels B/W dans son pas
MAC, au lieu de ne recevoir qu'une hauteur imposée. Surface imposée, ν=0, coupe planaire.

1. Fond nul : même prédicteur que l'advection reçue et même pas sous la précision f32.
2. Source manufacturée : signe -S connu ; champ affine de perturbation et fond affine,
   termes croisés comparés à leur expression analytique, témoin sans terme en défaut.
3. Fournisseurs B et pression W réels planaires : somme avant contraction, réponse
   non nulle, contre-épreuve sans source ; projection à la tolérance existante S199.
4. Éponge : σ=0 identité ; facteur exponentiel analytique, intérieur intact,
   énergie décroissante sur un champ solénoïdal sous couvercle homogène. Aucun seuil
   de réflexion transmis d'un autre solveur.
5. Métadonnées, non-planarité, non-finis, pression refusée, expiration/reprise :
   publication atomique ; pas sans allocation. Rejouer la suite et l'empreinte delta_filters.

Le banc de réception doit traverser le candidat ; ni une simple somme a posteriori
ni une suppression de S ne reçoit son couplage. Les fournisseurs sont évalués à toutes
les faces ; la décimation B4, la surface mobile et les résidus de frontière sont hors lot.
