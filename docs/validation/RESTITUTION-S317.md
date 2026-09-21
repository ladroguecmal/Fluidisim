# L'ordre D : le volume net de δ a un receveur — S317

2026-09-21. **Ordre D** du lot 2 ([ADR-181](../adr/ADR-181-conservation-transfert-oriente-et-ordre-du-lot-2.md)
D10), ouvert par l'utilisateur ; décisions de construction dans
[ADR-185](../adr/ADR-185-ordre-d-receveur-sous-i15.md).

Machine de référence (ADR-174 D1), référence CPU de δ. `src/regional_level.rs`,
`src/delta3d_transfer.rs` (`Ledger3`), `src/delta3d_balance.rs` (`perturbation_volume_x`) ;
`examples/transfert_oriente.rs`, modes `bilan_interieur` et `restitution`.

**En une phrase.** Le flux à la ligne de contrôle ferme le bilan de l'intérieur de δ au plancher
d'arrondi ; une région locale adossée à chaque ligne le reçoit, contre un reçu ; l'attente tombe à
**exactement zéro** et le bilan δ + régions se ferme — pour la **représentation**, pas pour le
monde.

---

## 1. Qui a le droit de recevoir un volume issu de δ

ADR-181 D1 nomme la couche : V en contenant, un niveau **régional** de B en eau ouverte, jamais W.
Mais δ est calculé sur le client, sans autorité (I-04) ; B et V sont répliqués (I-03), V tenu par
le serveur (I-10) ; et *aucun chemin d'énergie ne va du client vers le monde répliqué* (I-11). I-15
tranche sans arbitrage : un volume issu de δ **n'entre jamais** dans l'état répliqué de B ni de V.

D'où [ADR-185](../adr/ADR-185-ordre-d-receveur-sous-i15.md) : en eau ouverte, le receveur est le
niveau de B **tel que ce client le représente** — le statut de `W_local` pour W ; en contenant,
**δ s'asservit à V** (porte E, C21). Cette session construit le premier et écrit la règle du second.

## 2. La grandeur : le flux à la ligne ferme l'intérieur

ADR-179 D3 avait fixé la grandeur à restituer — le flux sortant à la surface de contrôle, pas
l'activité de l'éponge. Restait à montrer qu'elle **ferme** un bilan. Volume de contrôle : les
colonnes entre la première face hors de l'éponge gauche et la ligne de l'ordre C, où aucune éponge
n'agit ; `Volume3::perturbation_volume_x`, nouvel accesseur de mesure, en lit le volume sur la
même hauteur compensée que le bilan global. Même paquet que S316 (`λ` = 2 m, σ = 3 m, 2 cm).

| | 25 cm | 12,5 cm |
|---|---:|---:|
| résidu final `ΔV + ΣQ_sortant` | −1,6·10⁻¹⁰ m³ | −2,5·10⁻¹⁰ m³ |
| résidu maximal sur le trajet | 5,4·10⁻¹⁰ | 3,0·10⁻¹⁰ |
| flux absolu cumulé | 0,130 m³ | 0,053 m³ |
| sortant à droite | +1,17·10⁻⁴ | +5,29·10⁻⁵ |
| sortant à gauche | **−1,37·10⁻⁴** | **−5,37·10⁻⁵** |
| variation de l'intérieur | +2,0·10⁻⁵ | +8,2·10⁻⁷ |
| volume retiré par les éponges | **−5,9·10⁻⁵** | **−8,6·10⁻⁶** |

Prédiction écrite avant : résidu ≤ 10⁻¹⁰ m³ — **manquée d'un facteur 3 à 5**, au plancher d'arrondi
pourtant (2 à 6·10⁻⁹ du flux cumulé). La borne était trop serrée ; le bilan, lui, ferme.

**L'eau entre aussi par l'arrière.** Ce qui sort devant avec le paquet, une onde longue du second
ordre le ramène par derrière : à 12,5 cm, sortant et entrant s'équilibrent à 2 % près, et
l'intérieur ne gagne presque rien. **Le paquet déplace de l'eau de l'arrière vers l'avant.** Un
receveur n'a donc de sens qu'**à chaque ligne**, avec son signe.

**Les éponges ajoutent de l'eau.** Leur volume retiré est négatif : elles ramènent au repos un
niveau abaissé. Les compter en plus des lignes serait un double comptage — et à contresens. C'est
ADR-179 D3, vérifié.

## 3. Le receveur

