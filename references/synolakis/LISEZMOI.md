# Les mesures de Synolakis (1987) — l'onde solitaire sur la plage canonique

*Téléchargé le 2026-10-08 (S712), avec l'accord de l'utilisateur.*

- **Source** : le banc d'essai du NOAA Center for Tsunami Research, « Solitary wave on a canonical beach » :
  `https://nctr.pmel.noaa.gov/benchmark/Laboratory/Laboratory_CanonicalBathymetry/index.html`.
- **Le fichier** : `CanonicalBathymetry_H_0.03.xls` (27 648 octets). Malgré « 0.03 » dans son nom, c'est le cas **H/d = 0,3**, celui qui
  déferle (la page le légende ainsi). Il est converti en `feuille_1.csv` par Excel, en lecture seule : point-virgule pour séparateur,
  virgule décimale.
- **L'expérience** : le bassin de Caltech (31,73 m de long, 39,97 cm de large). Une pente de 1:19,85, dont le pied est à 14,95 m du batteur.
  Le déferlement a lieu dès que H/d dépasse 0,045 sur cette plage.
- **Les colonnes** : `x` et `Amp` aux instants `t = 15, 20, 25, 30`, en grandeurs sans dimension :
  - `x/d`, compté depuis le rivage au repos, positif vers le large ;
  - `η/d` ;
  - `t·√(g/d)`, l'instant 0 étant celui où l'onde est centrée en `X₁ = X₀ + L`. Ici `X₀ = 19,85` et `L = arccosh(√20)/γ`, avec
    `γ = √(3H/4d)`.

C'est une donnée, lue par les essais ; rien n'y est exécuté.
