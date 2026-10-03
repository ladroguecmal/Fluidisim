# Les références vidéo — S471

2026-10-04. **Six vidéos fournies par l'utilisateur** pour le banc visuel ([ADR-216](../adr/ADR-216-le-banc-visuel.md)), mesurées dans la page
qui les lit (`outils/banc_visuel.js`) ; **seuls des nombres entrent ici** — ni image ni vidéo. Le propos de l'utilisateur est cité tel
qu'il l'a écrit ; ce que la vidéo montre et ce que nos scènes peuvent lui opposer sont de l'agent.

**Comment lire les mesures.** Par zone (un rectangle en fractions du cadre, `[x0, y0, x1, y1]`, y vers le bas), sur une image réduite à
160 à 240 pixels de large (rappelé dans chaque fiche) ; les grandeurs statiques sont la médiane sur les images. Luminances linéaires, **rapportées à la médiane
de la zone** — l'exposition et la balance des blancs d'une vidéo publiée sont inconnues. Les définitions : l'en-tête de
`outils/banc_visuel.py`.

**Reproduire.** Ouvrir la vidéo dans le navigateur ; coller `outils/banc_visuel.js` dans la page — sous YouTube, ses *Trusted Types*
demandent une politique : `trustedTypes.createPolicy('banc', {createScript: s => s})`, puis `eval(politique.createScript(texte))` ;
puis `BancVisuel.lancer(document.querySelector('video'), {zones, fps, debut, fin, facteur})` avec les réglages de la fiche, et relire
`window.__banc`. Les nombres de chaque fiche : `docs/validation/references-video/<V>-<id>.json`. L'égalité avec le calcul de nos
rendus : servir le dépôt (`python -m http.server`), ouvrir une page qui charge `/outils/banc_visuel.js`, `BancVisuel.images(urls,
zones, dt, facteur)` sur des captures, puis `python outils/banc_visuel.py --egalite=<json> --zones=… --dt=… --facteur=… <captures>`.

**Limites, dites.**
- Une vidéo publiée est compressée et étalonnée. Le débit servi change d'une lecture à l'autre (360 à 720 pixels de large) : la part
  haute fréquence en dépend.
- `ecume` ne voit rien dans une zone tout entière blanche, puisque la médiane y est blanche (V2, la zone `ecume`) : c'est `p50_absolu`
  et la teinte qui la disent.
- `periode_s` égale à la moitié de la durée signale une dérive (la caméra qui bouge), pas une vague.
- Les coupes en fondu d'un montage passent inaperçues (V3, V6).
- `anisotropie` mêle les rides et le dégradé d'ensemble de la zone (V1, 9,7 : le bleu qui fonce vers l'horizon).

**Reproductible, et sensible à la résolution.** V6 mesurée deux fois à la même résolution : valeurs identiques. Servie en 720 puis
en 360 pixels de large, la même vidéo varie selon la grandeur :

| sensibilité | grandeurs | écart entre 720 et 360 pixels |
|---|---|---:|
| robustes | `periode_s`, `p50_absolu`, `creux_BsurG`, `cretes_BsurG`, `p05` | moins de 4 % |
| moyennes | `renouvellement_clairs_par_s`, `p25`, `p75`, `cretes_BsurR`, `anisotropie`, `p95` | 4 à 11 % |
| fines | `p99`, `periode_nettete`, `creux_BsurR`, `hf_part`, `contraste`, `mouvement_par_s`, `claire` | 16 à 32 % |

**Règle (ADR-216)** : une comparaison se fait à **largeur réduite égale** — celle de la référence, rappelée dans chaque fiche — et
un écart plus petit que celui de ce tableau n'est pas un défaut.

## V1 — `8u1LGaJ2fyA`

<https://www.youtube.com/shorts/8u1LGaJ2fyA> — 9.0 s, servie en 480 × 854, mesurée à 15 images/s
de 0.0 à 9.0 s, réduite par 2.

