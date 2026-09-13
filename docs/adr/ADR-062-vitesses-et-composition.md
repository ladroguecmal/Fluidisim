# ADR-062 — Les vitesses rendent la composition B+W cohérente

- **Statut : ACTÉE**, S79, 2026-09-08, délégation technique.
- **Applique** ADR-061 ; produit `composition.rs`, vitesse horizontale radiale et correction B.
- **Corrige** la vitesse verticale du B minimal, sans réécrire les décisions historiques.

## Défaut rencontré dans B

B utilise eta=a sin(kx-omega t), donc deta_dt=-a omega cos(kx-omega t). Il renvoyait pourtant
u_total.z=+a omega cos : opposé à la condition cinématique linéaire w=deta_dt.
Le contrôle orbital existant examinait la composante horizontale ; il ne protégeait pas ce signe.
Le code prend désormais le signe négatif. Un test estime la dérivée par différence centrée de
hauteur à ±1 ms sur trois instants ; l’ancien signe échouerait. Tolérance absolue d’essai 0,001 m/s.

La correction modifie les hashs de conformité, sans modifier eta ni la dispersion. Références
recalculées après contrôle physique puis enregistrées avant check : C02
`0x9babd7e12935c263` → `0x0a3a3bcc945db263`, C18
`0xd57d81f47d9f8611` → `0x85c8bc610f551d11`. Les deux scénarios check passent.
Ce changement n’est pas un assouplissement des contrôles physiques. A192 suit le défaut.

## Vitesse de W

Pour le potentiel radial de surface, grad_horizontal psi donne
u_r=sum c_i omega_i J1(k_i r) sin(phase_i), dirigée radialement.
Au centre, la vitesse horizontale est nulle par symétrie. La composante verticale linéaire est
deta_dt. Sample expose maintenant horizontal_velocity ; le support périodique de comparaison
la calcule également, sans laisser un zéro de repli pour cette nouvelle grandeur.

Les dérivées sont contrôlées par différence centrée du potentiel à ±1 mm et de la hauteur
à ±1 ms, trois instants ; écart absolu <2e-6 dans le scénario radial. Les quantifications de
phase et leurs limites de longue durée ne sont pas supprimées par ce contrôle ponctuel.

## Noyau ponctuel de composition

`compose` reçoit un WaterSample de B, le journal, les champs radiaux préconstruits dans l’ordre
server_seq et la requête locale. Il n’alloue pas, ne construit pas de champ pendant l’évaluation
et ne publie rien avant de réussir. L’hôte garantit que B et W décrivent le même point, instant,
plan d’eau, axes et milieu ; cette correspondance n’est pas encore portée par un WaterSystem.
Le test d’intégration appelle réellement Background::eval puis compose sur le point commun.

Un journal à perte connue est refusé. Absence de perte connue ne prouve toujours pas une livraison
réseau complète. Chaque champ doit correspondre **au contenu complet** de la confirmation
associée ; nombre différent, champ périmé, mauvais ordre ou source différente sont refusés.
Prédictions et rejets restent hors du chemin autoritaire. L’authentification et la réception
interplateforme restent des préalables hôte, pas une propriété créée par cette fonction.

Élévation, dérivée et vitesses s’additionnent. Les pentes de B se récupèrent par -n_xy/n_z ;
on ajoute celles de W, puis normalise (-pente_x,-pente_y,1). Ajouter des normales unitaires
serait faux. La borne d’ensemble vaut pi*B.steepness + somme des bornes spectrales W.
Elle doit respecter max_slope hôte, à calibrer B2. Cette interprétation de B.steepness est
celle du **B minimal actuel**, somme a*k/pi ; pas un contrat universel d’un autre fournisseur B.
La cambrure publiée est cette borne/pi. C’est une borne conservative, pas la cambrure mesurée
sur une crête particulière. Aération de B conservée : W n’en produit pas encore.

Même avant naissance, les bornes W restent incluses : refus conservateur possible, aucune
validité inventée. Aucun événement hors rayon ou hors temps ne devient de l’eau nulle.
La fonction reçoit B par valeur : un échec tardif ne modifie pas l’échantillon appelant.
Le code valide les entrées finies et la normale verticale positive, sans authentifier un
WaterSample fabriqué hors de Background. Il s’agit d’une intégration limitée, pas du service complet.

## Réception et suite

Cinq tests ajoutés : dérivée verticale B, dérivées radiales, composition physique et normale,
refus de correspondance/domaine/perte, intégration avec le vrai B. C02/C18 check repassés après
changement explicable des hashs. La physique C04 n’est pas déclarée corrigée par ce travail.

S78-1 réalisée comme noyau ponctuel ; WaterSystem, lots, index spatial et publication concurrents
restent ouverts. Prochaine construction : préparer les champs du journal dans une mémoire hôte
bornée et évaluer un lot avec statut explicite, sans construction par point ni sortie partielle.
Ce chemin permettra de mesurer le coût réel B+W. Les profils non reçus, plusieurs référentiels,
réseau et rétention S72-2 restent ouverts. Invariants I-01/I-03/I-06/I-08/I-11/I-14 relus, inchangés.

## Note corrective du 2026-09-10 (S139)

« Elle doit respecter max_slope hôte, à calibrer B2 » (§50) est faux deux fois : B2 ne mesure pas
de limite de pente (ADR-093), et cette limite **se dérive** au lieu de se calibrer — SPEC-001 §4,
`πH/λ` à `H/λ = 1/7`, soit 0,4488 ([ADR-094](ADR-094-d-ou-vient-la-limite-de-pente.md)).

La suite du §50 — « c'est une borne conservative, pas la cambrure mesurée sur une crête
particulière » — reste exacte, et le facteur est désormais chiffré : **1,7950713** pour la part
d'un impact radial. Mais la borne d'ensemble `π·B.steepness + Σ bornes W` **additionne une pente
exacte et des bornes L1 de facteurs différents** : aucun seuil unique ne peut être physiquement
juste pour tous ses termes tant qu'elle reste hétérogène. Voir
[PENTE-REELLE-S139](../validation/PENTE-REELLE-S139.md) §5.

## Note corrective du 2026-09-13 (S203)

« Une pente exacte » dans la note précédente est faux pour B : `π·B.steepness = Σ aᵢkᵢ` est une
borne L1, ce que §52 disait déjà (« borne conservative »). Sa marge sur la pente réelle est
≥ 1,061 et ≤ 1,443 sur la recette S201, et elle suffit à faire refuser chaque point d'une
composition sur une mer de Hs > 1,107 m (recette JONSWAP de S201, Tp 6 s). Voir
[IMPACT-W-S203](../validation/IMPACT-W-S203.md) §2 et **A245**.

## Note datée du 2026-09-13 (S205) — ADR-128

« La borne d'ensemble vaut pi*B.steepness + somme des bornes spectrales W. Elle doit respecter
max_slope hôte » est **remplacée** pour le refus : le budget ne somme plus que les perturbations
([ADR-128](ADR-128-le-budget-de-pente-borne-les-perturbations.md)). La cambrure publiée reste
cette borne d'ensemble divisée par π, B comprise, dans le même ordre ; aucun bit publié ne change
pour un lot que l'ancienne règle admettait.
