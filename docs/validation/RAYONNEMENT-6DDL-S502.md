# La coque amortie sur ses six degrés de liberté — S502 (liste 6.1, validée)

*S502, 2026-10-06, en autonomie.* [RAYONNEMENT-COQUE-S336](RAYONNEMENT-COQUE-S336.md) avait mesuré par δ la masse ajoutée et
l'amortissement de rayonnement de la coque de la porte D **en pilonnement** ; elle roulait, tanguait et glissait sans perte.

## Reproduire

- `cargo test --manifest-path code/Cargo.toml --release --offline -p water-core -- s502 --nocapture` — quatre essais ; suite du cœur :
  663 essais.
- `cargo run -p water-core --release --offline --example rayonnement_coque -- --mode <roulis|tangage|lacet|cavalement|embardee>
  --omega <a,b,c>` — lignes `RAYONNEMENT` ; ≈ 3 min par mode à 25 cm (lancés par `outils/calcul.py`).

## 1. La construction

- **Le moment de δ** (`delta3d_cut::solid_wall_load`, `Volume3::solid_load`) : chaque triangle de coupe porte sa pression en son centre de
  gravité et son moment `(g − centre) × p·dA` ; la force, aux mêmes opérations, au bit de `solid_force`. Cas de réponse connue : un pavé
  incliné noyé sous une pression hydrostatique, moment contre `r_c × F` à 7,4·10⁻⁴, 1,5·10⁻⁴, 9,9·10⁻⁶ aux mailles de 5, 2,5, 1,25 cm.
- **La mesure** (`rayonnement_coque --mode`) : rotation `θ = Θ·sin ωt` (0,05 rad) autour d'un axe passant par le centre, ou translation
  horizontale (5 cm) ; en régime établi, `M = −A·θ̈ − B·θ̇` ajusté sur deux périodes.
- **Le corps** (`RigidBody`) : `added_inertia` et `radiation_damping_angular`, diagonaux dans ses axes, constantes d'archétype ;
  l'amortissement angulaire est relatif à la rotation de la surface sous le centre, `(∂w/∂y, −∂w/∂x, 0)`. Nuls par défaut : rien ne change.

## 2. Mesuré par δ (25 cm, domaine de 16 × 16 × 2 m)

| mode | ω (rad/s) | A | B | résidu |
|---|---|---|---|---|
| roulis | 2 / 2,5 / 3 | 214 / 209 / 204 kg·m² | 71 / 99 / 130 N·m·s | 8,4 / **15,8** / 6,0 % |
| tangage | 3,5 / 4 / 4,5 | 2 941 / 2 745 / 2 674 kg·m² | 4 780 / 4 738 / 4 353 N·m·s | 8,6 / 9,4 / 9,7 % |
| cavalement | 2 / 3 / 4 | 615 / 461 / 320 kg | 450 / 924 / 1 668 N·s/m | 9,6 / 5,6 / 2,8 % |
| embardée | 2 / 3 / 4 | 2 477 / 1 939 / 923 kg | 1 692 / 4 958 / 6 761 N·s/m | 15,8 / 16,7 / 6,7 % |
| lacet | 2 / 3 / 4 | 1 809 / 2 228 / 1 534 kg·m² | 264 / 1 933 / 7 001 N·m·s | 70,9 / 27,7 / 7,7 % |

Les ordres prévus : roulis `ζ₄₄` = 0,016 (prévu 0,01 à 0,05 — une barge rayonne peu en roulis), tangage `ζ₅₅` = 0,10 (prévu de l'ordre du
pilonnement, 0,16). **Le résidu** du roulis à 2,5 rad/s est fait de dix sauts discrets, jusqu'à 15 N·m sur un moment de 69 N·m, aux
franchissements de faces par les arêtes de la coque — la cause de S336 (A317, la résolution). Celui de l'embardée et du lacet aux basses
pulsations vient du domaine : à 2 rad/s l'onde fait 15 m, et elle revient des murs du domaine de 16 m pendant la mise en régime.

**Constantes de l'archétype** : roulis 210 kg·m² et 80 N·m·s, tangage 3 000 kg·m² et 4 800 N·m·s (interpolés à leur pulsation propre) ;
cavalement 460 kg et 920 N·s/m, embardée 1 940 kg et 4 960 N·s/m, lacet 2 230 kg·m² et 1 930 N·m·s (à 3 rad/s, une onde de 7 m : un
mouvement horizontal n'a pas de pulsation propre). ± 10 à 30 % selon le mode.

## 3. Le corps du jeu

| lâcher en eau calme | mesuré | référence |
|---|---|---|
| roulis, 0,05 rad : période / décrément | 2,8793 s / 0,0989 | 2,8917 s / 0,0998 (ζ = 0,0159) |
| tangage, 0,05 rad | 1,9682 s / 0,6273 | 1,9696 s / 0,6275 (ζ = 0,0994) |
| sans amortissement : crêtes | 0,05000 puis 0,05000 rad | constantes |
| cavalement / embardée / lacet : taux de décroissance | 0,25143 / 0,96591 / 0,26890 s⁻¹ | 0,25137 / 0,96498 / 0,26883 |

La raideur de référence est celle du proxy, mesurée par une inclinaison de 10⁻⁴ rad (roulis 5 475 N·m/rad pour 5 689 analytique : 16 × 8
points, B6).

## 4. Les critères, écrits avant

| critère | | |
|---|---|---|
| (1) la force au bit ; le moment hydrostatique d'un pavé incliné à 1 % | au bit ; 9,9·10⁻⁶ | tenu |
| (2) `A`, `B` en roulis et tangage à trois pulsations, résidu ≤ 10 %, `B` > 0 | `B` > 0 partout ; résidu 15,8 % au roulis à 2,5 rad/s | **manqué à une pulsation** (sauts de résolution, A317) ; tenu ailleurs |
| (3) lâcher : période et décrément à ±2 % ; sans amortissement, crêtes constantes | ≤ 0,9 % ; constantes | tenu |

## 5. 6.1

Les manques que la liste nommait sont faits : W derrière la requête (S494–S495), C11 (S498), B6 (S499–S500), l'amortissement des autres
degrés de liberté (ici). **6.1 validée** — la flottabilité des objets importants, sous ses limites écrites : des constantes d'archétype
à ± 10 à 30 % (25 cm de maille, A317), figées à une pulsation ; un pavé (une carène quelconque demande des points portant leur profil,
ADR-227 §3). La liste : **7 points validés sur 120**.
