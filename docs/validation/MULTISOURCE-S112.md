# S112 — Champ commun aux sources de pression du journal

2026-09-09. Prolonge ADR-069/074/075 et le bilan S104 ; aucun changement de modèle,
format ou décision d'autorité. S111-1 réalisée dans le périmètre ci-dessous.

## Construction

`bound_pressure::Prepared::from_journal` prépare un seul champ spectral depuis toutes
les sources publiées d'un journal. Journal vide : Empty ; source en attente : Pending.
Il n'utilise jamais published pour contourner une attente. Recette et contexte complet
identiques exigés pour toutes les sources : référentiel, cellule, gravité, densité,
rectangle et fenêtre temporelle. Aucune conversion implicite entre contextes.
Ces contrôles et la date précèdent le calcul ; un refus numérique ultérieur peut
modifier le pool candidat, sans publier de vue partielle. Les sources restent valides
par leurs constructeurs. Le journal est emprunté aussi longtemps que le champ retourné
est utilisé : impossible de l'admettre ou modifier en parallèle de cette vue en Rust sûr.

Ordre : identifiant croissant du journal, puis segments dans leur ordre, pour chaque
nœud spectral. L'itérateur est clonable, sans allocation d'un tableau concaténé.
Le calcul existant est partagé dans une primitive interne ; le chemin monotrajet
conserve ses validations et son ordre numérique. La nouvelle voie permet un instant
antérieur à la naissance de certaines ou de toutes les sources, dans le contexte :
leur réponse et leur pression sont alors nulles, comme dans le noyau modal. Le chemin
monotrajet historique conserve son refus avant sa première naissance.

Par mode : additionner réponses complexes q et qdot de toutes les sources, puis
additionner leurs pressions actives P. L'énergie dépend des carrés de ces sommes ;
la puissance est -poids*Re(P*conj(qdot)). Les interférences entre sources sont ainsi
conservées, comme celles entre segments. Aucune énergie par source n'est ajoutée.
L'enveloppe de pente S106 est recalculée sur les coefficients totaux. Les requêtes
locales et monde B+pression existantes peuvent utiliser cette vue, à la date préparée.
Le bilan demeure celui de la pression seule, pas celui de B+pression.

Pools de champs inchangés : un emplacement par nœud, pas par source. Temps de
préparation proportionnel au nombre total de segments ; la reconstruction ponctuelle
ne parcourt que le champ total préparé. C'est une structure du code, pas une mesure
de performance. Aucun coût chronométré ni budget cible certifié dans cette session.

## Réception

Quatre tests supplémentaires sur recette16×24, demi-spectre192 modes. Ce choix réduit
le coût du test de contrat ; aucune précision continue revendiquée pour cette recette.

Deux sources distinctes : première à(0,0),10 Pa,vitesse(2,0),naissance0 ; seconde
à(1,1),7 Pa,vitesse(0,2),naissance0,5 s. Durée2 s chacune. σ1 m,coupure6/m,
g9,81f32,densité1025 ; rectangle[-8,12]²,fenêtre0–8 s.

- Arrivées inversées au journal : sept grandeurs locales, énergie et puissance
  identiques en bits. Six dates de0 à8 s, quatre points dont les coins. Somme des
  champs séparés reçue à1e-7 absolu ; énergie totale différente de la somme isolée
  de plus de1e-4 J à1,5 s. La comparaison de linéarité partage le noyau numérique.
- Deux sources coïncidentes à10 Pa reçues contre une seule à20 Pa pour énergie et
  puissance, seuil1e-7. Sources coïncidentes10/-10 Pa : énergie, puissance et enveloppe
  exactement nulles. Ces témoins interdisent une addition des énergies isolées.
- Travail intégré par milieux de0 à2,5 s, naissances/extinctions alignées sur les
  subdivisions ; raffinement250/500/1000 pas. Énergie finale0,2008518278599 J.
  Résidus2,533535e-6 /6,372630e-7 /1,638411e-7 J. Assertions :<3e-6 et réduction
  de plus de moitié à chaque raffinement. Somme instrumentale f64 hors runtime.
  À8 s puissance nulle, énergie conservée au seuil1e-7 J.
- Journal vide, saturé avec attente, contexte/recette incompatibles, date hors fenêtre
  et pool insuffisant refusés ; témoin valide après refus. Pas de fusion partielle de
  contextes ni de suppression silencieuse de sources gênantes.

Réception release exécutée avec les quatre tests ; suite debug complète consignée au
journal. Les tests historiques restent responsables de leurs hashes et réceptions.
Cette campagne reçoit la superposition et la comptabilité du spectre discret, pas la
résolution spatiale du nouveau montage ni sa concordance avec un oracle indépendant.

## Suite

**S112-1, S113 :** réception du montage multisource contre une référence indépendante
f64 et raffinement radial/angulaire sur les sept grandeurs, énergie et puissance,
puis mesure du coût complet aux résolutions retenues. Garder distinctes linéarité,
bilan fermé et précision spatiale. La restauration d'un journal multisource est déjà
possible en format ; un scénario de reprise numérique multisource reste à exercer.
Impacts, milieu variable, durabilité et autorité interplateforme restent ouverts.
76 ADR,193 angles,17 invariants,6 spécifications,23 cas inchangés.
