# S217 — Décohérence du sillage : similitude et histoire du forçage

## Protocole déclaré avant campagne

**Objet** : A255, part dynamique ; savoir si une table dépendant seulement du temps réduit
depuis extinction pourrait remplacer le majorant directionnel d'ADR-134.

**Techniques présentes** : source gaussienne du cœur, demi-spectre, préparation exacte du
journal, coefficients d'image rebasés, majorant directionnel ADR-134. Recherche hors ligne
par récurrence trigonométrique f64 sur grille puis raffinement local, vérifiée contre le cœur.
**Absentes** : GPU, LOD, visibilité, mutualisation, solveur δ. Aucun coût d'image mesuré.
**Domaine** : une trajectoire droite à charge constante en milieu uniforme profond ; g=9,81,
rho=1025. Les variantes sont des fixtures de banc, pas des paramètres de production.
**Rang de passage** : sans objet, aucun temps d'exécution publié.

### Dérivation depuis le modèle exécuté

La dispersion du noyau est `omega² = g k`. Avec `q = sigma k`, `T = sqrt(sigma/g)`,
`u = v/sqrt(g sigma)`, `d = D/T` et `tau = (t-D)/T`, la réponse à une source mobile
contient à la fois `sqrt(q)` et le Doppler `q_x u`. L'intégration entre naissance et
extinction conserve `d` dans les amplitudes et phases modales. La charge multiplie
linéairement tout le champ : elle s'élimine dans le rapport majorant/maximum, à l'arrondi près.

Une homothétie `sigma -> a sigma`, `kmax -> kmax/a`, `x -> a x`,
`v -> sqrt(a) v`, `D -> sqrt(a) D`, `t -> sqrt(a) t`, `F -> a³ F`
(g et rho fixes) conserve les pentes et leurs rapports. Cette **similitude conditionnelle**
ne supprime ni `u`, ni `d`, ni `sigma*kmax`, ni la forme du trajet. Elle ne prouve aucune
loi à une variable pour la famille générale. Source : équations exécutées dans
`modal_pressure`, cuisson `gaussian_spectrum`, force/aire dans `wake_source` ; SPEC-001 §1.

### Montages et critères

Base : sigma=2 m, cutoff=3 rad/m, v=3 m/s, F=19 620 N, D=8 s, quatre tronçons,
origine (-12,0), emprise [-64,64]×[-48,48] m. Grille 1 m puis recherche locale à
pas 0,05 m ; contrôle grille 0,5 m avec raffinement à 0,025 m. Recette 64×128,
puis 128×256 et 256×512 sur les cas discriminants. Maxima toujours minorants échantillonnés.

- Homothéties a=0,5 et 2 : même temps réduit ; écarts relatifs de rapport attendus <0,1 %.
- F/2 et découpage en huit tronçons : témoins de linéarité et de représentation (<0,1 %).
- Vitesse 1,5 et 6 m/s, durée 4 et 16 s variées séparément ; après-extinction à
  tau=0,4,12,24. Un écart de rapport >5 % persistant sous raffinement **réfute**
  la courbe universelle en tau sur ces fixtures. Ce seuil est un discriminant de banc,
  pas une tolérance physique ni une garde de production.
- Forçage séparé : t/D=0,25 et 0,75, sans les ranger dans tau positif.
- Vérifier la pente reconstruite contre `Prepared::sample_batch` à l'argmax et en
  plusieurs points fixes ; écart absolu <1e-5, seuil de banc déclaré. Zéro refus masqué.
- Pour interpréter les cas discriminants, variation du maximum <1 % entre les deux
  dernières quadratures et les deux grilles. Sinon publier « non convergé ».

La durée d'image annoncée par ADR-132 accompagne chaque recette. Une mesure au-delà est
diagnostique et ne reçoit pas une image. Aucune enveloppe universelle sûre ne se déduit
du succès de quelques maxima : pas de table de production sans justification supplémentaire.

