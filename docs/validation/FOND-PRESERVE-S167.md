# S167 — Préserver un fond exact sans supprimer son défaut physique

## 1. Dérivation et protocole avant mesure

Suite S166-1/A222 ; mêmes équations et fenêtre [30,90] m que S166.
Noter L(Q)=-∂xF(Q), d=T-Q. L'équation continue est
d_t=-∂x[F(Q+d)-F(Q)] + S, avec **S=L(Q)-Q_t**.
La source physique est nulle pour une solution exacte, pas pour un fond quelconque.

Le candidat discrétise la différence de flux par `delta_numerical` S163, puis intègre
d seul en RK2, Q réévalué à chaque étage et S physique injectée aux mêmes temps :

```
d1 = d0 + dt [D(Q0,d0)+S0]
d+ = (d0+d1+dt [D(Q1,d1)+S1])/2
```

Aucun incrément temporel de Q n'est soustrait : il figure déjà dans S. Si S=0 et d=0
jusqu'aux fantômes, D=0 exactement. C'est une propriété d'équilibrage, différente de
l'identité au schéma total S164. Conserver ce schéma comme témoin, pas comme verdict.

Trois cas, référence d'onde simple S166 commune, centre initial 60 m, largeur 8 m,
direction droite, durée 6 s avant choc :

1. Q onde exacte a=0,05, total T=Q : résidu initial nul, S=0.
2. Q même onde, total exact de la même famille avec a=0,06 : perturbation non nulle,
   évolutive et non linéaire ; comparer directement à T analytique, pas à une superposition.
3. Q onde de a=0,05 **figée à t=0**, total T onde évolutive a=0,05. d initial nul,
   S=-∂xF(Q), dérivée analytique. Contre-épreuve omettant S dans le candidat.

Pour le fond figé, H'=-(x-60)·(H-1)/32 ; M=2H(c-c0), M'=(3c-2c0)H'.
S_h=-M', S_q=-[2M M'/H-M² H'/H²+gH H']. La conservation fournit ces expressions.
Une source omise doit échouer : préserver arbitrairement un fond faux n'est pas un succès.

Fantômes T analytique aux deux étages pour isoler la source intérieure ; ce n'est pas
une nouvelle réception du bord autonome S166. N global équivalent 120/240/480/960,
dt<=0,2 dx/sqrt(g), contrôle demi-pas N240. Échantillons aux centres comme S166.
Normalisations hauteur 0,05 m, débit 0,05 sqrt(g) m²/s pour les trois cas ; publier aussi
l'erreur de perturbation normalisée par 0,01 m pour le deuxième cas.

## 2. Bilans et critères

Attendre résidu nul au seuil 1e-10 pour le fond exact seul ; convergence spatiale contre
T pour les deux cas non nuls, sans fixer de seuil physique. Vérifier les tests S165/S166
après extension du support. Aucun choix d'API ou de schéma de production.

Le bilan du candidat est celui du **résidu avec source** : somme(d+−d0)dx égale
l'intégrale RK2 des flux différentiels et de S. Pour le total, ajouter somme(Q+−Q0)dx.
Ce n'est pas nécessairement le bilan des flux Rusanov du total aux frontières : publier
aussi cet écart, ne pas appeler « conservatif pour le total » un simple bilan résiduel.
Les sommes d'échantillons de Q ne sont pas des intégrales exactes en cellules.

## 3. Résultats

P3 : 15 montages, trois voies, 45 évolutions. Q exact seul reste à d=0 exactement ;
perturbation ajoutée convergente ; source physique du fond figé indispensable.
La référence d'onde simple S166 est extraite en support partagé, sans duplication.

**Diagnostic ajouté après le premier passage :** le flux cohérent du candidat est
delta_flux_numérique + F(Q) physique aux faces, et non le flux Rusanov total ancien.
Le second bilan est donc complété par ce flux corrigé, intégré aux deux étages RK2.
Il reste un défaut de quadrature : Q est échantillonné aux centres et en deux temps.
Ce troisième bilan permet de ne pas appeler « perte de masse » le seul changement de flux.

### 3.1 Fond exact et perturbation

E est max |h−h_exact|/0,05 sur l'espace et les 6 s. Pas nominal.

| N | Fond exact, témoin S164 | Fond exact, candidat | Perturbé, témoin | Perturbé, candidat | Candidat, échelle perturbation 0,01 |
|---|---:|---:|---:|---:|---:|
| 120 | 0,217463 | 0 | 0,262775 | 0,050670 | 0,253350 |
| 240 | 0,129671 | 0 | 0,158371 | 0,031292 | 0,156462 |
| 480 | 0,072124 | 0 | 0,088399 | 0,017806 | 0,089030 |
| 960 | 0,038313 | 0 | 0,047131 | 0,009597 | 0,047983 |

