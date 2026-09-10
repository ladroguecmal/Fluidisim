# S163 — Intégrer le résidu couplé en Saint-Venant

2026-09-10. Action S162-1, ADR-112. Instrument 1D, lit plat, eau strictement mouillée,
hors runtime. Aucun choix de solveur δ 3D ni seuil de bascule.

## 1. Équation et schéma déclarés avant construction

Référence : `Shallow1D`, Rusanov d'ordre un en espace, SSP-RK2 en temps, murs réfléchissants.
Ordre un retenu pour rendre le flux résiduel explicitement vérifiable, sans reconstruction ni
limiteur non linéaire supplémentaire. Le modèle est celui documenté en tête de `shallow.rs`.

État conservatif total `T=(h,q)` avec `q=hu` ; fond choisi `Q=(H,M)` ; résidu stocké
`d=(z,r)`, avec `h=H+z`, `q=M+r`. **r n'est pas une vitesse perturbative.**
Le flux physique est `F(h,q)=(q,q²/h+gh²/2)`. Pour `U=M/H`, notre dérivation donne :

```
ΔF(Q,d) = F(Q+d)-F(Q)
ΔF_h = r
ΔF_q = 2Ur - U²z + (r-Uz)²/(H+z) + gHz + gz²/2
```

À une interface de Rusanov, `a_T` et `a_Q` sont les maximums de `|q/h|+sqrt(gh)` des deux
états adjacents, évalués respectivement sur T et Q. La différence des flux **numériques** est :

```
ΔF_num = (ΔF_gauche+ΔF_droite)/2
          - [a_T saut(d) + (a_T-a_Q) saut(Q)]/2
```

Le dernier terme est un couplage **numérique**, absent de l'équation continue. L'omettre
ne reçoit plus le même schéma discret. Aucun calcul d'un total avancé puis soustrait : le
résidu possède ses tableaux, ses dérivées et ses deux étages RK2.

Deux fonds testés avec la même référence totale :

- **Fond évolué**, `Q_t=L(Q)` : `d_t=L(Q+d)-L(Q)`, calculé par divergence de ΔF_num.
- **Fond figé non uniforme**, `Q_t=0` : ajouter `L(Q)` à cette divergence. Cette source
  est l'analogue conservatif de `−S` de SPEC-004 §6.1. Même à résidu initial nul, elle doit
  faire évoluer le total : la bosse figée n'est pas une solution de Saint-Venant.

Murs : hauteur copiée, débit inversé, séparément pour Q et d ; pas imposé identique pour
les deux intégrations. Vérifier le Courant à chaque étage, la finitude et la profondeur positive,
sans saturation silencieuse. Le fond évolué est aussi reçu seul contre `Shallow1D`.

## 2. Réception annoncée

1. Vérifier ΔF_num contre une différence directe de flux sur des états mouillés variés,
   avec débit du fond et résidu positifs/négatifs ; vérifier d=0.
2. Réception temporelle : deux bosses régulières, fond et perturbation ; comparer hauteur
   **et débit** reconstruits à la référence totale à chaque pas, pas seulement au dernier.
   Seuil d'instrument **1e-10**, normalisé par l'amplitude initiale et par celle-ci fois
   `sqrt(gh0)` pour le débit. C'est une tolérance d'arrondi f64, pas un seuil physique.
3. Contre-épreuves distinctes : retirer `g(H-h0)z` en conservant le transport sur profondeur
   de repos ; retirer `(a_T-a_Q)saut(Q)` ; retirer la source d'un fond figé non uniforme.
   Chaque défaut doit faire échouer le même critère, avec un témoin complet qui passe.
4. Raffiner séparément pas et mailles ; publier l'écart de reconstruction et l'écart de
   discrétisation de la référence comme deux grandeurs différentes. Ne pas appeler un accord
   entre deux écritures du même schéma une validation physique.
