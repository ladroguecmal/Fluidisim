# SPEC-004 — Signatures des interfaces

- **Statut** : proposée — dernier document avant l'écriture de code
- **Session** : S04
- **Dépend de** : ADR-007 (emplacements de solveur), ADR-020 (bibliothèque autonome), SPEC-003 (harnais)
- **Détaille** : ADR-007 §2 et ADR-020 §2
- **Complétée par** : [`SPEC-006`](SPEC-006-chemin-pousse.md) *(S09)* — le **chemin poussé**, c'est-à-dire
  ce que le système publie de sa propre initiative. Ce document-ci ne couvre que le chemin **tiré**
  et le **branchement**. Trois corrections de S09 y sont reportées ci-dessous.

Les signatures sont données dans un pseudo-C++ qui se transpose sans perte en C++ ou en Rust. Le
choix du langage reste ouvert et n'a aucune incidence ici.

---

## 1. Règles générales — valables pour toutes les interfaces

### 1.1 Unités et repères

| Grandeur | Type | Unité | Repère |
|---|---|---|---|
| Temps | `SimTime` = `u64` | microsecondes | `T_sim`, absolu monde (ADR-003) |
| Position | `vec3<f32>` | mètres | **local au référentiel**, `|x| < 4096 m` (I-08) |
| Durée de pas | `f64` | secondes | — |
| Budget | `f32` | millisecondes | — |
| Volume | `i64` | millilitres | couche V uniquement (ADR-010) |

**Aucune fonction de cette spécification n'accepte une coordonnée monde.** Le type ne l'exprime
pas. La conversion est un acte explicite de l'hôte, effectué une fois par lot.

Les API chaudes travaillent **par lot** : un `FrameId` porté une fois, puis un tableau de
positions. La forme scalaire existe pour la commodité et n'est jamais utilisée dans une boucle
chaude — c'est aussi ce qui rend le coût de l'indirection d'ADR-020 négligeable.

### 1.2 Contrat de fils d'exécution

Colonne la plus importante du document. Elle est rarement écrite, et son absence est ce qui rend
une interface impossible à utiliser correctement six mois plus tard.

| Fonction | Appelable depuis | Concurrente | Peut bloquer | Peut allouer |
|---|---|---|---|---|
| `EvalWaterBatch` | n'importe quel fil | **oui, N lecteurs** | non | non |
| `IFluidSolver::step` | ordonnanceur uniquement | non | non | scratch seul |
| `IFluidSolver::sample_batch` | n'importe quel fil | oui | **jamais** | non |
| `IWaveSolver::inject_event` | fil de simulation | non | non | pool seul |
| `IWaveSolver::publish_snapshot` | fil de simulation | non | non | pool seul |
| `IBathymetryProvider::try_get` | n'importe quel fil | oui | **jamais** | non |
| `ISink::*` | n'importe quel fil | oui | non | non |

**Conséquence non évidente.** `EvalWaterBatch` étant lue par le fil physique, le fil audio et le
fil d'IA pendant que la simulation avance, la couche W ne peut pas être mutée en place. Elle
publie un **instantané immuable par tick** ; les lecteurs prennent une poignée et la conservent le
temps de leur travail. Ce n'est pas une optimisation : sans cela, un lecteur voit un jeu de paquets
à moitié mis à jour, et le défaut est intermittent, rare et introuvable.

### 1.3 Erreurs

Aucune exception ne franchit une interface. Toute fonction faillible renvoie un `Result`.

Trois niveaux, distincts :

- **Violation de contrat** (dx hors bornes, pointeur nul, référentiel inconnu) — assertion en
  développement, comportement **défini** en production : valeur bornée, journalisation, jamais de
  comportement indéterminé.
- **Échec attendu** (données non résidentes, budget insuffisant) — code de retour, à traiter.
- **Mauvaise configuration** — détectée par `validate_config()` au chargement, jamais à
  l'exécution. Une eau mal configurée doit échouer bruyamment au démarrage, pas produire
  silencieusement une mer fausse.

### 1.4 Versionnement

Chaque interface porte `static constexpr u32 ABI_VERSION`. Les structures de capacités ne
grandissent que par ajout en fin, avec un champ `size` en tête. Un solveur fourni par un tiers doit
pouvoir être compilé séparément.

---

## 2. Types fondamentaux

```cpp
using SimTime = uint64_t;                   // µs, T_sim
struct FrameId  { uint32_t v; };
struct DomainId { uint32_t v; };
struct ProxyId  { uint32_t v; };

enum class Layer : uint8_t { B = 1, W_rep = 2, W_local = 4, Delta = 8 };
struct LayerMask { uint8_t bits; };

// Consommateurs gameplay, audio, IA, rendu — ADR-004 §4
struct WaterSample {
    float eta;              // élévation le long du radial local
    vec3  displacement;     // déplacement de Gerstner
    vec3  u_total;          // vitesse de surface = ORBITALE + COURANT. Le nom le dit — cf. §9
    vec3  normal;
    float deta_dt;
    float steepness;        // pour écume et seuil de déferlement
    float aeration;         // fraction volumique d'air — ADR-014 §5.2
};
```

### 2.1 Deux types d'échantillon, et pourquoi les fusionner serait une erreur

> **Réception S182 — 2026-09-12.** Le consommateur différentiel mixte est reçu
> dans les chemins d'actualisation, admission, saturation/reprise, renouvellement et
> restauration existants : dérivées et source identiques en bits à la reconstruction
> directe. [CYCLE-DIFFERENTIEL-S182](../validation/CYCLE-DIFFERENTIEL-S182.md).
> Aucun changement de contrat ADR-117 ; coût et consommateur perturbatif non reçus.

