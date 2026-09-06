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
| Champ `F`, poignée GPU | **tick de simulation** ou diviseur *(corrigé en S13, E06)* | 1 à 2 passes GPU par cascade (ADR-014 §6) |
| Champ `F`, agrégat CPU | 10 Hz | l'audio dose un lit, il ne suit pas une crête |
| Champ `A`, agrégat CPU | 10 Hz | occlusion acoustique (ADR-016 §4.3) |
| Traversabilité — marée | **1 / 30 s** + événements de seuil | ADR-018 §6 |
| Traversabilité — crue, vanne | 1 / 2 s | ADR-018 §6 |
| Traversabilité — nœud V | 5 Hz | ADR-018 §6 |
| Traversabilité — brèche | **événement immédiat** | ADR-018 §6 |
| Polyligne de déferlement | à l'activation d'une région, puis sur changement d'état de mer | SPEC-005 §2 |

> **Note corrective (S13, écart E06).** Cette ligne portait « par tick de rendu », en contradiction
> avec la règle énoncée juste au-dessus — toute cadence est un diviseur entier du tick de simulation
> — et avec ADR-012 §7, qui pose que le rendu est « indépendant du taux d'images » et « interpole ».
> Une publication cadencée sur le rendu n'est pas reproductible, l'ordonnanceur ne peut pas la
> budgéter, et sur une machine à 144 Hz elle ferait près de cinq fois le travail d'une machine à
> 30 Hz — pour un champ qu'ADR-014 §6 déclare « toujours actif, c'est le socle ». Le rendu interpole
> la poignée comme il interpole tout le reste.

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
inventera la sienne — c'est exactement le mécanisme qui a produit l'écart E04 de la revue S08.

> **Note corrective (S13, écart E03).** Ce paragraphe dérive `ring_slots` de la mémoire allouée,
> tandis que §3.4 le dérive du rapport de cadences (« au moins 4 emplacements pour un consommateur à
> 10 Hz »). Deux règles pour un même champ, à une section d'écart, sans dire laquelle l'emporte.
> **La règle est `ring_slots = max(règle de cadence de §3.4, 2)`**, et la mémoire du canal doit le
> permettre. Si elle ne le permet pas, c'est une **mauvaise configuration** — détectée par
> `validate_config()` au chargement et jamais à l'exécution (SPEC-004 §1.3). Le mécanisme existait ;
> il suffisait de l'invoquer.

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

struct WaveEvent {                       // 50 octets — corrigé en S13, écart E04
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
passe donc de 45 à **50 octets**, et le débit d'ADR-009 §2 de 900 à **1 000 o/s par joueur
intéressé** dans une zone chargée à 20 événements/s. Toujours négligeable devant le trafic
d'entités.

> **Note corrective (S13, écart E04).** Ce paragraphe annonçait « de 40 à 45 octets ». Les deux
> chiffres étaient faux : la structure d'origine d'ADR-009 §2 en sommait **45** et non 40, et
> l'étendue en somme **50**. Cinq octets avaient été ajoutés à une base fausse, retrouvant par
> coïncidence la taille réelle de l'original. `ListenerAggregate` (§4.2) portait la même erreur —
> deux structures sur deux dans ce document. Aucune conclusion ne change ; c'est la **classe de
> contrôle** qui manquait, personne n'ayant jamais additionné les champs d'un `struct`.

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
    float      extent_m;         // côté de la cascade, en mètres — **< 4096 m** (I-08), cf. S14
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

> **Clarification (S13).** Remettre au rendu une poignée de texture GPU n'enfreint pas **I-13**
> (« le rendu ne pilote pas la physique […] ne peut jamais partager les structures de calcul »). La
> texture est un **produit publié**, immuable sur le tick : le rendu ne peut ni y écrire, ni
> influencer la simulation par elle. Ce qu'I-13 protège est le tableau de blocs de δ (ADR-006 §5),
> pas les champs que le système publie. La clarification est écrite parce que quelqu'un invoquera
> I-13 pour refuser cette section.

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
};                               // 50 octets — corrigé en S13, écart E04

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

