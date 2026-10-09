# ADR-289 — Des solveurs éprouvés seuls avant d'être combinés

- **Statut : actée**, S747, 2026-10-09. Proposée par l'utilisateur, discutée, acceptée (*« Ok parfait »*).

## Contexte

De S734 à S747, la 3D, prise pour référence depuis S690, s'est révélée fausse sur l'essai le plus simple d'un modèle de vagues : elle ne
garde pas une onde solitaire sur un fond plat (S739). La cause est le tassement des particules (S740). Le défaut est resté caché cent
sessions : l'essai canonique n'avait jamais été fait, et la 3D se jugeait elle-même (ADR-287).

L'utilisateur, le 2026-10-09 : il aurait testé chaque solveur seul, sous stress, avant de les combiner, et il a demandé pourquoi il n'y a pas
un solveur « complet » en 2D et un en 3D. Il a ensuite demandé un avis réfléchi, et non calqué sur ses mots.

## Décision

**D1 — Trois familles de solveurs, chacune la moins chère qui soit juste dans son domaine.**
- **Le spectral** (B et W) pour le large : les vagues courtes devant la profondeur, que même SGN fausse.
- **Le 2D moyenné** (Saint-Venant, SGN) pour les côtes, les rivières et les crues : la hauteur et la vitesse moyenne sur la verticale.
- **La 3D** (APIC) pour le local : le retournement, les jets, l'air, les corps.

Un solveur 2D « complet dans tous les cas » n'existe pas : moyenné sur la verticale, il ne peut ni retourner une vague ni porter l'air.

**D2 — Chaque solveur est éprouvé seul avant d'être combiné**, sur un banc d'essais à **référence indépendante** (une solution exacte, des
mesures publiées), jamais contre un autre solveur du projet. Un essai de stress sans référence ne prouve que la robustesse.

**D3 — L'ordre du travail** :
1. **Finir la correction de la 3D.** La projection de densité, sous l'une de ses variantes, ou une autre approche si aucune ne tient le
   banc ; le déferlement juste n'est pas encore prouvé.
2. **Le banc de la 3D**, réduit mais rigoureux, 6 à 8 essais : le repos, l'onde solitaire, la levée, le ballottement, la rupture de barrage,
   la remontée, le déferlement contre Synolakis, un corps qui flotte. Une version courte est rejouée à chaque changement du cœur, la complète
   quand le cœur change en profondeur (un essai 3D coûte 10 à 60 min).
3. **Refaire les résultats importants** avec la 3D corrigée : le plongeon de R43, les témoins du sélecteur, Synolakis.
4. **Le banc de la 2D telle qu'elle est**, Saint-Venant et SGN séparément. La fusion des deux en un solveur 2D (la dispersion qu'on allume,
   le passage à Saint-Venant au déferlement, le séchage) est **décidée après**, selon ce que le banc montre et ce que le jeu demande : c'est
   un gros chantier, et la 2D n'est pas aujourd'hui le point faible.
5. **Les combinaisons** (les raccords), chacune jugée contre une référence indépendante.
6. **Le sélecteur** (ADR-284, ADR-285 D4).

**D4 — Les nouveautés attendent** (le LOD, les boîtes, le sélecteur) tant que D3.1 à D3.3 ne sont pas faits. Le risque, plusieurs sessions
de vérification avec peu de progrès visible, est accepté par l'utilisateur.

## Conséquences

- Les mesures de déferlement et de remontée faites avec l'ancienne 3D (S647, S690–S730, S734) sont **à refaire** ; les raccords, la masse
  exacte et les boîtes restent vrais.
- ADR-287 D1 (le banc canonique) devient l'étape D3.2 de ce plan.
