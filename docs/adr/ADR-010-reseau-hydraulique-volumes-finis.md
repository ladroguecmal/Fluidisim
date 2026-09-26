# ADR-010 — Réseau hydraulique des volumes finis (couche V)

- **Statut** : proposée
- **Session** : S01
- **Résout** : `zones_ouvertes §17`, `architecture_globale §2.2`
- **Dépend de** : ADR-001, ADR-002, ADR-009

---

## 1. Décision

Les volumes finis ne sont pas des petits océans. Ils forment un **graphe hydraulique** résolu à
basse fréquence, en arithmétique entière, séparé de toute simulation de surface libre.

```
Node  = contenant  (citerne, compartiment, piscine, flaque, nappe, tronçon de canalisation)
Edge  = ouverture  (orifice, seuil de débordement, vanne, matériau poreux, pompe)
```

C'est la pièce que les documents sources décrivaient par ses symptômes (`§17` : fuite,
débordement, infiltration, ruissellement, évacuation, seuil de création de flaque) sans jamais
nommer la structure qui les unifie.

---

## 2. Nœud

```
struct HydroNode {
    id            : u64
    frame         : FrameRef
    volume_ml     : i64          // millilitres, entier — jamais un flottant
    capacity_ml   : i64
    liquid_id     : u8           // eau, carburant, méthane… (angle mort A17)
    shape_lut     : LUT*         // volume → hauteur, monotone, précalculé hors ligne
    sky_exposure  : f16          // 0..1, précalculé — pluie
    absorb_rate   : f16          // ml/s vers le sol, dépend du matériau
    flags         : u16
}
```

**`shape_lut` est essentiel.** Un compartiment n'est pas un prisme : la relation volume → hauteur
d'une cale, d'un fond de citerne bombé ou d'une dépression de terrain est non linéaire. Elle est
calculée hors ligne à partir du maillage (coupes horizontales, 64 entrées suffisent) et donne
gratuitement la hauteur, la surface de flottaison et le centre de carène — donc l'entrée du modèle
de ballottement d'ADR-008 §4.

**Surface libre en référentiel accéléré.** Le plan d'eau est perpendiculaire à `g_eff`, pas à Z.
La hauteur d'une ouverture est donc évaluée par sa distance signée au plan de surface orienté
selon `g_eff`. Sans cela, un vaisseau qui accélère ne verrait pas son réservoir fuir par le hublot
latéral qui se retrouve « en bas ».

## 3. Arête et loi de débit

Orifice (loi de Torricelli, la formulation demandée implicitement par `§17`) :

```
Δh = h_amont − max(h_aval , h_orifice)
Q  = C_d · A · √(2 · |g_eff| · Δh)            C_d ≈ 0,62 (orifice à arête vive)
                                              C_d ≈ 0,82 (ajutage arrondi)
```

Seuil de débordement (déversoir rectangulaire, largeur b, charge H au-dessus du seuil) :

```
Q = (2/3) · C_d · b · √(2·|g_eff|) · H^(3/2)         C_d ≈ 0,60
```

Ordres de grandeur utiles : un trou de 10 cm² sous 1 m de charge débite ≈2,7 L/s. Un trou de
1 dm² sous 3 m : ≈47 L/s. Ces chiffres suffisent à équilibrer un gameplay d'avarie sans aucune
simulation fine.

> **Correction (S03).** Ce paragraphe indiquait initialement qu'un tel orifice vide une citerne de
> 1 m³ en ≈6 min. C'est la valeur à **charge constante**. La charge décroît pendant la vidange, et
> le temps réel s'obtient par intégration :
>
> ```
> t_vidange = (A_réservoir / (C_d · a_orifice)) · √(2·h₀/g)  =  728 s ≈ 12 min
> ```
>
> Soit **deux fois plus long**. L'erreur est systématique et affecte tout équilibrage d'avarie
> construit sur le débit initial. Cas de non-régression : `C12` dans `validation/CAS-CANONIQUES.md`.

## 4. Intégration déterministe

- Pas fixe de 100 ms (10 Hz), aligné sur `T_sim`.
- Débits calculés en flottant, puis **quantifiés en millilitres avec report de reste** : chaque
  arête conserve son résidu fractionnaire. Aucune masse n'est créée ni perdue, et le résultat est
  reproductible.