---

## 5. Traversabilité

Le canal qu'ADR-018 §1 exige, et le seul dont la valeur soit **autoritaire** au sens d'I-15. C'est
ce qui en fait le plus contraint des quatre.

### 5.1 L'échantillon

```cpp
struct TraversabilitySample {          // 20 octets
    half   depth;               // profondeur d'eau au-dessus du sol
    half   flow_speed;          // norme du COURANT de surface — jamais l'orbitale, cf. §5.5
    half   hazard;              // HR = d·(v+0,5) — SPEC-002 §5, ADR-018 §3
    half   ice_h;               // épaisseur de glace porteuse, 0 si absente
    half   ice_capacity_kg;     // dérivé de `ice_h` — cf. la note ci-dessous
    half   temp;                // hypothermie, gel
    int8_t trend;               // −1 descend · 0 stable · +1 monte
    uint8_t flags;              // bit 0 : sous-cellule disponible (§5.3)
    float  t_next_cross;        // secondes avant le prochain franchissement de seuil
    CrossCause t_next_cause;    // ce qui produirait ce franchissement — cf. §5.4
};
```

> **Pourquoi `ice_capacity_kg` est publié alors qu'il se dérive de `ice_h`.** La charge admissible
> suit `P ∝ h²` (Gold, SPEC-002 §4) : n'importe quel consommateur pourrait la recalculer. C'est
> précisément le problème — l'IA, le gameplay et l'audio la recalculeraient avec trois constantes
> légèrement différentes, et un agent traverserait une glace que le gameplay juge rompue. La
> dérivation est donc faite **une fois, par le système d'eau**, et `ice_h` reste la source de vérité
> dont elle sort (L25). Le champ vaut 0 tant que l'arbitrage n°2 n'a pas répondu que le projet veut
> de la glace.

### 5.2 Publication par tuiles, pas par région

Republier une région entière parce qu'un compartiment s'inonde serait absurde. L'unité de
publication est la **tuile** — un bloc de 16 × 16 cellules `HydroGrid`, soit 1 024 m de côté et
5 Ko par instantané. Une zone de 4 km × 4 km tient en seize tuiles, ≈80 Ko.

```cpp
struct TileDesc {
    FrameId    frame;
    uint64_t   tile_morton;
    CadenceClass cadence;      // Maree | Debit | NoeudV | Immediat — §2.3
    uint8_t    subdivision;    // 0 = cellule HydroGrid ; >0 = sous-cellule, cf. §5.3
    uint32_t   sequence;       // propre à la tuile
};

struct TraversabilityView {
    SnapshotStamp                       stamp;
    span<const TileDesc>                tiles;
    span<const TraversabilitySample>    samples;   // concaténées, dans l'ordre des tuiles
    span<const CrossingEvent>           crossings; // §5.4
};
```

**Chaque tuile porte sa cadence, et non le canal.** Les quatre lignes de traversabilité d'ADR-018 §6
— marée à 1/30 s, crue à 1/2 s, nœud V à 5 Hz, brèche immédiate — décrivent des *phénomènes*, pas
des canaux. En faire quatre canaux obligerait l'IA à quatre abonnements et à les réconcilier pour
répondre à une seule question : « puis-je passer ici ? ». Une tuile change de classe de cadence
quand ce qui la gouverne change — une vanne s'ouvre en amont, la tuile passe de `Maree` à `Debit`.

`sequence` est **par tuile** : un consommateur constate qu'une tuile précise a sauté des
publications, et non que « quelque chose » a été manqué quelque part.

### 5.3 La sous-cellule : réutiliser celle d'ADR-006, ne pas en inventer une

ADR-018 §7.2 le dit sans détour : « la cellule `HydroGrid` de 64 m est trop grossière pour un gué ».
Un gué fait quelques mètres de large ; publié à 64 m, il est soit absent, soit étalé sur toute une
cellule.