> **Complément S181 — 2026-09-12.** La requête mixte différentielle par lot de
> WorldPos compose B, les impacts du journal et la pression publiée, dans cet ordre.
> Contexte et instant utilisent les contrôles du montage existant ; sortie atomique,
> profondeur relative au plan moyen. DifferentialSample conserve rho et calcule la
> source après sommation ; la pression imposée est déjà dans p_dyn.
> [ADR-117](../adr/ADR-117-composition-differentielle-mixte.md),
> [COMPOSITION-DIFFERENTIELLE-S181](../validation/COMPOSITION-DIFFERENTIELLE-S181.md).
> Les cycles vivants avec ce consommateur restent à recevoir sous S181-1.

> **Complément S180 — 2026-09-12.** Le Field spectral préparé fournit les dérivées
> profondes à son instant de préparation : PressureDifferential contient le
> BackgroundSample, la densité et la pression imposée de surface avec son gradient.
> p_dyn comprend déjà la prolongation profonde de cette pression : ne pas l'ajouter
> une seconde fois à la source volumique. Aux commutations, dérivée de la branche
> active ; sortie par lot atomique. [ADR-116](../adr/ADR-116-differentiel-de-pression-forcee.md)
> et [DIFFERENTIEL-PRESSION-S180](../validation/DIFFERENTIEL-PRESSION-S180.md).
> Composition multisource et exposition monde restent à recevoir (S180-1).

> **Complément S179 — 2026-09-12.** RadialImpact fournit le même type différentiel
> profond et sa composition locale avec B, avec lot atomique sur scratch hôte.
> Limite du gradient au centre dérivée, g/rho de construction conservés ; contexte
> de repère B/W déclaré par l'hôte. ADR-115 et
> [DIFFERENTIEL-W-S179](../validation/DIFFERENTIEL-W-S179.md). Pression forcée,
> multisource et solveur δ3D restent à construire pour cette interface.

> **Complément S178 — 2026-09-11.** B fournit aussi `grad_p_dyn` (Pa/m) et
> `laplacian_u` (1/(m s)). `BackgroundSample::momentum_residual(rho,nu)` forme le
> résidu continu S, à **soustraire**, en m/s² ; rho identique à l'échantillonnage,
> nu cinématique uniforme. Hydrostatique et gravité déjà compensées. ADR-114 et
> [SOURCE-B-S178](../validation/SOURCE-B-S178.md). B+W et fermeture δ restent ouverts.

> **Implémentation partielle S177 — 2026-09-11.** `background::BackgroundSample` et
> les méthodes `Background::differential*` existent pour B profond linéaire uniforme.
> z est relatif au plan moyen, z<=0 ; grad_u[i][j]=∂u_i/∂x_j, p_dyn en Pa avec rho
> fourni. `WaterSample` historique inchangé. ADR-113 et
> [FOURNISSEUR-B-S177](../validation/FOURNISSEUR-B-S177.md) fixent cette première réception.
> S178 ajoute gradient de pression, Laplacien et résidu continu (ADR-114) ; B+W et
> fermeture du solveur ne sont pas reçus par ce seul type.

```cpp
// Consommateur unique : un solveur δ en régime perturbatif — cf. §6
struct BackgroundSample {
    float eta;
    vec3  grad_eta;
    vec3  u;
    vec3  du_dt;            // dérivée temporelle locale
    mat3  grad_u;           // gradient de vitesse
    float p_dyn;
    vec3  grad_p_dyn;       // Pa/m
    vec3  laplacian_u;      // 1/(m s)
};
```

`BackgroundSample` pèse ≈80 octets contre ≈56 pour `WaterSample`, et son calcul coûte plusieurs
fois plus cher (dérivées analytiques de toutes les composantes). Le fusionner ferait payer à
chaque requête de flottabilité, d'audio et d'IA des dérivées dont elles n'ont aucun usage.

> **Correction (S09, écart E05).** Le champ s'appelait `u`. Renommé **`u_total`** parce qu'un nom
> court invitait à le prendre pour le courant : le produit d'emportement de SPEC-002 §5,
> `HR = d·(v+0,5)`, attend un **courant**, et calculé sur la vitesse totale il oscillerait à la
> période de la houle avec une amplitude `πHs/T` = 0,63 m/s à `Hs = 1 m`, `T = 5 s` — le double du
> seuil qui sépare « faible » de « dangereux ».
>
> La grandeur juste, le courant seul, est publiée par le **chemin poussé** :
> `TraversabilitySample.flow_speed` (SPEC-006 §5.5). Ce n'est pas un doublon : ce document sert
> l'interrogation ponctuelle, l'autre sert la navigation, et ils n'ont ni la même cadence ni le
> même régime d'autorité.

> **`WaveEvent` — où il est défini (S09, écart E04).** Cette spécification l'employait sans le
> définir ; il vivait dans ADR-009 §2, un document de réseau, alors qu'il traverse trois frontières.
> Sa définition de référence, avec les trois champs demandés par ADR-016 §6, est désormais
> **[`SPEC-006` §3.1](SPEC-006-chemin-pousse.md)**. Elle n'est pas recopiée ici : une donnée a une
> seule source de vérité (SPEC-005 §1), et cela vaut aussi pour une déclaration de type.

---

## 3. Point d'entrée du système