`RegionalLevel` ([source](../../code/water-core/src/regional_level.rs)) :

- **une région, jamais un océan** : un rectangle adossé à un segment de ligne de contrôle, du côté
  sortant, sur une profondeur **déclarée** ; le niveau est le volume reçu divisé par l'aire. Une
  région qui déborderait de sa cellule est refusée : l'océan ne s'obtient pas en élargissant ;
- **rien sans reçu** : `receive` augmente le volume de la région **puis** délivre un `Receipt` du
  même montant, qui ne se construit nulle part ailleurs, ne se copie pas, et se **consomme** au
  registre par `Ledger3::account_restitution` — la seule autre façon, avec le transfert à W, de
  faire baisser l'attente ;
- **la frontière publie zéro**, par choix déclaré (ADR-185 D8) ; le terme existe pour que le jour
  où l'onde longue sera construite, aucun appelant n'ait à changer d'équation ;
- **le monde n'est pas revendiqué** : `global_conservation_claimable` devient faux dès qu'une
  région locale a reçu quoi que ce soit ; `representation_closed` dit ce que l'ordre D peut dire.

Sept essais neufs, **verts au premier passage** : niveau = volume / aire déclarée ; restitution pas à
pas qui ferme la représentation **sans** revendiquer le monde ; registre immobile sans reçu ; une
région qui rend plus qu'il n'est sorti **crée** de l'eau, et le registre le dit ; signe porté ;
niveau nul hors de la région, d'un autre repère ou d'une autre cellule ; refus nommés.

## 4. La restitution sur le banc

Une région par ligne, jusqu'au bord du domaine ; à chaque pas, flux signé → registre → région →
reçu → registre.

| | 25 cm | 12,5 cm |
|---|---:|---:|
| reçu à gauche (aire ; profondeur) | −1,37·10⁻⁴ m³ (9 m² ; 18 m) | −5,37·10⁻⁵ m³ (4,5 m² ; 18 m) |
| reçu à droite | +1,17·10⁻⁴ m³ (24 m² ; 48 m) | +5,29·10⁻⁵ m³ (12 m² ; 48 m) |
| niveau gauche / droite | −15,2 / +4,9 µm | −11,9 / +4,4 µm |
| attente des deux registres | **0 exactement** | **0 exactement** |
| eau créée | 0 | 0 |
| représentation fermée ; monde revendicable | oui ; **non** | oui ; **non** |
| bilan δ intérieur + régions, final (maximal) | −1,6·10⁻¹⁰ (5,4·10⁻¹⁰) m³ | −2,5·10⁻¹⁰ (3,0·10⁻¹⁰) m³ |

L'attente vaut **exactement** zéro, et non « au résidu près » : la région reçoit la même suite
d'additions que le registre compte. Le seul écart de la représentation est le résidu du transport,
publié.

**En usage** : quelques micromètres de niveau, pour un paquet de 2 cm — invisibles contre les 3 mm
d'image. Mais **comptés**, et c'était la demande : *« même lorsque son effet local sur le niveau
moyen est négligeable »* (ADR-180).

## 5. Ce que l'ordre D ne fait pas encore

| manque | pourquoi | où |
|---|---|---|
| **le rayonnement** de l'anomalie hors de sa région | une anomalie de niveau en eau ouverte part en onde longue à `√(g·h)` ; ici la frontière publie zéro | ADR-185 D8, file active |
| **le cas couplé** — la bande B/W traverse la ligne | le flux à la ligne mélangerait alors ce que δ produit et ce que B/W pousse (ADR-179 D4) | ordre E |
| **le contenant** — δ asservi à V | V autoritaire sur le serveur ; δ ne lui rend rien | porte E, C21 |
| **l'affichage** — le niveau dans la composition B + W + δ | `sample` rend le niveau ; la composition ne l'ajoute pas encore | à décider avec le rendu |
| **la production GPU** | le compteur de la carte n'existe pas (A302) | lot 7 |

## 6. Limites

Référence CPU, une machine (A98). Un paquet, en eau profonde, deux mailles ; un domaine à deux
rangées en `y`, donc deux lignes de contrôle et non un contour fermé à quatre côtés — le cas 3D
général demande une région par segment de contour. Sans fond B/W : `band_in` est nul par
construction, ce qui rend la séparation de l'entrant et du sortant triviale (ADR-185 D6). Le
volume reçu par une région y reste indéfiniment (D8).