**L'utilisateur** : *« Hauteur d'homme, vue de la plage vers la mer, distance de  l'eau est 2m, il ne s'agit pas d'un océan mais d'une mer car pas d'agitation, calme, pas de vents violents. »*

**Ce qu'elle montre** : Caméra à la main qui descend de la mer vers le sable ; le ciel n'occupe que 4 % du haut. Seule la mer lointaine reste dans le cadre tout du long : c'est la zone mesurée.

**Nos scènes** : **Aujourd'hui** : la mer B de `saut.tscn` jusqu'à l'horizon, vue à 1,7 m — la mer lointaine se compare (S472). Le rivage, non.

**Zones** : `mer_lointaine` [0.05, 0.06, 0.95, 0.3].

| grandeur | mer_lointaine |
|---|---:|
| p05 | 0,512 |
| p95 | 1,61 |
| p99 | 2,63 |
| contraste | 0,166 |
| creux_BsurG | 2,49 |
| creux_BsurR | 90,3 |
| cretes_BsurG | 2,06 |
| claire | 0 |
| ecume | 0 |
| hf_part | 0,451 |
| anisotropie | 9,7 |
| p50_absolu *(non comparable)* | 0,116 |
| mouvement_par_s | 0,436 |
| periode_s | 4,5 |
| periode_nettete | 420 |
| renouvellement_clairs_par_s | — |
| coupes | 0 |

## V2 — `ZKNXFgcVlrE`

<https://www.youtube.com/shorts/ZKNXFgcVlrE> — 8.5 s, servie en 480 × 854, mesurée à 15 images/s
de 0.0 à 8.5 s, réduite par 2.

**L'utilisateur** : *« Vue hauteur d'homme, vue de la plage, distance de 20-30m,énorme formation de vague, Grande vague vers 5m de haut, beaucoup de mouvement, beaucoup d'écume. »*

**Ce qu'elle montre** : La vague de The Wedge qui se forme et déferle ; le ciel en haut, la face de la vague au milieu, l'eau blanche en bas.

**Nos scènes** : **Hors de portée** : le déferlement et l'écume (suspendue depuis 2026-09-26) ; attend la lame (C10-4) et l'écume.

**Zones** : `ciel` [0.05, 0.03, 0.95, 0.2] ; `face` [0.08, 0.45, 0.92, 0.62] ; `ecume` [0.02, 0.76, 0.98, 0.92].

| grandeur | ciel | face | ecume |
|---|---:|---:|---:|
| p05 | 0,93 | 0,486 | 0,557 |
| p95 | 1,06 | 3,8 | 1,14 |
| p99 | 1,08 | 4,88 | 1,18 |
| contraste | 0,003 | 0,188 | 0,0817 |
| creux_BsurG | 1,57 | 0,916 | 0,985 |
| creux_BsurR | 1,56 | 1,3 | 0,944 |
| cretes_BsurG | 1,53 | 0,981 | 0,984 |
| claire | 0 | 0,0415 | 0 |
| ecume | 0 | 0,177 | 0 |
| hf_part | 0,118 | 0,415 | 0,547 |
| anisotropie | 0,962 | 1,08 | 2,42 |
| p50_absolu *(non comparable)* | 0,132 | 0,0842 | 0,724 |
| mouvement_par_s | 0,0518 | 4,35 | 1,27 |
| periode_s | 2,43 | 4,23 | 4,23 |
| periode_nettete | 579 | 5142 | 1134 |
| renouvellement_clairs_par_s | — | 8,51 | — |
| coupes | 1 | 0 | 0 |

## V3 — `65j2_BPyRQA`

<https://www.youtube.com/shorts/65j2_BPyRQA> — 10.9 s, servie en 480 × 854, mesurée à 15 images/s
de 0.0 à 10.8 s, réduite par 2.

**L'utilisateur** : *« Vue de haut depuis un rocher, vague impact avec des rochers, explosion, écume, agitation, océan »*