```cpp
class WaterSystem {
public:
    static Result create(const WaterConfig&, const HostServices&, WaterSystem** out);

    void      begin_tick(SimTime t, double dt);
    void      push_solid_proxies(FrameId, span<const SolidProxyDesc>);
    void      push_events(span<const WaveEvent>);          // venus du réseau — ADR-009
    void      end_tick();                                   // ordonnance, exécute, publie

    void      eval_water_batch(FrameId, span<const vec3> pts,
                               SimTime t, LayerMask, SampleHints,
                               span<WaterSample> out) const;

    const IEventChannel&  events() const;                   // bus à N lecteurs — SPEC-006 §3
    const Metrics&        metrics() const;                  // SPEC-003 §6
};

> **Correction (S09, écart E04).** `drain_outgoing_events()` figurait ici et a été **supprimée**,
> pour deux motifs distincts et tous deux suffisants.
>
> 1. **Un drain a un consommateur unique** : le premier qui appelle vide la file pour les autres.
>    Tolérable tant que la transduction δ→W était son seul client ; intenable dès qu'ADR-016 §2 fait
>    du même flux le bus de l'audio, que le rendu y prend ses déclenchements et le gameplay ses
>    causes.
> 2. **Le mot « outgoing » était un résidu.** Il datait de la conception qu'ADR-021 §3 a remplacée,
>    où un client remontait ses événements transduits au serveur. Ce chemin n'existe plus : la
>    transduction ne produit que du `W_local`, injecté dans W *à l'intérieur* du système. L'écart
>    R03 de S05 avait supprimé le chemin de données ; la fonction qui le servait avait survécu.
>
> Le remplacement est `events()`, publication immuable à N lecteurs — SPEC-006 §3.2.
```

`SampleHints` porte le **LOD spectral par lot** (ADR-004 §4) : longueur d'onde minimale à sommer,
et non un niveau de détail par point. Une bouée ne demande pas les composantes de 500 m ; un
porte-conteneurs ne demande pas celles de 30 cm.

`HostServices` est l'agrégat des sept services d'ADR-020 (§8 ci-dessous).

---

## 4. `IFluidSolver` — couche δ

```cpp
struct SolverCaps {
    uint32_t size;                    // pour l'extension par ajout
    bool  substitutive;               // peut posséder le champ total
    bool  air_phase;                  // poches, cavités — ADR-015
    bool  moving_solids;
    bool  variable_gravity;           // g_eff par pas — I-07
    bool  needs_background_derivatives;
    float dx_min, dx_max;
    uint32_t max_blocks;
    DeterminismTier tier;             // D1 / D2 / D3 — SPEC-003 §2
    uint8_t latency_frames;           // 0 = CPU synchrone
};

struct StepResult {
    float    consumed_ms;
    SimTime  valid_at;                // état correspondant exactement à cet instant
    uint32_t substeps;
    Degrade  degraded;                // bitflags — voir ci-dessous
    float    work_remaining;          // 0 = pas complet
    float    max_cfl;
    float    mass_drift_per_s;
};

enum class Degrade : uint32_t {
    None = 0, SubstepsCut = 1, PressureItersCut = 2,
    BlocksDropped = 4, FrequencyHalved = 8, SecondaryDisabled = 16
};

class IFluidSolver {
public:
    virtual SolverCaps caps() const = 0;
    virtual Result validate_config(const DomainConfig&) const = 0;
    virtual Result configure(const DomainConfig&, IBackgroundField*, IGravityField*) = 0;

    virtual void add_blocks(span<const BlockCoord>) = 0;      // initialisés à δ = 0
    virtual void remove_blocks(span<const BlockCoord>) = 0;
    virtual void set_solid_proxies(span<const ProxyId>) = 0;

    virtual StepResult step(double dt_target, float budget_ms) = 0;

    virtual uint64_t sample_batch(span<const vec3> pts,
                                  span<DeltaSample> out) const = 0;   // renvoie l'âge en µs

    virtual span<const SectorFlux> drain_boundary_flux() = 0;
    virtual Result restore(const SeedState&) = 0;      // amorçage depuis une graine — ADR-022 §3
    virtual void   reset() = 0;
};

// Obtenue UNIQUEMENT par un hôte de cuisson (SPEC-005 §7.1).
// Un hôte de jeu n'en reçoit jamais de pointeur : il ne peut donc pas condenser.
class ISeedProducer {
public:
    virtual Result condense(SeedState* out) const = 0;
};
```

### 4.1 Trois points de contrat qui ne se négocient pas

**`step` avance exactement `dt_target`, ou dit qu'il ne l'a pas fait.** La somme des sous-pas vaut
`dt_target` ; sinon `valid_at < t + dt_target` et `work_remaining > 0`. Sans cette règle, deux
solveurs qui sous-cyclent différemment finissent à des instants différents et aucune comparaison
n'a de sens (SPEC-003 §5.2).

**La dégradation est rapportée, jamais silencieuse.** Un solveur qui coupe ses itérations de
pression pour tenir son budget doit le dire. Une dégradation silencieuse est une dégradation non
mesurable : l'ordonnanceur ne peut pas réviser sa priorité, et le banc `starve` (SPEC-003 §9.1) ne
peut rien constater.

**`mass_drift_per_s` est rapporté à chaque pas, pas seulement en test.** C'est le capteur qui
détecte un solveur qui fuit en production. Le coût est d'une soustraction.