La subdivision employée est **celle qu'ADR-006 §2 porte déjà** depuis la revue croisée S05
(écart R07, angle mort A58), et non une seconde. C'est la leçon L22 appliquée par anticipation :
deux documents qui inventent séparément leur propre subdivision produisent deux découpages
divergents, et l'on ne s'en aperçoit qu'à l'intégration.

`subdivision > 0` sur les tuiles qui contiennent une rivière ou un trait de côte ; le nombre
d'échantillons de la tuile est alors `(16 · 2^subdivision)²`. La liste des tuiles subdivisées est
une donnée d'auteur dérivée du squelette hydrographique (SPEC-005 §2) : elle ne se décide pas à
l'exécution, sans quoi la mémoire du canal ne serait pas dimensionnable au démarrage (I-06).

### 5.4 `t_next_cross` : une prédiction sans son hypothèse est un piège

ADR-018 §4 tire de l'analycité de B un bénéfice réel : `depth(x, t)` est calculable à l'avance, donc
le système peut annoncer « ce gué se ferme dans quarante minutes ». C'est le genre de capacité
qu'aucune simulation n'offrirait à ce prix (L13).

**Ce qui n'apparaît qu'en écrivant le champ** : cette prédiction n'est vraie que *toutes choses
égales par ailleurs*. Elle suppose que seule la marée agit. Qu'une vanne s'ouvre en amont, qu'un
barrage cède, qu'une crue arrive — et la valeur publiée reste là, parfaitement fausse, jusqu'à la
prochaine republication de la tuile, c'est-à-dire jusqu'à trente secondes plus tard pour une tuile
de classe `Maree`. Un PNJ ayant planifié un trajet côtier sur cette valeur le maintiendra.

Une prédiction se publie donc **avec la cause qu'elle suppose**, et s'invalide quand cette cause
n'est plus seule :

```cpp
enum class CrossCause : uint8_t {
    Aucune,      // t_next_cross non significatif
    Maree,       // analytique, fiable à l'horizon publié
    Debit,       // dérive d'un régime de rivière — valable tant que l'amont ne change pas
    Commande     // conséquence d'une commande V déjà passée (vanne ouverte, porte fermée)
};
```

**Règle.** Toute commande de la couche V — ouverture de vanne, rupture, brèche — invalide
immédiatement les prédictions des tuiles situées à l'aval, qui repassent en classe `Immediat` le
temps d'être recalculées et republient `CrossCause::Aucune` tant que la nouvelle valeur n'est pas
établie. **Publier `Aucune` est un résultat**, pas un échec : c'est la seule façon de dire à l'IA
« je ne sais plus », et l'absence de cette valeur est ce qui rend une prédiction dangereuse.

Les **franchissements** effectifs, eux, sont des événements :

```cpp
struct CrossingEvent {
    uint64_t   cell;            // morton, éventuellement sous-cellule
    SimTime    t;
    Threshold  crossed;         // seuils d'ADR-018 §2 : 0,15 / 0,50 / 1,00 / 1,30 m,
                                // gué véhicule, tirant d'eau, charge de glace
    int8_t     direction;       // +1 franchi vers le haut, −1 vers le bas
};
```

Ils ne transitent **pas par le réseau**, et c'est une conséquence directe de §2.6 : toutes les
entrées du signal étant répliquées ou cuites, chaque participant — serveur compris — dérive le
même franchissement au même instant de simulation. Répliquer un `CrossingEvent` reviendrait à
envoyer une donnée que le destinataire sait déjà calculer. C'est le même raisonnement qui a fait
disparaître le chemin client → serveur en ADR-021 §3.

### 5.5 `flow_speed` est un courant, et le nom le dit

`hazard` applique `HR = d·(v + 0,5)` où `v` est un **courant** (SPEC-002 §5, ADR-018 §3). La
vitesse de surface publiée par le chemin tiré, `WaterSample.u`, est explicitement « orbitale +
courant » : calculé sur elle, `HR` oscillerait à la période de la houle, avec une amplitude
`πHs/T` = **0,63 m/s à Hs = 1 m et T = 5 s** — le double du seuil qui sépare « faible » de
« dangereux ». Un gué serait mortel une demi-période sur deux.