**Ce qu'elle montre** : Un montage de plans (Big Sur) : la gerbe sur les rochers, l'eau blanche qui s'étale. Les coupes, en fondu, ne sont pas détectées.

**Nos scènes** : **Hors de portée** : les obstacles fixes dans la mer, la gerbe d'impact, l'écume.

**Zones** : `cadre` [0.02, 0.02, 0.98, 0.98] ; `gerbe` [0.02, 0.08, 0.7, 0.45] ; `bas` [0.02, 0.55, 0.98, 0.95].

| grandeur | cadre | gerbe | bas |
|---|---:|---:|---:|
| p05 | 0,0356 | 0,165 | 0,182 |
| p95 | 2,5 | 2,85 | 1,95 |
| p99 | 2,94 | 4,34 | 2,28 |
| contraste | 0,205 | 0,329 | 0,228 |
| creux_BsurG | 1,97 | 1,93 | 1,83 |
| creux_BsurR | 3,69 | 3,41 | 3,06 |
| cretes_BsurG | 1,41 | 1,5 | 1,32 |
| claire | 0 | 0,0148 | 0 |
| ecume | 0,00534 | 0,00147 | 0,00352 |
| hf_part | 0,306 | 0,456 | 0,325 |
| anisotropie | 1,74 | 1,83 | 1,25 |
| p50_absolu *(non comparable)* | 0,291 | 0,16 | 0,39 |
| mouvement_par_s | 3,58 | 6,26 | 3,31 |
| periode_s | 5,43 | 5,43 | 2,72 |
| periode_nettete | 26780 | 5442 | 1150 |
| renouvellement_clairs_par_s | 10,5 | 8,77 | 2,08 |
| coupes | 0 | 0 | 0 |

## V4 — `I3NdxKg5RKM`

<https://www.youtube.com/shorts/I3NdxKg5RKM> — 6.3 s, servie en 360 × 640, mesurée à 15 images/s
de 0.0 à 6.2 s, réduite par 2.

**L'utilisateur** : *« Vidéo zoomé sur les mini vaguelettes en bord de plage, mouvement, bulle, écume et épaisseur fin. »*

**Ce qu'elle montre** : L'eau mince qui monte et redescend sur le sable (le jet de rive), à la période de 2,1 s mesurée ; servie en 360 × 640.

**Nos scènes** : **Hors de portée** : la plage en pente, le jet de rive, les bulles ; attend une scène de rivage.

**Zones** : `eau_mince` [0.05, 0.3, 0.95, 0.6] ; `rivage` [0.05, 0.6, 0.95, 0.85].

| grandeur | eau_mince | rivage |
|---|---:|---:|
| p05 | 0,283 | 0,281 |
| p95 | 1,97 | 3,13 |
| p99 | 2,15 | 4,25 |
| contraste | 0,165 | 0,147 |
| creux_BsurG | 1,21 | 0,751 |
| creux_BsurR | 1,46 | 0,833 |
| cretes_BsurG | 1,46 | 1,33 |
| claire | 0 | 0,014 |
| ecume | 0 | 0,0256 |
| hf_part | 0,258 | 0,267 |
| anisotropie | 2,88 | 1,62 |
| p50_absolu *(non comparable)* | 0,339 | 0,0494 |
| mouvement_par_s | 3,4 | 7,08 |
| periode_s | 2,09 | 2,09 |
| periode_nettete | 516 | 601 |
| renouvellement_clairs_par_s | — | 10,7 |
| coupes | 0 | 0 |

## V5 — `B-5QkrY07xI`

<https://www.youtube.com/shorts/B-5QkrY07xI> — 23.9 s, servie en 360 × 640, mesurée à 15 images/s
de 0.0 à 20.0 s, réduite par 2.

**L'utilisateur** : *« Vidéo pour le mouvement de l'eau dans une eau peux profondes et calmes. Reflet et couleurs. »*

