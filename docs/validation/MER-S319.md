# L'ordre E, premiers échelons : sous une vraie mer, δ ne reste pas petit — S319

2026-09-21. **Ordre E** du lot 2 ([ADR-181](../adr/ADR-181-conservation-transfert-oriente-et-ordre-du-lot-2.md)
D10) : le couplage complet, sur des configurations progressivement plus complexes. Le premier
échelon devait être un témoin nul ; il a trouvé un défaut qui les précède tous.

Machine de référence (ADR-174 D1), référence CPU de δ. `examples/transfert_oriente.rs`, modes `mer`
et `mer_paquet` ; options `MER_TRACE`, `MER_DUREE`, `MER_PROFIL`.

**En une phrase.** Sous une houle B ordinaire, sans aucune perturbation, δ **ne reste pas petit** :
il croît pendant des dizaines de secondes — jusqu'à trois fois l'amplitude de la houle —, à un taux
qui dépend de l'amplitude et presque pas de la maille ; tout ce que la ligne de contrôle lit alors
vient de cette dérive, et la restitution de S317 reçoit des centaines de fois le volume d'un paquet.

**État présent (S322).** Le taux de croissance **ne dépend pas du pas de temps** : 0,1015 à 0,1007 s⁻¹
de 20 à 2,5 ms, 0,8 % sur un facteur huit (§8). Ce n'est pas un défaut d'intégration du pas couplé ;
la cause reste à nommer, et la voie d'A289 à choisir.

---

## 1. Les échelons prévus

Les ordres C et D avaient été mesurés sur un paquet **seul**, sans fond B. L'ordre E devait ajouter,
un par un : **E1**, une houle B autour du domaine, δ nul au départ — le témoin nul de l'ordre D : une
mer que δ ne perturbe pas ne doit rien faire restituer ; **E2**, le paquet de l'ordre C dans cette
mer ; **E3**, un contour fermé à quatre côtés en trois dimensions. Houle : une composante, λ = 4 m
(`k·h` = 3,9, eau profonde), 5 cm puis 2,5 cm, vers `+x`, échantillonnée sur les faces de δ à chaque pas
(`BackgroundGrid3`).

Critère écrit avant la mesure : en E1, le volume restitué sur 20 s reste **sous 10 %** du volume net
d'un paquet de 2 cm (S317). Et une question que je ne savais pas trancher : le flux de δ à la ligne
contient-il le transport de la houle ?

## 2. E1 — une mer que δ ne perturbe pas fait restituer trois cents fois un paquet

| 20 s | 25 cm, houle 5 cm | 25 cm, houle 2,5 cm | 12,5 cm, houle 5 cm |
|---|---:|---:|---:|
| δ maximal | **2,83 cm** | 0,28 cm | **2,46 cm** |
| reçu à droite | 3,4·10⁻² m³ | 1,9·10⁻³ m³ | 9,5·10⁻³ m³ *(largeur moitié)* |
| reçu à droite / paquet de 2 cm | **292** | 16 | **179** |
| bilan de l'intérieur, final | 6,8·10⁻⁴ m³ | 7,2·10⁻⁵ m³ | 8,7·10⁻⁵ m³ |

**Le critère est manqué d'un facteur mille.** δ croît **le long du sens de propagation** de la houle
— 3 mm à l'entrée, 2,7 cm à partir de 30 m — : une onde de **désaccord**. Elle dépend à peine de la
maille (2,83 puis 2,46 cm) et **fortement de l'amplitude** (dix fois moins à amplitude moitié, soit
≈ `a³`). Calé en phase sur B, ce δ fait passer à la ligne un transport croisé `a_B·a_δ·ω/2` —
≈ 2,8·10⁻² m³ en 20 s à 25 cm, 3,4·10⁻² lus —, que la restitution prend pour de l'eau sortie de δ.
Et le bilan de l'intérieur ne ferme plus au plancher d'arrondi : le couplage à B apporte du volume
**dans** l'intérieur, pas seulement à travers ses bords.

## 3. Le remède du témoin, et pourquoi il ne tient pas