> **Correction (S10, [ADR-022](../adr/ADR-022-persistance-de-l-eau.md)).** `condense` figurait ici,
> aux côtés de `restore`, avec `CondensedState`. Deux changements :
>
> - le type est désormais `SeedState` — une **condition initiale 2D cuite**, généralisation du
>   `CoastalState` de SPEC-005 §6 — et non la sérialisation d'un état de solveur. Aucun état de δ
>   n'est jamais sérialisé (**I-17**) ;
> - `condense` **quitte cette interface** pour `ISeedProducer`, que seul un hôte de cuisson obtient.
>   Laissée sur `IFluidSolver`, où tout hôte y accède, elle aurait fini par être appelée en jeu.
>   Suivant L19, l'interdit est rendu inexprimable plutôt qu'écrit dans une consigne de revue.

---

## 5. `IWaveSolver` — couche W

```cpp
class IWaveSolver {
public:
    virtual WaveCaps caps() const = 0;
    virtual Result configure(const RegionConfig&,
                             IBathymetryProvider*, IHydroSampleProvider*) = 0;

    virtual Result inject_event(const WaveEvent&) = 0;   // ordonné par event.id — ADR-009 §2
    virtual void   advance(SimTime t) = 0;

    virtual SnapshotHandle publish_snapshot() = 0;       // immuable, N lecteurs — §1.2
    virtual void  release_snapshot(SnapshotHandle) = 0;

    virtual void  sample_batch(SnapshotHandle, span<const vec3>, SimTime,
                               span<WaterSample> out) const = 0;
    virtual HeightFieldHandle height_field(RegionId) const = 0;  // rendu, domaine substitutif

    virtual uint32_t active_packets() const = 0;
    virtual void     prune(uint32_t max_packets) = 0;    // dégradation rang 6 — ADR-012 §4
};
```

### 5.1 Une exigence de cette interface tranche partiellement le banc B2

`advance(t)` doit être une **fonction pure du journal d'événements et de `t`**. C'est ce qui rend
l'arrivée en cours de partie gratuite (ADR-003 §3) et le déterminisme D1 atteignable.

- Des **paquets d'ondes lagrangiens** satisfont cette exigence par construction : chaque paquet est
  entièrement déterminé par son événement source et par le temps écoulé.
- Un **champ de hauteur 2D intégré** ne la satisfait pas : son état à `t` dépend de tout
  l'historique d'intégration. Le déterminisme D1 exige alors des **points de reprise** périodiques,
  qu'il faut stocker, répliquer et transmettre à tout joueur qui rejoint — précisément le coût
  réseau qu'ADR-009 avait supprimé.

Ce n'est pas une décision : c'est un **coût caché du candidat « champ 2D » qui doit entrer dans le
protocole de B2**, faute de quoi la comparaison se fera sur la seule qualité visuelle et retiendra
l'option la plus chère en réseau sans que personne ne s'en aperçoive avant l'intégration.

---

## 6. `IBackgroundField` — le contrat perturbatif

C'est l'interface la plus subtile de la spécification, et celle dont une implémentation incomplète
ferait paraître ADR-001 défaillant pour des raisons indiagnostiquables.

```cpp
class IBackgroundField {
public:
    virtual void sample_batch(span<const vec3> pts, SimTime t,
                              span<BackgroundSample> out) const = 0;
    virtual bool is_smooth_at(float dx) const = 0;    // autorise l'échantillonnage grossier
};
```

> **Correction (S05, écart R12).** Cette interface portait aussi `lambda_cut()`. `λ_cut` est la
> frontière entre les couches W et δ : elle appartient à `WaterConfig` et transite par
> `DomainConfig`. Le champ de fond n'a aucune raison de la connaître, et la lui confier aurait fait
> d'une décision d'architecture un attribut d'implémentation.

### 6.1 Pourquoi les dérivées sont indispensables

> **Clarification S176 — 2026-09-11.** La formule différentielle ci-dessous décrit
> un résidu physique continu ; elle ne définit pas àelle seule le résidu d’un schéma
> discret. Les essais S167–S175 distinguent `Lphys(Q)-Qt` et `Lnum(Q)-Qt` (convention
> conservative1D, signe opposé au `S` soustrait ci-dessous). Le second retrouve le
> solveur total numérique ; le premier peut préserver un fond physique connu.
> Il faut déclarer opérateur et quadratures dans l’implémentation, sans les confondre.
> Aucun schéma3D choisi. Voir [BILAN-B4-S176](../validation/BILAN-B4-S176.md).

En régime perturbatif on écrit `u = U + u'`, `p = P + p'`, où `(U, P)` est le fond. En reportant
dans les équations du mouvement, l'équation de la perturbation fait apparaître un **terme source**
égal au résidu du fond dans les équations discrètes :

```
∂u'/∂t + (U·∇)u' + (u'·∇)U + (u'·∇)u' = −∇p'/ρ + ν∇²u' − S
avec   S = ∂U/∂t + (U·∇)U + ∇P/ρ − ν∇²U
```

`S` n'est pas nul : le fond résout les équations d'ondes **linéaires**, pas les équations discrètes
du solveur. Un solveur qui ne reçoit que `η` et `u` ne peut pas former `S`, donc résout une
équation qui n'est pas celle de la perturbation. Le symptôme : le domaine dérive lentement par
rapport au fond, la frontière redevient visible, et l'on conclut à tort que la décomposition en
couches ne fonctionne pas.

**À vérifier explicitement au banc B4**, qui est le juge d'ADR-001 : si B4 échoue, la première
hypothèse à écarter est un terme source incomplet.

