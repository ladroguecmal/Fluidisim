# ADR-023 — Quatre mécanismes restés à spécifier

- **Statut** : proposée
- **Session** : S12
- **Ferme** : ADR-008 §5.3 et §5.4, ADR-013 §7.4, ADR-015 §7.3 — les quatre points remontés par
  [`AUDIT-POINTS-OUVERTS-S11`](../registres/AUDIT-POINTS-OUVERTS-S11.md) §7.4
- **Dépend de** : ADR-008, ADR-010, ADR-013, ADR-014, ADR-015, SPEC-001, SPEC-002, SPEC-006

---

## 1. Pourquoi ces quatre-là, et ce qu'ils ont en commun

L'audit S11 a trouvé quatre points ouverts qui disaient « à spécifier » sans qu'aucune session ne
l'ait jamais pris en charge : ce n'étaient ni des mesures, ni des arbitrages, ni des dépendances
inter-équipes, mais du travail de conception rendu invisible par l'endroit où il était inscrit.

**Leur origine est commune ; leur cause ne l'est qu'à moitié**, et il vaut mieux le dire que de
forcer une unification qui n'existe pas :

- le **terme d'impact** (§2) et le **nageur** (§3) sont les **deux frontières du domaine de validité
  du modèle de flottabilité d'ADR-008** — l'une dans le temps, l'impact étant plus bref que le tick ;
  l'autre dans la nature du corps, un nageur n'étant pas passif ;
- les **sites turbulents permanents** (§4) et la **coalescence des poches d'air** (§5) n'ont de
  rapport ni avec eux ni entre eux.

Deux des quatre se résolvent en **élargissant un mécanisme existant** plutôt qu'en en écrivant un
nouveau, et un troisième en réutilisant un canal déjà spécifié. C'est le résultat qu'on doit
attendre d'un corpus mûr, et le signe qu'il l'est.

| § | Mécanisme | Résolution | Nouveau mécanisme ? |
|---|---|---|---|
| 2 | Terme d'impact (*slamming*) | impulsion de masse ajoutée, sous-tick | **oui**, mais il réutilise le tenseur de masse ajoutée d'ADR-008 §2 |
| 3 | Nageur en surface | seconde condition d'entrée au mode contraint d'ADR-008 §3 | **non** |
| 4 | Sites turbulents permanents | polyligne de déferlement dégénérée, dérivée de `H/h = 0,78` | **non** |
| 5 | Coalescence des poches T2 | addition des moles, niveau commun par `shape_lut` | **non** |

---

## 2. Le terme d'impact — ce qu'on publie n'est pas la pression

### 2.1 Pourquoi l'intégration quasi-statique le manque

Le modèle d'ADR-008 §2 intègre la pression sur des points d'échantillon à chaque tick. Une entrée
dans l'eau est plus brève que le tick, et l'échantillonnage la traverse sans la voir.

Théorie de Wagner : une carène de **relèvement de fond** `β` entrant à la vitesse `v` mouille une
demi-largeur qui croît comme `c(t) = (π/2)·v·t/tan β`. Le temps pour mouiller une demi-largeur `b` :

```
t_impact = 2·b·tan β / (π·v)
```

| Cas | `b` | `β` | `v` | `t_impact` | à 30 Hz |
|---|---|---|---|---|---|
| Étrave de vedette retombant de vague | 1,0 m | 30° | 5 m/s | **73 ms** | 2,2 ticks |
| Corps humain tombant de 3 m | 0,20 m | 45° | 7,7 m/s | **17 ms** | 0,5 tick |
| Coque plate claquant à plat | 2,0 m | 10° | 4 m/s | **56 ms** | 1,7 tick |

**L'impact dure de l'ordre du tick, ou moins.** Un terme de force échantillonné à 30 Hz le rate ou
le double selon la phase — et le défaut est intermittent, donc introuvable.

### 2.2 La pression de pic est ce qu'on ne sait pas

Le coefficient de pression maximal d'un dièdre (Wagner) vaut :

```
C_p = 1 + ( π / (2·tan β) )²          p_max = C_p · ½ ρ v²
```

| `β` | `C_p` | `p_max` à `v = 5 m/s` | à `v = 10 m/s` |
|---|---|---|---|
| 45° | 3,5 | 43 kPa | 173 kPa |
| 30° | 8,4 | 105 kPa | 420 kPa |
| 20° | 19,6 | 245 kPa | 980 kPa |
| 10° | 80,4 | **1,0 MPa** | **4,0 MPa** |
| 5° | 323 | 4,0 MPa | 16 MPa |

Deux enseignements, et ils vont dans des directions opposées :

- **c'est l'angle qui domine, pas la vitesse.** Passer de 30° à 10° multiplie la pression par
  **dix** ; doubler la vitesse ne la multiplie que par quatre. Un fond plat est le vrai danger, et
  c'est la raison d'être du relèvement de carène sur toute coque rapide ;
