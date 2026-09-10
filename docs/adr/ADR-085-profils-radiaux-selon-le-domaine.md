# ADR-085 — Dimensionner le profil radial au domaine commun

- **Statut : ACTÉE**, S125, 2026-09-09, délégation technique.
- **Complète** ADR-060, ADR-066 et ADR-084 ; aucun modèle physique remplacé.
- **Traite** S124-1 et A202, choix du profil après mesure du coût.

## Motif mesuré

[COUT-PROFIL-IMPACT-S125](../validation/COUT-PROFIL-IMPACT-S125.md) : à domaine et points
identiques, N256 coûte 3,93 à 4,15 fois N64 par point. Avec le domaine étendu R128,
le rapport au montage N64/R16 atteint 8,85 à 9,38. Un champ passe de 1632 à 6240 octets.
Le coût supplémentaire est réel même si la pression dominait le cycle historique S118.
Une domination dans un scénario ne constitue pas un budget disponible pour tous les autres.

S124 montre aussi que N256 n'est pas nécessaire partout : à α=2, parmi les onze cas,
un atteint sa portée avec N64, six demandent N128, deux N256, deux restent refusés.
Ce classement est celui de la fixture ; α et les sources physiques restent à calibrer B2.

## Décision

**Ne pas passer le défaut global à N256.** Les paramètres par défaut de `RadialImpact`,
`Prepared`, `RenewalController` et `LiveWater` restent N64 ; les fixtures historiques restent
explicitement à leur résolution reçue. N128 et N256 sont des choix explicites pour les
montages qui exigent davantage de portée ou d'horizon.

Le choix porte sur le **service et son domaine commun**, pas sur un impact isolé ni sur le
réglage graphique d'un client : les pools existants sont homogènes en N. Pour dimensionner
un nouveau montage, prendre le plus petit de 64, 128, 256 qui satisfait la résolution pour
**tous** les événements attendus dans ce contexte. La plage API 64–256 d'ADR-060 demeure
valide ; les trois valeurs sont les profils mesurés, pas une restriction nouvelle du type.

La relation existante donne une estimation initiale :

`N ≥ (2/π) · (hi − lo) · (R + c_g,max · T)`.

R et T sont le domaine et l'horizon numériques communs, T depuis la naissance (ADR-066).
La construction effective fait foi aux frontières arrondies et pour les autres refus.
Ne pas augmenter N pour masquer `Regime`, `Reach`, `Steepness` ou un défaut de source.
Si aucun profil ne couvre le domaine, le refus reste explicite ; réduire la portée ou
l'horizon constitue un autre contrat, jamais un repli silencieux.

Un profil admissible doit ensuite recevoir sa **précision physique** et son coût au niveau
du cycle complet. Le présent ADR autorise le dimensionnement des prochains bancs ; il ne
reçoit pas les neuf cas géométriques comme eau utilisable en jeu.

## Identité, mémoire et application

Sur 1792 composantes du même montage, 1495 changent en bits entre N64 et N256 malgré des
écarts absolus faibles. Un même service autoritaire doit donc partager N entre participants,
comme ses autres paramètres numériques (I-03/I-15). Aucune adaptation de N à la charge
locale n'est autorisée par cette décision. WLIV V1 encode et vérifie déjà N (ADR-068) ;
une restauration dans un autre N reste refusée. Aucun codec ne change.

I-06/I-16 : dimensionner les octets des pools au démarrage, N étant la taille du tableau
de nœuds effectivement alloué. Déduire les emplacements possibles des allocations et du
coût mesuré ; ne pas ajouter une capacité gameplay arbitraire. Le changement de profil
dans un service vivant ou un pool hétérogène demanderait une construction distincte.

Application immédiate : défaut N64 maintenu et contrat documenté sur les deux points
d'entrée du code. Aucun algorithme de sélection dynamique ajouté, aucune sortie historique
déplacée. I-03, I-05, I-06, I-08, I-14, I-15, I-16 relus ; aucun invariant amendé.

## Ce qui reste ouvert

- **S125-1 / A203 :** recevoir le champ à portée étendue contre une référence indépendante,
  sur toutes ses composantes et près des frontières spatiale et temporelle ; commencer
  par N128/R64 et N256/R128, λ4/horizon4 s, avant l'intégration au cycle mixte.
  Comparer à une quadrature spectrale raffinée et une référence de Bessel indépendante,
  avec contrôle de convergence de l'oracle. Les sorties finies de S125 ne suffisent pas.
- Générateur physique d'ADR-055 et calibration B2, profondeur finie, budget cible et coût
  du cycle complet étendu, admission dynamique mixte, bilan mixte et durabilité disque.

> **Suivi S126 — 2026-09-09 :** S125-1 réalisée sur les deux fixtures prévues,
> [RECEPTION-ETENDUE-S126](../validation/RECEPTION-ETENDUE-S126.md). Précision des sept
> composantes reçue au critère annoncé, oracle raffiné ; A203 partielle. La réception
> d'un paquet transporté au loin reste distincte : A204/S126-1, rayon et horizon à
> dimensionner ensemble. La décision de profil ne change pas.

## Note corrective du 2026-09-10 (S138)

Cette décision renvoie **des paramètres de source** « à calibrer par B2 ». C'est faux : B2 choisit
la technologie de W et `λ_cut`, et aucune de ses métriques ne mesure ce qu'un objet qui entre dans
l'eau émet. La calibration de la source relève de **B10**, étendu en S137 de deux métriques à cet
effet. Voir [ADR-093](ADR-093-ou-se-calibre-la-source-d-impact.md).

*S137 avait corrigé trois ADR portant ce renvoi ; l'audit S138 en a trouvé six. Ce correctif-ci
est le résultat de la recherche que S137 n'avait pas faite.*