### 6.2 L'optimisation qui rend le coût acceptable

> **Suivi S194 — 2026-09-12 :** [le couplage de deux trains est mesuré](../validation/COUPLAGE-DEUX-TRAINS-S194.md),
> [ADR-123](../adr/ADR-123-le-domaine-de-validite-de-la-superposition.md). Additionner deux
> sources évoluées indépendamment tient sous 2 % seulement en dessous d'une cambrure de
> **0,009** par train en eau profonde ; à **0,0125** le budget est franchi en **5,4
> périodes**, à **0,014** avant la fin de la première. **A217 reçoit sa réponse** : la
> variable qui gouverne l'addition en eau profonde est la cambrure, avec la durée et le
> désaccord de triade comme deux variables supplémentaires qu'A217 ignorait. L'écart varie
> de 13 % au plus sur quatre géométries de couple : la cambrure décide, pas la géométrie.
> **Voie de correction chiffrée** : l'écart est la réponse à un forçage croisé explicite,
> et corriger sa seule part quadratique suffirait près de la frontière. `n` sources ne sont
> pas extrapolables (A240).

> **Suivi S193 — 2026-09-12 :** [surface non linéaire dispersive reçue contre Stokes](../validation/SURFACE-LIBRE-NL-S193.md),
> ordre en amplitude **M=3** acté par [ADR-122](../adr/ADR-122-l-ordre-en-amplitude-d-un-vehicule-non-lineaire.md) :
> harmonique liée à **0,4555 %** et décalage de fréquence à **1,6454 %**, sous 2 %.
> L'ordre deux rend le bon profil mais **la moitié** du décalage de fréquence, et la
> fraction captée dépend du régime : vérifier un profil ne suffit pas à recevoir un
> schéma tronqué en amplitude. Le fournisseur S191 n'est toujours pas injecté, aucun
> couplage de deux trains n'est mesuré, A217 reste ouverte. Suite S193-1.

> **Suivi S192 — 2026-09-12 :** [surface libre x-z linéaire reçue Airy](../validation/SURFACE-LIBRE-2D-S192.md),
> trois profondeurs, cinq périodes, vitesse fine≤1,732796 %. Ce véhicule n'injecte
> pas encore le fournisseur S191 ; aucune réception couplée ni addition de ces
> erreurs au budget source. Suite S192-1 : non-linéarité de surface/référence Stokes,
> puis comparaison perturbatif/total. Seuil2 % acquis ; A50 reste partielle.

> **Suivi S191 — 2026-09-12 :** le profil S190 est reçu avec projection discrète,
> [PROJECTION-B4-S191](../validation/PROJECTION-B4-S191.md), budget **1,371947 %**
> sous 2 %. [ADR-121](../adr/ADR-121-la-projection-lineaire-et-la-borne-de-composition.md)
> distingue l'enveloppe mesurée s+t de la borne algébrique avec résidu. Aucun `N`
> universel ni `is_smooth_at` runtime reçu ; surface libre et bords physiques à construire.

> **Décision active S190 — 2026-09-12 : le critère est fixé à 2 %** d'erreur de
> champ perturbatif par l'utilisateur, [ADR-120](../adr/ADR-120-b4-tolerance-de-deux-pour-cent.md).
> La réception conserve la norme maximum de vitesse `u'` de S185, normalisée par la
> perturbation de référence ; budget conjoint espace+temps, référence qualifiée.
> `N` est un nombre de points, pas le pourcentage. Il se dimensionne avec le placement,
> la profondeur et la cadence sous ce seuil. [S190](../validation/B4-TOLERANCE-S190.md)
> reçoit un profil de banc **14×14×8, extrapolation 80 ms**, budget 1,800653 % réserve
> comprise. Pas de `N` universel ni de garantie runtime `is_smooth_at` déduite de ce
> seul montage. Les mentions historiques « erreur acceptable à décider » ci-dessous
> sont remplacées par cet arbitrage ; le facteur 64 reste non reçu.

`BackgroundSample` est cher, et un domaine perturbatif en demande un par cellule et par sous-pas.

Le fond est cependant **lisse à l'échelle de `dx`** : ses longueurs d'onde valent au minimum
`λ_cut`, soit des dizaines de cellules. Il est donc échantillonné sur un réseau grossier — une
cellule sur quatre par axe est le point de départ proposé — puis interpolé trilinéairement.

Facteur d'économie : **64**. C'est ce qui fait passer le terme source de rédhibitoire à
négligeable, et c'est le rôle de `is_smooth_at(dx)` que de garantir la validité de
l'approximation.

> **Note corrective (S08, écart E08).** « Une cellule sur quatre » n'est pas une constante : c'est
> le rapport `λ_cut/dx` qui décide, et il varie d'un facteur 2,5 entre deux régimes déjà écrits.
>
> | Régime | `dx` | `λ_cut/dx` | points par `λ_cut` après décimation ×4 |
> |---|---|---|---|
> | Scénario nominal (SPEC-003 §3) | 0,10 m | 40 | 10 — confortable |
> | Zone de déferlement (SPEC-001 §2.4) | 0,25 m | 16 | **4 — deux fois Nyquist** |
>
> (`λ_cut = 4 m`, valeur de départ proposée par ADR-005.) À quatre points par longueur d'onde,
> l'interpolation trilinéaire perd une fraction notable de l'amplitude de `S` — et le symptôme est
> précisément celui que §6.1 décrit comme indiagnostiquable. L'économie s'effondrerait donc dans le
> type de domaine le plus gros, celui-là même qu'elle devait rendre abordable.
>
> **La contrainte s'écrit `dx ≤ λ_cut/N`**, `N` à fixer au banc B4 dont c'est déjà un paramètre
> direct (§10.3). `is_smooth_at(dx)` renvoie faux quand elle n'est pas satisfaite, et le solveur
> retombe sur un échantillonnage plein — plus cher, mais juste.

