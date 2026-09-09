# S106 — Requête monde commune B + pression

2026-09-09. S105-1 réalisée pour le candidat linéaire local. Prolonge la composition
ADR-062 et la requête commune ADR-065, sans modifier les décisions de ces ADR.

## Chemin construit

`bound_pressure::Prepared::sample_world_batch` reçoit le fond associé à un référentiel
et une cellule (`BoundBackground`), le contexte complet, l'instant, les points monde,
le plafond de pente et deux tampons hôte. Elle contrôle contexte, gravité et instant
avant les points, y compris pour un lot vide. Le fond B actuel emploie g=9,81 : une
pression préparée avec une autre gravité est refusée sur ce chemin de composition.

Chaque point est converti une seule fois par rapport à l'ancre entière de B. Cette
même position locale sert au fond et à la pression. `Background::eval_local` est une
primitive interne issue du corps de `eval`, avec les mêmes opérations ; le chemin
monde historique la délègue après conversion. Le déplacement vertical du point est
contrôlé pour la représentabilité, mais les vitesses restent celles de la surface,
comme dans le fond et le candidat existants : pas de profil vertical ajouté.

La sortie additionne élévation, dérivée temporelle, deux vitesses horizontales et
vitesse verticale. Les pentes sont additionnées avant normalisation de la normale.
L'aération de B est conservée ; aucune aération dérivée de la pression n'est inventée.
Le potentiel n'a pas de champ dans WaterSample ; il reste accessible dans la requête
locale du candidat. Ses bilans énergie/puissance restent ceux de la pression seule,
ils ne deviennent pas un bilan énergétique de B+pression.

## Contrôle de pente

Pour chaque coefficient complexe total q et poids spectral w, à l'instant préparé :

    enveloppe_W = Σ (|w*kx| + |w*ky|) * (|Re q| + |Im q|)
    enveloppe_BW = pi * steepness_B + enveloppe_W

L'inégalité triangulaire et |sin|,|cos|≤1 majorent la norme euclidienne de la pente
par cette somme L1 dans l'arithmétique exacte du spectre discret. Les coefficients
sont ceux du champ total : les interférences entre segments sont déjà conservées.
La somme est calculée une fois à la préparation et refuse un résultat non fini.
L'enveloppe B reprend le contrat de composition ADR-062. Le plafond est fourni par
l'hôte en pente sans unité ; aucun seuil universel ni calibration ajouté.

**Limites de cette enveloppe :** évaluation f32 sans arrondi dirigé, conservatrice
algébriquement mais pas certificat formel des erreurs flottantes ; ni borne sur la
solution continue ni réception universelle de résolution. Elle peut refuser des
champs dont les pentes locales sont faibles. Le test d'annulation le vérifie : choisir
un plafond entre pente locale et enveloppe doit refuser, même si le point est doux.
WaterSample.steepness transporte enveloppe_BW/pi, conformément au chemin B+impacts
existant ; ce n'est pas la pente mesurée au point.

## Publication et réception

Le scratch peut être modifié ; la sortie n'est copiée qu'après réussite de tous les
points. Refus indexés des points hors emprise, conversion extrême ou fond invalide ;
refus explicites du contexte, temps, capacité et plafond de pente. Préfixe publié,
queue inchangée. Aucun nouvel appel d'allocation par inspection ; pas de chronométrie.

Trois tests supplémentaires sur recette8×8 de contrat et un fond à une composante :

- Sept composantes physiques (élévation, dérivée, trois vitesses, deux pentes via la
  normale) comparées à la composition séparée ; normale unitaire, aération conservée.
  Identité locale de B avant/après extraction de la primitive. Translation entière
  près de i64::MAX : tous les bits de sortie identiques aux points autour de zéro.
- Dernier point hors emprise ou extrême, mauvais référentiel/cellule/temps/gravité,
  scratch insuffisant, fond NaN et plafond invalide : refus ; sorties conservées.
  Nouvelle requête valide acceptée et queue de sortie intacte.
- Normale confrontée aux différences spatiales de l'élévation, pas réel après
  quantification des points monde ; tolérance2e-5 sur cette fixture. Plafond entre
  pente ponctuelle et enveloppe : refus explicite, pas de faux succès par annulation.

Les calculs spectral/modal et les recettes ne changent pas ; ces tests reçoivent la
composition, pas la précision physique d'une nouvelle résolution ni d'un profil de coque.

## Suite

**S106-1, S107 :** exécuter et mesurer un scénario hôte B+pression sur le virage reçu
128²/112×80, préparation à plusieurs instants et lots64, contrôles de refus/reprise,
référence directe et coût complet. Ne pas revendiquer le coût S103/S104 pour ce chemin.
Le raccordement aux impacts existants, les causes de pression répliquées, le journal,
LiveWater et la sauvegarde des sillages restent ouverts. L'association géométrique
hôte reste déclarative ; pas de rotation ni de milieu variable dans cette tranche.
Suite debug complète :132 core +93 harnais =225 réussis, cinq ignorés ; trois tests
ciblés aussi réussis en release. Quatre avertissements préexistants.
 73 ADR,193 angles,17 invariants,6 spécifications,23 cas inchangés.

**Mise à jour S107, 2026-09-09 :** S106-1 réalisée sur fixture ; voir
[CYCLE-PRESSION-S107](CYCLE-PRESSION-S107.md). Suite S107-1 : source versionnée et codec.
