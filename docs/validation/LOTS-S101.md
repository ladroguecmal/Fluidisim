# S101 — Lot atomique, optimisation par tuiles rejetée

2026-09-08. S100-1 instruite : API de lot construite, accélération non retenue.

`Field::sample_batch` prend points, scratch hôte et sortie. Les capacités et tous les
points sont contrôlés avant calcul. Le scratch reçoit les résultats ; seule une réussite
intégrale copie son préfixe vers la sortie. Erreur de phase ou résultat non fini : sortie
inchangée, scratch potentiellement modifié. Un lot vide ne modifie rien. Les queues des
buffers sont conservées. Aucun chevauchement mutable permis par les emprunts Rust sûrs.

Le calcul élémentaire a été extrait dans Slot::accumulate, sans changement d'opérations.
Le premier candidat parcourait huit points par tuile, modes en boucle extérieure. Il
conservait les sommes par point mais s'est montré plus lent. Il est retiré : le lot final
appelle le scalaire reçu puis publie atomiquement. L'API ne promet aucun gain de vitesse.

## Mesures locales

Protocole S98, demi-spectre, même machine, 3 échauffements puis 21 mesures release.
Médianes en microsecondes ; séries successives, pas une comparaison statistique entrelacée.

| Essai | Scalaire 64 | Lot 64 | Scalaire 121 | Lot 121 |
|---|---:|---:|---:|---:|
| Tuiles de huit, rejetées | 20590,9 | 24872,1 | 41212,7 | 60916,4 |
| Parcours final scalaire | 20299,7 | 20050,0 | 41548,2 | 38682,8 |

Le lot final reste du même ordre de coût ; les écarts locaux ne démontrent pas une
accélération. Un point : 281,7 µs scalaire, 355,9 µs lot. Scratch supplémentaire de
28 octets par point, soit 3388 octets pour 121 points. Champs inchangés (40 octets/mode).

Hashes scalaires et lots identiques : b435322317c15b62 (1), ceaa83d65bd3a69b (64),
f1d889f97488bc37 (121). Oracle f64 complet hors chronométrage : écart maximal 5,478e-9,
sous les seuils reçus. Aucun changement de recette ni de modèle.

Test dédié : tailles 0/1/7/8/9/64/121, comparaison des sept composantes en bits ; queues,
capacités insuffisantes, NaN au dernier point, refus de phase après un point valide,
résultat interne non fini et reprise valide. La sortie reste inchangée dans chaque refus.

## Suite

S100-1 close comme expérience : contrat de lot réalisé, gain par tuiles rejeté. Le coût
reste élevé. **S101-1, S102 :** mesurer séparément la conversion spatiale de phase et
l'évaluation sinus/cosinus ; rechercher une évaluation conjointe qui partage la réduction
d'angle, conserver les hashes existants ou recevoir explicitement tout changement.
Conformité interplateforme, budget cible, puissance/travail et LiveWater restent ouverts.

**Actualisation S102 :** S101-1 réalisée, [TRIGONOMETRIE-S102](TRIGONOMETRIE-S102.md). Gain isolé sans gain global établi. Suite S102-1 : résolution et emprise.