> **Note corrective (S187, 2026-09-12) — la contrainte de densité est nécessaire et
> insuffisante.** Ce qui précède écrit la contrainte comme `dx ≤ λ_cut/N` : une **densité** de
> points par longueur d'onde. Deux mesures la corrigent sans l'annuler.
>
> **1. La longueur d'onde qui compte n'est pas `λ_cut`.** Un mode profond décroît en
> `exp(k z)`, donc les modes courts sont morts avant d'atteindre un consommateur en
> profondeur. Mesuré sur la source réelle
> ([COMPOSITION-ERREURS-S186](../validation/COMPOSITION-ERREURS-S186.md) §8.2), le contenu
> présent à `z ∈ [−4,05 ; −0,80] m` est **5 à 16 fois plus lisse** que la coupure de la
> recette. Compter les points par `λ_cut` est donc trop pessimiste pour un consommateur
> profond — et trop optimiste pour un consommateur de surface. Le critère porte sur
> l'échelle qui **survit à la profondeur du consommateur** (**A230**).
>
> **2. Une densité ne dit rien du placement, et le placement pèse plus.** La métrique du
> dépôt est un **maximum**, et ce maximum vit sur la maille de bord la plus haute du domaine.
> [RESEAU-GRADUE-S187](../validation/RESEAU-GRADUE-S187.md) §8.4 : à nombre de nœuds **égal**,
> ancrer le dernier nœud sur cette maille au lieu de le laisser déborder du domaine fait
> passer l'erreur de **41,2 % à 6,8 %** — un facteur six, gratuit. La graduation du pas selon
> la courbure vaut 1,4 de plus. Les deux règles sont actées par
> [ADR-118](../adr/ADR-118-le-reseau-d-echantillonnage-ancre-et-gradue.md) : **ancrer d'abord,
> graduer ensuite, et s'arrêter quand l'axe cesse d'être le plus grossier** (**A231**).
>
> Ce que `N` reste : un paramètre de B4, et **le seul des trois que le banc n'a pas encore
> fixé**, parce que fixer une erreur acceptable est une décision et non une mesure. Le
> facteur 64 annoncé plus haut reste un objectif de coût, non une validité démontrée :
> [CONSOMMATION-S184](../validation/CONSOMMATION-S184.md) a mesuré que le gain **est**
> exactement le rapport des nœuds, et S186/S187 ce qu'il coûte en justesse.

---

## 7. Solides

```cpp
struct SolidProxyDesc {
    ProxyId  id;
    FrameId  frame;
    ShapeKind kind;              // Sdf | Convex | Triangles | Capsules
    ShapeRef  shape;
    Pose      pose_begin, pose_end;   // début et fin du tick
    vec3      lin_vel, ang_vel;
    uint16_t  material_id;            // audio, écume — ADR-016 §6
    float     inv_mass;               // 0 = statique, frontière imposée
};

class ISolidSource {
public:
    virtual Pose  pose_at(ProxyId, SimTime) const = 0;      // interpolé dans le tick
    virtual void  query_sdf(ProxyId, span<const vec3>, span<float> out) const = 0;
    virtual vec3  point_velocity(ProxyId, const vec3&) const = 0;
    virtual void  accumulate_force(ProxyId, const vec3& f, const vec3& tau) = 0;
};
```

### 7.1 `pose_at` : le détail qui produit un escalier si on l'omet

Un solveur qui sous-cycle quatre fois par tick et utilise la pose de **début de tick** pour les
quatre sous-pas fait avancer la coque par sauts. À 10 m/s et 30 Hz, chaque saut vaut 33 cm — soit
plusieurs cellules à `dx = 0,10 m`. Le résultat est un volume déplacé incorrect, une gerbe qui
pulse à la fréquence du tick, et un couplage bruyant.

D'où l'interpolation entre `pose_begin` et `pose_end`. L'hôte connaît les deux : il vient
d'intégrer la physique du solide.

### 7.2 Retour de force

`accumulate_force` alimente la pose de **rendu** uniquement (ADR-008 §1, invariant I-04).
L'accumulation n'a donc pas besoin d'être déterministe — mais elle doit être **bornée**, et le
plafond (8 cm, 3°) est appliqué par le système d'eau, pas laissé à la confiance de l'appelant.

---

## 8. Les six autres services d'hôte

> **Précision par rapport à ADR-020 §2**, qui annonçait sept interfaces : l'horloge n'en est pas
> une. `T_sim` est un **paramètre poussé** à `begin_tick`, ce qui est plus strict — un système qui
> ne peut pas lire l'heure ne peut pas en dépendre par accident. La surface d'hébergement compte
> donc six interfaces et un paramètre.

```cpp
struct HostServices {
    IAllocator*            alloc;
    IJobSystem*            jobs;
    ISink*                 sink;
    IBathymetryProvider*   bathy;
    IHydroSampleProvider*  hydro;
    IGpuBackend*           gpu;     // peut être nul : hôte serveur et harnais en mode check
};
```

### 8.1 `IAllocator` — l'invariant par construction