Un δ **témoin** — même domaine, même mer, sans le paquet — avance au même pas, et l'on restitue la
**différence** des flux : ce que la mer seule fait passer s'annule. Prédiction écrite avant : la
différence retrouve le paquet seul **à 30 % près**. Mesuré à 25 cm, sous 5 cm de houle : **18 fois** le
paquet à droite, et le **mauvais signe** à gauche. **Manqué.**

La trace du témoin dit pourquoi : **la mer seule, sans paquet, fait monter δ à 17 cm en 42 s.**
Une perturbation de 2 cm ne se superpose pas linéairement à une dérive de cette taille.

## 4. La dérive longue : plus grande que la mer

δ sous la **seule** houle, 120 s :

| t | 25 cm, houle 2,5 cm | 25 cm, houle 5 cm | 12,5 cm, houle 2,5 cm |
|---:|---:|---:|---:|
| 20 s | 2,3 mm | 2,8 cm | 3,0 mm |
| 40 s | 6,2 mm | **16,5 cm** | 5,6 mm |
| 60 s | 12,8 mm | 13,4 cm | 11,8 mm |
| 90 s | 37,6 mm | 12,0 cm | 24,7 mm |
| 115 s | **77 mm** | 16,0 cm | **39,5 mm** |
| taux, 20 → 60 s | ≈ 0,043 s⁻¹ | ≈ 0,10 s⁻¹ *(20 → 40 s)* | ≈ 0,034 s⁻¹ |

δ finit **plus grand que la houle qui le porte** — trois fois sous 2,5 cm. **Aucune session n'avait fait
vivre un domaine δ plus de quelques secondes sous une houle** : la scène de S302 durait six secondes.

## 5. Numérique ou physique

**Pas d'abord numérique.** L'advection croisée du pas couplé est explicite et centrée — un schéma
instable en principe —, mais son taux croîtrait comme `1/dx²` à pas de temps fixe : à maille moitié,
quatre fois plus vite. Mesuré : **un peu plus lentement** (0,034 contre 0,043 s⁻¹). Le raffinement
n'est pas la cause, et il n'est pas le remède.

**Ce qui reste s'accorde avec A289**, et le dépasse. B est **linéaire** ; δ porte l'onde **totale**, dont
la vitesse de phase croît avec l'amplitude (`Δω/ω = (ak)²/2`). La différence des deux — δ, par
définition — grandit comme le déphasage accumulé. A289 l'avait chiffrée en 2D, par formule :
croissance **linéaire**, « ≈ 7 cm/min sous `ak` = 0,06 ». Mesurée ici en 3D, elle est **plus rapide**
et plus grande qu'un simple déphasage — lequel borne δ à `2a` —, et son allure est exponentielle sur une
minute. Le prolongement de B au-dessus du plan moyen (ADR-154) et la bande de couplage restent des
suspects secondaires, non éprouvés.

## 6. Ce que cela change

- **La restitution de S317 ne vaut qu'en eau calme.** En mer réelle, ce qu'elle reçoit est la dérive de
  δ, pas le volume d'une perturbation. ADR-185 D6 renvoyait ce cas à l'ordre E ; il se règle en amont.
- **L'ordre E est bloqué par A289**, dont le déclencheur — « premier domaine appelé à vivre plus de dix
  périodes sous `ak` ≥ 0,05 » — est atteint. Les trois voies qu'A289 nommait deviennent un choix à
  faire : un **rappel lent** de δ vers zéro dans l'intérieur, une **durée de vie bornée** des domaines
  (I-12 rend leur recréation gratuite), ou la **dispersion d'amplitude dans B** — cette dernière
  changeant la mer de tous les clients.
- **E3** (contour fermé) ne dépend pas de ce défaut mais n'apprendrait rien qui débloque ; il est
  reporté, daté dans la file.

## 7. Limites

Une houle monochromatique, deux amplitudes, deux mailles ; une tranche à deux rangées en `y` ; 120 s au
plus. La cause n'est pas démontrée : elle est **restreinte** — pas l'advection centrée, compatible avec
A289, plus rapide que sa formule. Un spectre de houle réel, plus long, plus doux, peut dériver moins
vite ; il n'a pas été essayé.

