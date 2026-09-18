# Cuisson du sillage — S267

## Contrat avant construction

Consommateur : cuisson de la grille de l'hôte, avant chaque rendu, mêmes huit bandes
ADR-148 et reconstruction S234. La boucle additionne chaque mode dans un tableau
local indexé dynamiquement. Hypothèse à mesurer : huit accumulateurs nommés avec
sélection explicite évitent un coût d'adressage/spill du compilateur. Ce mécanisme
n'est pas tenu pour acquis sans mesure. Aucun changement de fréquence, de modes,
de pas de grille ni d'ordre des sommes à l'intérieur d'une bande.

Critères : témoins conservés ; sept images S266 identiques au bit ; comparaison des
neuf grilles GPU au bit aux âges 1/3/12/16 s, avec/sans filtre, poses référence et
rasante, et retour à un instant déjà évalué ; tests hôte et réception spectrale S249
(3 mm) conservés. Coût : secteur début/fin, aucune autre charge GPU du projet,
1280×720, vent 5, reflets filtrés, 120 images après dix de chauffe, référence/rasante.
Retenir si réduction médiane de cuisson >=25 % et eau totale >=10 % aux deux poses.
Ces seuils de rentabilité sont des choix de lot, pas des constantes physiques.
Si l'identité échoue : diagnostiquer, ne pas assouplir le seuil. Si le coût échoue :
rejeter cette variante et déclarer la nouvelle piste avant construction.

I-09 : aucun mélange temporel, réalisation toujours évaluée à l'instant demandé.
I-03 : cœur et ordre modal inchangés ; GPU cosmétique local, pas de réception
multiplateforme. I-06 : mêmes buffers réservés, pipeline créé à l'initialisation.

Comparaison à J2 : les bords ouverts couplés débloquent une capacité physique absente.
Le présent lot se borne au poste dominant du rendu accepté encore au-dessus de 2 ms ;
une fois ce poste traité ou la piste rejetée, revenir à J2 avant nouvelle calibration
cosmétique. C'est un changement de poste, pas un troisième affinage des reflets.


## Réception et intégration — 2026-09-18

**Retenue par défaut**, `bake_named` dans la fenêtre, les captures et les sondes.
Le shader historique `bake` est conservé via `--sillage-cuisson-directe`.
Même boucle modale, mêmes huit bandes, même ordre de sommation et même grille.
Le gain de l'écriture explicite est mesuré ; l'attribution précise aux registres,
à l'adressage ou aux spills n'a pas été profilée. Aucun nouvel ADR : simple
implémentation équivalente d'ADR-148, sans changement de contrat ni d'invariant.

### Précision et invariance

- `--sillage-cuisson-verify` : 20 comparaisons, **3 514 240 flottants au bit**, toutes
  valeurs finies. Âges 1/3/12/16/1 s, référence/rasante, filtre actif/inactif. Retour
  temporel exact dans les quatre configurations ; quatre dérivées par nœud, les
  huit bandes et leur total vérifiés lorsque le filtre est actif.
- Sept PPM `r9_final_*` **identiques intégralement** aux `r8_final_*` S266 : référence,
  fond seul, haute, plongeante, rasante, impact proche, large horizon. Aucun nouveau
  verdict perceptif requis sur une image inchangée. Le témoin à 29 s demeure hors
  de l'horizon honnête de sa recette (18,53 s), avertissement conservé.
- `--multi --spectral-verify` : 36 cas historiques S249, erreur maximale de hauteur
  **0,339985 mm**, inférieure aux 3 mm acquis. Pentes max 0,000939958 / 0,000224651.
  Huit modes isolés : conservation proche et rejet Nyquist exacts ; retour caméra au bit.
- Tests hôte release hors ligne verrouillés : **19 réussis, 1 ignoré, 0 échec**.
  Cœur inchangé : sa suite complète n'est pas rejouée. Deux avertissements de
  compilation préexistants (paramètre `frame`, fonction `footprint`) inchangés.

### Coût et domaine de validité

Windows, RTX 5070 Laptop, DX12, alimentation secteur 99 % constatée avant/après les
**deux passages**, sans autre travail GPU du projet. 1280×720, grille projetée
641×361, trois sillages/4 096 modes, grille locale au pas de 1,125 m, âge initial
12 s puis 130 images à 60 Hz dont dix de chauffe. Mer à 5 m/s, CWM, modulation 2,
queue, reflets filtrés 3×3 optimisés S266, ciel clair, visibilité et filtrage spectral.
Pas de FFT, de LOD temporel ni de parallélisme CPU persistant.

Premier passage, médianes en ms :

| pose | eau témoin | eau retenue | cuisson témoin | cuisson retenue | p95 eau retenue | max eau retenue |
|---|---:|---:|---:|---:|---:|---:|
| référence | 2,264800 | 1,744448 | 1,083264 | 0,584992 | 1,786144 | 2,961696 |
| rasante | 2,237184 | 1,737184 | 1,063872 | 0,573600 | 1,772480 | 1,794752 |

Gain : **46,00 / 46,08 % en cuisson**, **22,98 / 22,35 % en eau totale**. Les deux
critères sont tenus. Une pointe dépasse 2 ms en référence, ainsi que le témoin
(4,222368 ms) ; sa cause n'est pas attribuée, elle n'est pas effacée.

Un second passage est justifié par cette pointe et confirme les médianes :

| pose | eau témoin | eau retenue | cuisson témoin | cuisson retenue | p95 eau retenue | max eau retenue |
|---|---:|---:|---:|---:|---:|---:|
| référence | 2,266016 | 1,742112 | 1,086272 | 0,583840 | 1,771744 | 1,835520 |
| rasante | 2,242560 | 1,737856 | 1,066240 | 0,574304 | 1,785408 | 1,797952 |

CPU préparation/transfert/soumission : médianes 4,108 / 4,167 ms, maxima
26,233 / 26,292 ms au second passage, préparation du sillage incluse. **Aucun gain
CPU ni budget global de 2 ms reçu.** La médiane GPU est sous 2 ms ; la borne sur
chaque image et le coût CPU restent ouverts (A265, A278). Les chronomètres GPU
excluent ciel, transfert, lecture et présentation, comme les preuves précédentes.

`allocations_update_max=0` aux huit mesures. Aucun nouveau buffer GPU permanent,
aucun nouveau stockage de champ par image ; un pipeline supplémentaire initialisé
au démarrage pour garder le témoin. Les grilles restent à 2,25 Mio. La lecture
comparative alloue uniquement dans le banc hors ligne, pas dans la boucle d'image.
La pile verrouillée conserve son reçu S240 : pas de nouvelle campagne de fenêtre.

### Reproduction

Depuis `viewer/`, options communes pour ce rendu :
`--multi --vagues --modulation --ciel-clair --vent=5 --reflets-filtres`.

- `--sillage-cuisson-verify` : identité des grilles et retour temporel.
- `--sillage-cuisson-bench` : témoin/optimisé aux deux poses.
- `--revue=r9_final` : sept captures locales, dossier `captures/s267`.
- `--sillage-cuisson-directe` : témoin S266, y compris en fenêtre.
- `--multi --spectral-verify` seul : réception spectrale historique sans CWM.

Journaux locaux : `identite.log`, `images.log`, `spectral.log`, `named-bench.log`,
`confirmation-bench.log`, dans `viewer/captures/s267`.

Arrêt du lot : identité et rentabilité reçues, pas de nouvelle calibration cosmétique.
Suite prioritaire comparée : bords ouverts de J2 ; coût CPU et pointes restent dans
la file avec leurs déclencheurs, sans nouveau seuil ni baisse d'ambition.
