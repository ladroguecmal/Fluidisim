# S200 — Contrats d'exécution du noyau δ

2026-09-13. S199-1/A244. Le fond coupé S199-2 reste un lot distinct.

## Construction

`delta_projection::Volume` ne clone plus les vitesses ni la direction de pression
pendant le pas. Les opérations et leur ordre restent ceux de S199. Les sorties
temporaires sont empruntées ; les tampons déjà présents sont repris puis rendus,
sans allocation. Trois sauvegardes préallouées conservent u, w et p : en cas de
non-fini après calcul, les trois champs publiés sont restaurés avant le refus.
Les tableaux de travail sont recalculés au pas suivant ; leurs valeurs intermédiaires
ne font pas partie de l'état public. Les entrées η/fond/domaine ne sont pas mutées.

Pour C=nx·nz, U=(nx+1)·nz, W=nx·(nz+1), les octets de stockage numérique demandés
à l'hôte sont `4·[4(U+W)+2nx+C] + 8·6C`. Ils comprennent quatre tableaux f32 pour
chaque famille de faces, fond/surface/fraction et six tableaux f64 par cellule.
Cela corrige la comptabilité de S199 après son passage en f64 et inclut les nouvelles
sauvegardes. Les tailles se calculent avec contrôle de débordement avant allocation.
Ces octets excluent les en-têtes Vec et le surcoût de l'allocateur ; le protocole
d'hôte reste une réservation comptable, les Vec sont construits à l'initialisation.

## Réceptions propres

`code/water-core/tests/delta_runtime.rs` installe un allocateur global déléguant
à System. Le compteur est local au thread pour éviter le bruit du harnais parallèle.
Le JobSystem utilisé exécute ses réductions sur ce thread ; aucun worker distant
n'est couvert par ce reçu. Le cœur reste sans code unsafe ; seule l'instrumentation
d'allocateur du test en utilise pour déléguer à System.

- Une allocation réelle témoin incrémente le compteur. Les pas de limites500/1/0/500
  n'allouent pas, **premier pas compris** ; les plafonds1/0 annoncent la dégradation.
- Un état de vitesse fini extrême provoque un non-fini **pendant le calcul**. Refus
  sans allocation, u/w/p identiques à l'entrée. Après réinjection de vitesses normales,
  le pas suivant est identique à un noyau neuf : les tampons ne restent pas contaminés.
- La réservation d'octets correspond aux tableaux typés ; une dimension débordante
  est refusée avant appel à l'hôte. Les refus de configuration ne promettent pas de
  rendre des réservations déjà effectuées pour d'autres causes : pas d'API de libération.

Le reçu atomique concerne les retours `Err` numériques du noyau, pas un panic d'hôte
ni une interruption de processus. Le test historique des appels d'hôte demeure utile,
mais le test global est celui qui reçoit désormais l'absence d'allocation du pas.

## Précision, capacités et budget : portée explicite

Le solveur de pression garde ses tableaux f64 expérimentaux afin que ce lot ne change
pas la méthode numérique. **I-08 reste non reçu pour l'intégration de production** :
aucune dérogation n'est actée en documentant ce fait. Les sommes ordonnées de l'hôte
et la précision du solveur sont deux objets distincts. Un lot doit recevoir la
pression f32 ou décider d'une exception par ADR avant admission du noyau.

`step(dt,max_iters,jobs)` reste un pas à plafond d'itérations. Cela borne les opérations
de projection, pas le temps mural, et le coût d'advection/diagnostic reste dû même à
zéro itération. **I-05/ADR-007 en millisecondes restent non reçus** : ni horloge cachée
ni conversion arbitraire itérations→ms. Le raccordement à un budget temporel et sa
dégradation exigent une réception de coût incluant les phases incompressibles du pas.

Les capacités ne doivent plus annoncer un référentiel accéléré général pour un g
scalaire constant ni des bornes de résolution/CFL non mesurées. Une valeur absente
signifie « non reçue », jamais une borne zéro. Aucun choix de famille δ ni nouveau
contrat de production n'est acté : ce module reste un candidat, B3 non admissible.

## Résultats et suites

Rejeu release du filtre S199 : **0x0ad3f695685ca27a inchangée**, mêmes valeurs et
mêmes verdicts (plat1,947 ; lisse0,898 ; marche0,895). Aucun gain spatial revendiqué.
Le relevé est archivé avec la clôture ; aucun nouveau seuil de justesse, 2 % acquis.

Workspace debug **342 réussis, cinq ignorés** (246 cœur+3 intégration+93 harnais).
Les trois nouveaux tests passent aussi en release. Huit tests unitaires du noyau
rejoués après correction des capacités. La compilation rapporte des avertissements
préexistants dans les exemples et le harnais, aucun échec.
[Relevé filtre et synthèse des tests](CONTRATS-DELTA-S200-MESURES.md).

**A244 partiellement traitée** : allocations et refus numériques corrigés, capacités
resserrées. **S200-1 : préciser/recevoir la précision de pression et le budget temporel**,
avant toute admission runtime. **S199-2 : reconstruire les flux ouverts** reste
la prochaine avancée physique du noyau. Les deux objets restent dans la file,
la première réception ne doit pas être présentée comme la clôture globale d'A244.