`flow_speed` est donc la norme du seul courant : contributions de B, de W répliqué et des champs
de courant C1 (ADR-011), **sans la composante orbitale et sans δ**. C'est la résolution de l'écart
E05 de la revue S08 ; le champ correspondant de SPEC-004 est renommé `u_total` pour que le chemin
tiré cesse de proposer une grandeur qu'on prendra pour l'autre.

### 5.6 δ n'entre pas dans ce canal, et la portée de l'omission est bornée

§2.6 l'impose : le signal est calculé depuis les seules couches répliquées. Une gerbe soulevée par
une explosion proche ne modifie donc pas la traversabilité publiée, alors qu'elle est visible.

L'écart est borné par l'argument de fermeture d'ADR-021 §3.2 : δ ne contient, par construction, que
ce qui est plus court que `λ_cut` — du court, du local et du bref. Une perturbation capable de
changer une décision de cheminement dépasserait `λ_cut` et appartiendrait donc à W, où elle est
répliquée et prise en compte.

Cette fermeture a la même condition de validité que celle d'ADR-021 : elle tient tant que `λ_cut`
sépare effectivement les deux couches. Si le banc B2 conduisait à le relever, **ce canal serait à
réexaminer en même temps qu'ADR-021 §3.2** — à ajouter au protocole de B2, qui porte déjà ce
critère de recevabilité.

---

## 6. La polyligne de déferlement

Le quatrième objet publié, et celui qu'on oublie parce qu'aucun ADR ne lui est consacré : ADR-016 §2
« exige que le système d'eau publie la polyligne de déferlement », SPEC-005 §2 la liste comme donnée
dérivée à trois consommateurs — audio, ordonnanceur, rendu — et ADR-016 §8.2 laisse son format
ouvert. Il est fermé ici.

```cpp
struct BreakerVertex {
    FrameId  frame;
    vec3     pos_local;
    half     dissipated_kw_per_m;   // flux d'énergie dissipé — cf. la note d'unité
    half     crest_dir[2];          // direction de propagation de la crête
    half     surf_width_m;          // largeur de la zone de déferlement — ADR-005 §4.1
};

struct BreakerLineView {
    SnapshotStamp                 stamp;
    span<const RegionId>          regions;
    span<const uint32_t>          first_vertex;   // index de début par région
    span<const BreakerVertex>     vertices;
};
```

> **Note d'unité, seconde occurrence du même piège.** Le flux d'énergie d'une houle vaut
> `P = E·c_g` (SPEC-001 §3), soit 15,3 kW/m pour `Hs = 2 m`, `T = 8 s`. Comme `E ∝ Hs²`, une mer à
> `Hs = 8 m` dépasse **300 kW/m** — au-delà du plafond d'un `half` exprimé en W/m (65 504). Le champ
> est donc publié en **kW/m**. C'est exactement la même erreur que `displaced_ml` en §3.1, trouvée
> pour la même raison et dans le même document : un `half` ne se choisit pas sur la précision voulue
> mais sur l'**étendue** de la grandeur, et le cas extrême du projet doit être calculé avant que
> l'unité ne soit fixée.

**Ce n'est pas une donnée d'exécution, c'est une donnée cuite republiée.** La polyligne est dérivée
hors ligne de la bathymétrie et de l'état de mer (SPEC-005 §2, ADR-005 §4.1) ; le chemin poussé la
rend disponible à chaud pour les régions actives et la republie quand l'état de mer ou la phase de
marée change d'état stocké. Elle n'est jamais retouchée à l'exécution — c'est la règle de source de
vérité unique de SPEC-005 §1.

**Contrainte d'ordonnancement.** L'ordonnanceur en est consommateur (SPEC-005 §2) : il s'en sert
pour décider d'activer une plage. Elle doit donc être publiée **avant** que le domaine de
déferlement n'existe, c'est-à-dire au moment de l'activation de la région et non à celui de la
création du domaine. C'est la seule des quatre publications dont la cadence est gouvernée par
l'ordonnanceur plutôt que par un phénomène physique.