- Limiteur obligatoire : `transfert ≤ min(volume_amont, capacité_libre_aval)`, appliqué après un
  passage de normalisation quand plusieurs arêtes vident le même nœud (sinon un nœud presque vide
  alimente trois fuites et devient négatif).
- Chaînes de nœuds : 2 à 4 itérations de Gauss-Seidel par pas suffisent pour un réseau ouvert.
  **Un réseau fermé sous pression** (canalisations pleines, pompes) demande une résolution
  implicite du champ de pression : reporté en v2, à ne pas improviser.

## 5. Pluie, absorption, création et destruction de flaques

- **Pluie** : `Q = intensité · surface_libre · sky_exposure`. `sky_exposure` est précalculé par
  lancer de rayons hors ligne ; un abri, un pont, un couvercle donnent 0 sans code spécifique.
  Cela répond exactement à `architecture_globale §2.2`.
- **Absorption** : `Q_abs = absorb_rate · surface_de_contact`, propre au matériau du sol.
- **Création d'une flaque** : un transfert sans nœud aval accumule dans un *accumulateur de
  terrain* attaché à la cellule `HydroGrid`. Un nœud n'est instancié qu'au franchissement de
  `V_min` (proposition : **2 L**). En dessous, l'eau n'existe que sous forme d'un état de mouillage
  de surface, sans masse et sans nœud.
- **Destruction** : un nœud dont le volume passe sous `V_min/2` pendant plus de 5 s est retiré et
  converti en mouillage. Hystérésis nécessaire, sinon une fuite lente crée et détruit un nœud
  chaque seconde.

Le mouillage de surface (angle mort A13) est donc le **puits universel** du système : tout ce qui
est trop petit pour être un volume devient de l'humidité qui s'évapore. Cela évite d'avoir à
répondre à la question « quand un transfert devient-il trop faible » par un seuil arbitraire :
en dessous du seuil, l'eau ne disparaît pas, elle change de représentation.

## 6. Couplage V ↔ δ

Un nœud peut demander un domaine δ substitutif (joueur qui entre dans une piscine, eau qui déferle
dans une coursive en gîte). Le passage est un **transfert de propriété de masse**, explicite :

```
V → δ :   le nœud est gelé, sa masse M est remise au domaine, le domaine s'initialise
          au niveau donné par shape_lut(M) orienté selon g_eff
δ → V :   le domaine rend M' = masse mesurée ; l'écart M' − M est reporté comme perte
          contrôlée et journalisé (un écart systématique révèle une fuite du solveur)
```

Le journal d'écart de masse est un **outil de diagnostic**, pas une formalité : un solveur δ qui
perd 3 % de masse par seconde est inutilisable en régime substitutif, et le défaut serait
autrement invisible pendant des mois.

> **Note corrective (S14, [ADR-025](ADR-025-propriete-de-la-masse-entre-V-et-delta.md)).** Le
> **transfert de propriété de masse** décrit ci-dessus est remplacé. Il contredisait **I-04** — la
> masse finale d'un compartiment, donc un chavirement, était déterminée par un solveur δ jamais D1 —
> et il n'avait **aucune histoire côté serveur**, celui-ci exécutant V (ADR-022 §5.1) et jamais δ
> (I-10) : il ne pouvait ni geler le nœud ni recevoir `M'`.
>
> **La masse appartient au nœud en permanence.** Le domaine δ est amorcé depuis `shape_lut(volume)`
> — la partie ci-dessus qui était juste — puis **forcé** vers le volume autoritaire du nœud par une
> relaxation lente (`τ ≈ 1 s`). Il ne rend aucune valeur autoritaire. `M' − M` reste mesuré et
> journalisé, **uniquement comme diagnostic** — ce que ce paragraphe disait déjà être son intérêt
> principal — et devient un critère d'admission en régime substitutif (ADR-025 §3.3).
>
> Aucun message n'est échangé : V étant déterministe bit à bit (I-03), chaque participant intègre la
> même valeur.

## 7. Persistance

Un nœud V est un objet persistant du monde, indexé par cellule `HydroGrid`. Politique proposée :

