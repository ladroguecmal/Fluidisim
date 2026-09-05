# SPEC-006 — Le chemin poussé : ce que le système d'eau publie

- **Statut** : proposée — porte trois des quatre interfaces inter-équipes en attente d'accord
- **Session** : S09
- **Dépend de** : SPEC-004 (chemin tiré, services d'hôte), ADR-012, ADR-014, ADR-016, ADR-018, ADR-021
- **Résout** : SPEC-004 §10.6 — écart E04 (gravité 1) ; angles morts A65 et A68
- **Convention** : même pseudo-langage, mêmes unités et mêmes repères que SPEC-004 §1.1. Ce
  document n'y déroge nulle part ; ce qu'il ajoute, ce sont des règles que le chemin tiré n'avait
  pas besoin d'énoncer.

---

## 1. Pourquoi un second document d'interfaces

SPEC-004 décrit deux chemins : le chemin **tiré**, où un appelant demande la valeur de l'eau en un
lot de points (`EvalWaterBatch`, `sample_batch`), et le chemin de **branchement**, où un hôte
fournit au système ses solveurs et ses six services. Aucun des deux ne décrit ce que le système
d'eau **émet de sa propre initiative**.

Quatre objets doivent pourtant l'être, et chacun est exigé nommément par un ADR :

| Objet publié | Exigé par | Consommateurs |
|---|---|---|
| **Bus d'événements** — le flux de `WaveEvent` | ADR-016 §2 | audio, rendu, réseau, gameplay |
| **Champ d'écume `F`** et **champ d'aération `A`** | ADR-014 §2, §5 | rendu, audio, flottabilité |
| **`TraversabilitySample`** par cellule `HydroGrid` | ADR-018 §1 | IA / navigation, gameplay |
| **Polyligne de déferlement** | ADR-016 §2, SPEC-005 §2 | audio, ordonnanceur, rendu |

Aucun n'avait de signature. Le défaut est invisible à la relecture de SPEC-004 — il n'y a rien à y
relire — et il s'est manifesté ailleurs : **trois ADR ont chacun décrit sa propre publication dans
son propre vocabulaire**, ce qui garantissait trois implémentations divergentes (L22, à trois
exemplaires). Sa conséquence pratique est plus lourde encore : trois des quatre interfaces
inter-équipes que `00_INDEX.md` présente comme « attendant une réponse humaine » n'avaient aucun
document à soumettre. On ne peut pas demander à l'équipe audio de confirmer une interface qui
n'existe nulle part sous forme de signature.

### 1.1 Ce qui distingue un chemin poussé d'un chemin tiré

Ce ne sont pas deux formes de la même chose. Ils diffèrent sur cinq points, et c'est de ne pas
avoir écrit ces cinq points que naissent les divergences.

| | Chemin tiré | Chemin poussé |
|---|---|---|
| Qui décide de l'instant | le consommateur | le producteur |
| Qui décide du contenu | le consommateur, point par point | le producteur, une fois pour tous |
| Coût d'un consommateur de plus | **proportionnel** — chaque appel resomme les paquets W | **nul** — l'instantané est déjà là |
| Ce que signifie « pas de réponse » | erreur ou donnée non résidente | la cadence n'est pas encore échue : normal |
| Fraîcheur | par construction, l'instant demandé | **variable, et donc à publier** |

La troisième ligne est celle qui motive l'existence du chemin, et elle se chiffre. ADR-018 §1
interdit à la navigation d'interroger `EvalWater` « sans quoi la navigation devient un consommateur
majeur ». Le rapport :

```
publication : une zone de 4 km × 4 km = 3 906 cellules HydroGrid de 64 m,
              republiées une fois par 30 s (ADR-018 §6)   →  130 évaluations/s
interrogation : 200 agents interrogeant à 10 Hz            →  2 000 évaluations/s
```

Quinze fois moins de travail — mais surtout, **le coût de la publication ne dépend pas du nombre
d'agents.** Doubler la population double la seconde ligne et laisse la première inchangée. C'est
la propriété que le chemin tiré ne peut structurellement pas offrir, et c'est elle qu'on achète
en acceptant de publier une donnée que personne n'a peut-être demandée.

Symétriquement, la cinquième ligne est le prix à payer : un consommateur du chemin tiré sait à
quel instant correspond ce qu'il lit, un consommateur du chemin poussé ne le sait que si on le lui
dit. D'où §2.4.

### 1.2 Portée

**Dans ce document** : les quatre canaux ci-dessus, leurs cadences, leur contrat de fils
d'exécution, leur politique de dégradation et leur régime d'autorité.

**Hors de ce document** : l'interrogation point par point (SPEC-004 §3), le branchement des
solveurs et les services d'hôte (SPEC-004 §4 à §8), et les données cuites sur disque (SPEC-005) —
dont ce document est en revanche un **consommateur** : la polyligne de déferlement publiée en §6
est la donnée cuite de SPEC-005 §2, republiée à chaud pour les régions actives.

---

## 2. Règles générales du chemin poussé

### 2.1 Un canal déclare quatre propriétés, toujours les mêmes

Aucun canal n'est publié sans que ces quatre lignes soient écrites. C'est la contrainte de forme
qui empêche un cinquième canal d'être ajouté un jour dans un vocabulaire à lui.

```cpp
struct ChannelDesc {
    uint32_t     size;              // extension par ajout en fin — SPEC-004 §1.4
    ChannelId    id;
    Cadence      cadence;           // cf. §2.3
    Shape        shape;             // Tuiles | FluxEvenements | PoigneeGpu | Agregat | Polyligne
    Authority    authority;         // Repliquee | Locale — cf. §2.6
    uint8_t      ring_slots;        // profondeur d'anneau, dérivée du profil — cf. §2.5
};
```

### 2.2 L'instantané immuable — et pourquoi un drain est un défaut

SPEC-004 §1.2 a établi la discipline pour la couche W : `publish_snapshot` produit un instantané
**immuable**, les lecteurs prennent une poignée et la conservent le temps de leur travail, faute de
quoi « un lecteur voit un jeu de paquets à moitié mis à jour, et le défaut est intermittent, rare
et introuvable ». Le chemin poussé applique cette discipline **à tous ses canaux, sans exception**.

> **Correction apportée à SPEC-004 §3.** `drain_outgoing_events()` est un canal à **consommateur
> unique** : le premier qui appelle vide la file pour tous les autres. Ce n'était pas visible tant
> que la transduction δ→W était son seul client, mais ADR-016 §2 fait du même flux le bus de
> l'audio, et le rendu comme le gameplay le veulent aussi. Trois consommateurs sur un drain, c'est
> soit un vol silencieux d'événements, soit une convention non écrite sur qui a le droit d'appeler
> — c'est-à-dire un défaut intermittent de plus.
>
> Le drain est remplacé en §3 par une publication à N lecteurs. Voir SPEC-004 §9, ligne ajoutée.

Le contrat est le même pour les quatre canaux :

```cpp
template <class View>
class IChannel {
public:
    virtual ChannelDesc desc() const = 0;
    virtual SnapshotHandle acquire() const = 0;   // jamais bloquant ; peut renvoyer l'invalide
    virtual View          view(SnapshotHandle) const = 0;
    virtual void          release(SnapshotHandle) const = 0;
};
```

`acquire` **ne bloque jamais** et n'alloue jamais. Une poignée invalide n'est pas une erreur : elle
signifie que rien n'a encore été publié sur ce canal, ce qui est l'état normal des premières frames
et de toute région qui vient d'être activée.

### 2.3 Le chemin poussé n'est pas au rythme du tick

Chaque canal a sa cadence propre, **diviseur entier du tick de simulation** (30 Hz, ADR-012 §7).
Un consommateur ne suppose jamais « une publication par frame » — c'est l'hypothèse implicite qui
rend une interface inutilisable dès qu'on veut économiser.

| Canal | Cadence nominale | Origine de la valeur |
|---|---|---|
| Bus d'événements | **par tick**, 30 Hz | un événement retardé est un son en retard (ADR-016 §3) |
| Champ `F`, poignée GPU | par tick de rendu | 1 à 2 passes GPU par cascade (ADR-014 §6) |
| Champ `F`, agrégat CPU | 10 Hz | l'audio dose un lit, il ne suit pas une crête |
| Champ `A`, agrégat CPU | 10 Hz | occlusion acoustique (ADR-016 §4.3) |
| Traversabilité — marée | **1 / 30 s** + événements de seuil | ADR-018 §6 |
| Traversabilité — crue, vanne | 1 / 2 s | ADR-018 §6 |
| Traversabilité — nœud V | 5 Hz | ADR-018 §6 |
| Traversabilité — brèche | **événement immédiat** | ADR-018 §6 |
| Polyligne de déferlement | à l'activation d'une région, puis sur changement d'état de mer | SPEC-005 §2 |

Les quatre lignes de traversabilité ne sont pas quatre canaux : c'est **un canal dont les tuiles ont
des cadences différentes** selon le phénomène qui les gouverne. Une tuile de haute mer se republie
toutes les 30 s ; la tuile d'un compartiment qui s'inonde, cinq fois par seconde. Découper par
cadence plutôt que par phénomène donnerait à l'IA quatre abonnements à réconcilier pour une seule
question.

### 2.4 L'âge est publié, jamais supposé

Tout instantané porte son horodatage, et tout ce qui a traversé le GPU porte en plus son âge de
lecture arrière — même discipline qu'`IGpuBackend::poll_readback` (SPEC-004 §8.4), et pour la même
raison : « il n'existe aucune fonction de lecture bloquante », le piège est absent du vocabulaire
plutôt que signalé par une règle de revue.

```cpp
struct SnapshotStamp {
    SimTime  published_at;      // instant de simulation auquel le contenu correspond
    uint64_t readback_age_us;   // 0 si le canal n'a jamais quitté le CPU
    uint32_t sequence;          // strictement croissant par canal — détection de saut
};
```

`sequence` est ce qui permet à un consommateur de constater qu'il a **manqué** des publications
plutôt que de le découvrir par un comportement bizarre. C'est aussi ce qui rend la dégradation de
§7 observable au lieu d'être silencieuse — même argument que `StepResult.Degrade` en SPEC-004 §4.1.

### 2.5 Aucune allocation : l'anneau, et le lecteur lent

I-06 vaut ici comme partout : les instantanés viennent d'un **anneau dimensionné au démarrage**.
Conformément à I-16, le profil de qualité déclare de la **mémoire**, jamais un nombre d'emplacements ;
`ring_slots` se calcule à l'initialisation à partir du coût mesuré d'un instantané et de la mémoire
allouée au canal.

Politique quand un lecteur lent retient une poignée alors que l'anneau est plein : **le plus ancien
est recyclé de force, avec avertissement sur `ISink`, et la poignée du retardataire devient
invalide à sa prochaine lecture.** Un consommateur qui ne rend jamais ses poignées ne peut donc pas
faire tomber la simulation ; il ne se pénalise que lui-même, et le journal dit qui c'est.

C'est la proposition que SPEC-004 §10.5 laissait ouverte pour les instantanés W. Elle est ici
tranchée pour tous les canaux, W compris : **une seule politique**, faute de quoi chaque canal
inventera la sienne — c'est exactement le mécanisme qui a produit E04.

### 2.6 Autorité : ce qu'I-15 impose à une donnée publiée

C'est la règle la moins évidente du document, et celle dont l'oubli coûterait le plus cher.

> **I-15.** Une grandeur dérivée est autoritaire si et seulement si tous les participants peuvent
> la calculer à l'identique à partir de données répliquées.

Un canal publié est donc autoritaire **si et seulement si toutes ses entrées le sont**. Appliqué :

| Canal | Entrées | Autorité |
|---|---|---|
| Bus d'événements | `WaveEvent` émis par le serveur depuis la cause (ADR-021 §3) | **répliquée** |
| Traversabilité | B, W répliqué, courants C1 cuits — **δ exclu** | **répliquée**, sous la condition ci-dessous |
| Polyligne de déferlement | bathymétrie cuite + état de mer répliqué | **répliquée** |
| Champ `F` | sources B, W **et δ** (ADR-014 §3.3) | **locale** |
| Champ `A` | déjà scindé `A_rep` / `A_local` (ADR-014 §5.2, ADR-021 §5) | scindée |

**La condition sur la traversabilité est une contrainte de conception, pas une observation.** Le
signal de navigation doit être calculé depuis les seules couches répliquées. Si une contribution de
δ y entrait — l'eau projetée par une explosion proche, par exemple — deux clients ne prendraient
plus la même décision de cheminement, et un PNJ traverserait un gué chez l'un et se noierait chez
l'autre. Le défaut serait rare, non reproductible et attribué au réseau.

Deux conséquences en découlent, toutes deux utiles :

- **le serveur peut évaluer le signal sans simuler d'eau.** I-10 lui interdit d'exécuter W ou δ,
  mais toutes les entrées de la traversabilité sont analytiques ou cuites, et ADR-009 lui donne
  déjà l'amplitude d'un événement W en forme fermée. Un franchissement de seuil peut donc être une
  **décision autoritaire** — un gué qui se ferme, un bateau reflotté — et pas seulement un indice
  local ;
- **un canal qui mêlerait entrées répliquées et locales se scinde à la source, jamais après.**
  C'est ce qu'ADR-021 §5 a imposé au champ `A`, et la même forme s'applique à `F` si un
  consommateur gameplay lui vient un jour. Moyenner les deux produirait une grandeur ni
  reproductible ni cosmétique — le pire des deux.

### 2.7 Contrat de fils d'exécution

Colonne omise dans presque toutes les interfaces de publication, et sans laquelle un consommateur
audio finira par lire depuis le fil de simulation « parce que ça marchait ».

| Fonction | Appelable depuis | Concurrente | Peut bloquer | Peut allouer |
|---|---|---|---|---|
| `IChannel::acquire` / `view` / `release` | n'importe quel fil | **oui, N lecteurs** | **jamais** | non |
| `IChannel::desc` | n'importe quel fil | oui | non | non |
| `publish_*` (interne au système) | fil de simulation uniquement | non | non | anneau seul |
| `register_listener` (§4.3) | fil de simulation, entre deux ticks | non | non | pool seul |

Aucune fonction de ce document n'est appelable depuis un fil qui détient un verrou du système
d'eau : il n'y en a pas à détenir, et c'est voulu.

---

## 3. Le bus d'événements

C'est le canal le plus partagé du système : l'audio en fait son bus (ADR-016 §2), le rendu y prend
ses déclenchements de gerbe, le gameplay ses causes, et le réseau y voit passer ce qu'il a lui-même
livré. C'est aussi celui dont l'écriture a fait remonter deux défauts qu'aucune relecture n'avait
vus.

### 3.1 `WaveEvent`, au complet

`WaveEvent` vivait dans ADR-009 §2, c'est-à-dire dans un document de réseau, alors qu'il traverse
trois frontières. Il est ici porté dans une spécification d'interface, avec les trois champs
qu'ADR-016 §6 demande d'ajouter « avant de figer le format ».

```cpp
enum class EventKind : uint8_t {
    Impact, Sillage, Explosion, Effondrement, Transduction,
    Deferlement, Vanne, Fuite, Rupture
};

enum class EventOrigin : uint8_t {  // cf. §3.3
    Serveur,              // répliqué, autoritaire
    AnticipationLocale,   // prédit par le client, sera réconcilié par id
    TransductionLocale    // W_local, cosmétique — ADR-005 §3, ADR-021 §3
};

struct WaveEvent {                       // 45 octets
    uint64_t  id;                        // server_seq — ordre total, déduplication
    uint32_t  frame_id;
    f16vec3   origin_local;              // dans le référentiel de `cell`
    uint64_t  cell;                      // morton 60 bits — HydroGrid
    SimTime   t_birth;                   // µs, dans T_sim
    EventKind kind;
    half      energy;                    // J/m ou J selon kind
    half      dir[2];
    half      lambda;
    half      ttl_hint;

    // --- ajoutés en S09, demandés par ADR-016 §6 ---
    uint16_t  material_id;               // coque métal, bois, roche, sable, chair
    half      displaced_l;               // volume déplacé, en LITRES — cf. la note ci-dessous
    uint8_t   flags;                     // bit 0 : above_surface · bits 1-2 : EventOrigin
                                         // bit 3 : retraction — cf. §3.3
};
```

> **`displaced_ml` ne peut pas exister sous ce nom.** ADR-016 §6 demande un `displaced_ml`, et
> SPEC-004 §1.1 fixe la convention de volume à des millilitres. Mais un `half` en millilitres sature
> à **65 504 ml, soit 65 litres** — dépassé par n'importe quelle claque de coque, et de trois ordres
> de grandeur par une explosion. Le champ est donc publié en **litres** : mêmes deux octets,
> plafond à 65 m³, et une précision relative de 0,1 % très au-delà de ce que « la taille perçue »
> demande. La convention en millilitres reste celle de la couche V (ADR-010), qui compte des
> volumes de contenants et non des volumes projetés.
>
> C'est L20 à l'œuvre : l'incompatibilité entre l'unité demandée et le type disponible n'apparaît
> qu'en écrivant la structure.

**Coût réseau, revérifié.** Les trois champs sont sur la structure **répliquée** — ils doivent
l'être : le serveur émet l'événement depuis la cause (ADR-021 §3), et un client qui n'a pas
assisté à la cause n'a aucun moyen de retrouver le matériau ou le volume déplacé. La structure
passe donc de 40 à 45 octets, et le débit d'ADR-009 §2 de 800 à **900 o/s par joueur intéressé**
dans une zone chargée à 20 événements/s. Toujours négligeable devant le trafic d'entités —
vérification faite, pas supposée.

### 3.2 Publication, et non drainage

```cpp
struct EventView {
    SnapshotStamp        stamp;
    span<const WaveEvent> events;   // triés par `id` croissant — ADR-009 §2
};

using IEventChannel = IChannel<EventView>;
```

**Trié par `id`**, ce qui rend la déduplication et la réconciliation possibles par simple parcours,
et non par table de hachage chez chaque consommateur.

> **Ce qui remplace `drain_outgoing_events()` (SPEC-004 §3).** Deux défauts distincts, tous deux
> réels :
>
> 1. **Un drain a un consommateur unique.** Le premier qui appelle vide la file pour tous les
>    autres. Tant que la transduction δ→W était le seul client, cela ne se voyait pas ; avec
>    l'audio, le rendu et le gameplay sur le même flux, c'est soit un vol silencieux d'événements,
>    soit une convention non écrite sur qui a le droit d'appeler.
> 2. **Le mot « outgoing » est un résidu.** Il datait de la conception qu'ADR-021 §3 a remplacée,
>    où les événements transduits par un client remontaient au serveur. Ce chemin **n'existe
>    plus** : la transduction ne produit que du `W_local`, injecté dans W *à l'intérieur* du
>    système. Plus rien n'a à sortir vers le réseau. La signature avait survécu à la décision qui
>    la vidait de son objet — R03 avait supprimé le chemin de données, pas la fonction qui le
>    servait.
>
> `drain_outgoing_events()` est donc **supprimée**, et non renommée : les événements de
> transduction sont injectés en interne, et paraissent sur le bus comme les autres, marqués
> `TransductionLocale`.

### 3.3 Anticipation et rétractation — ce qu'un bus partagé oblige à traiter

ADR-009 §7.2 autorise un client à émettre localement, par anticipation, l'événement que le serveur
émettra ; les deux se réconcilient par `id`. C'est bon pour la latence et **audible si on n'en
tire pas les conséquences** : l'anticipation locale et l'événement serveur arrivent séparés du
temps d'aller-retour réseau, et l'audio jouerait deux fois le même impact à 100 à 300 ms
d'intervalle. C'est un défaut qu'un testeur signale sans savoir le nommer — la famille exacte du
décalage audiovisuel d'ADR-016 §3.

Le bus publie donc, quand la réconciliation a lieu, un **événement de rétractation** : même `id`
local, `flags` bit 3 armé. Un consommateur qui a déjà agi annule ; un consommateur qui n'avait rien
fait l'ignore.

Trois lignes de contrat, qui suffisent :

- un consommateur **peut** agir sur un événement `AnticipationLocale`, à condition de savoir
  annuler ;
- un consommateur qui ne sait pas annuler **ignore** les `AnticipationLocale` et n'agit que sur
  `Serveur` — c'est le comportement correct par défaut, et il coûte une comparaison ;
- une rétractation arrive toujours **après** ce qu'elle rétracte, et jamais plus tard que le
  `ttl_hint` de l'événement anticipé.

### 3.4 Le délai de propagation appartient au consommateur

Le son parcourt 343 m/s : un événement à 500 m s'entend 1,46 s plus tard, à 3 km 8,7 s plus tard
(ADR-016 §3, SPEC-002 §6). Le système d'eau **n'applique pas ce retard** — il ne connaît ni la
position de l'auditeur, ni le nombre d'auditeurs, ni ce que l'audio veut faire des sons dont la
source a disparu entre-temps.

Ce qu'il fournit, et qui suffit : `t_birth` en `T_sim` et `origin_local` avec sa cellule. Le retard
est une soustraction chez le consommateur.

**Conséquence de dimensionnement, non évidente.** Un consommateur cadencé plus lentement que le bus
— l'audio à 10 Hz contre un bus à 30 Hz — doit lire **tous les instantanés depuis son dernier
`sequence`**, et non le plus récent. La profondeur d'anneau du canal d'événements se dimensionne
donc ainsi :

```
ring_slots ≥ (cadence_bus / cadence_du_consommateur_le_plus_lent) + 1
```

soit au moins 4 emplacements pour un consommateur à 10 Hz. Un anneau plus court fait perdre des
événements silencieusement — sauf que `sequence` (§2.4) le rend constatable, ce qui est
précisément son rôle.

---

## 4. Écume et aération : un champ, deux publications

ADR-014 pose un champ de moussage `F` à deux canaux et un champ d'aération `A`, tous deux calculés
sur GPU. Trois consommateurs les veulent, et **ils n'en veulent pas la même chose** :

| Consommateur | Ce qu'il veut | Où il vit |
|---|---|---|
| Rendu | la texture, telle quelle | GPU |
| Audio | l'intégrale de `F` autour de l'auditeur (ADR-016 §2), l'occlusion par `A` (§4.3) | CPU, fil audio |
| Flottabilité | `A` **au point** du solide | CPU — et c'est le chemin **tiré**, cf. §4.5 |

Publier une seule forme aux trois impose soit une lecture arrière de texture à qui n'en a pas
besoin, soit une texture à qui ne sait qu'en faire. Le canal a donc **deux publications distinctes
du même champ**, et c'est la décision structurante de cette section.

### 4.1 Au rendu : une poignée, jamais une copie

```cpp
struct FoamCascadeDesc {
    FrameId    frame;            // référentiel de l'ancre — I-07, I-08
    vec3       anchor_local;     // |anchor_local| < 4096 m
    float      extent_m;         // côté de la cascade, en mètres
    uint16_t   texels;           // côté en texels ; résolution ouverte — ADR-014 §7.1, banc B9
    uint8_t    level;            // 0 = la plus fine
};

struct FoamGpuView {
    SnapshotStamp            stamp;
    span<const FoamCascadeDesc> cascades;   // du plus fin au plus grossier
    span<const BufferId>        textures;   // RG16F : canal actif, canal résiduel — ADR-014 §2.2
};
```

Deux canaux dans une seule texture, parce qu'ADR-014 §2.2 en fait un vecteur à deux composantes et
que les séparer doublerait les lectures pour rien.

**Ce qui sort d'une cascade est perdu**, et c'est voulu (ADR-014 §2.3) : hors de vue. La conséquence
d'interface est qu'une cascade **change d'ancre** quand l'observateur se déplace ; `anchor_local` et
`frame` sont donc republiés à chaque instantané, et un consommateur qui aurait mémorisé une ancre
lirait un champ décalé. C'est la raison pour laquelle la description voyage avec la texture au lieu
d'être demandée une fois à l'initialisation.

### 4.2 À l'audio : un agrégat minuscule, calculé là où il est déjà

L'audio veut « la quantité d'écume réellement présente » autour de l'auditeur (ADR-016 §2). Ramener
la texture au CPU pour l'intégrer serait absurde :

```
une cascade de 1024² en RG16F pèse 4,2 Mo   ·   quatre cascades, 30 fois par seconde
   → ≈500 Mo/s de lecture arrière, plus une à trois frames de latence
l'agrégat correspondant                      → quelques dizaines d'octets
```

La réduction se fait donc **sur le GPU, dans la passe qui produit déjà `F`**, et seul le résultat
traverse. C'est la forme générale de la règle : *le chemin poussé publie au CPU des réductions,
jamais des champs.*

```cpp
struct ListenerAggregate {
    ListenerId id;
    half  foam_active[3];        // intégrale du canal actif   sur r = 10, 50, 200 m
    half  foam_residual[3];      // intégrale du canal résiduel, mêmes rayons
    half  aeration_sector[16];   // cf. §4.3
    half  immersion;             // fraction du volume de l'auditeur sous la surface
};                               // 45 octets

struct FoamAggregateView {
    SnapshotStamp                  stamp;   // `readback_age_us` est ici toujours non nul
    span<const ListenerAggregate>  listeners;
};
```

Trois rayons plutôt qu'un, parce qu'un lit d'ambiance et un déferlement à trente mètres ne se
dosent pas avec la même intégrale, et que le coût marginal d'un rayon supplémentaire dans une
réduction GPU est nul.

**Les auditeurs sont déclarés, pas devinés :**

```cpp
Result register_listener(ListenerId, FrameId, const vec3& pos_local);
void   unregister_listener(ListenerId);
void   update_listener(ListenerId, FrameId, const vec3& pos_local);   // par tick
```

Conformément à I-16, **le nombre maximal d'auditeurs ne se déclare pas dans un profil** : il se
calcule à l'initialisation en divisant la mémoire allouée au canal par la taille d'un agrégat. Un
profil qui écrirait `auditeurs_max = 8` finirait par contredire la mémoire qui l'entoure — c'est
exactement le défaut R04.

### 4.3 L'occlusion acoustique par l'aération, sans inventer une seconde discrétisation

ADR-016 §4.3 établit qu'un rideau de bulles est acoustiquement opaque, et que « un sillage est une
couverture acoustique ». ADR-016 §8.4 laissait ouvert le moyen : intégrale de `A` le long de chaque
segment auditeur-source, ou tabulation grossière.

L'intégrale par segment est un chemin **tiré**, dont le coût croît avec le nombre de sources — le
défaut même qui a motivé ce document. La tabulation retenue est donc **azimutale, en 16 secteurs**,
publiée par auditeur :

```
aeration_sector[j] = ∫ A dl  le long du rayon du secteur j, jusqu'à une portée de coupure
```

**Seize secteurs, et non un nouveau découpage** : c'est exactement la sectorisation azimutale
qu'ADR-005 §3 emploie déjà pour mesurer le flux sortant d'un domaine. Inventer ici une seconde
discrétisation aurait produit deux implémentations voisines et divergentes — le mécanisme de L22,
qu'on évite en le voyant venir plutôt qu'en le corrigeant après.

**Limite à dire, parce qu'elle se rencontrera.** Une intégrale par secteur ne distingue pas un
rideau de bulles à 5 m d'un rideau à 50 m dans la même direction : deux sources alignées derrière
le même sillage reçoivent la même occlusion, ce qui est juste, mais une source *entre* l'auditeur
et le rideau reçoit une occlusion qu'elle ne devrait pas subir. Le raffinement — deux bandes
radiales par secteur, soit 32 valeurs — double l'agrégat et reste minuscule. **À trancher au banc,
pas maintenant** : la question est de savoir si le défaut s'entend, et cela se mesure (§9.3).

### 4.4 Ce que le chemin poussé ne publie pas de `F`

Le champ d'écume est d'autorité **locale** (§2.6) : il contient des sources δ (ADR-014 §3.3), qui
ne sont ni répliquées ni reproductibles. Aucune décision de jeu ne s'en déduit.

Si un besoin gameplay lui venait — repérer un navire à son sillage, par exemple — la réponse n'est
pas de « rendre `F` déterministe », c'est de le **scinder à la source** en `F_rep` et `F_local`,
exactement comme ADR-021 §5 l'a imposé au champ `A`. La part `F_rep`, issue de B et de W répliqué,
est déjà identique chez tous les clients sans être répliquée (ADR-014 §2.3) : elle serait
autoritaire au titre d'I-15 sans qu'aucun octet ne transite.

### 4.5 L'aération pour la flottabilité reste sur le chemin tiré

`WaterSample.aeration` (SPEC-004 §2) donne `A` au point, et c'est la bonne forme : la flottabilité
interroge les points de sa coque, elle ne consomme pas un lit d'ambiance. Un lecteur ne doit
**jamais** utiliser `ListenerAggregate` pour calculer une portance — l'agrégat est une intégrale
autour d'un auditeur, pas une densité locale, et l'erreur donnerait un navire qui s'enfonce parce
qu'il y a de l'écume à deux cents mètres.

Cette ligne est reprise dans le tableau des refus, §8.
