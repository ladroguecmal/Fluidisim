# La pluie dans V, exposition dynamique — S378

2026-09-26. Liste **5.5** (pluie selon l'exposition au ciel), *absente* jusqu'ici. Décisions :
[ADR-204](../adr/ADR-204-la-pluie-arete-de-v.md), au service d'[ADR-202](../adr/ADR-202-niveau-de-detail-des-contenants.md)
D3 et d'[ADR-203](../adr/ADR-203-reponses-aux-zones-d-ombre-d-adr-202.md) D2 (bâche entière ou demi posée et retirée en temps
réel). La météo elle-même n'est pas construite (à la fin, ADR-197 D5) : ceci est l'entrée que V en recevra. Référence CPU du
cœur, arithmétique entière.

## Reproduire

- Commit `66b7ded9` (P4 de S378) ou plus récent.
- `cargo test --manifest-path code/Cargo.toml --release --offline -p water-core --lib s378 -- --nocapture` — une trentaine
  de secondes de construction, 0,02 s d'essais ; lignes `PLUIE_S378` : `gagne_ml=319999 attendu_ml=320000`, `159999 / 160000`,
  `0 / 0`, `159999 / 160000` (bâche posée à 30 min), `239999 / 240000` (demi-bâche posée à 30 min) ; `bilan : entre=62221 ml
  sorti=528983 ml` ; `debordement charge_mm=0.8560 attendue_mm=0.8569 ecart=-0.11% debit_deversoir=1.7778e-4
  pluie=1.7778e-4`.
- Le reste de V : `… --lib hydro` (l'empreinte des trajectoires de S372, inchangée) ; la suite du cœur, 543 réussis.

## En une phrase

La pluie entre dans V par une arête du ciel vers le contenant — surface d'ouverture × exposition × intensité —, au
millilitre près sur une heure ; une bâche ou une demi-bâche, posée ou retirée à tout instant, en est la commande
sauvegardée ; et une piscine à débordement sous la pluie déverse exactement la pluie, à la charge analytique près de 0,11 %.

## 1. La construction

`Flow::Rain { catchment_mm2 }` : `from` et `to` désignent le nœud qui reçoit ; débit `intensité (mm/h) × ouverture (mm²)
× exposition` (la commande, 0 à 1 000) ; même quantification à report de reste et même borne de place libre que les autres
arêtes ; la normalisation par nœud amont l'ignore, l'application ne retire rien. `Meteo { pluie_mm_h }` entre par
`step_meteo` ; `step` est `step_meteo(Meteo::SEC)`. Étiquette 3 dans l'empreinte de la base de sauvegarde ; l'exposition
passe par la liste des commandes de WVST v2, sans format nouveau.

## 2. Les critères, écrits avant

| critère | mesure | |
|---|---|---|
| 1 — sans pluie, trajectoires existantes au bit | empreinte de S372 (quatre montages) | **inchangée** ; 45 essais V |
| 2 — une heure à 10 mm/h sur 32 m² | ouverte / demi-bâche / bâche | **319 999 / 159 999 / 0 ml** pour 320 000 / 160 000 / 0 (±1) |
| 2 | bâche entière posée à 30 min ; demi-bâche posée à 30 min | **159 999 / 239 999 ml** pour 160 000 / 240 000 |
| 3 — piscine à débordement, 20 mm/h, deux heures, pompe arrêtée | charge sur le seuil ; débit du déversoir | **0,8560 mm** pour 0,8569 (−0,11 %) ; **1,7778·10⁻⁴ m³/s = la pluie** |
| 4 — bilan, capacité, refus, sauvegarde | pluie entrée = gagné + rejeté, à chaque pas de 20 000 ; contenant plein ; intensité −1, NaN, ∞ ; `to ≠ from` ; demi-bâche | exact ; fermé ; `Domain` ; `Capacity` ; restaurée |

**Une erreur du critère écrit**, pas de la mesure : le plan donnait 240 000 ml pour une bâche **entière** posée à 30 min ;
c'est la valeur d'une **demi**-bâche (160 000 + 80 000). La bâche entière donne 160 000. Les deux cas sont éprouvés, et
l'essai le dit. Le millilitre manquant de chaque cas est la troncature du nanolitre à chaque pas (moins de 0,04 ml sur une
heure) plus le reste qui n'a pas franchi le millilitre.

## 3. Limites

- **La moitié de 5.5** : l'absorption par le sol, la pluie hors contenant (flaques, ADR-010 §5) ne sont pas faites.
- **Une intensité pour tout le réseau** ; une intensité par nœud viendra avec la météo, si elle le demande.
- **L'exposition est un nombre** : qui la calcule à partir des objets posés (lancer de rayons, occultants mobiles) est
  l'affaire de l'hôte et de la prévision (ADR-204 D3) — non construit.
- Ni neige, ni grêle, ni évaporation (ADR-203 D6) ; les rides de la pluie sont un effet de rendu, non fait (ADR-202 D3).
- Aucun consommateur encore : ni hôte, ni météo, ni scène ; déterminisme vérifié sur cette machine seulement.