**Ce qu'elle montre** : Une baie calme sous un ciel de cumulus, la côte à l'horizon (au milieu du cadre) ; servie en 360 × 640.

**Nos scènes** : **Aujourd'hui** : la mer B calme de `saut.tscn`, vue d'un quai (le ciel, l'eau lointaine, l'eau proche) — S472.

**Zones** : `ciel` [0.05, 0.03, 0.95, 0.35] ; `eau_lointaine` [0.02, 0.55, 0.98, 0.68] ; `eau_proche` [0.02, 0.75, 0.98, 0.95].

| grandeur | ciel | eau_lointaine | eau_proche |
|---|---:|---:|---:|
| p05 | 0,725 | 0,66 | 0,674 |
| p95 | 1,89 | 1,33 | 1,72 |
| p99 | 2,27 | 1,45 | 2,04 |
| contraste | 0,0347 | 0,116 | 0,227 |
| creux_BsurG | 2,25 | 1,16 | 0,88 |
| creux_BsurR | 3,87 | 1,58 | 1,29 |
| cretes_BsurG | 1,13 | 1,09 | 0,976 |
| claire | 0 | 0 | 0 |
| ecume | 0,0336 | 0 | 0,000994 |
| hf_part | 0,167 | 0,728 | 0,673 |
| anisotropie | 1,09 | 3,15 | 2,15 |
| p50_absolu *(non comparable)* | 0,366 | 0,23 | 0,117 |
| mouvement_par_s | 0,761 | 1,34 | 3,6 |
| periode_s | 10 | 10 | 10 |
| periode_nettete | 6820 | 47420 | 4604 |
| renouvellement_clairs_par_s | — | — | 15 |
| coupes | 0 | 0 | 0 |

## V6 — `IcWXWEe1N9M`

<https://www.youtube.com/shorts/IcWXWEe1N9M> — 124.2 s, servie en 360 × 640, mesurée à 10 images/s
de 0.0 à 40.0 s, réduite par 2.

**L'utilisateur** : *« Vidéo sous l'eau, effet de lumière, vision courte ou non, couleurs, caustique. »*

**Ce qu'elle montre** : Un montage de deux minutes sous la surface (Méditerranée) : la surface vue de dessous, le fond, les rochers, un nageur ; mesuré sur les 40 premières secondes.

**Nos scènes** : **En partie** : la caméra sous l'eau (R26, la piscine et la mer) et les caustiques ; la visibilité et la couleur se comparent (S472).

**Zones** : `cadre` [0.02, 0.02, 0.98, 0.98] ; `haut` [0.05, 0.02, 0.95, 0.35] ; `bas` [0.05, 0.6, 0.95, 0.95].

| grandeur | cadre | haut | bas |
|---|---:|---:|---:|
| p05 | 0,174 | 0,394 | 0,185 |
| p95 | 4 | 3,46 | 3,96 |
| p99 | 6,73 | 5,29 | 5,94 |
| contraste | 0,365 | 0,173 | 0,448 |
| creux_BsurG | 1,63 | 1,96 | 1,22 |
| creux_BsurR | 19,3 | 36,3 | 12,5 |
| cretes_BsurG | 1,22 | 1,49 | 1,1 |
| claire | 0,0501 | 0,0313 | 0,0487 |
| ecume | 0 | 0 | 0 |
| hf_part | 0,28 | 0,331 | 0,284 |
| anisotropie | 1,46 | 1,49 | 1,41 |
| p50_absolu *(non comparable)* | 0,0618 | 0,05 | 0,0786 |
| mouvement_par_s | 7,54 | 3,32 | 8,44 |
| periode_s | 20 | 16,1 | 13,3 |
| periode_nettete | 2154 | 1905 | 735 |
| renouvellement_clairs_par_s | 7,11 | 6,89 | 8,5 |
| coupes | 0 | 17 | 0 |