## Résultats — 2026-09-13

Instrument : `code/water-core/examples/decoherence_sillage_s217.rs`, deux tests debug et
release réussis (récurrence/direct avec témoin nul ; module de hauteur libre non invariant
avec énergie conservée). **54 mesures initiales, neuf contrôles affinés**, aucun refus masqué.
Relevés : [campagne](DECOHERENCE-SILLAGE-S217-MESURES.md),
[64×128, pas 0,5](DECOHERENCE-SILLAGE-S217-R64.md),
[128×256](DECOHERENCE-SILLAGE-S217-R128.md), [256×512](DECOHERENCE-SILLAGE-S217-R256.md).

### La similitude tient, la courbe à un seul âge ne tient pas

Rapport **majorant publié ADR-134 / maximum échantillonné** ; ce n'est pas le majorant
directionnel exact de l'instrument S216 (dont 3,98 était un rapport).

| fixture, 64×128 / pas 1 m | tau=0 | tau=4 | tau=12 | tau=24 |
|---|---:|---:|---:|---:|
| base D=8 s, v=3 m/s | 1,419271 | 1,637742 | 2,037249 | 2,608574 |
| homothétie a=0,5 | 1,419272 | 1,637742 | 2,037249 | 2,608575 |
| homothétie a=2 | 1,419271 | 1,637742 | 2,037249 | 2,608574 |
| charge /2 | 1,419271 | 1,637742 | 2,037249 | 2,608574 |
| huit tronçons au lieu de quatre | 1,419271 | 1,637742 | 2,037249 | 2,608574 |
| v=1,5 m/s | 1,567996 | 2,273846 | 2,863033 | 4,121943 |
| v=6 m/s | 2,188866 | 2,345018 | 2,326607 | 2,840283 |
| D=4 s | 1,155067 | 1,451600 | 2,277500 | 3,187533 |
| D=16 s | 1,626576 | 2,006040 | 2,604755 | 3,438438 |

Les trois témoins passent <0,1 % ; les homothéties diffèrent d'au plus 0,000001 dans
les rapports imprimés. Sigma=4 est ici **admis**, car cutoff est réduit à 1,5 : le refus de
S216 à sigma=4/cutoff=3 portait sur le couple, pas sur sigma seul.

Les différences initiales à tau=0 excluent aussi de rendre ces courbes identiques par
un simple changement multiplicatif d'échelle de temps depuis extinction : zéro reste zéro.
Cela **n'exclut pas** une normalisation d'amplitude, un décalage d'âge effectif ou une
famille de courbes paramétrée par vitesse/durée. Ces pistes ne sont pas reçues ici.

### La réfutation survit aux raffinements

| fixture à tau=4 | 64×128, h=1 | 64×128, h=0,5 | 128×256, h=0,5 | 256×512, h=0,5 |
|---|---:|---:|---:|---:|
| base | 1,637742 | 1,637742 | 1,637434 | **1,637275** |
| v=1,5 | 2,273846 | 2,273800 | 2,272638 | **2,272143** |
| D=16 | 2,006040 | 2,005546 | 2,005959 | **2,005464** |

Écart à la base **+38,8 %** en vitesse, **+22,5 %** en durée, contre 5 % requis pour
réfuter. Maximum spatial : variation <0,025 % au demi-pas, **<0,002 %** entre les deux
dernières quadratures. Ces trois cas sont **dans la durée d'image annoncée**, y compris
à la recette initiale. L'écart reconstruction/cœur aux six points de contrôle par mesure
culmine à **2,383e-6**, sous le seuil de banc 1e-5. Ce contrôle ne certifie pas tous les
points du domaine ni une borne continue.

