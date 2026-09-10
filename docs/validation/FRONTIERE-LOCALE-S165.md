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

### 3.1 Sortie sur fond constant

Erreurs maximales normalisées sur les 16 s ; pas nominal.

| N global | Bord fond seul, hauteur | Débit | Hauteur dans le cœur | Résidu au bord | Premier dépassement 1 % |
|---|---:|---:|---:|---:|---:|
| 120 | 1,103356e-3 | 1,213698e-3 | 6,483041e-5 | 0,181778 | 5,800797 s |
| 240 | 1,967080e-3 | 2,110233e-3 | 8,647025e-5 | 0,243805 | 6,406375 s |
| 480 | 3,173814e-3 | 3,348846e-3 | 9,596060e-5 | 0,312023 | 6,875374 s |

À N240, l'erreur avant 4 s est 1,11e-14. Le passage est donc exercé, pas seulement
supposé. Le premier dépassement précède l'arrivée du sommet : gaussienne étendue et diffusion
numérique ; ce n'est pas une mesure de célérité. Raffiner l'espace **augmente** l'écart au bord
dans ces essais et augmente aussi la perturbation qui l'atteint. Ne pas extrapoler une loi
de réflexion à partir de cette table ; les références de grilles sont différentes.

| N240, diviseur du pas | Fond seul, hauteur | Oracle retardé, hauteur | Oracle retardé, débit |
|---|---:|---:|---:|
| 1 | 1,967080e-3 | 2,796993e-6 | 3,743136e-6 |
| 2 | 1,967726e-3 | 1,180552e-6 | 1,770564e-6 |
| 4 | 1,967909e-3 | 5,402117e-7 | 8,621857e-7 |

Le fond seul ne converge pas vers l'oracle en réduisant dt à dx fixé. Le retard d'étage
converge, avec rapports 2,37 puis 2,19 en hauteur, proches de l'ordre un ; il demeure
distinct de l'oracle cohérent aux étages. Ce constat prolonge L246.

### 3.2 Fond variable et information extérieure

Au pas nominal, fond prescrit d'amplitude 0,05 m, mêmes états totaux initiaux :

| N | Fond seul, hauteur | Fond seul, débit | Hauteur dans le cœur |
|---|---:|---:|---:|
| 120 | 0,431314 | 0,269581 | 0,431314 |
| 240 | 0,440460 | 0,270769 | 0,440460 |
| 480 | 0,445125 | 0,272929 | 0,445125 |

Ce sont environ 4,4 cm d'écart de hauteur pour une normalisation de 10 cm. À N240,
le défaut dans le cœur est plus de 5000 fois celui de la bosse sur fond constant.
La division du pas par quatre laisse 0,440462 : ce n'est pas une erreur de source temporelle.
L'oracle et l'oracle retardé sont insensibles au choix de Q à l'arrondi près ; le fond seul
change la condition totale au bord quand on change Q.

**Ce montage n'affirme pas qu'un δ local devrait porter une compensation globale.**
Il maintient volontairement le même total pour isoler la frontière : d initial compense Q
partout, et le résidu extérieur n'est donc pas nul. Imposer d=0 hors de la fenêtre change
le problème. Même avec une bosse d'amplitude nulle (total global au repos), le test observe
un écart >0,1 normalisé dans le cœur, tandis que l'oracle reste au seuil d'arrondi.
Une réception limitée à une onde sortante sur fond uniforme n'aurait pas détecté cela.

### 3.3 Conservation et réception

Sur les 54 montages : défaut relatif du bilan ouvert <=6,97e-15, Courant <=0,213264,
contrôle du pas global auxiliaire <=2,23e-15, reconstruction locale oracle <=1,12e-13.
Le flux utilisé est conservé ; son adéquation à l'extérieur est une propriété séparée.
Ce bilan ne signifie pas masse constante dans la fenêtre.

Commandes depuis `code/` :

```
cargo test -p water-core --example frontiere_locale
cargo run -p water-core --release --example frontiere_locale
```

Cinq nouveaux tests propres passent (oracle/deux fonds, repos, défaut de bord malgré bilan,
raffinement du retard, compensation extérieure), avec trois tests host importés. Campagne
release finale reçue après inclusion des fantômes dans le contrôle du Courant.
Seuls deux fichiers d'exemple nouveaux sont ajoutés ; support S163/S164 et bibliothèques
inchangés. Leur réception précédente n'est pas présentée comme rejouée : exemples S164,
workspace 299/cinq ignorés S163. Pas de dépendance, allocation dans `Local::step` ni saturation.

## 4. Verdict et suite

**S164-1 réalisée, A220 traitée sur ce véhicule.** L'intérieur local et les données de bord
sont séparés, la traversée reçue, les erreurs de frontière distinguées des sources.
L'oracle est un instrument, pas une frontière exploitable dans le jeu.
**A221 / S165-1** : construire et mesurer une fermeture sans oracle, séparant information
entrante prescrite et information sortante issue de l'intérieur. Comparer extrapolation et
fermeture caractéristique sur ce véhicule, avec cas sortant puis onde de fond entrante ;
garder la compensation extérieure inconnue comme limite explicite, pas comme objectif
qu'une frontière locale pourrait satisfaire sans information.

A50 reste partielle ; A216/A217 inchangées. Aucun absorbeur choisi, aucun seuil de bascule,
aucun ADR nouveau. Ni δ 3D, ni interface B du runtime, ni interpolation grossière, ni forces
ou perception ; B4 non reçu intégralement. I-01/04/12/14/15 inchangés : aucune conséquence
sur la gratuité visuelle de création/destruction d'un domaine n'est mesurée ici.
