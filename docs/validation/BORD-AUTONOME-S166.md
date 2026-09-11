# S166 — Entrée prescrite et sortie autonome

## 1. Protocole avant mesure

Suite S165-1/A221. Véhicule mouillé Saint-Venant 1D, lit plat, Rusanov/RK2 S165,
fenêtre [30,90] m. Aucun oracle global dans la fermeture autonome.
À partir de h_t+u h_x+h u_x=0 et u_t+u u_x+g h_x=0, poser c=sqrt(gh) :
R±=u±2c vérifient (∂t+(u±c)∂x)R±=0 par substitution.
En régime subcritique |u|<c, un invariant entre à chaque bord, l'autre sort.

- Bord gauche : R+ prescrit par Q extérieur ; R- lu dans la cellule intérieure.
- Bord droit : R- prescrit par Q extérieur ; R+ lu dans la cellule intérieure.
- Reconstruction : u=(R++R-)/2, c=(R+-R-)/4, h=c²/g, q=hu.

Refuser explicitement les états secs, non finis, supercritiques ou c reconstruit <=0.
Recalculer les fantômes à **chaque étage** avec le total intérieur de cet étage.
Comparer quatre méthodes : invariants ci-dessus, extrapolation du total intérieur,
fond seul, et témoin analytique total aux deux étages. Ce dernier n'est pas une identité
discrète comme l'oracle S165 : il donne les états d'une solution continue exacte.

## 2. Référence dérivée et cas

Onde simple avant choc : h0(y)=1+a exp(-(y/8)²), u0=s·2(sqrt(g h0)-sqrt(g)), s=±1.
Un invariant est constant. Pour x'=s(x-x_c), résoudre
x'=y+[3 sqrt(g h0(y))-2 sqrt(g)]t, puis h=h0(y), u=s·2(sqrt(gh)-sqrt(g)).
Le transport de l'autre invariant reçoit directement cette formule.
L'inversion est unique si t·(3 sqrt(g)/2)·a·sqrt(2/e)/8<1, borne dérivée de
max |d exp(-(y/8)²)/dy|=sqrt(2/e)/8 et h>=1.

Durée 24 s, amplitudes 0,02 et 0,05 m, N global équivalent 120/240/480 (dx=1/0,5/0,25 m),
pas 0,2 dx/sqrt(g) et contrôle au demi-pas N240. Paramètres d'instrument, pas profils.
Sortie : centre initial 60 m, Q au repos. Entrée : centre 10 m (s=+1) ou 110 m (s=-1),
Q égal à l'onde simple connue ; d initial nul. Tester les deux directions.
L'extérieur résiduel inconnu de S165 n'est pas remplacé par une information inventée.

Mesurer max erreur hauteur/débit contre l'analytique, max écart au témoin analytique
discret (pour distinguer fermeture et erreur intérieure), erreur dans le cœur [40,80],
bilan de flux ouvert, Courant. Normaliser par a et a sqrt(g).
Le témoin doit converger en espace ; aucune identité à l'arrondi exigée contre le continu.
Témoins : repos, invariants de l'onde simple, refus supercritique, entrée omise par
extrapolation. Le seuil 1e-10 S165 garde seulement bilan et identités algébriques.

## 3. Mesures et réception

Première campagne : 32 montages, quatre fermetures, soit 128 évolutions. Six nouveaux
tests reçus ; huit tests S165 rejoués après extension du support aux callbacks de bord.
Le callback reçoit le total intérieur du prédicteur au second étage ; l'ancienne interface
à fantômes imposés est conservée par délégation, sans modification de sa sémantique.

**Diagnostic ajouté après première mesure :** l'écart au continu reste nettement plus grand
que l'écart au témoin utilisant les mêmes mailles. Pour distinguer perte d'amplitude et
simple déphasage, relever aussi, pour l'entrée, la différence des maxima de hauteur
vers 12 s, quand la crête est dans la fenêtre ; normalisation par a. Ce diagnostic ne
change aucune réception ni tolérance. Aucun seuil perceptuel ou absorbeur choisi.

### 3.1 Entrée et sortie, a=0,05 m

Pas nominal, direction droite ; la symétrie gauche est reçue par test à 1e-10.
E est le maximum normalisé de hauteur contre l'analytique sur 24 s. D est le maximum
de différence entre champs discrets, fermeture contre témoin analytique aux mêmes mailles.
**D n'est pas E moins E_témoin** : le code compare les champs à chaque cellule et instant.

| N | Cas | E témoin | E caractéristique | D caractéristique | D extrapolation | D fond seul |
|---|---|---:|---:|---:|---:|---:|
| 120 | Sortie | 0,278168 | 0,278904 | 1,248765e-3 | 1,909834e-3 | 1,008467e-2 |
| 240 | Sortie | 0,174820 | 0,175214 | 5,013184e-4 | 9,830953e-4 | 1,206785e-2 |
| 480 | Sortie | 0,101265 | 0,101412 | 1,668834e-4 | 5,178742e-4 | 1,381547e-2 |
| 120 | Entrée | 0,422737 | 0,424256 | 2,625612e-3 | 0,981833 | 0 |
| 240 | Entrée | 0,313619 | 0,314832 | 1,505704e-3 | 0,993870 | 0 |
| 480 | Entrée | 0,218780 | 0,219445 | 7,278878e-4 | 0,997005 | 0 |

