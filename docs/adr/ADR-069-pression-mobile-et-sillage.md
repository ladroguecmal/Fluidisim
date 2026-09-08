# ADR-069 — Construire le sillage depuis un forçage de pression mobile

- **Statut : ACTÉE**, S89, 2026-09-08, délégation technique.
- Complète W4 d'ADR-054. Première brique physique, hors chemin autoritaire existant.

## Choix

Retenir comme premier candidat un forçage de pression se déplaçant à la surface d'une eau
profonde uniforme, linéaire, sans courant, capillarité ni dissipation. Le mouvement déplace
le forçage ; il ne suffit pas à déterminer son intensité. La pression et son futur profil
spatial seront des données explicites du modèle de coque, à recevoir séparément.
Ne pas remplacer ce problème par une suite d'anneaux Impact normalisés indépendamment.

Base théorique consultée : [Li et Ellingsen, pression de surface dépendant du temps](https://arxiv.org/pdf/1511.02610)
formulent les vagues générées par une pression transitoire, notamment mobile. Les conditions
de surface linéaires avec pression et la dispersion à profondeur finie sont explicites dans
[l'article JFM de 2022, §2.1, équations 2.3 et 2.7](https://www.cambridge.org/core/journals/journal-of-fluid-mechanics/article/water-waves-generated-by-moving-atmospheric-pressure-theoretical-analyses-with-applications-to-the-2022-tonga-event/F8E33F8D5A70F63C8499D020A2247005).
Leurs applications ne constituent pas une validation de coque ou du jeu. La réduction profonde
et la formule de segment ci-dessous sont dérivées pour le prototype du dépôt.

## Réponse construite

Convention du prototype : pression réelle `Re(P(t) exp(i k·x))`, amplitude P en Pa,
élévation `Re(q(t) exp(i k·x))`, `k=|k|>0`, pesanteur g et densité ρ injectées.
Les conditions de surface donnent `q_dot=k ψ`, `ψ_dot=-g q-P/ρ`, donc

```
q_ddot + ω² q = -k P(t)/ρ,       ω² = g k.
P(t) = P0 exp(-i k·x0) exp(-i Ω t),   Ω = k·v,
```

durant un segment rectiligne de durée T. L'eau part du repos pour la contribution de ce
segment. Le résultat s'ajoute aux contributions précédentes, pas à leurs énergies.
La solution de Duhamel est calculée sans division par `ω²-Ω²` :

```
J(a,t) = t sinc(a t/2) exp(-i a t/2)
A = exp(i ωt) J(Ω+ω,t),     B = exp(-i ωt) J(Ω-ω,t)
q = F (A-B)/(2 i ω),         q_dot = F (A+B)/2
F = -k P0 exp(-i k·x0)/ρ.
```

`sinc(0)=1` conserve la résonance `Ω=±ω`, au lieu d'ajouter un epsilon physique arbitraire.
Pour |x|<1e-4, sinc utilise `1-x²/6+x⁴/120` ; reste analytique ≤|x|⁶/5040, hors arrondis.
Après extinction, q et q_dot se propagent librement par rotation harmonique. Aucun TTL ne
supprime l'onde. Un démarrage abrupt produit aussi une onde si la vitesse est nulle : ce n'est
pas un défaut, c'est la pression initialement absente qui apparaît.

`pressure_mode.rs` est un instrument f64, comme `dispersif.rs`, avec libm et temps relatif f64
après soustraction entière de la naissance. Il n'est pas raccordé à LiveWater, Impact V1,
WLIV ou au runtime répliqué. I-03/I-08 ne sont pas assouplis : un portage déterministe et une
réception du domaine numérique seront nécessaires avant exposition gameplay.

## Énergie et segmentation

Pour ce mode cosinus réel, moyenne spatiale par unité d'aire :

```
E = ρ/4 (g |q|² + |q_dot|²/k)         [J/m²]
dE/dt = -Re(P conj(q_dot))/2           [W/m²].
```

La seconde identité découle directement de l'équation forcée. Elle fixe le signe et les
facteurs de normalisation. Elle ne donne pas l'énergie totale d'un sillage localisé : le mode
est étendu, de moyenne non nulle en énergie sur une aire infinie.

Une trajectoire rectiligne scindée en deux segments contigus, avec origine du second avancée
de vT, conserve exactement la réponse analytique. Le test retrouve cette identité à l'arrondi
près. En revanche, `E(q1+q2)` contient des termes croisés : additionner `E(q1)` et `E(q2)`
change le résultat. Le travail du segment actif doit être évalué contre la vitesse totale,
pas seulement contre sa contribution isolée. La fonction power actuelle reçoit **un segment
isolé** ; elle n'est pas un comptable multi-segments. Leçon L203.

## Vérifications exécutées

Cinq tests release puis suite debug : signe de l'enfoncement sous pression stationnaire et
solution fermée ; énergie conservée après extinction jusqu'à 20 s ; comparaison de q et q_dot
à une quadrature temporelle indépendante (40 000 points) pour Ω/ω = -1, 0, 0,5, 1±1e-10,
1 et 2 ; translation spatiale/temporelle près de u64::MAX ; segmentation, orientation
perpendiculaire, refus de valeurs invalides et de débordement de date.

Travail intégré sur 4 s (20 000 milieux de pas), k=1 rad/m, P0=10 Pa, g=9,81, ρ=1025 :

| Vitesse m/s | E à 4 s, J/m² | Écart absolu travail/énergie, J/m² |
|---:|---:|---:|
| 0 | 3,590262008062e-6 | 5,87e-14 |
| 2 | 2,353878113597e-2 | 9,31e-11 |
| √9,81 ≈ 3,13209195 | 1,951237457025e-1 | 1,18e-13 |
| 6 | 1,784988831768e-3 | 4,68e-11 |

Tolérances de régression : 2e-10 m sur q et 2e-9 m/s sur q_dot contre quadrature,
2e-8 J/m² sur travail. Ces seuils et les fixtures ne sont pas des tolérances de production ;
à calibrer lors de la réception spatiale. Aucune limite de linéarité ni précision pour tous
les paramètres finis n'est certifiée. L'instrument ne borne pas encore les très grands arguments.

## Ce qui reste ouvert

S88-1 partielle : forçage modal et travail construits, pas de sillage localisé, d'angle de Kelvin
reçu, de résistance de vague complète ou de lien coque/pression calibré. Le champ spatial exige
une intégration sur les directions et longueurs d'onde avec normalisation et troncature reçues.
**S89-1, prochaine session S90 :** construire la superposition d'un spectre de pression localisée
en translation et vérifier convergence spatiale/spectrale et travail total, avant codec ou intégration.
Pas de nouveau budget AAA, de sélection finale B2 ni d'autorité physique client. I-01, I-03,
I-06, I-08 et I-11 inchangés ; aucun nouvel angle numéroté.

> **Actualisation S90 — 2026-09-08.** S89-1 réalisée sur un profil gaussien en translation,
> ADR-070. Champ et bilan global par quadrature reçus sur fixtures ; première résolution
> refusée puis raffinement reçu sans relèvement du seuil. Trajectoire multi-segments : S90-1.
