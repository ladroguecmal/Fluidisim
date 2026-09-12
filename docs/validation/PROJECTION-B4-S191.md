# S191 — Projection discrète et réception B4 à 2 %

2026-09-12. S190-1 reprend S189-1. **Protocole avant simulation**.
Seuil inchangé : [ADR-120](../adr/ADR-120-b4-tolerance-de-deux-pour-cent.md).

## 1. Dérivation et correction de l'hypothèse de départ

Sur une grille et avec des conditions de bord fixées, le projecteur discret est
**linéaire** : `P(a+b)=Pa+Pb`. Coupler toutes les mailles ne détruit pas cette propriété.
La crainte de S189 (« la projection menace l'additivité parce qu'elle couple les
mailles ») ne découle donc pas du mécanisme annoncé. Ce qui peut changer : les pics,
le maximum du signal de référence, les interactions de l'advection et l'arrêt du
solveur de pression. L'additivité approchée de **plusieurs pas non linéaires** n'est
pas acquise par la linéarité de P ; elle reste à mesurer.

Autre distinction : `||a+b|| ≤ ||a||+||b||` est une identité de norme. La borne
`e(r,c) ≤ e(r,1)+e(1,c)` ne s'en déduit que si les **champs** se décomposent ainsi.
Avec un résidu R d'additivité, la borne garantie est `e ≤ s+t+||R||/M`.
Les phrases historiques « la somme ne suppose rien » confondent ces deux énoncés.
Le budget ADR-119 reste le critère conservateur essayé ; son respect se vérifie
sur le couple, il n'est pas supposé par le programme.

## 2. Projecteur compatible avec le véhicule existant

On conserve les vitesses aux centres, dx=0,25 m et les fantômes nuls du bloc S185.
Sur les cellules intérieures, `C_x p = (p[i+1]−p[i−1])/(2dx)` avec extension nulle,
idem y/z. `D u = C_x u_x + C_y u_y + C_z u_z` et
`G p = (C_x p,C_y p,C_z p)`. En somme discrète intérieure, `G = −Dᵀ`.

```
A = −DG = DDᵀ
A phi = −D u*
u = u* − G phi ; phi = dt · p_correction/rho, en m²/s
```

Le système résout la correction de pression, pas la pression dynamique déjà dans S.
L'opérateur **A doit être le produit réel D/G**, jamais le Laplacien sept points
indépendant de l'advection/viscosité : les différences centrées composées portent
sur deux mailles et ont des lignes de bord particulières. Le programme applique
G puis −D pour ne pas substituer un stencil ressemblant.

Le projecteur est construit dans un support de banc séparé. Sur un nombre **pair**
de cellules intérieures, la matrice centrale 1D à extension nulle est inversible ;
DDᵀ est définie positive. Le banc refuse les dimensions impaires pour éviter de
masquer des noyaux en damier. Ce choix permet de garder le pas de S185 intact ;
**ce n'est pas un choix de discrétisation pour le solveur δ**.

Gradients/divergence et résolution de pression en f64 de banc ; état avant/après en
f32 comme S185. CG à résidu relatif 1e-9, plafond 256 itérations et refus si non
convergence ; comparaison plus serrée à 1e-12. Ces seuils d'instrument doivent rendre
leur effet négligeable face aux 2 %, condition vérifiée ci-dessous, pas présumée.
Publication du champ seulement après résolution et contrôles, sortie intacte au refus.

**Portée des bords** : extension nulle des opérateurs, pas paroi MAC, pas condition
cinématique de surface libre, pas entrée/sortie ouverte de l'océan. Une projection
reçue ici ne vaut pas réception physique de ces frontières.

## 3. Réception propre du projecteur

Avant la campagne :

1. Champ manufacturé gradient + rotationnel discret, non nuls : éliminer le gradient,
   conserver le rotationnel ; contrôler l'adjoint, la linéarité, l'idempotence et la
   non-augmentation de la norme L2. Seuil relatif 2e-6 après publication f32.
2. Petit domaine : comparaison avec une matrice D dense assemblée indépendamment et
   élimination de Gauss de DDᵀ. Inclure les cellules de bord dans la comparaison.
3. Zéro reçu exactement ; non-finis, tailles invalides, fantômes non nuls et plafond
   nul refusés, sans modification de l'entrée.
