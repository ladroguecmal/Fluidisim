# S169 — Assemblage autonome, équilibré et conservatif

## 1. Dérivation avant mesure

Suite S168-1/A224. Résidu en moyennes S168, Q exact connu, S=0 ; RK2 sur d,
flux total = flux différentiel numérique + flux physique de Q intégré dans le temps.
La fermeture doit produire un fantôme égal à Q fantôme lorsque d intérieur est nul.

S166 transportait un invariant **total** depuis l'intérieur. Pour Q variable, ses valeurs
diffèrent entre cellule intérieure et fantôme. Le candidat transporte plutôt l'écart :
ΔR_sortant=R(T_intérieur)−R(Q_intérieur), avec R±=u±2sqrt(gh).
Au fantôme, R_sortant=R(Q_fantôme)+ΔR_sortant ; invariant entrant=R(Q_fantôme).
Gauche : sortant R− ; droite : sortant R+. Hypothèse : aucun résidu entrant prescrit.

Reconstruction sans annulation à d=0 : δu=ΔR/2, δc=−ΔR/4 à gauche, +ΔR/4 à droite ;
δh=(2c_Q δc+δc²)/g, δq=H_Q δu+u_Q δh+δh δu. Ajouter ces écarts à Q fantôme.
Cette écriture donne exactement Q pour ΔR=0 sans seuil artificiel. Recalculer aux deux
étages avec les totaux et Q intérieurs de l'étage. Refus secs/non finis/supercritiques S166.

Quatre voies : témoin T exact en moyennes, transfert total S166, transfert d'écart ci-dessus,
fond seul. Les fantômes exacts ne sont consultés que par le témoin et les diagnostics.

## 2. Protocole

Fenêtre[30,90] m, référence onde simple S166 en moyennes/quadratures S168, durée24 s,
avant choc ; largeur8 m. Cas :

- **Entrée pure** : Q=T a0,05, centre initial10 m vers la droite (110 m vers la gauche).
  d=0 doit rester nul pendant l'entrée puis l'approche de la sortie du fond variable.
- **Sortie sur repos** : Q=(1,0), T a0,05 centre60 m. Perturbation initialement intérieure.
- **Sortie sur fond variable** : Q a0,05, T a0,06, centre55 m vers la droite (65 vers la gauche).
  Les deux champs sont des solutions non linéaires exactes, pas une superposition.

Les gaussiennes ne sont pas à support compact : mesurer l'écart d'invariant entrant
analytique omis par la fermeture, y compris celui dû aux moyennes. Ne pas le déclarer nul.
La fermeture ne peut pas recevoir un résidu entrant inconnu par hypothèse.
N120/240/480 vers la droite ; contrôles gauche et demi-pas N240 :15 montages, quatre voies.

Mesurer max erreur hauteur/débit contre T moyen, max écart de champ au témoin aux mêmes
mailles, max résidu et bilan de volume sur les **flux effectivement utilisés**, pas sur
ceux du témoin. Normalisations0,05 m et0,05 sqrt(g) m²/s ; bilan relatif au volume initial.
Seuil instrumental1e-10 pour préservation, identités et bilan, aucun seuil physique.

Tests des supports S165–S168 à rejouer s'ils sont modifiés ; extraire les références communes
sans les dupliquer. Aucune API runtime adoptée ni performance de production reçue.

## 3. Résultats

### 3.1 Préservation et traversée

Entrée pure : Oracle, Anchored et Background gardent d=0 exactement, en hauteur et
débit. L'ancienne fermeture Total crée une erreur normalisée de hauteur7,437547e-4,
2,316569e-4 puis6,358646e-5 à N120/240/480. Elle converge, mais ne préserve pas Q.
L'échange maximal cumulé vaut0,7088372 m² : le fond traverse réellement le domaine.
L'accord de Background et Oracle est attendu puisque Q=T ; il n'est pas une réception
supplémentaire. Sur repos, Total et Anchored coïncident à l'arrondi, comme la dérivation l'exige.

Sortie, direction droite, pas nominal. E=max erreur hauteur/0,05 contre l'analytique
moyenné ; D=max différence de champ au témoin, calculée avant la norme.