```cpp
class IAllocator {
public:
    virtual void* alloc_persistent(size_t, size_t align) = 0;   // phase d'initialisation seule
    virtual void* alloc_scratch(size_t, size_t align) = 0;      // remis à zéro à chaque tick
    virtual void  seal() = 0;   // après quoi alloc_persistent échoue
};
```

`seal()` transforme l'invariant I-06 en propriété mécanique : après scellement, toute allocation
générale **échoue** au lieu d'être comptée. Le défaut ne peut plus se glisser en production
derrière un compteur que personne ne regarde.

### 8.2 `IJobSystem` — le déterminisme n'est pas facultatif

```cpp
class IJobSystem {
public:
    virtual void parallel_for(uint32_t n, uint32_t grain, JobFn) = 0;
    virtual void parallel_reduce_ordered(uint32_t n, uint32_t grain,
                                         ReduceFn, MergeFn, void* acc) = 0;
    virtual uint32_t worker_count() const = 0;
};
```

`parallel_reduce_ordered` fusionne les résultats partiels dans **l'ordre des indices**, jamais dans
l'ordre d'arrivée. C'est la seule façon d'atteindre le régime D2 (SPEC-003 §2), et donc la seule
façon de pouvoir reproduire un bogue qui survient une fois sur cinquante. Toute accumulation
flottante du système passe par cette primitive ; `parallel_for` est réservé aux écritures
disjointes.

Corollaire imposé : **changer `worker_count` change la vitesse, jamais le résultat.** C'est une
assertion du harnais, pas une intention.

> **Note corrective (S20, [ADR-029](../adr/ADR-029-ce-que-la-premiere-ligne-de-code-a-appris.md)
> §3).** Le corollaire est vrai **à condition que le découpage soit fixé**. L'addition flottante
> n'étant pas associative, deux grains différents donnent deux sommes différentes — sur
> `[1 ; 10¹⁶ ; −10¹⁶ ; 1]`, le grain 1 donne `1,0` et le grain 2 donne `0,0`. Un système de tâches
> qui choisirait son grain d'après le nombre de fils — le réglage naturel — rendrait donc le
> corollaire **faux silencieusement**.
>
> **`grain` est une donnée du contrat**, fixée par l'appelant et jamais dérivée de la machine.
> L'assertion du harnais s'énonce : *à `n` et `grain` égaux, le résultat est identique quel que soit
> `worker_count`*. Cas consigné sous forme exécutable dans `code/water-harness/src/host_impl.rs`.

### 8.3 `IBathymetryProvider` — un domaine ne bloque jamais

```cpp
class IBathymetryProvider {
public:
    virtual bool   is_resident(const Aabb&, float lod) const = 0;
    virtual void   request(const Aabb&, float lod, Priority) = 0;
    virtual bool   try_get(const Aabb&, float lod, BathyView* out) const = 0;
};
```

`is_resident` est un **test d'admission de l'ordonnanceur** (ADR-012 §1) : un domaine n'est pas
créé tant que ses données ne sont pas là. Sans lui, on obtient soit un blocage sur le streaming,
soit un domaine qui simule au-dessus d'un trou et produit un fond plat au milieu d'une côte.

`IHydroSampleProvider` suit la même forme pour la grille de paramètres d'ADR-004 §2.2.

### 8.4 `IGpuBackend` — la lecture synchrone n'existe pas

```cpp
class IGpuBackend {
public:
    virtual BufferId create_buffer(size_t, Usage) = 0;
    virtual void     destroy_buffer(BufferId) = 0;
    virtual void     dispatch(const DispatchDesc&) = 0;
    virtual TimerId  begin_timer();                  // horodatage GPU — SPEC-003 §6
    virtual void     end_timer(TimerId);
    virtual bool     poll_timer(TimerId, uint64_t* ns_out);

    virtual ReadbackId request_readback(BufferId, Range) = 0;
    virtual bool       poll_readback(ReadbackId, span<byte>* out, uint64_t* age_us_out) = 0;
};
```

Il n'existe **aucune fonction de lecture bloquante**. Le piège d'ADR-007 §4.1 n'est pas signalé
par une règle de revue : il est absent du vocabulaire. `poll_readback` renvoie toujours l'âge, ce
qui oblige l'appelant à décider quoi en faire.

`ISink` reçoit journal, métriques et compteurs ; il ne renvoie rien et ne bloque jamais.

---

## 9. Ce que l'interface rend impossible — et pourquoi

Cette section a autant de valeur que les signatures. Chaque ligne correspond à une demande qui
sera formulée pendant le développement, et qui doit être refusée avec son motif.

| Demande prévisible | Refus | Motif |
|---|---|---|
| « Une lecture GPU synchrone, juste pour déboguer » | absente de l'API | elle survivrait au débogage — ADR-007 §4.1 |
| « Passer une position monde, c'est plus simple » | le type ne l'exprime pas | I-08, précision `f32` |
| « Exposer le tableau de blocs au rendu » | aucun accesseur | I-13, ADR-006 §5 |
| « Une allocation, juste au démarrage d'un domaine » | `seal()` la fait échouer | I-06, battement d'allocation |
| « Laisser le solveur interroger la scène » | pas de service de requête | ADR-020 §2, testabilité |
| « Que δ pousse une force sur le corps rigide » | `accumulate_force` ne va qu'au rendu | **I-04** |
| « Un `dt` variable par domaine, décidé en interne » | `step` avance exactement `dt_target` | comparabilité, SPEC-003 §5.2 |
| « Sommer en parallèle sans ordre, c'est plus rapide » | `parallel_reduce_ordered` seul | régime D2 |
| « Vider la file d'événements pour la traiter » | il n'y a plus de drain | consommateur unique — SPEC-006 §3.2 |
| « Calculer un danger d'emportement depuis la vitesse de surface » | elle s'appelle `u_total` | l'orbitale vaut 0,63 m/s à Hs = 1 m — SPEC-006 §5.5 |
| « Faire interroger `EvalWater` par la navigation » | le signal est publié, pas interrogeable | 15× le coût, et croissant avec la population — SPEC-006 §1.1 |

