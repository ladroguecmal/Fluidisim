# Coût des reflets validés — S266

Contrat : [ADR-162](../adr/ADR-162-ciel-precalcule-des-reflets.md).
R7 acceptée par l'utilisateur le 2026-09-17 : « Très bien continue ».

## Protocole avant construction

1. Images témoins R7 directes : sept poses à 5 m/s au bit ; conserver les fichiers S265.
2. Cache contre fonction GPU directe sur des directions couvrant la sphère, les coutures,
   pôles, horizon et soleil. Valeurs finies ; publier erreur moyenne, p99 et maximale.
   L'erreur radiométrique ponctuelle est un diagnostic, l'acceptation se fait sur l'image.
3. Les sept poses aux mêmes réglages : erreur RGB sRGB moyenne ≤0,25 niveau/255,
   p99 ≤2 niveaux et maximum ≤16 niveaux, **par image**. Seuils d'approximation numérique
   déclarés pour conserver R7, pas seuils physiques ni vérité perceptive universelle.
4. Coût eau : baisse d'au moins 10 % de la médiane contre le calcul direct dans les deux
   poses référence/rasante, 1280×720, 120 images après chauffe, secteur ; le budget 2 ms
   reste distinct. Rapporter p95, coût initial de cuisson, mémoire et allocations d'update.
5. Cache réutilisé après mouvement caméra/temps ; invalidé après changement clair/brume,
   retour clair identique. Pas de cuisson répétée par image en régime.
6. Tests de l'hôte et `--reflets-verify`, sans nouveau passage du cœur inchangé.

Arrêt : optimisation intégrée si les critères sont tenus ; sinon conserver la mesure du rejet
et le témoin. L'erreur de quadrature 3/5 et le budget CPU du sillage restent des limites distinctes.

## Résultat : cache rejeté, 2026-09-18

Construction conservée dans Git à `e60b8fa` (512²) ; variante 1024² testée par changement
unique de SIZE. Aucun cache conservé dans l'hôte final : le code S265 est rétabli avant
l'optimisation suivante. Cette issue applique le critère d'arrêt, sans relever de seuil.

512² : MAE RGB des sept images 0,0017–0,0737, P99 ≤1, maximum 22 >16 : rejet précision.
1024² : MAE 0,0013–0,0300, P99 ≤1, maximum 19 >16 sur référence et fond seul : rejet précision.
Les sept témoins directs sont identiques au bit aux PPM R7 S265.

1024², 48 Mio, secteur 99 % début/fin, RTX 5070 Laptop DX12, 1280×720, ordre 3,
120 images après chauffe : médiane GPU eau référence 2,785→2,669 ms (**4,2 %**),
rasante 2,507→2,392 ms (**4,6 %**), sous le minimum de gain 10 % : rejet coût.
Cuisson initiale 0,151 ms, une seule sur les deux poses ; update zéro allocation.
Premier passage de coût 512² écarté : il avait chevauché la fin des captures GPU.

Contrôle directionnel : 8 501 directions, clair/brume/clair, retour au bit et réutilisation
caméra/temps reçus. Au ciel clair 1024² : erreur HDR moyenne 0,000760, P99 0,027453,
max 0,474048 ; brume 0,000220 / 0,000527 / 0,001195. Diagnostic, pas réception radiométrique.
Journaux et images dans `viewer/captures/s266` (`cache1024-verify.log`, `bench1024.log`).

## Replanification : sommes de covariance, avant construction

Le filtre de la queue est strictement nul après Nyquist, les composantes étant triées par k.
À partir du premier poids nul, la covariance manquante est exactement la somme de toutes les
covariances restantes, indépendante de la caméra. Précalculer cette somme suffixe dans le
buffer de queue ; conserver le témoin par `--reflets-somme-directe`. Pas de changement
architectural : même fermeture ADR-161, même quadrature et même ciel, ordre de sommation seul changé.
Critères : GPU contre oracle aux trois vents (tolérances S265), images sept poses mêmes seuils
0,25 / 2 / 16, update sans allocation, baisse d'au moins 10 % aux deux poses dans le même
passage. Aucun seuil modifié après mesure ; budget 2 ms toujours distinct. Si reçu, intégrer ;
sinon publier aussi le rejet et conserver R7.