**Limites explicites** : au niveau initial, tau=24 dépasse la durée annoncée pour base,
ses homothéties, v=1,5 et v=6 ; D=16 dépasse aussi à tau=12. Ces lignes sont des
diagnostics, pas une réception d'image. Les variations indépendantes de cutoff, de la
forme du trajet et des deux axes spectraux ne sont pas une campagne complète ici ;
elles ne sont pas nécessaires à la réfutation à tau=4, mais seraient nécessaires pour
recevoir une autre loi générale. Aucune hypothèse de décroissance monotone n'est retenue.

### Le forçage ne donne pas non plus une monotonie générale

Pour la base, entre t/D=0,25 et 0,75, le majorant passe de 0,082825 à 0,119596.
Pour la source lente, il passe de **0,059152 à 0,052456** : une source encore active peut
voir son majorant **diminuer**. Ces valeurs sont dans la durée annoncée ; la phase active
reste séparée du régime libre. « Le majorant croît pendant le forçage » décrivait la
fixture S216, pas un théorème du modèle.

## Correction du mécanisme : la hauteur d'un mode ne tourne pas seule

`ModalPressure::sample` évolue le couple hauteur/vitesse selon

```
eta(t) = eta(D) cos(omega s) + velocity(D)/omega sin(omega s)
velocity(t) = velocity(D) cos(omega s) - omega eta(D) sin(omega s), s=t-D.
```

Les coefficients sont complexes. **|eta| n'est pas invariant** en général ; le couple
diagonalisé porte deux rotations opposées. C'est `omega² |eta|² + |velocity|²` qui est
invariant dans l'équation exacte. Le test mono-mode stationnaire vérifie simultanément
la conservation à 1e-5 et une variation relative de |eta|² >1 %. La somme des modules
de hauteur utilisée dans l'enveloppe peut donc osciller après extinction : la base
publie 0,122009 à D et 0,115171 à D+1,806095 s.

Les explications « le majorant reste figé » de S215/L290 doivent être corrigées ;
**ni la décision d'ADR-133 pour l'impact, ni l'inégalité d'ADR-134 ne sont invalidées**.
Le resserrement reste nécessaire, mais son mécanisme comprend la réponse libre complète,
pas seulement une rotation de chaque coefficient de hauteur.

## Verdict et suite

**A255 reste ouverte, mieux délimitée.** Similitude conditionnelle reçue ; courbe unique
du rapport en temps réduit depuis extinction **réfutée sur cette famille**. Aucune
table ajoutée à la bibliothèque, aucun champ ni contrat d'admission changé. Aucun ADR
nouveau : l'étude n'adopte aucun mécanisme de production.

**Suite recommandée, file J1/W** : construire une borne de pente locale dépendant du
champ préparé et de l'emprise, avec marge spatiale démontrée (par exemple depuis une
borne de Hessienne), puis mesurer si elle resserre le budget à coût utilisable. Cette
piste exploite directement les coefficients et leur histoire, sans attendre une courbe
universelle. Déclarer les refus et le coût avant de l'employer ; un maximum de grille
sans reste n'est pas un majorant. A254 (somme des sources) reste ouvert ; loi GPU,
J2/δ et V-noyau conservés dans la feuille de route.

### Reproduction

Depuis `code/` :

```
cargo test --offline -p water-core --example decoherence_sillage_s217
cargo test --offline --release -p water-core --example decoherence_sillage_s217
cargo run --offline --release -p water-core --example decoherence_sillage_s217 -- 64 128 1 all
cargo run --offline --release -p water-core --example decoherence_sillage_s217 -- 64 128 0.5 all --check
cargo run --offline --release -p water-core --example decoherence_sillage_s217 -- 128 256 0.5 all --check
cargo run --offline --release -p water-core --example decoherence_sillage_s217 -- 256 512 0.5 all --check
```

**Vérification d'intégration S217** : suite workspace complète release hors réseau,
356 tests réussis, cinq ignorés, aucun échec. Deux tests d'exemple en plus, debug/release.
La suite debug complète a été interrompue pendant les balayages hérités, puis remplacée
par la suite complète optimisée ; aucun succès debug global revendiqué.
