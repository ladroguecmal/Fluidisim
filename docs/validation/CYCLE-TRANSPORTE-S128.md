# S128 — Le montage transporté dans le service B+W

2026-09-10. S127-1. Applique ADR-063, ADR-066, ADR-067, ADR-068 et ADR-085.
Bibliothèque inchangée ; nouveau scénario `cycle_transported_water`.

## Montage et chemin reçu

Un impact S127 : N256, rayon80 m, λ4 m, énergie0,01 J, g9,81, ρ1025,
profondeur20 m, pente maximale0,1, naissance0, **TTL source4 s conservé**.
Fond B réel :32 composantes, Hs0,01 m, Tp6 s, direction0, graine0, ancre monde
à1 000 000 m, référentiel7/cellule9. Ces valeurs sont celles d'un banc, pas une calibration.
Les64 points couvrent le centre, quatre directions et des rayons croissants ; deux sont
exactement sur le bord80 m. La conversion monde→local est celle du chemin de production.

Le scénario suit cet ordre :

1. Confirmation serveur dans un `LiveWater<256>` vide ; horizon4 s, requêtes0 et4 s.
2. Refus à4 s+1 µs, puis renouvellement à24 s et requêtes4 s,4 s+1 µs,12,137119 s et24 s.
3. Sauvegarde WLIV24 ; destruction du service source ; restauration dans un service vide
   de même N, puis répétition des cinq dates0/4/4+1 µs/12,137119/24 s.
4. Renouvellement24→48 s, requêtes24 s+1 µs,36,271829 s et48 s ; sauvegarde WLIV48.
5. Refus contrôlés et vérification de la dernière publication ; nouvelle destruction et
   restauration48, puis requêtes non monotones48/24/4/0/48 s.

Les sources de `LiveWater` sont détruites avant les reprises ; aucun champ de l'ancien
service ne sert à restaurer la cible. Les buffers de sauvegarde sont en mémoire : aucune
durabilité disque ou récupération après crash n'est reçue.

## Identité et refus

**20 lots de64 = 1280 points-temps** : chaque composante de chaque sortie est comparée
en bits, pas seulement par hash. La référence construit directement un `RadialImpact<256>`
d'horizon48 depuis le même événement et le journal confirmé, puis appelle la composition
ponctuelle avec le même B. Elle ne passe ni par LiveWater, ni par WLIV, ni par la requête
en lot. Cette comparaison reçoit **l'acheminement et la reprise** ; la composition et les
noyaux physiques restent partagés, avec réception physique distincte en S127.

Dix valeurs : η, dη/dt, steepness, aération, normale xyz et vitesse totale xyz.
Toutes sont finies ; la case de sortie après le lot reste intacte. À48 s, les64 résultats
diffèrent de B seul en bits : la contribution W n'a pas été effacée au TTL.

| refus provoqué | résultat observé | conservation vérifiée |
|---|---|---|
| requête après4 ou48 s, avant renouvellement | `Point(0, Domain)` | toutes les sorties |
| dernier point à81 m | `Point(63, Domain)` | toutes les sorties, malgré63 points valides |
| extension à64 s, R80/N256 | `Prepare(Field(1, Resolution))` | sauvegarde publiée identique, aucune attente |
| instant48 s+1 µs, horizon demandé48 s | `Horizon` | sauvegarde publiée identique |
| dernier octet de WLIV48 manquant | refus de restauration | sauvegarde publiée et requête48 conservées |
| restauration256 dans une cible128 | `Context` | sauvegarde vide de la cible identique |

Le nom `Domain` des refus temporels de la requête vient de la composition actuelle ; le
rapport le conserve tel qu'il est. L'horizon est disponible par `renewal_deadline` ;
`current()` seul n'affirme pas qu'une date donnée soit calculable. Aucun contrat changé ici.

WLIV fait **289 octets** pour cet événement. Le header conserve N256 et l'âge48 s ;
la sauvegarde réémise après chaque restauration est identique à l'entrée.

| date | hash des64 sorties |
|---|---|
| 24 s+1 µs | `7ed7cd1a46c53eee` |
| 36,271829 s | `8103cbc89831cfb9` |
| 48 s | `565bdb15e3ac7503` |

Ces trois hashes concordent en debug et release locaux ; aucune conformité interplateforme
supplémentaire n'est établie.

## Coût mesuré

Reproduction :

`cargo run --release --manifest-path code/Cargo.toml -p water-core --example cycle_transported_water`

