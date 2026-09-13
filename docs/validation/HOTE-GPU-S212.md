# Sillage du cœur dans l'hôte GPU — S212, 2026-09-13

## Résultat et portée

`viewer/` affiche maintenant, en plus de la mer S201 et de l'impact S203, un **sillage prescrit
issu du cœur** : source `wake_source::Wake` (ADR-103) admise au journal de pression, préparée à
chaque image par `bound_pressure::Prepared::from_journal`, publiée par la méthode neuve
`render_components` et sommée par sommet sur GPU. **Exactitude reçue, coût refusé** : le GPU
reproduit le cœur à 0,09 mm près, mais ce chemin coûte 10,9 ms de CPU et jusqu'à 4,1 ms de GPU
par image contre 2 ms d'eau (ADR-125). **J1 reste partiel.** Aucun résultat GPU n'est autoritaire
(I-04, I-15). Aucun ADR ajouté ou modifié ; aucune dépendance ajoutée.

## Publication du cœur

`spectral_pressure::Field::render_components(origin, out)` et son enveloppe
`bound_pressure::Prepared::render_components(context, time, origin, out)` écrivent un
`[A, B, kx, ky]` par nœud du demi-spectre, avec

```
η(origine + q) ≈ Σ A cos(k·q) − B sin(k·q)        pente = −k (A sin(k·q) + B cos(k·q))
A + iB = poids · η̂(t) · exp(i k·origine)          phase k·origine repliée en PhaseQ32
```

Aucun temps ni aucune coordonnée absolue ne quitte le cœur (I-08) ; aucun atan2 (le cœur n'a pas
de libm). Mêmes contrôles de champ et d'instant que `sample_batch`, refus atomiques (contexte,
instant, capacité, origine non finie ou hors ±4096 m). Test `render_components_rebase_against_sample_s212`
: reconstruction contre `sample_batch` à 1e-5 de la norme L1 des coefficients, trois origines,
quatre décalages. **Témoin vérifié** : un signe de B inversé fait échouer le test.

Un seul `Wake` de huit tronçons suffit pour un trajet scripté : un tronçon futur rend une réponse
nulle (`ModalPressure::sample`). L'émetteur progressif (ADR-104) sert l'émission en jeu, pas ce banc.

## Fixture, déclarée avant mesure

Froude de S156 transporté par similitude (σ 2 m au lieu de 1 m, même coupure réduite σk = 6) :
σ 2 m, coupure 3 rad/m, recette **64×128** (4 096 nœuds du demi-spectre) ; huit tronçons de 2 s à
[3, 0] m/s sous 19 620 N, départ (−24, 4) m à la naissance de l'impact ; contexte 40 s ; emprise
[−64, −48]–[64, 56] m ; repère/cellule 0 ; g 9,81, ρ 1025. Le GPU applique la même emprise que le
cœur et n'ajoute rien au-dehors. Témoin de résolution : **128×256** (16 384 nœuds).

## Exactitude GPU contre cœur

Référence : B `eval` + impact direct + sillage `sample_batch`. 6 988 sondes : grille S211, centre
et bord de l'impact, et 424 sondes de couture à ±1 cm des quatre bords de l'emprise.

| Âge (s) | Impact | Sillage | Erreur max hauteur (mm) | RMS (mm) |
|---:|:---:|:---:|---:|---:|
| 0 | oui | oui (nul) | 0,077657 | 0,012456 |
| 4 | oui | oui | 0,072926 | 0,012588 |
| 8 | oui | oui | 0,070810 | 0,012341 |
| 16 | oui | oui | 0,072271 | 0,011743 |
| 24 | oui | oui | 0,077346 | 0,011031 |
| 39 | oui | oui | 0,075042 | 0,012520 |
| 40,01 | oui | non | 0,089370 | 0,012678 |
| 3, caméra (125,−330,12), fond 1e6 s | oui | oui | 0,064552 | 0,012394 |
| 8, témoin | non | non | 0,070959 | 0,012333 |

Pentes : erreur max 1,24e-4. Tolérance déclarée 3 mm : **reçue sur toutes les lignes**. L'erreur
reste celle de S211 : le sillage n'ajoute rien de mesurable à l'écart de sommation.

**Ce que cet accord ne dit pas.** Référence et image passent par la même préparation ; l'accord
reçoit la sommation GPU, pas la fidélité du sillage. Celle-ci ne se lit que dans le témoin.

## Fidélité de la recette, couture et admission — cœur seul

| Âge (s) | Amplitude max 128×256 (mm) | Écart 64×128 / 128×256 (mm) | Relatif | Couture 64×128 / 128×256 (mm) | Enveloppe de pente |
|---:|---:|---:|---:|---:|---:|
| 4 | 154,3 | 0,567 | 0,37 % | 0,53 / 0,18 | 0,1351 |
| 8 | 117,0 | 1,355 | 1,2 % | 1,37 / 0,59 | 0,1449 |
| 16 | 132,8 | 2,180 | 1,6 % | 2,28 / 2,00 | 0,1650 |
| 24 | 45,4 | 4,324 | 9,5 % | 5,62 / 3,35 | 0,1570 |
| 39 | 31,5 | 8,768 | 28 % | 12,70 / 14,48 | 0,1580 |

- **La recette 64×128 est honnête pendant le forçage et le décroche après** : sous 2 % jusqu'à
  16 s, 9,5 % à 24 s, 28 % à 39 s. Cohérent avec la borne radiale d'ADR-107 transportée par
  similitude (radial 64 décrochait entre 15 et 20 s à σ 1 m, soit ~21–28 s à σ 2 m) — **cohérent,
  pas reçu** : 128×256 n'est pas une référence convergée, seulement plus fine (S156 §3).