| N | Cas | E témoin | E Anchored | D Anchored | D Total | D Background |
|---|---|---:|---:|---:|---:|---:|
|120|Repos|0,277294|0,278014|1,240500e-3|1,240500e-3|1,006438e-2|
|240|Repos|0,174654|0,175046|5,004190e-4|5,004190e-4|1,206204e-2|
|480|Repos|0,101235|0,101382|1,668010e-4|1,668010e-4|1,381368e-2|
|120|Variable|0,075613|0,075731|1,643577e-4|3,599170e-4|1,007725e-3|
|240|Variable|0,050963|0,051007|6,978624e-5|1,200199e-4|8,322959e-4|
|480|Variable|0,031403|0,031414|2,472311e-5|3,796466e-5|7,608955e-4|

Le candidat est plus proche du témoin sur fond variable ; cela ne garantit pas une
erreur au continu toujours plus faible. À N240, Total a même E légèrement inférieur
par compensation d'erreurs. La fermeture ne reçoit pas un coefficient de réflexion.
L'erreur du cas variable rapportée à la perturbation initiale de1 cm reste25,5 % à
N240 sur24 s. La fermeture ne résout pas la dissipation intérieure.

### 3.2 Information entrante et bilan

L'écart d'invariant entrant de la référence, omis par l'hypothèse du bord, est mesuré
et normalisé par0,05 sqrt(g). Sur repos :5,184045e-5 →1,295535e-5 →3,235227e-6 ;
sur fond variable :3,475457e-5 →1,753983e-5 →2,004423e-5. Il n'est donc pas déclaré
nul, même dans la famille d'ondes simples : invariants des moyennes et moyennes des
invariants diffèrent, et les queues gaussiennes ne sont pas compactes. Aucune réception
d'un résidu entrant arbitraire inconnu n'est déduite de ces essais.

Volume : maximum relatif1,890711e-15 sur **les60 évolutions**, avec le flux différentiel
de chaque variante et le flux physique intégré de Q. Le flux du témoin n'est jamais
substitué à celui du candidat. Les variantes imparfaites ferment aussi le volume.
Courant<=0,217650. Maximum absolu du cumul d'échange du candidat à N240 :0,7096071 m²
sur repos,0,8509321 m² sur fond variable ; sortie effectivement exercée.

### 3.3 Réception

Depuis `code/` :

```
cargo test -p water-core --example assemblage_autonome --example volume_moyen --example fond_preserve --example bord_autonome --example frontiere_locale
cargo run -p water-core --release --example assemblage_autonome
```

Quatre nouveaux tests reçus : identité algébrique sur Q variable, entrée pure contre
ancien bord, sortie/convergence, fond variable dans les deux directions (symétrie à1e-9).
24 tests précédents rejoués après extension du callback et extraction des références
partagées :8 S165,6 S166,5 S167,5 S168. Campagne15 montages/quatre voies=60 évolutions,
trois grilles, gauche N240 et demi-pas N240. Aucun échec, aucune bibliothèque modifiée.
Workspace299/cinq ignorés reçu S163, non relancé. Quadrature de banc, coût runtime non reçu.

## 4. Verdict et suite

**S168-1 réalisée, A224 traitée sur véhicule subcritique1D à fond exact connu.**
Préservation, sortie et conservation sont reçues ensemble. Aucun ADR ni schéma runtime
adopté ; ni3D, choc, eau sèche ou résidu entrant inconnu reçu. I-01/04/12/14/15 inchangés.
A216/A217 inchangées ; A50 reste partielle.

**Suite S170 : S169-1/A50**, mesurer enfin l'échantillonnage grossier de la source,
paramètre explicite de PLAN-BENCHMARK §B4 et SPEC-004 §6.2. Le véhicule dispose des
contrôles intérieurs et de bord nécessaires : reprendre un fond figé asymétrique à
source physique connue, comparer source moyenne exacte, interpolation sur réseau
décimé et omission. Raffiner séparément dx et le réseau source, observer dérive et
bilan sans les confondre. Ne pas conclure à une faillite de l'architecture sur un défaut
de source interpolée. Aucun nouvel angle numéroté nécessaire : ce travail relève d'A50.

**Suivi S170 : S169-1 réalisée sur véhicule**, voir [SOURCE-DECIMEE-S170](SOURCE-DECIMEE-S170.md).
Sensibilité au réseau et à sa phase mesurée ; injection artificielle prédite. A50 reste
partielle ; discrétisation conservative de la source suivie A225/S170-1.