Ryzen AI 7 350, Windows x86_64 MSVC, rustc1.97.0/LLVM22.1.6, profil release du dépôt.
Une seconde de mise en régime de **toutes** les opérations avant mesure ; quinze blocs
alternent l'ordre des six opérations. Chaque bloc donne la moyenne de32 exécutions.
Min/max sont donc les extrêmes des moyennes de blocs, pas les pires latences individuelles.
Entrées/sorties consommées avec `black_box`, assertions de réception hors chronométrage.

| opération | médiane campagne1 | médiane campagne2 | min/max campagne2 |
|---|---:|---:|---:|
| renouvellement4→24 seul | 2,375 µs | 2,331 µs | 2,275 / 2,888 µs |
| requête64 à48 s | 0,95749 ms | 0,95273 ms | 0,91171 / 1,01985 ms |
| sauvegarde48 seule | 0,109 µs | 0,106 µs | 0,091 / 0,163 µs |
| restauration48 seule | 2,456 µs | 2,469 µs | 2,434 / 14,706 µs |
| cycle24→48→reprise48, trois requêtes64 | 3,02731 ms | 2,92758 ms | 2,84083 / 3,25019 ms |
| renouvellement24→48 et requête64 | **0,95638 ms** | **0,96160 ms** | 0,92432 / 1,08021 ms |

La première exécution de développement, avant ajout de la dernière opération, donnait
0,97108 ms pour la requête et3,00353 ms pour le cycle : contexte de mise au point seulement.
La fluctuation des moyennes ne permet pas d'interpréter quelques microsecondes d'écart
entre requête seule et renouvellement+requête. Les mesures très courtes incluent le coût
de lecture de l'horloge ; le chiffre de sauvegarde n'est pas un coût disque.

**Frontières du chronométrage :** B et pools déjà construits. Avant chaque mesure, le
service est réinitialisé hors chrono à une sauvegarde d'âge4,24 ou48 selon l'opération.
Le cycle complet mesuré commence donc sur un service déjà admis à4 s : renouvellement24,
requête24, renouvellement48, requête48, sauvegarde48, restauration sur cible existante,
requête48. Il inclut ces opérations et leurs trois requêtes ; il exclut admission initiale,
construction de B/pools, disque et comparaison à la référence.

Un cycle ordinaire renouvellement+requête coûte ici environ**0,96 ms pour un impact et64
points**, soit environ15 µs par point en moyenne. Aucun budget cible ni capacité de montée
en charge n'est certifié par ce scénario à une source. Pas de pression dans ce montage.

## Mémoire allouée du scénario

| allocation mesurée | octets |
|---|---:|
| deux pools de champs pour un service, un emplacement chacun | 12480 |
| deux journaux pour un service, un enregistrement chacun | 208 |
| somme des pools d'un service | **12688** |
| temporaire de restauration, un enregistrement | 104 |
| deux buffers de requête de64 sorties | 5120 |
| une sauvegarde complète | 289 |

Le benchmark possède simultanément source et cible : deux fois les pools de service.
Ces tailles n'incluent pas B, l'arène de l'hôte, les objets de contrôle, les points monde,
les sauvegardes supplémentaires du banc ni les tableaux de chronométrage. Les pools hôte
sont fixes et B est scellé ; aucune nouvelle allocation de production introduite.

## Validation et suite

**S127-1 réalisée sur fixture.** Deux campagnes release finales et une exécution debug
avec `--verify-only` passent toutes les assertions, avec hashes identiques. Production
inchangée ; suite256/cinq ignorés vérifiée en S125, non relancée ici. La nouvelle campagne
n'est pas comptée artificiellement comme de nouveaux tests unitaires.

**S128-1, prochaine session S129 :** fermer le bilan énergétique du **candidat** N256/R80/48 s
en mesurant sa partie cinétique depuis ses nœuds réellement construits, comme S78 le faisait
à courte durée, puis comparer au bilan indépendant S127 et raffiner l'intégration radiale.
S127 reçoit le bilan de la référence et la surface du candidat ; S128 reçoit l'acheminement
de cette surface. Aucun de ces succès ne mesure son énergie cinétique en profondeur.

Défaut N64 et ADR-085 inchangés. Autres sources/paramètres, calibration B2, profondeur finie,
pression mixte, admission dynamique mixte, bilan mixte et durabilité disque restent ouverts.
85 ADR,204 angles,17 invariants,6 spécifications,23 cas ; aucun nouvel angle ou ADR.
