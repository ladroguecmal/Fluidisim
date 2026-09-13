# Premier hôte GPU B + impact — S211, 2026-09-13

## Résultat et portée

`viewer/` ouvre une fenêtre locale winit/wgpu avec la mer S201 et l'impact S203/S205,
caméra mobile, pause, relance et témoin sans impact. **J1 reste partiel : sillage absent**,
cadence interactive complète non instrumentée. Aucun résultat GPU n'est autoritaire.
Le lancement et les commandes sont dans [viewer/README](../../viewer/README.md).

Accord explicite « oui » aux 254 sources inventoriées S210 ; récupération dans le cache
Cargo, verrou conservé sans modification, aucun vendoring. Compilation hors réseau reçue.
`code/` conserve zéro dépendance externe. Aucun ADR ajouté ou changé.

## Construction et précision

`Background::render_components` publie amplitude, kx/ky et phase repliée à l'origine caméra,
dans un tableau fourni par l'hôte. Refus atomiques, aucune allocation dans cette méthode.
Le temps entier est replié **avant** conversion ; aucun temps absolu f32 envoyé au GPU.
L'essai CPU couvre 15 s, 1e6 s et u64::MAX, deux origines et les refus domaine/capacité.

L'impact est le champ radial N256, R52 m, horizon56 s, entrée de demi-largeur1 m et vitesse8 m/s,
fraction transmise0,005, sur Hs1,5 m/Tp6 s, 32 composantes, graine201. Profil `RadialTable`
au pas λ/16, interpolation Hermite de hauteur et pente. Une même fonction WGSL sert au
calcul des sommets et au contrôle compute. Référence : `Background::eval` et impact radial
**direct**, sans interpolation. La relance R rejoue ce champ à sa position fixe.

Tolérance déclarée avant les essais : **3 mm** de hauteur (marche S201), pente mesurée.
Chaque ligne porte 6 564 sondes : grille étendue, centre, rayon exact et extérieur.

| Âge impact (s) | Impact actif | Temps fond (s) | Erreur max hauteur (mm) | RMS hauteur (mm) |
|---:|:---:|---:|---:|---:|
| 0 | oui | 12 | 0,077657 | 0,012406 |
| 1 | oui | 13 | 0,064611 | 0,012429 |
| 3 | oui | 15 | 0,072777 | 0,012346 |
| 6 | oui | 18 | 0,069529 | 0,012174 |
| 56 | oui | 68 | 0,071913 | 0,012978 |
| 56,01 | non | 68,01 | 0,072032 | 0,012982 |
| 3, caméra (125,−330,12) m | oui | 1 000 012 | 0,065058 | 0,012179 |
| 3, témoin | non | 15 | 0,072777 | 0,012344 |

Maximum des erreurs de pente : 0,000123650, sans seuil de réception nouveau.
Hauteur reçue sur ce domaine de sondes. **Ce résultat ne remplace pas les coutures S203/S208**,
la réception physique, ni un contrôle du maillage à toutes les poses de caméra.

## Coût local

Windows x86_64, Rust1.97, release, NVIDIA GeForce RTX5070 Laptop GPU, backend DX12.
Commande depuis la racine :

```powershell
cargo run --release --offline --locked --manifest-path viewer/Cargo.toml -- --verify
```

10 images de mise en régime, puis120 échantillons, attente GPU entre images.
Mesures finales après ajout du second format et refus des horodatages nuls/inversés :

| Format / grille | CPU préparation-transfert-soumission médiane / max (ms) | GPU eau médiane / p95 / max (ms) |
|---|---:|---:|
| 640×360 / 321×181 | 0,470200 / 2,109700 | 0,018304 / 0,020512 / 0,021472 |
| 960×540 / 481×271 | 0,460600 / 1,953400 | 0,048576 / 0,052672 / 0,059008 |

Horodatage GPU entre début et fin de **la passe d'eau** ; ciel, transfert, lecture et présentation
exclus. CPU mesuré avant attente de lecture. On ne somme pas ces chiffres pour annoncer une
cadence : **60 images/s et coût complet de l'eau à 2 ms ne sont pas reçus**. La pointe CPU
640×360 dépasse2 ms ; aucun budget de production garanti. Une seule machine testée.

La sélection multibackend initiale s'est arrêtée nativement avec 0xc0000005 pendant la création
de l'instance, avant obtention d'un adaptateur. DX12 seul réussit. Windows utilise donc DX12 ;
la cause pilote/backend exacte n'est pas identifiée. Les autres plateformes gardent la sélection
par défaut, non testée. Pas de nouvelle dépendance introduite par ce correctif.

## Image, fenêtre et contrôles

Deux images locales reproductibles par `--verify` : `captures/s211/impact.ppm` et
`background.ppm`, 640×360, instant+3 s, caméra S201. PNG de prévisualisation générés localement
et inspectés ; aucun binaire versionné. Mer lisible, impact discret dans la houle, horizon continu
à cette pose. Grille projetée à pas nominal2 px, surbalayage18 %, distance de projection plafonnée
à1500 m ; aspect visuel de banc (ciel, Fresnel, soleil, brouillard), sans réception photoréaliste.

Fenêtre réellement ouverte, pause et retour Home observés, témoin B exercé, déplacement flèche
et PageUp exercés, agrandissement observé sans erreur, fermeture Échap avec code0.
Rotation par clic droit implémentée mais non exercée par l'outil de contrôle ; les poses extrêmes
et le changement de DPI restent à éprouver. `--smoke` est disponible, non exécuté séparément.

Suite CPU complète hors réseau : **349 réussis, 5 ignorés, aucun échec**
(251+4+1+93 ; ignorés2+3). Compilation release du viewer et contrôle GPU réussis.
Les buffers de données sont réutilisés, mais les allocations de toute la pile graphique
et les recréations au redimensionnement ne satisfont pas encore une réception I-06 de production.

## Suite portée

S212, file J1 / W : **intégrer un sillage issu du cœur dans l'hôte**, comparer au CPU,
puis mesurer le coût de la scène complète. Conserver le contrôle des angles rasants, de la
caméra et de la cadence dans les critères de sortie de J1. J2/δ général et ouverture de V
au plus tard avec J2 restent dus selon [la feuille de route](../FEUILLE-DE-ROUTE.md).
