# ADR-140 — Restaurer les écarts de V et ses restes de débit

- **Statut** : acté sur délégation technique.
- **Session** : S229, 2026-09-14.
- **Précise** : ADR-022 §4.2–4.3 et §5.1, ADR-010 §4, ADR-139.

Le volume seul ne permet pas de continuer V à l'identique : chaque ouverture porte un reste
fractionnaire que le prochain pas consomme. La persistance conserve les volumes différents de
l'état d'auteur **et** les restes non nuls. Les plans restent dérivés ; aucun champ δ n'est écrit.

## Contrat construit

Une `Baseline` emprunte les nœuds et ouvertures d'auteur et les formes validées. Elle fixe
l'ordre des indices, l'identité du graphe/référentiel fournie par l'hôte et la révision des assets.
Les restes d'auteur sont nuls. La configuration est immuable : toute modification de topologie,
origine, loi, coefficient, forme ou valeur d'auteur exige une nouvelle base ; aucune migration
silencieuse ne réinterprète une ancienne sauvegarde.

Une empreinte FNV-1a 64 bits, mécanisme déjà présent dans le cœur, couvre les champs de la base
dans un ordre et un encodage entiers définis, y compris **tous les sommets ordonnés** des
tétraèdres ou toutes les entrées des anciennes tables, et la version du calcul. Elle détecte les
incompatibilités accidentelles ; ce n'est ni une preuve sans collision ni une authentification.
La révision et l'identité explicites se vérifient aussi. Les données cuites restent chez l'hôte.

Format `WVST` version 1, entiers little-endian, sans disposition mémoire Rust : en-tête de
80 octets, listes triées de `(indice u32, valeur i64)` de 12 octets, puis contrôle FNV de 8 octets.
L'en-tête contient longueur totale, empreinte, identité, révision, instant `T_sim`, durée du
prochain pas et bits de `g_eff`, puis les deux nombres d'écarts. Les valeurs de volume sont
absolues pour les seuls nœuds modifiés ; les arêtes n'écrivent que leur reste non nul.
Taille : `88 + 12 × (nœuds modifiés + restes non nuls)` octets.

La capture se fait entre deux pas. La restauration retourne le contexte temporel et gravitaire ;
l'hôte doit le consommer et fournir les mêmes entrées futures. Le pas actuel accepte un `dt`
positif quelconque : le format le conserve, sans imposer artificiellement 100 ms au codec.

Validation intégrale avant toute écriture : version, taille exacte, intégrité accidentelle,
identité de base, contexte, indices uniques triés, volumes bornés et restes dans `1..=999999`.
Les entrées égales à l'état d'auteur sont non canoniques et refusées. La restauration reconstruit
d'abord la base, puis ses écarts ; les emplacements absents ne gardent jamais un ancien état.
Les tranches de destination et de sortie sont fournies par l'appelant ; aucune allocation.
Une capture invalide ne modifie pas son tampon ; une restauration refusée ne modifie aucun nœud
ni aucune arête. On ne simule pas un pas lors de la restauration (L201).

## Portée et réception

Premier graphe fixe du noyau V : les champs non construits (`liquid_id`, flags, nœuds dynamiques,
vannes/pompes, couplage V↔δ) ne sont pas inventés dans le format. Leur construction appellera une
version/migration explicite. Le codec fournit la charge utile ; transport réseau, autorité,
durabilité disque et assemblage `WaterPersistentState` B/W/V ne sont pas reçus par lui.

Réception : continuation C19-V sur le pas réel, restes non nuls, témoin d'omission discriminant,
tables et géométrie orientée, reconstruction depuis une destination sale, refus atomiques de
corruptions/troncatures/configurations modifiées, et compteur d'allocation positif puis nul.
La comparaison bit à bit locale ne reçoit pas I-03 sur une seconde plateforme.
