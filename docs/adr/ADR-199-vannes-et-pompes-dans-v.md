# ADR-199 — Vannes et pompes dans V

- **Statut : actée**, S372, 2026-09-26, par le projet (décision technique interne, ADR-028).
- **Précise** [ADR-010](ADR-010-reseau-hydraulique-volumes-finis.md) §1 et §3 — *« Edge = ouverture (orifice, seuil
  de débordement, vanne, matériau poreux, pompe) »* — pour la vanne et la pompe ; ADR-010 n'est pas réécrit.
- **Précise** [ADR-140](ADR-140-restauration-du-graphe-V.md) (« vannes/pompes […] appelleront une version/migration
  explicite ») : le format WVST passe en version 2.
- Liste **5.4** ; preuve : [VANNES-POMPES-S372](../validation/VANNES-POMPES-S372.md).

## 1. Le constat

Le noyau V (S224–S229) connaît l'orifice et le déversoir, **fixes**. Un jeu d'avarie et d'inondation demande ce que le
joueur ou le système **commande** : fermer une vanne de coursive, ouvrir un sas de ballast, lancer une pompe de cale. Ce
sont des états qui changent pendant la partie, répliqués, que V doit intégrer au bit (I-03) et que la sauvegarde doit
garder.

## 2. Décisions

**D1 — Une commande par arête, séparée de sa configuration.** `Opening::control_pm`, entier de 0 à 1 000, **1 000 par
défaut** ; il appartient à l'**état** du graphe (comme le volume d'un nœud et le reste d'une arête), pas à sa base d'auteur.
Il est posé par l'hôte entre deux pas, depuis un événement de jeu répliqué — V ne le décide jamais. Un pas qui lit une
commande hors de 0..=1 000 est refusé, atomiquement.

**D2 — La vanne est une ouverture commandée.** Orifice : section × `c/1 000` sous Torricelli ; déversoir : largeur ×
`c/1 000` sous la loi des trois demis (une vanne-batardeau, qui réduit la lame). À 1 000, les lois sont **celles d'avant,
au bit** ; à 0, rien ne passe. Le coefficient de débit reste celui de l'arête : sa variation avec l'ouverture (une
vanne-guillotine à moitié fermée n'a pas le `C_d` d'un orifice à arête vive) est **à calibrer**, par arête, sur la
courbe du constructeur.

**D3 — La pompe, en réseau ouvert.** `Flow::Pump { max_flow_mlps, shutoff_head_um, outlet_um }` : la prise est la
position de l'arête (dans le nœud amont), le refoulement `outlet_um`. Courbe **parabolique** de pompe centrifuge,
`H(Q) = H0·(1 − (Q/Qmax)²)` — la forme usuelle d'une courbe de pompe entre le point de barrage et le débit maximal
(Karassik et al., *Pump Handbook*, McGraw-Hill ; approximation déclarée, les constantes sont données par l'auteur de l'arête).
Point de fonctionnement contre la **hauteur statique** `Δh = cote de refoulement − surface amont` — la cote de
refoulement est la sortie, ou la surface du receveur si la sortie est noyée —, pertes de charge des conduites négligées :
`Q = Qmax·√(n² − Δh/H0)` si `n²·H0 > Δh`, sinon **zéro** (clapet anti-retour : jamais de retour par la pompe). **Prise
dénoyée** (surface amont sous la prise) : zéro — la pompe tourne à sec. **Vitesse** `n = c/1 000` par les lois de
similitude (`Qmax·n`, `H0·n²`). Le débit passe ensuite par la quantification, la normalisation et les limiteurs
existants : la masse reste exacte.

**D4 — Ce qui n'est pas fait, et reste où il est.** Le **réseau fermé sous pression** (canalisations pleines, pompe
qui pousse dans une conduite fermée) reste la v2 d'ADR-010 §4 — une pompe D3 débite toujours dans un nœud à surface
libre ou à l'air. Ni temps de manœuvre (la commande saute d'un pas à l'autre ; une rampe est l'affaire de l'hôte), ni
puissance ni énergie consommée, ni cavitation, ni pertes de charge, ni matériau poreux.

**D5 — L'instantané WVST version 2.** Une troisième liste d'écarts, les commandes différentes de la base d'auteur,
comptée dans les quatre octets réservés de l'en-tête. La base d'auteur hache la commande d'auteur et la loi de pompe ; la
configuration comparée à la base **exclut la commande**. Une sauvegarde version 1 est **refusée** (`Version`), sans
migration : aucune n'existe hors des essais — c'est la migration explicite qu'ADR-140 annonçait, et elle ne réinterprète
rien.

## 3. Ce qui changerait la décision

Un réseau de canalisations fermées dans un niveau (D4) ; une vanne dont la loi n'est pas proportionnelle à l'ouverture
(courbe à pourcentage égal : remplacer la section par une table) ; une pompe volumétrique (débit indépendant de la
hauteur : une seconde loi, pas une modification de celle-ci).