4. Après **chaque** projection de campagne,
   `dx·max|Du|/max|u*| ≤ 2e-6` (zéro traité à part). Publier le pire résidu et les
   itérations maximales. Témoins sans projection et à signe inversé doivent échouer
   sur le gradient manufacturé. Le coût CPU n'est pas un critère de cette campagne.

## 4. Campagne B4 et critères

Même source et même bloc/fenêtre que S190, repos initial ; on ajoute P après chaque
`Block::step`. Référence R pleine/cadence 1 ; R2 à dt/2 avec source chaque demi-pas.
Réserve q = max|R−R2|/M, M=max|R| **projeté**. Une troisième évolution pleine avec CG
resserré mesure qP=max|R−R_serré|/M ; exiger qP ≤ 1e-5 (0,001 %). Réserve totale q+qP.
La réserve temporelle reste une mesure entre schémas, pas une borne au continu.

Extrapolation causale, cadences 1/2/4/8/16 ; horizontal plein, graduation verticale
Nz=8 (profil S190), 10, 12, 14. Le plein est chargé directement. Profil et axes seuls
rejoués **sans projection** à c=8 pour retrouver les chiffres S190 avant comparaison.

Pour chaque couple : s/t/e/e2 comme S190, avec M projeté commun. Reçu si
`s+t+q+qP ≤ 0,02`, `e+q+qP ≤ 0,02`, `e2+qP ≤ 0,02`, et pression qualifiée.
Mesurer aussi max|Δu_composé−Δu_spatial−Δu_temporel|/M et vérifier
`e ≤ s+t+résidu`. Publier e/(s+t) seulement si s+t est non nul ; aucun verdict de
« loi » forcé sur des différences d'arrondi. Le budget de 2 % reste inchangé si tous
les profils testés échouent. Choisir parmi les reçus le moins d'évaluations de source
sur 100 pas, `nœuds·ceil(100/c)`, puis le plus petit e+réserve ; pas un optimum CPU.

Deux exécutions release et contrôles du projecteur en debug/release. Une sortie
intégrale debug peut compléter le rejeu si son coût reste raisonnable ; aucun transfert
de résultat entre cibles n'est certifié. Pas de nouveau solveur de production.

## 5. Ce que ce lot doit fermer

Le profil S190 passe-t-il sous projection ? La projection elle-même est-elle reçue
contre une référence algébrique indépendante ? L'additivité et sa borne survivent-elles
sur ce montage ? Les résultats négatifs se publient avec leur cause mesurée.
Surface libre, bords physiques, comparaison substitutive, forces et perception restent
hors de ce lot ; la file plurielle S190 conserve les autres chantiers.

### Correction de montage pendant P3b — 2026-09-12

Le premier passage est **interrompu par son témoin plein** : demander 14 nœuds au
générateur gradué en rend 12 après dédoublonnage ; le programme l'avait nommé plein
à tort, et son contrôle d'additivité dégénérée échoue. Le témoin 14 est corrigé en
indices pleins explicites `[1..14]`, comme le protocole l'exigeait. Aucune assertion
assouplie, aucun changement du générateur historique. Les demandes 10 et 12 rendent
respectivement **9 et 11 nœuds réels**, qui sont ceux comptés dans le coût. Les
étiquettes Nz désignent la demande ; les indices publiés désignent le réseau réel.
La première sortie incomplète n'est pas une réception. Rejeu intégral requis.

## 6. Résultats et réception

Commande : `cargo run --release --manifest-path code/Cargo.toml -p water-core --example projected_b4`.
Contrôles : `cargo test [--release] --manifest-path code/Cargo.toml -p water-core --example projected_b4`.
Les trois tests passent en debug et release. La campagne corrigée s'achève sans
refus du projecteur ; maximum **44 itérations**, divergence normalisée maximale
**4,921623567e-8**, sous le seuil d'instrument 2e-6. Le resserrement de pression
donne **qP=0,000011025 %**, sous 0,001 %. Aucune durée CPU n'est revendiquée.