---

## 8. S322 — le pas de temps, à maille fixe

2026-09-22. **La question** que la lecture de `simufluid` a posée
([LECTURE-SIMUFLUID-S320](../registres/LECTURE-SIMUFLUID-S320.md) §1) : chez eux, le résidu dérive
sous houle à un taux porté par le **nombre de pas par période**, jusqu'à changer de signe. S319
n'avait fait varier que la maille. Si notre croissance suit le pas, elle est au moins en partie un
défaut d'intégration du pas couplé, et ajouter la dispersion d'amplitude à B ne la soignerait pas.

### Reproduire

- Commit `db1c984a` ou plus récent ; machine de référence, CPU, un processus par pas.
- `MER_TRACE=1 MER_DUREE=40 MER_DT_US=<µs> cargo run -p water-core --release --offline --example transfert_oriente -- mer 0.25 0.05`
  pour 20 000, 10 000, 5 000 et 2 500 µs ; `TRACE t=… delta_max=…` sort sur l'erreur standard, une
  ligne par seconde simulée.
- Sans `MER_DT_US`, le banc est celui de S319 : `perturbation_max_m=2.828526496887207e-2` et
  `recu_droite_sur_paquet=292.1912` à 20 s — les chiffres publiés en §2.
- Durées de calcul pour 40 s simulées : 59, 112, 205 et 378 s.

### Mesure

E1 : houle d'une composante, 5 cm, λ = 4 m, `T` = 1,6 s ; δ nul au départ ; maille 25 cm. Taux =
pente de `ln δ_max` entre 10 et 30 s, vingt et un points, moindres carrés. **Prédiction écrite avant
la mesure** : écart < 10 % entre 20 et 2,5 ms si le modèle domine ; > 30 % si le pas couplé
contribue.

| pas | pas par période | taux, 10 → 30 s | R² | δ à 10 s | 20 s | 30 s | 39 s |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 20 ms | 80 | **0,1015 s⁻¹** | 0,995 | 0,90 cm | 2,82 cm | 7,65 cm | 16,5 cm |
| 10 ms | 160 | **0,1012** | 0,995 | 0,90 | 2,81 | 7,46 | 15,8 |
| 5 ms | 320 | **0,1009** | 0,995 | 0,91 | 2,81 | 7,40 | 15,4 |
| 2,5 ms | 640 | **0,1007** | 0,995 | 0,91 | 2,80 | 7,37 | 15,2 |

**Écart entre 20 et 2,5 ms : 0,8 % du taux.** Il décroît de façon monotone à chaque division du pas
— 3, 3 puis 2·10⁻⁴ s⁻¹ —, soit un taux limite vers 0,100 s⁻¹. Seule la saturation, après 35 s, bouge
un peu plus : 16,5 à 15,2 cm à 39 s.

### Verdict

**Selon la règle écrite avant : le modèle domine ; le pas couplé n'y est pour presque rien.** Chez
`simufluid`, le taux changeait de signe entre 256 et 4 096 pas par période ; ici, il ne bouge pas
d'un pour cent entre 80 et 640.

Ce qui est **exclu** : une erreur d'intégration en temps du pas couplé — et en particulier
l'instabilité de l'advection croisée explicite et centrée, dont le taux, à maille fixe, serait
proportionnel au pas et aurait été divisé par huit. S319 l'avait déjà écartée par la maille.

Ce qui **reste**, et qui n'est pas démontré : une propriété du modèle semi-discret — le couplage de
B **linéaire** et de δ porteur de l'onde **totale** (A289), la bande de couplage, le prolongement de B
au-dessus du plan moyen (ADR-154). La croissance dépasse toujours ce qu'un simple déphasage
permettrait (`2a`) ; son mécanisme reste à nommer, et **le fait ne dit pas lequel**.

**Pour l'arbitrage d'A289** : un remède purement numérique du pas — sous-pas, ordre en temps plus
élevé — ne servirait à rien. Les trois voies restent ouvertes, et le choix revient à l'utilisateur.
