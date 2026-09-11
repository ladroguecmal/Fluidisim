# S168 — Moyennes de cellules et flux intégrés

## 1. Dérivation et protocole avant mesure

Suite S167-1/A223. Pour une solution exacte Q de Q_t+∂xF(Q)=0,
Qbar_i(t)=1/dx ∫cellule Q(x,t)dx et I_face=∫t..t+dt F(Q(face,s))ds vérifient
dx(Qbar_i+−Qbar_i0)=I_gauche−I_droite. Recevoir cette identité **avec deux intégrations
indépendantes**, sans reconstruire I depuis la variation de Qbar ni réciproquement.

Le résidu S167 garde son flux numérique différentiel et son RK2. Pour Q exact, Sbar=0.
La variation totale sur la fenêtre est alors l'intégrale RK2 du flux différentiel net
plus I_gauche−I_droite. Pour un Q figé, Sbar_i=[F(Q_gauche)−F(Q_droite)]/dx exactement
en espace : sa somme télescope vers les flux physiques du bord, non vers une somme
de dérivées ponctuelles. Aucune retouche a posteriori de l'état ou du bilan.

Référence onde simple S166, centre **55 m** au lieu de60 pour casser la symétrie,
largeur8 m, direction droite, fenêtre[30,90], durée6 s. Trois cas S167 :
Q=T onde exacte a0,05 ; Q a0,05 et T a0,06 (perturbation non nulle) ; Q figé a0,05,
T onde évolutive a0,05. Fantômes T en moyennes de cellules aux deux étages, fournis
par la référence : le bord autonome reste hors réception de cette session.

N global équivalent120/240/480, pas nominal0,2 dx/sqrt(g) et demi-pas N240.
Comparer trois représentations : centres + flux RK2 (témoin S167), moyennes + flux RK2
(isole le temps), moyennes + flux intégrés (candidat). La deuxième et la troisième ont
le même état évolué : seul le diagnostic d'intégration du flux prescrit change.

Quadrature adaptative de Simpson : intégration du polynôme interpolant les extrémités
et le milieu, poids1/6,4/6,1/6. Comparer un panneau à ses deux moitiés ; le terme dominant
en pas^4 est divisé par16, d'où estimateur différence/15 et extrapolation. Tolérance
absolue1e-12 sur chaque composante, contrôle1e-13, profondeur maximale déclarée ; refuser
la non-convergence. Tolérances d'instrument, pas seuils physiques. La quadrature spatiale
évalue Q, la temporelle évalue son flux : aucun flux n'est inféré de l'état avancé.

## 2. Réceptions attendues

Fond exact d=0 ; bilan local de Qbar et bilan total de la fenêtre <1e-10 relatif au
volume initial (seuil S165 conservé). Flux net non nul constaté, y compris pour Q figé.
Perturbation et correction du Q figé comparées aux moyennes de T exact au raffinement.
Comparer aussi au continu sans confondre conservation et transport. Rejouer S167 si
son support change ; aucune bibliothèque ni dépendance nouvelle.

## 3. Résultats

### 3.1 Deux quadratures nécessaires

Fond exact seul, défaut maximal du bilan de volume relatif au volume initial de la
fenêtre (~60 m² par unité de largeur). Centre55 m : ne pas comparer directement ces
valeurs à la table S167, qui utilisait le centre60 m.

| N | Centres + flux RK2 | Centres + flux intégré | Moyennes + flux RK2 | Moyennes + flux intégré |
|---|---:|---:|---:|---:|
| 120 | 3,136502e-7 | 2,902729e-7 | 2,337733e-8 | 6,809141e-16 |
| 240 | 7,868959e-8 | 7,284474e-8 | 5,844851e-9 | 1,177952e-15 |
| 480 | 1,968973e-8 | 1,822848e-8 | 1,461245e-9 | 2,136605e-15 |

À N240 au demi-pas, le défaut « moyennes + RK2 » tombe à1,461245e-9, facteur quatre.
Celui « centres + intégré » reste7,284474e-8 : les deux contributions sont distinguées.
Le candidat complet reste à1,179995e-15 ; il ne présente plus le défaut de quadrature
mesuré S167 au seuil annoncé. **Q exact reste intact et d reste exactement nul**.