5. Cas limites : résidu nul, fond uniforme, perturbation négative sans cellule sèche,
   réflexion aux murs. Conservation de masse sur domaine fermé ; momentum global non conservé
   attendu à cause des forces des murs.

Les amplitudes, dimensions et durées de la sonde sont des fixtures de réception déclarées dans
le code ; aucune valeur n'est un paramètre du système de production. S161 reste un diagnostic
distinct (ordre deux/HLL), ses coefficients ne sont pas comparables directement à cette campagne.

## 3. Construction et résultats

Le véhicule est dans `code/water-core/examples/support/residu_shallow.rs` ; la campagne et
la référence indépendante dans `examples/residu_couple.rs`. Le véhicule ne connaît pas
`Shallow1D` : ses flux résiduels sont développés algébriquement, son fond et son résidu ont
leurs propres tableaux et étages. Seule la campagne interroge la référence pour comparer.
Les allocations du véhicule se font à la construction, pas dans `step` ; aucune intégration
au runtime ni réception globale des pools hôte n'est revendiquée.

Commande depuis `code/` : `cargo run -p water-core --release --example residu_couple`.
Campagne du 2026-09-10 : domaine 120 m, h0=1 m, g repris de `shallow::G` (9,81),
bosse du fond 0,2 m en x=42 m de largeur 6 m ; perturbation 0,1 m en x=62 m de largeur 3 m.
Débits initiaux nuls. Durée 8 s. Nombre entier de pas choisi depuis `dt≈0,2 dx/sqrt(gh0)`
puis ajusté pour finir exactement à 8 s ; mêmes pas pour fond, résidu et référence totale.

Les erreurs sont les maxima **sur toutes les cellules et tous les pas**, normalisés par
`|a|+|b|` en hauteur et `sqrt(gh0)(|a|+|b|)` en débit. La masse est rapportée à la masse initiale.
N=240, dx=0,5 m :

| fond choisi | termes du résidu | erreur hauteur | erreur débit | dérive masse |
|---|---|---:|---:|---:|
| évolué | complets | 5,92119e-15 | 4,14606e-15 | 1,854e-15 |
| évolué | sans pression croisée `g(H-h0)z` | 7,26910e-3 | 5,24089e-3 | 1,390e-15 |
| évolué | sans couplage numérique | 2,73063e-4 | 2,79029e-4 | 1,506e-15 |
| figé | complets | 5,92119e-15 | 2,95339e-15 | 1,506e-15 |
| figé | sans pression croisée | 1,43907e-1 | 4,60363e-2 | 1,043e-15 |
| figé | sans couplage numérique | 1,10826e-3 | 2,07872e-3 | 1,043e-15 |
| figé | sans source `L(Q)` | 6,66505e-1 | 3,16244e-1 | 1,274e-15 |

Le maximum de Courant des étages reste entre 0,2255 et 0,2270 sur ces sept lignes ; borne
de contrôle 0,45. Aucun état sec ni saturation des références. Les cinq omissions échouent
au critère annoncé avec plus de cent fois sa tolérance ; les deux témoins complets passent.
**La conservation de masse seule laisse passer les cinq défauts.** Tous les flux restent
conservatifs même lorsqu'ils représentent une mauvaise équation : c'est attendu, pas rassurant.

### Cas limites et refus

- Résidu initial nul, fond évolué : le résidu reste **exactement nul** ; hauteur reconstruite
  identique à la référence. Le minuscule écart de débit (8,86e-17 normalisé) inclut la lecture
  publique `hauteur*vitesse`, le débit conservatif interne n'étant pas exposé.
- Même état initial, fond figé : résidu de hauteur jusqu'à **0,199881 m**, sans source locale.
  Il compense le fond qui n'est pas une solution. En retirant la source, il reste nul et le
  test de comparaison échoue. Un test qui imposerait toujours δ=0 sans événement serait faux.
