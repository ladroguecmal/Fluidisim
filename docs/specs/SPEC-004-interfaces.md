# SPEC-004 — Signatures des interfaces

- **Statut** : proposée — dernier document avant l'écriture de code
- **Session** : S04
- **Dépend de** : ADR-007 (emplacements de solveur), ADR-020 (bibliothèque autonome), SPEC-003 (harnais)
- **Détaille** : ADR-007 §2 et ADR-020 §2

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
    vec3  u;                // vitesse de surface (orbitale + courant)
    vec3  normal;
    float deta_dt;
    float steepness;        // pour écume et seuil de déferlement
    float aeration;         // fraction volumique d'air — ADR-014 §5.2
};
```

### 2.1 Deux types d'échantillon, et pourquoi les fusionner serait une erreur

```cpp
// Consommateur unique : un solveur δ en régime perturbatif — cf. §6
struct BackgroundSample {
    float eta;
    vec3  grad_eta;
    vec3  u;
    vec3  du_dt;            // dérivée temporelle locale
    mat3  grad_u;           // gradient de vitesse
    float p_dyn;
};
```

`BackgroundSample` pèse ≈80 octets contre ≈56 pour `WaterSample`, et son calcul coûte plusieurs
fois plus cher (dérivées analytiques de toutes les composantes). Le fusionner ferait payer à
chaque requête de flottabilité, d'audio et d'IA des dérivées dont elles n'ont aucun usage.

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

    span<const WaveEvent> drain_outgoing_events();          // transduction δ→W — ADR-005 §3
    const Metrics&        metrics() const;                  // SPEC-003 §6
};
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
    virtual Result condense(CondensedState* out) const = 0;
    virtual Result restore(const CondensedState&) = 0;
    virtual void   reset() = 0;
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

`BackgroundSample` est cher, et un domaine perturbatif en demande un par cellule et par sous-pas.

Le fond est cependant **lisse à l'échelle de `dx`** : ses longueurs d'onde valent au minimum
`λ_cut`, soit des dizaines de cellules. Il est donc échantillonné sur un réseau grossier — une
cellule sur quatre par axe est le point de départ proposé — puis interpolé trilinéairement.

Facteur d'économie : **64**. C'est ce qui fait passer le terme source de rédhibitoire à
négligeable, et c'est le rôle de `is_smooth_at(dx)` que de garantir la validité de
l'approximation.

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

---

## 10. Ce qui reste ouvert

1. **Langage et représentation des formes.** `ShapeKind` est volontairement ouvert ; la
   représentation retenue dépendra du solveur choisi en B3. Exigence minimale, non négociable :
   accepter une **frontière en mouvement avec sa vitesse**, pas seulement une géométrie.
2. **`CondensedState`** — format de sérialisation pour la persistance hors caméra. Reporté : sa
   forme dépend du solveur. Contrainte connue : il doit se relire sur une machine différente, donc
   pas de disposition mémoire brute.
3. **Granularité de `is_smooth_at`** — un point d'échantillonnage sur quatre est une proposition ;
   à mesurer au banc B4, dont c'est un paramètre direct.
4. **Surface minimale de `IGpuBackend`** — interface la plus risquée des six (ADR-020 §6.2). À
   prototyper contre un candidat réel avant de la figer.
5. **Budget d'instantanés W** — combien de poignées simultanées, et quelle politique si un lecteur
   lent en retient une trop longtemps ? Proposition : anneau de N instantanés, le plus ancien étant
   recyclé de force avec avertissement.