| Situation | Traitement |
|---|---|
| Nœud d'auteur (citerne, piscine du niveau) | persistant sans limite |
| Nœud créé par le jeu (flaque, inondation) | TTL de 30 min sans visite, puis converti en mouillage puis supprimé |
| Nœud créé par le jeu avec conséquence gameplay (compartiment inondé d'un navire joueur) | persistant, rattaché au propriétaire de l'objet |

> **Note corrective (S18, [ADR-027](ADR-027-les-cinq-arbitrages-tranches.md) §5).** La troisième
> ligne était l'arbitrage n°4 en attente — combien de temps l'eau d'un joueur absent persiste-t-elle ?
> **La question se dissout** : un nœud rattaché à un objet persiste exactement aussi longtemps que
> cet objet, sans règle propre à l'eau. Le chiffre le justifie — 20 octets par nœud, soit **4 Ko pour
> la flotte entière d'un joueur**, négligeable devant l'état du navire lui-même. Une politique
> propre coûterait plus en complexité qu'elle n'économiserait, et produirait un joueur qui retrouve
> son navire intact mais asséché.
>
> **La deuxième ligne reste la nôtre**, et c'est celle qu'il faut défendre : le TTL des nœuds sans
> propriétaire est la **borne supérieure de la persistance de l'eau à l'échelle du monde**
> (ADR-022 §4.3), et il ne s'allonge pas sur un seul argument de jeu.

## 8. Ce qui reste ouvert

1. Réseaux fermés sous pression (v2).
2. `V_min` et les TTL → calibration gameplay.
   → **S11** : l'enjeu a changé de nature. **ADR-022 §4.3** établit que le TTL des nœuds créés par le jeu est
   **la borne supérieure de la persistance de l'eau** : sans lui, chaque flaque jamais revisitée
   d'un monde persistant resterait dans l'état du monde. Ce n'est plus un réglage de confort mais
   un paramètre de **volume de stockage à l'échelle du monde**, et il ne s'allonge pas sur un seul
   argument de jeu.
3. ~~Comportement dans le vide (brèche vers l'espace) : `to_vacuum`.~~
   → **S11 : doublon.** La question est portée par **ADR-015 §7.2**, plus avancé — il nomme la
   formule (gaz parfaits en col sonique). Le mécanisme reste celui décrit ici : un type d'arête
   spécial `to_vacuum`. Dépendance inter-équipes : **gameplay spatial**, cinquième destinataire
   extérieur, absent de la liste de `00_INDEX.md` jusqu'en S11.
4. Mélange de liquides différents dans un même nœud (eau + carburant) : autorisé ou interdit ?
   Interdire est plus simple et probablement suffisant ; à confirmer.


## Note factuelle S227 — 2026-09-13 : géométrie et limiteur du noyau construit

**A266 reste ouverte.** La table horizontale du §2 ne donne pas le décalage normal du plan sous
une gravité inclinée. La réserve touche **aussi les prismes** dans le module S226 : à pente 0,3,
la cote centrale y vaut 1,04403 m pour 1 m attendu ; hublot central à 1,01 m, fuite de 506 ml en
dix pas. Voir la [correction de portée S227](../validation/GRAVITE-DIRIGEE-S226.md#note-corrective-s227--2026-09-13--la-portée-da266-comprend-les-prismes).
L'orientation correcte ne suffit pas à recevoir le volume géométrique. Cette note ne choisit
pas la nouvelle représentation ; le prochain ADR doit remplacer cette disposition.

**Le limiteur du §4 porte aussi sur les sommes entrantes.** Le noyau S224–S226 bornait chaque
arrivée séparément : trois sources remplissaient trois fois une même capacité libre. S227 ajoute
une réduction collective après les limites existantes, sans masse perdue ; la place libérée par
les sorties n'est disponible qu'au pas suivant. Cette correction d'implémentation ne reçoit pas
encore le réseau pressurisé ni les grandes topologies. Les parcours restent en O(nœuds × arêtes).

## Remplacement ciblé S228 — 2026-09-13

[ADR-139](ADR-139-volume-et-plan-oriente-des-contenants.md) remplace l'usage universel de la
table horizontale du §2 : le plan orienté est désormais inversé depuis le volume de la géométrie.
Les anciennes tables gardent leur seul domaine +Z. Lois de débit, autorité et état entier restent.

## Note datée S372 — 2026-09-26 : vannes et pompes

La vanne et la pompe du §1 sont construites en **réseau ouvert** ([ADR-199](ADR-199-vannes-et-pompes-dans-v.md),
[preuve](../validation/VANNES-POMPES-S372.md)) : une commande entière par arête, la vanne comme ouverture commandée, la
pompe par une courbe parabolique contre la hauteur statique. Le réseau fermé sous pression du §4 reste en v2.