## Résultat retenu : sommes suffixes et boucles fixes (ADR-163)

Le premier regroupement seul donnait 14,8 % en référence mais 8,2 % en rasante : critère de
coût encore manqué. Les boucles 3×3 ont ensuite été spécialisées à bornes constantes,
sans changement des poids, nœuds ou ordre. Le témoin conserve les boucles dynamiques.

### Qualité et tests

- **19 tests hôte réussis, 1 ignoré, 0 échec** ; test des suffixes contre somme f64 avec
  composante exclue, queue vide et phases sans effet. Cœur inchangé.
- 5 000 sondes par vent à 3/5/8,37 m/s : erreurs pente 2,400e-5 / 5,307e-5 / 9,319e-5,
  covariance 8,538e-8 / 2,154e-7 / 7,427e-7 ; déterminants min 0,513524 / 0,530496 /
  0,325611. Critères S265 tenus ; covariance positive et valeurs finies.
- **Sept témoins finaux identiques au bit à R7 S265**, par comparaison intégrale des PPM.
- Sept images optimisées : **maximum 1 niveau/255, P99 zéro**, MAE au plus 1,049e-5.
  Nombre de canaux changés / 2 764 800 : référence 7, fond seul 8, haute 17, plongeante 29,
  rasante 3, impact proche 2, large horizon 5. Ce sont les arrondis de sommation prévus.
  La spécialisation des boucles ne change aucun pixel par rapport aux suffixes seuls.
- Les sept poses incluent le sillage à 29 s hors de son horizon honnête historique ; cette
  limite est toujours annoncée et n'est pas reçue par cette comparaison.

### Coût reçu

Même machine/profil que le protocole, **aucune capture concurrente**, secteur 99 % début/fin,
ordre 3, 5 m/s, 1280×720, 120 images après chauffe, âge initial 12 s. Grille sillage, filtre
spectral, visibilité et ciel procédural actifs ; CPU un fil, LOD temporel absent.

| pose | témoin médiane ms | optimisé médiane ms | gain | p95 optimisé ms |
|---|---:|---:|---:|---:|
| référence | 2,737344 | 2,258912 | 17,48 % | 2,287136 |
| rasante | 2,493600 | 2,239232 | 10,20 % | 2,260512 |

Cuisson grille incluse : 1,082 / 1,064 ms sur les deux poses optimisées. Maximum eau optimisé
2,301 / 2,278 ms. CPU préparation/transfert/soumission médian 4,162 / 4,212 ms (témoin
4,350 / 4,127), pas de gain CPU revendiqué. `allocations_update_max=0` aux quatre passages.
Surcoût mémoire : buffer GPU +1 Kio, tableau temporaire CPU de pile 1 Kio ; aucune texture de ciel.

**Critère de gain tenu sur ce passage ; budget 2 ms toujours non tenu.** Le gain rasante est
proche de la borne de réception : ce passage n'est pas une garantie universelle ni une
caractérisation statistique inter-machines. Pas de réception temporelle ou multiplateforme.

### Reproduire

Depuis `viewer/`, options communes `--multi --vagues --modulation --ciel-clair --vent=5
--reflets-filtres` :
- `--revue=r8_final` : sept images optimisées ;
- `--reflets-somme-directe --revue=r8_temoinfinal` : témoin R7 ;
- `--reflets-suffixe-bench` : comparaison de coût ;
- `--reflets-verify` : oracle, à rejouer aussi aux vents 3 et 8,37.

Journaux locaux : `specialise-bench.log`, `final.log`, `temoinfinal.log`, `suffixe-verify-*.log`.
Image de référence optimisée : `r8_final_reference_12s.png`.
