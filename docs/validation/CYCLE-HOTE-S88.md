# S88 — Cycle hôte dynamique et coût mesuré

2026-09-08. Exemple `code/water-core/examples/bench_live_water.rs`, exécutable en release
et testable explicitement avec `cargo test -p water-core --example bench_live_water` depuis code/.
Aucun changement du modèle physique ni nouvel ADR. S87-1 réalisée dans ce scénario local.

## Parcours exercé

B spectral à 32 composantes, ancré à 1 000 000 m ; 64 points monde espacés de 0,25 m sur
une grille 8×8. N=128, rayon 16 m, gravité 9,81, densité 1025, profondeur 20 m. Sources
de 0,01 J et longueur d'onde 4 m, positions distinctes. Ce sont des fixtures, pas un profil AAA.

Le service initial dispose de trois emplacements par journal et par pool. Il reçoit une
prédiction puis sa confirmation, une seconde prédiction puis son rejet, et une autre confirmation.
Il reste deux impacts calculés et trois enregistrements ; le rejet occupe bien sa place.
Une requête B+W monde s'exécute à 1 s. La confirmation suivante sature le journal et bloque
la vue courante. La sauvegarde WLIV mesure 483 octets.

Le service source est détruit. Une cible neuve dispose de quatre emplacements par journal
et par pool. Seuls les octets sauvegardés relient les services. Après restauration, la commande
attend toujours ; sa nouvelle tentative à 12 s avec horizon 16 s réussit. Les trois impacts
sont interrogés par le chemin commun B+W aux 64 points monde.

Une référence construit directement les trois événements confirmés, insérés dans un ordre
différent, sans passer par LiveWater ni WLIV. Les hauteurs, dérivées, normales et vitesses
concordent bit à bit avec la cible restaurée. L'empreinte finale inclut dix scalaires par point :
**2518ba19f6e53c8d**, identique sur les répétitions release. Ce contrôle reçoit le cycle logiciel,
pas la physique commune aux deux chemins. Les réceptions physiques antérieures restent nécessaires.

Un dernier point monde extrême invalide aussi une requête après 63 points valides : toute la
sortie reste inchangée. Le scénario vérifie les rétractions et le refus d'accès pendant le blocage.

## Mesures locales

Windows, poste habituel Ryzen AI 7 350 ; compilation release. Trois échauffements, 21 parcours
mesurés. Microsecondes, p90 à l'indice floor((n−1)×0,9). Aucune autre campagne de tests lancée
simultanément par cette session ; charge système extérieure non contrôlée.

| Opération | Médiane | p90 | Maximum observé |
|---|---:|---:|---:|
| Cinq admissions et leurs reconstructions | 6,6 | 8,1 | 9,7 |
| Requête B+W, 2 impacts × 64 points à 1 s | 429,2 | 442,7 | 490,9 |
| Sauvegarde du service bloqué | 0,2 | 0,7 | 1,2 |
| Restauration et reconstruction des 2 impacts publiés | 2,8 | 4,4 | 4,6 |
| Reprise de la commande et extension des 3 impacts | 3,7 | 3,8 | 4,1 |
| Requête B+W, 3 impacts × 64 points à 12 s | 639,6 | 660,2 | 697,9 |

Les allocations/configurations hôte et les comparaisons de référence sont hors chronométrage.
Les appels de service et leurs contrôles internes sont inclus. Le refus de saturation n'est
pas chronométré ; le tableau n'est pas un temps total de démarrage. Sauvegarde en mémoire,
sans fichier, réseau ou crash disque ; 0,2 µs ne représente donc pas une latence de stockage.
Les deux requêtes diffèrent en nombre de sources et en instant : ne pas isoler un coût par
source en soustrayant ces deux lignes. Aucun maximum contractuel ni budget cible reçu.

Mémoire mesurée par les types du build : cible à deux journaux **832 octets**, deux pools
N128 **25 344 octets**, temporaire de restauration **416 octets**, sortie/temporaire de lot
**5120 octets**, sauvegarde **483 octets**. Ce ne sont pas les octets totaux du processus :
points monde, B, table Bessel, métadonnées, pile et référence de test s'ajoutent. Les tableaux
locaux illustrent la propriété hôte, pas un placement recommandé sur la pile du moteur.

## Conclusion de construction et suite

L'administration du petit service est nettement moins coûteuse que ses requêtes ; aucune
optimisation supplémentaire du codec ou du journal n'est justifiée par ce scénario. La mémoire
et le coût augmentent encore avec les sources retenues ; le service ne purge rien.

**S88-1, prochaine session S89 :** commencer la première source de sillage issue d'une
trajectoire en milieu profond uniforme : fixer la relation entre mouvement, forçage et énergie,
puis construire un candidat testable. Une suite d'anneaux Impact ne constitue pas à elle seule
une réception de sillage. W4/ADR-054 et ADR-011 cadrent ce travail ; les autres régimes restent
explicites. S72-2 rétention et durée arbitraire, autorité réseau complète, budgets et sélection
B2 restent ouverts. Aucun invariant amendé ni angle numéroté ajouté ; leçon L202.