**Ce qui reste suspendu à un arbitrage humain.** Si le trait de côte est mobile — arbitrage n°3,
ouvert, ADR-011 §6 et ADR-018 §4 — la polyligne se déplace avec la marée, et le nombre d'états
stockés est celui de la bibliothèque côtière (16, SPEC-005 §6). S'il ne l'est pas, un seul état
suffit. **Ce document ne tranche pas** : il publie une polyligne par phase de marée stockée, ce qui
dégénère proprement au cas d'une seule phase si la réponse est « côte fixe ».

---

## 7. Dégradation : en cadence, jamais en contenu

Un canal sous contrainte de budget **ralentit**. Il ne publie jamais un instantané partiel, jamais
une tuile à moitié remplie, jamais une liste d'événements tronquée en son milieu. Un consommateur
doit pouvoir croire ce qu'il lit ; ce dont il ne peut pas être sûr, c'est de **quand** il le lira,
et `stamp.sequence` (§2.4) le lui dit.

Ordre de dégradation. **Les rangs de ce document sont notés `P1` à `P5` et ne se confondent pas
avec les sept rangs d'ADR-012 §4**, qui portent sur les domaines et les paquets W.

> **Note corrective (S13, écart E02).** Ce paragraphe disait « cohérent avec les rangs d'ADR-012 §4 »
> en réutilisant leur numérotation. Deux échelles indépendantes portaient donc les mêmes numéros dans
> deux documents qui se citent — et depuis S12, ADR-022 §2.6 a posé que « le rang 5 ne s'applique pas
> aux domaines substitutifs », règle qui vise l'échelle d'ADR-012. Rapportée à celle-ci, elle se
> lirait « on n'élague pas les événements de transduction dans un déferlement » : contresens complet
> et parfaitement plausible.

| Rang | Ce qui cède | Ce qui ne cède jamais |
|---|---|---|
| 1 | agrégats `F` et `A` : 10 Hz → 5 Hz → 2 Hz | — |
| 2 | tuiles de classe `Maree` : 1/30 s → 1/60 s | — |
| 3 | cascades `F` : la plus grossière est abandonnée | la plus fine — c'est le socle (ADR-014 §6) |
| 4 | tuiles de classe `Debit` : 1/2 s → 1/4 s | |
| 5 | bus : les événements `TransductionLocale` puis `AnticipationLocale` sont élagués | **les événements `Serveur`, jamais** |
| — | *aucun rang* | tuiles `Immediat`, `CrossingEvent`, nœuds V à 5 Hz |

Le rang 5 est la transposition exacte d'ADR-021 §4 : on élague ce qui est local et cosmétique,
jamais ce qui porte une donnée répliquée. Élaguer un événement `Serveur` chez un joueur et pas chez
un autre produirait deux mondes différents — et ici, en prime, deux bandes-son différentes.

La ligne sans rang est celle qui compte : **une brèche, un franchissement de seuil et l'inondation
d'un compartiment ne se dégradent sous aucun budget.** Ce sont des faits de jeu, pas des ornements ;
s'ils ne tiennent plus, c'est une saturation à documenter (SPEC-003 §9.2), pas à absorber
silencieusement.

**Trois assertions à ajouter au banc `starve`** (SPEC-003 §9.1), sans lesquelles rien de ce qui
précède n'est vérifié :

1. aucun canal ne publie d'instantané dont le contenu soit incomplet, à aucun palier de budget ;
2. aucun événement `Serveur` n'est absent d'un instantané, à aucun palier ;
3. `sequence` reste strictement croissant par canal et par tuile — un saut est licite, un recul ou
   une répétition ne l'est pas.

---

## 8. Ce que l'interface rend impossible — et pourquoi

Même rôle qu'en SPEC-004 §9 : chaque ligne est une demande qui sera formulée pendant le
développement, et qui doit être refusée avec son motif. Suivant L19, on cherche d'abord la forme
d'interface qui rend la faute inexprimable ; la règle écrite n'est que le dernier recours.

