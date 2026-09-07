# C22 — couple d'oracles 76800/153600, S57, 2026-09-07

Exécution de l'action **S56-1**, selon le protocole fixé dans
[REFERENCE-C22-S56](REFERENCE-C22-S56.md). Aucun paramètre de montage, aucun seuil et aucun
critère n'ont été modifiés : le mode `c22-shallow-fin` a été lancé tel quel avec `76800`,
sur les mêmes huit grilles, le même filtre ×30 et le même test de stabilité.

Le but déclaré était de **mesurer une admission**, pas d'obtenir un verdict de convergence.
S56 laissait la question ouverte de façon explicite : l'erreur mesurée sur la grille 12800
(7,710700097e-9) tombait **entre** les deux seuils extrapolés — 6,94346e-9 en n⁻² et
7,77366e-9 avec l'exposant empirique 1,72145 — de sorte que **le choix du modèle
d'extrapolation, et non la mesure, aurait décidé du sort de la grille**.

## Conditions

Release locale, aucune suite de tests en concurrence, champs conservés en mémoire et jamais
sur disque (I-17). `a = 0,01 m`, `sigma = 1 m`, `h0 = 1 m`, `L = 40 m`, `t = 1 s`, `CFL = 0,45`.
Huit grilles 100 → 12800, deux oracles 76800 et 153600 calculés une seule fois et réutilisés
par les quatre fenêtres. Réutiliser les champs ne rend pas les fenêtres indépendantes.

## Coût mesuré

| Poste | Mesure |
|---|---:|
| Oracle 76800 | 165,833 s |
| Oracle 153600 | 672,560 s |
| Huit grilles et mesures | 6,040 s |
| **Campagne** | **844,433 s** *(14 min 04 s)* |

Le budget annoncé par S56 était **827,467 s** : l'écart est de **+2,05 %**. Le rapport des
deux temps d'oracle vaut 4,0557 pour un facteur de taille 2 ; celui du premier oracle au
premier oracle de S56 vaut 2,2802 pour un facteur 1,5 (attendu 2,25, soit +1,3 %). Le coût
quadratique est donc confirmé sur trois tailles, et le poste « reste » est stable à 6,04 s
contre 6,035 s en S56.

## Écart des oracles et seuil

**Écart L1 76800/153600 : 2,709078717e-10**, seuil ×30 : **8,127236151e-9**.

| Couple | Écart L1 mesuré | Exposant local |
|---|---:|---:|
| 25600 / 51200 *(S48)* | 1,717298105e-9 | — |
| 51200 / 102400 *(S49, S56)* | 5,207593646e-10 | 1,72145 |
| 76800 / 153600 *(S57)* | 2,709078717e-10 | **1,61233** |

## Erreurs et admission

| nx | erreur / 76800 | erreur / 153600 | variation | retenue |
|---:|---:|---:|---:|:--:|
| 100 | 7,514859747e-5 | 7,514863831e-5 | 4,083374e-11 | oui |
| 200 | 2,380682552e-5 | 2,380691459e-5 | 8,906595e-11 | oui |
| 400 | 7,306611544e-6 | 7,306732642e-6 | 1,210987e-10 | oui |
| 800 | 1,981993534e-6 | 1,982127485e-6 | 1,339514e-10 | oui |
| 1600 | 5,048208214e-7 | 5,049571320e-7 | 1,363106e-10 | oui |
| 3200 | 1,253146793e-7 | 1,254472015e-7 | 1,325222e-10 | oui |
| 6400 | 3,120105093e-8 | 3,133409146e-8 | 1,330405e-10 | oui |
| **12800** | 7,635780951e-9 | **7,766762184e-9** | 1,309812e-10 | **non** |

La grille 12800 vaut **0,9556 fois** le seuil requis, contre 0,494 fois en S56. **Elle reste
refusée**, et le refus est conservé sans reconstruction de triplet au-delà.

## Ordres et verdict

| Fenêtre | Grilles retenues | Ordres | Verdict |
|---|:--:|---|---|
| 100–1600 | 5/5 | 1,63764980 · 1,63173548 · 1,84983833 | non concluant, p = 1,64, régime asymptotique non atteint |
| 200–3200 | 5/5 | 1,63173548 · 1,84983833 · 1,96062667 | non concluant, p = 1,63, régime asymptotique non atteint |
| 400–6400 | 5/5 | 1,84983833 · 1,96062667 · 2,01167003 | non concluant, p = 1,85, régime asymptotique non atteint |
| **800–12800** | **4/5** | 1,96062667 · 2,01167003 | non concluant, **stabilité non établie**, triplets insuffisants |

Bilan du harnais : **quatre grandeurs, zéro succès, zéro échec, quatre sans verdict** ;
sortie 0. Aucun ordre n'est publié pour un triplet incluant la grille rejetée.

## Vérification

123 tests réussis (38 cœur, 85 harnais), deux ignorés, avant la mesure ; compilation release
réussie ; les deux scénarios `check` restent verts, hashs `0x3e2c06a7b00e73e3` et
`0x1a8b0629a9f51b6e` inchangés. Aucun fichier de code n'a été modifié par cette session :
la campagne est une exécution du binaire construit sur `13851c1`.