Le bord caractéristique reçoit les deux sens ; l'extrapolation laisse sortir mais n'admet
presque pas l'entrée. En entrée **le fond seul et le témoin sont identiques par construction**,
car Q est la solution exacte totale prescrite : leur accord ne constitue pas une réception
supplémentaire. En sortie Q est au repos et ne fournit pas la perturbation sortante.
La fermeture caractéristique donne ici moins d'écart au témoin que l'extrapolation dans les
deux cas ; cette comparaison ne sélectionne pas une frontière pour le solveur 3D.

À N240, diviser dt par deux laisse E caractéristique à 0,175215 en sortie, 0,314929 en
entrée ; D vaut 5,014477e-4 et 1,507854e-3. L'erreur dominante n'est donc pas temporelle.
À a=0,02 m, E caractéristique décroît de 0,277496 à 0,099322 en sortie, de 0,401327 à
0,177120 en entrée, entre N120 et N480. Aucune loi universelle ajustée sur ces trois grilles.

### 3.2 La référence continue révèle ce que l'identité discrète laissait intact

Pour l'entrée a=0,05, crête vers 12 s, différence des maxima exact et simulé divisée par a :

| N | Perte témoin | Perte caractéristique |
|---|---:|---:|
| 120 | 0,215615 | 0,215615 |
| 240 | 0,127756 | 0,127756 |
| 480 | 0,071383 | 0,071383 |

La crête reste dans la fenêtre. Une perte de maximum ne peut pas s'expliquer uniquement par
un déplacement de la crête ; une composante dissipative est observée. Cette mesure ne
décompose pas toute l'erreur E en phase et amplitude. À N240, l'erreur de frontière D est
0,15 % de a, alors que E atteint 31 % de a, et la perte de crête 13 % de a.

Q est pourtant connu exactement et d initial est nul dans le cas entrant. La source
discrète S164 rend Q+d identique au solveur total ; elle transmet aussi ses défauts au
fond. C'est cohérent avec l'objectif d'identité de S163/S164, mais cette identité n'est
pas une preuve de préservation du fond analytique. La réception physique doit rester
distincte de celle du changement de variables. **A222** suit ce point, sans modifier la source ici.

### 3.3 Réception reproductible

Depuis `code/` :

```
cargo test -p water-core --example bord_autonome --example frontiere_locale
cargo run -p water-core --release --example bord_autonome
```

Six tests S166 reçus : invariants entrants/sortants, refus supercritique, équations de
conservation de l'onde simple par différences centrées, repos/entrée, convergence spatiale,
symétrie des directions. Huit tests S165 reçus après modification de son support partagé.
Campagne finale : 32 montages, quatre fermetures, 128 évolutions ; bilan relatif ouvert
<=2,04e-15 et Courant <=0,214735. Aucun écrêtage ; tableaux alloués avant les pas.
La référence reste avant choc (borne de croisement <0,61 sur les paramètres retenus).
Les états analytiques sont échantillonnés aux centres, pas intégrés en moyennes de cellules.
La réception n'établit pas un ordre supérieur du schéma spatial, qui reste d'ordre un.

Bibliothèques inchangées, workspace 299 tests/cinq ignorés reçu S163, non relancé ici.
S165 est rejoué ; les autres exemples gardent leurs réceptions précédentes.

## 4. Verdict et suite

**S165-1 réalisée, A221 traitée sur véhicule subcritique 1D à entrée connue.** Une fermeture
sans oracle est construite, ses hypothèses et ses refus sont explicites. Un résidu extérieur
arbitraire inconnu reste inconnu ; ni régime supercritique ni choc ni eau sèche reçu.
Pas d'ADR nouveau, pas de choix d'absorbeur ni d'API runtime, aucun seuil de qualité fixé.
A50 reste partielle ; A216/A217 inchangées. I-01/04/12/14/15 inchangés.

**Suite S167 : S166-1/A222**, séparer fidélité au schéma total et préservation d'un fond
exact. Dériver une discrétisation du résidu qui préserve d=0 pour Q solution exacte,
la comparer à la source S164 sur l'onde simple puis sur une perturbation ajoutée.
Ne pas supprimer silencieusement le défaut physique d'un fond approximatif : la source
doit distinguer ce défaut du résidu numérique de Q. La comparaison au total restera un
diagnostic distinct, et non un critère d'identité imposé au nouveau candidat.

**Suivi S167 : S166-1 réalisée sur véhicule**, voir [FOND-PRESERVE-S167](FOND-PRESERVE-S167.md).
Le candidat préserve Q exact et reçoit une perturbation ; source physique d'un Q figé
conservée. A222 traitée dans ce périmètre ; clôture du volume total suivie par A223/S167-1.