- Fond uniforme : les trois omissions deviennent sans effet et **passent** ; ce sont leurs
  témoins nuls, qui empêchent un test rejetant simplement tous les modes dégradés de réussir.
- Creux initial de −0,1 m, eau mouillée : les deux fonds passent.
- Bosses rapprochées du mur (x=10 et 15 m), durée 12 s : réflexions reçues avec les deux fonds.
- Cellule sèche et débit NaN : deux tests exigent le refus explicite. Aucun écrêtage ne transforme
  ces états en résultat. Le domaine mouillé de cet instrument est volontairement restreint.

Sur ces huit cas limites à N240, erreur hauteur maximale 1,77636e-14 et débit 8,95703e-15.
Les tests utilisent N120 pour réduire leur coût ; la campagne release utilise N240.

## 4. Deux écarts à ne pas confondre

Raffinement spatial, fond évolué et même fixture, dt proportionnel à dx :

| N | erreur de reconstruction, max hauteur | sans couplage numérique | référence, écart L2 à la grille précédente |
|---:|---:|---:|---:|
| 120 | 5,18104e-15 | 3,21178e-4 | — |
| 240 | 5,92119e-15 | 2,73063e-4 | 1,67222e-2 |
| 480 | 2,51651e-14 | 1,91246e-4 | 1,30197e-2 |
| 960 | 1,61352e-13 | 1,19368e-4 | 9,07266e-3 |

La référence fine est moyennée deux cellules par cellule grossière, à l'instant final,
et la L2 est normalisée par 0,3 m. Cette différence inclut les discrétisations spatiale et
temporelle ainsi que les initialisations ponctuelles ; ce n'est pas une erreur contre solution
exacte. Les ratios ne reçoivent pas un ordre asymptotique. Le fond seul est lui aussi reçu
contre sa propre référence à chaque pas, avec la même tolérance 1e-10.

**L'accord du couplage reste à l'arrondi quand le champ de référence change au pourcent.**
Le premier ne certifie donc pas la précision physique du second. Le défaut de couplage
numérique décroît au raffinement mais demeure détectable : sa disparition dans la limite
continue ne justifie pas son omission à la résolution effectivement utilisée.

Raffinement temporel seul, N240 : erreur de reconstruction hauteur 5,92119e-15,
1,48030e-14 et 3,25665e-14 pour facteurs 1/2/4 de division du pas nominal. Les écarts L2
successifs de référence sont **3,82064e-5** et **9,53036e-6**, rapport proche de quatre.
Les arrondis d'un nombre entier de pas sont conservés ; aucune étape d'observation ne
recoupe l'intégration.

## 5. Verdict

**S162-1 réalisée sur ce véhicule** : le résidu est intégré, les termes sont explicites,
la référence est indépendante de son intégration et les contre-épreuves sont reçues.
**A218 traitée dans ce périmètre** ; ADR-112 reste applicable. Aucun ADR nouveau nécessaire.

**A50 partiellement exercée** : le fond figé non uniforme donne une source non nulle que le
test sait retirer. Mais le fond analytique instationnaire du projet, les dérivées interpolées,
la bathymétrie, la dispersion et les frontières de domaines locaux ne sont pas ici.
La comparaison est celle de deux écritures du **même schéma 1D**, sans réception de B4 complet,
de δ 3D, des forces ni de la perception. Aucun seuil de bascule n'est proposé.

**Suite S164, S163-1 / A219 :** remplacer le fond évolué ou figé par une onde analytique
instationnaire prescrite. Recevoir la source spatiale **et temporelle** à chaque étage RK,
à même état total initial ; comparer l'emploi de la dérivée continue du fond et de ses
incréments discrets. Le fond réévalué n'est pas en général celui qu'un RK2 avancerait :
la clôture temporelle n'est pas reçue par ce lot.


Réception finale : 299 tests workspace réussis, cinq ignorés ; cinq tests nouveaux de l'exemple
et trois tests host importés réussis en debug, campagne complète avec assertions reçue en release.