| Demande prévisible | Refus | Motif |
|---|---|---|
| « Vider la file d'événements, c'est plus simple » | il n'y a pas de drain | un drain a un consommateur unique — §3.2 |
| « Lire la texture d'écume au CPU pour l'audio » | aucune fonction ne rend une texture au CPU | ≈500 Mo/s et trois frames de latence — §4.2 |
| « Calculer une portance depuis `ListenerAggregate` » | le type ne porte aucune densité locale, seulement des intégrales | §4.5 ; la portance passe par `WaterSample.aeration` |
| « Calculer un danger depuis la vitesse de surface » | `flow_speed` est sur ce canal ; l'autre s'appelle `u_total` | l'orbitale vaut 0,63 m/s à Hs = 1 m — §5.5 |
| « Interroger la traversabilité en un point » | aucune fonction ponctuelle ; seulement des tuiles | c'est le retour à `EvalWater` qu'ADR-018 §1 refuse |
| « Utiliser `t_next_cross` sans regarder la cause » | `CrossCause` est dans la structure, pas à côté | une prédiction sans son hypothèse — §5.4 |
| « Mémoriser l'ancre d'une cascade une fois pour toutes » | l'ancre voyage dans chaque instantané | la cascade suit l'observateur — §4.1 |
| « Attendre la prochaine publication » | `acquire` ne bloque jamais et renvoie l'invalide | §2.2, et un fil audio ne se bloque pas |
| « Publier depuis un fil de travail » | `publish_*` n'est appelable que du fil de simulation | §2.7 |
| « Ajouter un cinquième canal vite fait » | `ChannelDesc` exige cadence, forme, autorité, anneau | c'est ce vide de forme qui a produit E04 |

---

## 9. Ce qui reste ouvert

1. **Résolution et nombre de cascades de `F`** — ouvert depuis ADR-014 §7.1, tranché au banc B9.
   Le `FoamCascadeDesc` de §4.1 est écrit pour ne pas dépendre de la réponse.
2. **Une ou deux bandes radiales pour l'occlusion par secteur** (§4.3). Le défaut connu — une source
   placée entre l'auditeur et un rideau de bulles reçoit une occlusion qu'elle ne subit pas — est
   décrit ; reste à savoir s'il s'entend. Mesure, pas arbitrage.
3. **Rayons de l'agrégat d'écume** — 10, 50 et 200 m sont un point de départ, à calibrer avec
   l'équipe audio contre les distances de coupure de son propre LOD (ADR-016 §7).
4. ~~**Le trait de côte mobile** conditionne le nombre d'états de la polyligne (§6).~~ **Tranché en
   S18 par [ADR-027](../adr/ADR-027-les-cinq-arbitrages-tranches.md) §4 : le trait de côte est
   mobile.** La polyligne est donc publiée **par phase de marée**, seize états stockés par plage
   comme la bibliothèque côtière — ce que §6 prévoyait déjà comme cas nominal. Le cas dégénéré à une
   seule phase n'a plus lieu d'être.
5. **Visibilité sous-marine pour l'IA** — ADR-018 §7.4 pose la question, la donnée existe
   (ADR-019 §3), l'usage est à confirmer avec l'équipe IA. Si elle est retenue, c'est un champ de
   plus dans `TraversabilitySample`, pas un canal de plus.
6. **Table des seuils par archétype d'agent et de véhicule** — ADR-018 §7.3, à obtenir des équipes
   concernées. Ce document publie les grandeurs ; les seuils appartiennent à qui les applique.
7. **Un canal de pluie sur l'eau ?** ADR-016 §8.3 le suggère. Il serait de forme `Agregat`, piloté
   par les mêmes données. À trancher avec l'audio.
8. ~~**Représentation binaire d'échange**~~ → **S11 : doublon**, porté par
   [SPEC-004 §10.1](SPEC-004-interfaces.md). Trois documents posaient la même question en se
   justifiant les uns par les autres — le symptôme à retenir étant qu'un point ouvert qui se
   justifie par le fait que d'autres le posent aussi ne devrait pas exister.