Le candidat conserve d=0 exactement en hauteur **et en débit** sur le fond exact.
Le témoin conserve sa propriété S164, mais amortit Q comme le total qu'il reproduit.
Sur T de la même famille avec amplitude 0,06, d est non nul et évolue ; l'erreur baisse
au raffinement et vaut environ un cinquième de celle du témoin. **Il reste 15,6 % d'erreur
rapportée à la perturbation de 1 cm à N240** : conserver Q ne reçoit pas à lui seul d.
La dernière colonne normalise par l'amplitude initiale ajoutée, pas par le maximum ultérieur
du résidu, qui peut varier par déphasage non linéaire.
Au demi-pas N240, E perturbé candidat=0,031296 ; le défaut n'est pas dominé par dt.

### 3.2 Un fond figé ne doit pas être préservé

| N | Témoin S164 | Candidat avec S physique | Candidat sans S |
|---|---:|---:|---:|
| 120 | 0,217463 | 0,227024 | 0,997346 |
| 240 | 0,129671 | 0,135771 | 0,998284 |
| 480 | 0,072124 | 0,075451 | 0,998215 |
| 960 | 0,038313 | 0,040099 | 0,998284 |

La source physique permet de retrouver l'évolution au raffinement. L'omettre laisse presque
toute l'erreur d'un fond figé ; les fantômes exacts seuls ne la corrigent pas.
Le candidat n'est pas meilleur dans tous les cas : ici il est légèrement moins précis que
le témoin S164. La distinction physique/numérique est reçue, pas une supériorité universelle.
Les voies « équilibrée » et « source omise » coïncident par construction dans les deux
premiers cas, où la vraie source est nulle ; ce ne sont pas des confirmations indépendantes.

### 3.3 Trois bilans, trois propriétés

Bilan résiduel avec S et incrément exact de somme(Q) : défaut relatif <=2,80e-15 sur les
45 évolutions. Normalisation des bilans : volume initial de la fenêtre, environ 60 m² par
unité de largeur, pas amplitude de la perturbation. Courant <=0,217651.

Fond exact seul, candidat :

| N | Écart au flux Rusanov total ancien | Écart au flux corrigé |
|---|---:|---:|
| 120 | 7,052058e-5 | 2,078204e-6 |
| 240 | 3,316266e-5 | 5,200756e-7 |
| 480 | 1,607166e-5 | 1,300514e-7 |
| 960 | 7,910231e-6 | 3,251488e-8 |

Le premier écart décroît approximativement comme dx : il comprend le retrait de diffusion
numérique du fond. Le second décroît comme dx² à dt proportionnel à dx. Au demi-pas N240,
il vaut 4,905864e-7 : il demeure une composante spatiale. **Le volume total ne ferme donc
pas à l'arrondi**, même avec le flux corrigé. Q ponctuel et sa somme ne sont pas sa moyenne
conservative en cellules ; deux temps RK2 ne sont pas une intégrale exacte du flux analytique.

Le fond figé donne un bilan corrigé à l'arrondi **dans ce montage symétrique seulement** :
son flux de masse analytique net et la somme de S_h s'annulent par symétrie. Cela ne reçoit
pas le bilan d'un fond figé général. Le prochain montage doit casser cette symétrie.

## 4. Réception et suite

Depuis `code/` :

```
cargo test -p water-core --example fond_preserve --example bord_autonome --example frontiere_locale
cargo run -p water-core --release --example fond_preserve
```

Cinq nouveaux tests reçus : fond exact, perturbation évolutive/convergence, dérivée de flux
du fond figé, source omise, distinction des bilans et ordre deux du défaut corrigé.
Six tests S166 et huit S165 reçus après extraction de la référence et extension du support.
15 montages/45 évolutions release reçus ; seul le cinquième test a été ajouté après cette
campagne, sans changer le calcul. Bibliothèques inchangées ; workspace 299/cinq ignorés
reçu S163, non relancé. Aucune dépendance ni allocation par pas dans le support local.

**S166-1 réalisée, A222 traitée sur véhicule à source connue.** Le candidat préserve Q exact,
fait évoluer d non nul et garde le défaut physique d'un Q faux. Aucun ADR nouveau : ce
candidat expérimental n'est pas adopté comme schéma runtime et ne remplace pas la réception
d'identité de S164. A50 partielle, A216/A217 inchangées, pas de réception B4 générale.

**Suite S168 : S167-1/A223**, fermer le volume avec moyennes de Q en cellules et flux
physiques intégrés sur le pas, au lieu de sommes de valeurs ponctuelles. Dériver le bilan
avant code, recevoir Q exact puis une perturbation, et ajouter un montage asymétrique
pour que le flux net ne s'annule pas. Distinguer cette conservation totale du budget du
résidu et de la qualité de transport. Ni bord autonome, 3D, choc ou eau sèche reçu ici.

**Suivi S168 : S167-1 réalisée**, voir [VOLUME-MOYEN-S168](VOLUME-MOYEN-S168.md).
Moyennes et flux intégrés indépendamment ferment le volume, y compris sur Q figé
asymétrique. A223 traitée dans ce périmètre ; assemblage avec bord autonome suivi A224/S168-1.
