# L'effet moyen de la houle : le niveau au rivage, le courant de dérive — S672 (listes 2.7, 12.3)

*S672, 2026-10-07, en autonomie, vers la v2.* Une mer qui déferle (S669–S670) pousse aussi l'eau. Elle relève le niveau moyen au rivage
et entraîne un courant le long de la côte. 12.3 nommait ce courant parmi ce qui manque.

## Reproduire

- `cargo test --manifest-path code/Cargo.toml --release --offline -p water-core --lib s672 -- --nocapture` (≈ 3 s).

## Ce qui a été fait

Le module `houle_moyenne.rs` (catégorie O, un outil de cuisson) traite une côte uniforme le long de ses bords, rangée par rangée :

- **la contrainte de radiation** (Longuet-Higgins et Stewart 1964), par `ρ` : `S_ss = Σ E·(n·k_s²/k² + n − ½)` et
  `S_sn = Σ E·n·k_s·k_n/k²` ;
- **le niveau moyen** : `dη̄/ds = −(dS_ss/ds)/(g·(h + η̄))`, par trapèzes, implicite en `η̄`. `S_ss` peut dépendre de `η̄`, comme dans un
  déferlement saturé ;
- **le courant de dérive** : `c_f·⟨|u|·u_n⟩ = −dS_sn/ds`, résolu par bissection. La moyenne est prise sur le temps des vitesses au fond de
  toutes les ondes (une suite équirépartie de 8 192 instants), sans linéariser le frottement.

## Mesuré

**L'instrument** : trois solutions analytiques, chacune dans son domaine.

**Ce qui départage** :

- un `S_ss` faux (le `n − ½` oublié, un `cos²` de trop) s'écarte du creux de dizaines de % ;
- une intégration fausse s'écarte de la pente de Bowen ;
- une dérive au mauvais signe, ou un frottement mal moyenné, s'écarte de Longuet-Higgins.

Les bornes sont au double au moins du plancher calculé au plan, par le même équilibre intégré en Python.

| | référence | critère | mesuré |
|---|---|---|---|
| (1) le creux, houle de 1 m et 10 s, plage 1:50 de 80 m à 5 m | Longuet-Higgins et Stewart 1962 : `η̄ = −H²k/(8·sinh 2kh)` | < 0,5 % | **0,10 %** ; −1,34 cm à 5 m |
| (2) la remontée saturée, `H = γ(h + η̄)`, 30 s, de 3 m à 0,3 m | Bowen, Inman et Simmons 1968 : `K = 1/(1 + 8/(3γ²))` = 0,186 | < 2 % | **0,73 %** ; +50 cm à 0,3 m |
| (3) la dérive, 30 s, `c_f` = 0,01, 1° à 2 m, de 1,5 à 0,5 m | Longuet-Higgins 1970 : `V = (5π/16)·(γ/c_f)·tan β·√(gh)·sin θ` | < 3 % | **0,99 %** ; 6 cm/s à 1 m |
| (3) la même, 5° à 2 m | | rapporté | 10,9 % sous la formule : 27 cm/s à 1 m |

À 5°, `V/u_m` vaut 0,26. Le frottement n'est plus linéaire, et la moyenne exacte freine davantage que la formule linéarisée. C'est
l'effet attendu (Liu et Dalrymple 1978 le décrivent) ; la formule n'est que la limite d'un courant faible.

## Ce que cela dit

Le niveau moyen et le courant de dérive d'une côte droite se calculent maintenant depuis les ondes qui l'atteignent, sans constante
posée de tête hors de `c_f`. Une mer d'un mètre relève le rivage de plusieurs dizaines de centimètres. Une incidence de quelques degrés
y fait courir l'eau le long de la plage à plusieurs dizaines de cm/s.

**Ne fait pas** : le mélange latéral (le courant suit la dissipation sans s'étaler au-delà de la bande) ; la circulation 2D d'une côte
non uniforme (les courants d'arrachement) ; le reflux sous la surface ; la rétroaction du niveau et du courant sur la houle. `c_f` =
0,01 est un ordre de grandeur des plages de sable, pas une mesure.

**Suivant** : `Cote2D` reçoit le niveau et le courant de sa mer qui déferle (S673).
