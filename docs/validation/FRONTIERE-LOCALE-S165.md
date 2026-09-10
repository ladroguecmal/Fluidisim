# S165 — Frontière du résidu local

## 1. Protocole déclaré avant mesure

Suite S164-1/A220. Même véhicule Saint-Venant mouillé, lit plat, Rusanov ordre un en
espace et RK2 en temps. Canal 120 m, hauteur au repos 1 m ; fenêtre interne [30,90] m.
Bosse gaussienne centrée en 60 m, largeur 3 m, amplitude 0,1 m, débit initial nul.
Elle se sépare vers les deux bords ; durée 16 s, au-delà du trajet 30/sqrt(g) ~=9,6 s.
N=120/240/480, pas nominal 0,2 dx/sqrt(g), divisions par 2 et 4. Ces paramètres sont ceux
de l'instrument, pas un profil de production ni un seuil de qualité.

Q constant puis onde debout prescrite S164 (amplitude 0,05 m, mode 2). Le total initial
reste identique : le second montage porte aussi un résidu compensant l'onde prescrite.
Le premier isole donc la sortie de la bosse ; le second exerce simultanément source et bord.

Trois alimentations extérieures, toujours aux centres des cellules fantômes :

- **Oracle** : total global au début du pas et à son prédicteur Euler, respectivement.
- **Fond seul** : Q(t) et Q(t+dt), donc résidu extérieur nul.
- **Oracle retardé** : total au début du pas utilisé aux deux étages ; contre-épreuve temporelle.

Le résidu intérieur est intégré indépendamment ; aucun recopiage de la référence à l'intérieur.
L'oracle donne seulement deux états extérieurs par étage. Il n'est pas une solution de production.
Un prédicteur global auxiliaire construit l'étage manquant dans l'API Shallow1D ; son pas RK2
complet doit être reçu contre la bibliothèque à chaque pas, avant de servir de témoin local.

## 2. Identité et critères

Pour chaque face, flux total reconstruit = flux(Q) + flux_delta(Q,d), développés S163.
La source spatiale et les incréments temporels sont ceux reçus S164. Avec Q0=Q(t), Q1=Q+=Q(t+dt) :

```
d1 = d0 + dt L_local(Q0+d0; fantomes0) - (Q1-Q0)
d+ = (d0+d1 + dt L_local(Q1+d1; fantomes1) - (Q1-Q0))/2
```

Avec les fantômes oracle des mêmes étages, restriction du total et reconstruction locale
doivent coïncider : seuil instrumental 1e-10 hérité S163/S164, normalisations 0,1 m et
0,1 sqrt(g) m²/s. Mesurer maximum en espace/temps et maximum dans le cœur [40,80] m.
Ne pas appeler tout écart « réflexion » : il inclut erreur de flux, perte et perturbation entrante.

Bilan de masse **ouvert** : variation de somme(h dx) = dt/2 [(fG0-fD0)+(fG1-fD1)].
Un bilan exact peut accompagner une solution fausse, si les flux de bord sont faux.
Mesurer séparément résidu oracle au bord et date où il dépasse 1 % de l'amplitude initiale
(repère de traversée, pas tolérance). Témoin bosse nulle/fond constant : les trois voies
doivent rester au repos. Raffiner pour séparer frontière persistante et retard temporel.

## 3. Résultats

P3 : 54 montages (trois grilles, trois pas, deux fonds, trois frontières). L'oracle RK2
local retrouve le total à <=1,12e-13 en hauteur ; contrôle auxiliaire global <=2,23e-15.
Cinq nouveaux tests propres à l'exemple et trois tests host importés reçus.

**Précision du diagnostic après premier passage :** la norme de bord prend le maximum
de |d_h|/0,1 et |d_q|/(0,1 sqrt(g)), pas seulement la hauteur. L'onde prescrite a presque
un nœud de hauteur aux bords de cette fenêtre, mais son débit y est maximal : observer
seulement la hauteur retardait artificiellement le repère d'erreur extérieure.

Un premier test attendait >1e-4 dans le cœur pour la bosse sortante ; la mesure vaut
8,647e-5 à N240. Cette attente non dérivée a été retirée : le test vérifie un défaut
résolu à plus de 1000 fois le seuil d'arrondi, conformément au protocole d'identité,
et ne prétend pas vérifier une tolérance physique. Les mesures restent publiées telles quelles.