- **la formule diverge quand `β → 0`.** À relèvement nul elle prédit une pression infinie, ce qui
  est faux : le coussin d'air piégé et la compressibilité de l'eau l'écrêtent. **On ne connaît donc
  pas la pression de pic**, et un modèle qui la publie publie son incertitude.

### 2.3 Décision : publier l'impulsion, pas la pression

> **Le terme d'impact est une impulsion de masse ajoutée, appliquée sur le tick, et non une force
> de pression échantillonnée.**

Quand la carène mouille une demi-largeur `c`, elle entraîne l'eau que la théorie de la masse ajoutée
d'une plaque plane de demi-largeur `c` décrit : `m_a = ½·π·ρ·c²` par mètre de longueur. La quantité
de mouvement verticale transmise à l'eau entre le début et la fin de la pénétration vaut donc :

```
J = Δ(m_a) · v_rel              par mètre de longueur mouillée
```

| Cas | `c` final | `m_a` (par m) | `v_rel` | `J` (par m) | Section de 10 m |
|---|---|---|---|---|---|
| Étrave de vedette | 1,0 m | 1 571 kg/m | 5 m/s | 7,85 kN·s/m | 78,5 kN·s |
| Corps humain | 0,20 m | 62,8 kg/m | 7,7 m/s | 0,48 kN·s/m | *(≈0,3 kN·s au total)* |

Sur les 73 ms de l'étrave, cela fait une force moyenne de l'ordre de **1,1 MN** — cohérente avec la
pression de pic de 105 kPa appliquée sur la surface mouillée, ce qui est le contrôle qu'il fallait
faire.

**Trois raisons de préférer l'impulsion :**

1. **c'est un bilan de quantité de mouvement, il ne peut pas être faux.** La pression de pic dépend
   d'une théorie qui diverge ; l'impulsion, non — elle est bornée par la masse d'eau réellement
   accélérée ;
2. **c'est ce que l'intégrateur du solide veut recevoir.** Un moteur de corps rigides applique une
   impulsion en une opération exacte ; une force de 1,1 MN appliquée pendant un tick de 33 ms donne
   un résultat qui dépend du schéma d'intégration ;
3. **elle réutilise une grandeur déjà là.** ADR-008 §2 impose déjà un tenseur de masse ajoutée par
   archétype. Le terme d'impact n'ajoute pas de modèle physique, il exploite en régime transitoire
   celui qui servait en régime établi.

### 2.4 Le terme d'impact est autoritaire

Ses entrées sont l'état du solide — répliqué par la physique et le réseau — et `η`, `v_eau` issus de
**B + W** (ADR-008 §2). Aucune ne vient de δ. Il satisfait donc **I-15** et relève de la ligne
« poussée d'Archimède » de la table d'autorité d'ADR-008 §1, sans exception ni aménagement.

C'est ce qui permet qu'un claquement de coque **casse quelque chose** : dégât de structure, chute
d'un personnage, largage de cargaison. Un terme d'impact calculé depuis δ n'aurait pas pu, et
l'ensemble aurait dû être refait.

### 2.5 Sous-cyclage et déclenchement

L'impact est détecté quand la vitesse normale relative d'un point d'échantillon franchit la surface
vers le bas au-delà d'un seuil. Valeur de départ **`v_rel·n > 2 m/s`**, à calibrer : en deçà,
l'impulsion est inférieure au bruit de la flottabilité quasi-statique elle-même.

L'impulsion est intégrée **analytiquement sur le tick** à partir de `t_impact` (§2.1) et de la
vitesse d'entrée, sans sous-cyclage du solveur : les deux grandeurs sont connues en forme fermée,
et sous-cycler ne ferait qu'échantillonner plus finement une courbe qu'on sait intégrer.

Le relèvement `β` est une propriété d'archétype de coque, au même titre que le tenseur de masse
ajoutée — pas une mesure faite à l'exécution sur la géométrie.

### 2.6 Point de rupture, et le banc qui l'attrape

La théorie de Wagner suppose une entrée **rapide devant la période de la houle** et une surface
localement plane. Elle cesse de valoir quand la vitesse d'entrée devient comparable à la vitesse
orbitale de la surface — cas d'une coque qui accompagne la vague plutôt que de la percuter. Le
critère de déclenchement de §2.5 porte donc sur la vitesse **relative**, et non absolue, ce qui
traite le cas sans mécanisme supplémentaire.

À vérifier au banc **B6** (flottabilité), avec deux références analytiques : la conservation de la
quantité de mouvement totale eau + solide, et la décroissance en `1/tan β` de la durée d'impact.
Cas canonique **C20**, ajouté à `CAS-CANONIQUES.md`.