Abaisser la tolérance des deux intégrations de1e-12 à1e-13 donne9,410317e-16 à N240,
sans modification visible des erreurs de transport aux chiffres publiés. La proximité
de l'arrondi est donc observée, pas obtenue en compensant artificiellement le bilan.
Le code ne corrige ni Qbar ni le flux pour les faire coïncider.

### 3.2 Perturbation et fond figé asymétrique

E=max |h−h_exact|/0,05 contre les moyennes de la référence, sur l'espace et les6 s.

| N | E perturbation | Bilan total perturbation | E fond figé | Bilan total fond figé |
|---|---:|---:|---:|---:|
| 120 | 0,050438 | 7,413738e-16 | 0,226119 | 5,405954e-16 |
| 240 | 0,031238 | 1,031909e-15 | 0,135612 | 9,234267e-16 |
| 480 | 0,017796 | 1,245305e-15 | 0,075423 | 1,881575e-15 |

Le résidu non nul évolue et converge ; l'exactitude du volume ne supprime pas l'erreur
de transport. À N240, l'erreur rapportée à la perturbation initiale de1 cm est encore
15,6 %. Aucun seuil de qualité physique n'est reçu.

Pour Q figé, maximum absolu du cumul de flux physique net =5,392160e-5 m² par unité
de largeur, **non nul**. Le bilan avec source ponctuelle reste en défaut5,339892e-9 à
N240, tandis que la source moyenne télescopique ferme à l'arrondi. Le témoin symétrique
S167 ne pouvait pas vérifier cela. Pour Q évolutif, le même maximum vaut1,483994e-3 m².

### 3.3 Réception reproductible

Depuis `code/` :

```
cargo test -p water-core --example volume_moyen
cargo run -p water-core --release --example volume_moyen
```

Cinq nouveaux tests reçus : quadrature polynomiale/trigonométrique, identité locale par
intégrations indépendantes (hauteur et débit), Q exact et volume, Q figé asymétrique,
perturbation convergente. La campagne compte15 montages et deux évolutions par montage,
soit30 évolutions ; les différents diagnostics de flux sur un même état ne sont pas
comptés comme des évolutions indépendantes. Trois grilles, demi-pas N240 et tolérance
resserrée N240, trois cas. Courant<=0,217650 ; volume moyen/intégré<=2,14e-15.

Le nouveau fichier d'exemple est le seul code ajouté. Les supports S165–S167 et les
bibliothèques sont inchangés ; leurs tests ne sont pas présentés comme rejoués.
Workspace299/cinq ignorés reçu S163 ; tests S165/S166/S167 reçus S167. Aucun ajout de
dépendance. La quadrature adaptative appartient au banc : son coût n'est pas reçu pour
le runtime et ses allocations de tableaux précèdent l'intégration temporelle.

## 4. Verdict et suite

**S167-1 réalisée, A223 traitée sur véhicule à fond analytique connu.** Volume total
fermé au seuil annoncé, sans recalage d'état ni flux déduit du volume. Préservation,
bilan et erreur de transport restent trois réceptions distinctes. Aucun ADR nouveau
ni schéma runtime adopté ; A50 partielle, A216/A217 inchangées. I-01/04/12/14/15 inchangés.

**A224 / suite S169 : S168-1**, assembler le résidu équilibré en moyennes et la frontière
autonome S166. Le présent banc fournit encore les fantômes T exacts ; les deux composants
n'ont pas été reçus ensemble. Dériver une fermeture qui préserve Q au bord quand d=0,
puis exercer une perturbation sortante et une entrée connue, avec flux réellement utilisé
au bord dans le bilan total. Ne pas supposer que copier un invariant total de la cellule
intérieure vers le fantôme préserve un Q spatialement variable. Aucun résultat sur cet
assemblage n'est encore revendiqué ; ni 3D, eau sèche ou choc reçu.