Le témoin sans projection retrouve S190 : spatial **0,639282 %**, temporel et composé
**0,775379 %**. La référence projetée est non nulle :
`M=1,319937583e-4 m/s`, contre `7,993167674e-5` sans projection, rapport **1,651332283**.
Ce sont deux évolutions distinctes ; le projecteur est contractant en L2 à chaque
application, pas dans la norme maximum du champ final. Un dénominateur non projeté
aurait changé silencieusement le sens de « 2 % ». La réserve temporelle projetée
vaut **q=0,407704413 %**, elle ne se recopie pas de S190.

| réseau réel / cadence | s % | t % | e % | s+t+q+qP % | e+q+qP % | contre R2 % | verdict |
|---|---:|---:|---:|---:|---:|---:|---|
| **14×14×8 / 80 ms** | **0,368754** | **0,595477** | **0,540332** | **1,371947** | **0,948047** | **0,948036** | **reçu, retenu** |
| 14×14×8 / 160 ms | 0,368754 | 2,445069 | 2,391037 | 3,221538 | 2,798752 | 2,798741 | refusé |
| 14×14×9 (demande 10) / 80 ms | 0,240134 | 0,595477 | 0,484458 | 1,243327 | 0,892174 | 0,868647 | reçu |
| 14×14×11 (demande 12) / 80 ms | 0,122280 | 0,595477 | 0,595179 | 1,125473 | 1,002895 | 1,002884 | reçu |
| plein / 80 ms | 0 | 0,595477 | 0,595477 | 1,003193 | 1,003193 | 1,003182 | reçu |

**16 couples reçus / 4 refusés**. Le profil S190 reste le moins coûteux en évaluations
de source parmi les réseaux déclarés : 20384 contre 274400, réduction **13,461538**.
La projection ne justifie aucune économie supplémentaire de cadence : **160 ms est
refusé sur les quatre réseaux**. Les champs reçus n'impliquent pas une réception
temps réel, et les deux références partagent encore leur modèle spatial.

### Additivité et portée d'ADR-119

Le résidu d'additivité locale est **exactement nul** pour c=1 et le réseau plein.
Sur le profil retenu il vaut **0,002593561 % de M** ; maximum de campagne
**0,012774871 %**, sur demande Nz10/c16. L'enveloppe sans résidu est respectée
sur les vingt cases ; **ce constat ne la transforme pas en théorème**.
Avec le résidu mesuré du profil, la borne algébrique composée et les réserves valent
**1,374540 %**, toujours sous 2 % (marge **0,625460 point**).

[ADR-121](../adr/ADR-121-la-projection-lineaire-et-la-borne-de-composition.md) corrige
la portée d'ADR-119 : globalité et non-linéarité ne se confondent pas ; la triangulaire
entre trois évolutions requiert le résidu de leur décomposition. La projection fixe
préserve l'addition, l'évolution complète seulement approximativement sur ce montage.

## 7. Verdict et suite système

**S190-1/S189-1 réalisées dans le périmètre déclaré.** Projecteur reçu indépendamment,
profil S190 reçu avec projection sous les 2 % inchangés. A50 reste partielle ; ni
surface libre, ni bords physiques, ni comparaison substitutive complète reçus.

**S191-1 : construire une tranche 2D à surface libre du véhicule**, avec pression
atmosphérique et condition cinématique explicites, puis recevoir une onde de gravité
contre la relation analytique et son raffinement avant comparaison perturbatif/total.
Il s'agit d'un candidat de construction pour B3/B4, pas d'une sélection technologique
par cet essai. Le projecteur collocatif S191 ne se transfère pas implicitement : ses
bords n'étaient qu'un montage algébrique et le choix D/G doit être redérivé pour la
surface. Ne pas prolonger la seule campagne de réseaux une fois ce lot reçu.

Autres lignes de la file S190 relues et conservées : A213, B2/λ_cut/coupure,
bathymétrie, multiplateforme, V, forces/perception et dossier de réunions. Aucun
nouveau blocage humain ; aucun de ces points n'est déclaré clos par la projection.

Reproductibilité : **deux sorties release intégralement identiques**, empreinte
**0x52d7645d4548ebb2**. [Mesures intégrales](PROJECTION-B4-S191-MESURES.md).
Tests propres debug/release reçus (trois) ; campagne complète debug non relancée.
Les 331 tests workspace/cinq ignorés restent la réception S190, non rejouée :
bibliothèques et supports historiques inchangés. Aucun reçu interplateforme.
