# ADR-209 — L'advection de δ au second ordre en temps : le terme de Lax-Wendroff dans la prédiction

- **Statut : actée**, S391, 2026-09-26 ; autonomie technique (S71). Demande de l'utilisateur : *« Corrige A321 d'abord »*.
- **Corrige** l'angle mort **A321** (S390, sévérité 3) : à 30 Hz, la scène de la porte B explose en 24 à 40 s, quel que soit
  le solveur de pression ; à 60 Hz, en 72 s. **Ne remplace aucune décision** : la prédiction de δ n'avait jamais choisi son
  schéma en temps ; elle héritait de l'Euler explicite de la 2D.
- **Mesure** : banc `--delta3d-a321` (`viewer/src/delta3d_a321.rs`) et essai
  `second_order_advection_stops_the_ftcs_growth_s391` ; détail dans la preuve ([A321-S391](../validation/A321-S391.md)).

## 1. Ce qui a été mesuré

La prédiction avance les vitesses de δ par **Euler explicite et différences centrées** — l'auto-advection `u'·∇u'`
(`advect_mobile3`) et le transport par le fond `U·∇u'` (`extra3`), dans le cœur comme sur la carte. Ce schéma (FTCS) est
**instable pour tout pas** : il porte une anti-diffusion `(dt/2)·V_a·V_b·∂_a∂_b u`, dont le taux croît comme `dt·|V|²/dx²`
sur les modes de deux à quatre mailles. Attribué en S391 :

- le temps d'explosion suit `1/dt` : 72 s à 16,7 ms, 33 s à 25 ms, 23 à 40 s à 33,3 ms ;
- le mode qui croît est à l'échelle de la maille : sa part dans les vitesses horizontales monte de 0,4 % à 31 % avant
  l'explosion ;
- la pression est hors de cause (deux projections convergées explosent) ; éteindre un terme à la fois ne sauve pas la
  scène sur deux minutes (sans `u'·∇u'` : 68 et 75 s) ;
- **le terme qui manque à l'Euler explicite**, ajouté seul, la fait tenir deux minutes à 30 et à 60 Hz ;
- sur un courant uniforme de 2 m/s (Courant 0,264), un mode de quatre mailles croît ×7,55 en 60 pas (×7,4 prédits) ;
  corrigé, il décroît ×0,134 (`|G|² = 1 − C² + C⁴`, ×0,134 prédits).

## 2. Décisions

**D1 — Le terme de second ordre.** La prédiction ajoute, après l'advection de `u'` et avant les termes du fond et
l'éponge, `+ (dt²/2)·Σ_a Σ_b V_a·V_b·∂_a∂_b u`, avec `V` la vitesse qui transporte — celle de B à la face plus `u'`
interpolé au centre de la face —, dérivées secondes centrées, croisées en `(±1, ±1)/4`, vitesses du début du pas. Mêmes
faces que la prédiction ; une direction est omise dès qu'un de ses voisins sort de la grille de l'axe ; une face fermée
n'est pas touchée. À vitesse constante, c'est le schéma de Lax-Wendroff : second ordre en temps, stable pour un nombre de
Courant sous 1.

**D2 — Où il vit.** **Production** (`viewer/`, `btd` de `delta3d_step.wgsl`) : **actif par défaut** — c'est elle qui
explosait. **Référence** (`water-core`, `delta3d_advection.rs`) : **option** `Volume3::enable_advection_correction`, éteinte
par défaut, pour les deux pas — mobile et couplé. Les bancs qui comparent la production à la référence l'allument ; les
réceptions du cœur restent au bit.

**D3 — Migration du défaut du cœur, différée.** Allumer le terme par défaut dans le cœur change les empreintes de tout essai
qui passe par la prédiction ; à la vitesse des cuves et des contenants, son effet est de l'ordre de `(|V|·dt/dx)²`, sans
conséquence mesurée. **Déclencheur** : la prochaine réception de la référence à des vitesses de mer (≳ 0,5 m/s) sur plus de
vingt secondes, ou le raccord APIC ↔ colonnes (C5), qui prédit dans les deux représentations.

## 3. Écarté, et pourquoi

| alternative | raison |
|---|---|
| amont du premier ordre | viscosité `|V|·dx/2` ≈ 0,25 m²/s à 2 m/s : 3 %/s sur le paquet de 16 m de la porte B |
| Runge-Kutta d'ordre 3 | stable avec les différences centrées, mais trois prédictions par pas : le coût du premier étage triplé |
| semi-lagrangien (MacCormack) | un autre schéma d'advection, hors de la référence ; à reconsidérer avec APIC, qui transporte par particules |
| n'éteindre que `u'·∇u'` | retarde l'explosion (68 à 75 s à 30 Hz), ne l'empêche pas : `U·∇u'` porte aussi l'anti-diffusion |

## 4. Conséquences

- La porte C se remesure avec le terme (son coût s'ajoute à la prédiction) ; la scène à 30 Hz s'éprouve sur deux minutes.
- Les empreintes de la production changent (`--delta3d-empreinte`) : expliquées dans la preuve, champ par champ.