- **La couture croît avec l'âge, et elle est physique** : à 39 s la recette fine porte 14,5 mm au
  bord, plus que la grossière. Ce sont des ondes longues et rapides qui atteignent l'emprise, pas
  seulement un retour par récurrence. Au-delà de 24 s la marche au bord dépasse les 3 mm de S201.
- **Admission** : `bound_pressure::Prepared::sample_world_batch` à `BREAKING_SLOPE` admet B +
  sillage aux cinq âges (enveloppe ≤ 0,165 contre 0,449). **Non exercé** : la composition mixte
  impact + pression (`mixed_water`), où le budget conjoint est une somme (ADR-119).

Le contexte de 40 s de la fixture excède donc la durée honnête de sa recette et la tenue de son
emprise. Aucun garde ne le refuse, conformément à ADR-107 (A214) ; c'est consigné en **A251**.

## Coût local

Windows x86_64, Rust 1.97, release, RTX 5070 Laptop, DX12. `--verify` : 10 images de mise en
régime puis 120 échantillons. CPU sillage = `from_journal` + `render_components`, inclus dans le
CPU total. GPU = passe d'eau seule, comme S211.

| Nœuds | Format / grille | CPU sillage médiane / max (ms) | CPU total médiane (ms) | GPU eau médiane / p95 / max (ms) |
|---:|---|---:|---:|---:|
| 0 (S211) | 640×360 / 321×181 | — | 0,470 | 0,018 / 0,021 / 0,021 |
| 0 (S211) | 960×540 / 481×271 | — | 0,461 | 0,049 / 0,053 / 0,059 |
| 4 096 | 640×360 / 321×181 | 10,851 / 27,313 | 11,681 | 1,902 / 1,912 / 1,913 |
| 4 096 | 960×540 / 481×271 | 10,577 / 14,328 | 11,533 | 4,103 / 4,123 / 4,143 |
| 16 384 | 640×360 / 321×181 | 38,376 / 46,308 | 39,877 | 7,770 / 9,651 / 15,852 |
| 16 384 | 960×540 / 481×271 | 41,504 / 62,042 | 42,712 | 16,946 / 21,526 / 27,973 |

Deux lois, chacune linéaire, et elles ne portent pas sur la même variable :

- **GPU ∝ sommets × nœuds** : 7,6–7,9 ps par produit, stable sur deux formats et deux recettes.
- **CPU ∝ nœuds × tronçons** : 317–331 ns par produit, indépendant du format. C'est la préparation
  modale complète refaite à chaque image, que rien n'impose : dans la solution de Duhamel
  d'ADR-069, le temps n'entre que par des phases, et un tronçon achevé ne fait plus que tourner.

## Verdict

1. **Le chemin « préparer à chaque image, sommer par sommet » est reçu en exactitude et refusé en
   coût.** À la recette la plus grossière qui tienne 2 % pendant le forçage, la préparation CPU
   seule dépasse cinq fois le budget eau de 2 ms ; la passe GPU l'atteint à 640×360 (1,90 ms) et le
   double à 960×540 (4,10 ms) — un seul sillage, aucune marge pour un second.
   Un sillage de jeu plus long exige plus de nœuds (ADR-107), donc ce chemin empire.
2. **Incompatibilité mesurée, pas encore un arbitrage** (ADR-127 D7) : deux leviers techniques
   existent, chacun sur l'une des deux lois, aucun n'est mesuré, et aucun ne retire de fonction.
   - *Temps* (cœur) : ne plus préparer par image. Les tronçons achevés se réduisent, par nœud, à un
     état tourné de `ω·(t − t_réf)` ; seul le tronçon actif garde la forme fermée de Duhamel,
     préconstruite à l'admission. Coût attendu par image : une rotation par nœud plus une
     évaluation active — **prédiction à mesurer**, pas un chiffre reçu.
   - *Espace* (hôte) : ne plus sommer par sommet. Évaluer le sillage sur une grille locale dont le
     coût ne dépend pas du nombre de sommets. Une somme polaire par texel ne gagne rien
     (texels × nœuds) ; une **grille cartésienne de nœuds et une transformée** rend le coût
     proportionnel à la taille de grille. Change la quadrature du sillage d'image : exige sa propre
     réception contre le cœur, à la manière d'ADR-129.
   Si les deux leviers mesurés ne tiennent pas 2 ms, l'arbitrage revient à l'utilisateur.

## Image, fenêtre et contrôles

`--verify` écrit `captures/s212/scene.ppm` et `background.ppm` (640×360, âge 8 s, caméra S201),
non versionnées. **60 182 pixels différents**, écart max 28 niveaux, lignes 161–359 ; l'image de
différence amplifiée montre les anneaux d'impact et le motif de Kelvin derrière la source. À l'œil
sans différence, le sillage est discret dans la houle Hs 1,5 m, comme l'impact en S211.
`--smoke` : fenêtre ouverte, 120 images, fermeture code 0. Interaction manuelle non rejouée ;
R relance impact et sillage, B masque les deux.

Suite complète du workspace `code/` hors réseau : voir le journal S212 (décompte non recopié ici).

## Suite portée

**S213, file J1 / W : levier temporel dans le cœur** — représenter la réponse d'un sillage admis de
façon que l'image ne refasse pas la préparation modale ; recevoir contre `from_journal` et mesurer
le coût par image. Puis le levier spatial dans l'hôte, puis la composition mixte impact + sillage
et les coutures du sillage (A251). J2/δ général et V-noyau restent dus selon
[la feuille de route](../FEUILLE-DE-ROUTE.md).
