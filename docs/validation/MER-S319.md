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