---

## 10. Ce qui reste ouvert

1. **Langage et représentation des formes.** `ShapeKind` est volontairement ouvert ; la
   représentation retenue dépendra du solveur choisi en B3. Exigence minimale, non négociable :
   accepter une **frontière en mouvement avec sa vitesse**, pas seulement une géométrie.
   → **S11 : ce point porte deux questions** que d'autres documents posaient aussi — la
   représentation des solides (ADR-007 §5.4 y renvoie) et la **représentation binaire d'échange**,
   boutisme et alignement compris (SPEC-006 §9.8 et ADR-022 §7.4 y renvoient). Les deux dépendent
   du langage, qui est le premier terme de ce point.
2. **`CondensedState` — RÉSOLU en S10 par [ADR-022](../adr/ADR-022-persistance-de-l-eau.md), et la
   question était mal posée.** Il n'existe pas de persistance hors caméra : ADR-013 §6 avait dissous
   le mécanisme dès S01, et ces signatures lui avaient survécu. Le type qui subsiste est le
   `SeedState` — une donnée **cuite**, pas une capture d'exécution.
   *À noter : la contrainte énoncée ici — « il doit se relire sur une machine différente, donc pas
   de disposition mémoire brute » — était exactement celle d'un actif cuit, et dépourvue de sens
   pour une condensation en mémoire. Le point disait déjà ce qu'il était.*
3. **Granularité de `is_smooth_at`** — à dimensionner au banc B4 **sous le seuil de 2 %
   acté en S190 (ADR-120)**. Profil local reçu dans B4-TOLERANCE-S190 ; généralisation
   au solveur et aux autres contenus encore ouverte, aucune nouvelle décision de tolérance attendue.
   → **S11** : ce point proposait de mesurer « un point sur quatre ». Ce n'est plus le paramètre :
   la note corrective de §6.2 (S08, écart E08) a établi que le taux de décimation n'est pas une
   constante mais une **contrainte, `dx ≤ λ_cut/N`**, parce que le rapport `λ_cut/dx` passe de 40 à
   16 entre le scénario nominal et une zone de déferlement — soit de 10 à 4 points par longueur
   d'onde après décimation, deux fois Nyquist. **Ce qui se mesure à B4 est `N`.**
4. **Surface minimale de `IGpuBackend`** — interface la plus risquée des six (ADR-020 §6.2). À
   prototyper contre un candidat réel avant de la figer.
5. ~~**Budget d'instantanés W**~~ — **tranché en S09 par [`SPEC-006`](SPEC-006-chemin-pousse.md)
   §2.5**, et pour *tous* les canaux du chemin poussé, W compris : anneau dont la profondeur est
   **dérivée du profil** selon I-16 (et non déclarée), le plus ancien emplacement recyclé de force
   avec avertissement sur `ISink`, la poignée du retardataire devenant invalide à sa prochaine
   lecture. Une seule politique, faute de quoi chaque canal aurait inventé la sienne.
6. **Le chemin poussé — RÉSOLU en S09 par [`SPEC-006`](SPEC-006-chemin-pousse.md).** Le constat qui
   suit est conservé parce qu'il dit pourquoi le document existe ; les quatre canaux, leurs
   cadences, leur contrat de fils, leur dégradation et leur régime d'autorité y sont spécifiés, et
   `WaveEvent` y est défini avec ses trois champs audio. *(S08, écart E04, gravité 1)*. Cette
   spécification décrit deux chemins : le chemin **tiré** (`EvalWaterBatch`, `sample_batch`) et le
   chemin de **branchement de solveur**. Trois ADR en exigent un troisième, où le système d'eau
   *publie* par tick, à basse fréquence, ce que personne ne vient chercher point par point :

   | Document | Ce qui doit être publié |
   |---|---|
   | ADR-014 §2 | champ de moussage `F(x,t)`, deux canaux, texture 2D ancrée au monde |
   | ADR-018 §1 | `TraversabilitySample` par cellule `HydroGrid`, à 5 Hz (ADR-018 §6) |
   | ADR-016 §2, §6 | bus d'événements audio ; trois champs à ajouter à `WaveEvent` |

   Aucun n'a de signature ici, et **`WaveEvent` n'est défini nulle part dans ce document** alors
   qu'il traverse trois frontières (réseau, transduction δ→W, audio). Conséquence : trois des
   quatre interfaces inter-équipes en attente d'accord (audio, IA/navigation, part écume du rendu)
   n'ont aucun document à soumettre.

   Contraintes déjà établies que ce chemin devra respecter : instantané **immuable à N lecteurs**
   (§1.2) ; aucune allocation (I-06), donc anneau dimensionné par profil (I-16) ; le champ d'écume
   étant produit sur GPU, sa lecture retombe sur `poll_readback` et son âge (§8.4), jamais sur une
   lecture bloquante ; `WaveEvent` migre en §2 avec ses trois champs audio **avant que le réseau ne
   fige le format**.

   C'est un travail de session entière, pas une note corrective.
