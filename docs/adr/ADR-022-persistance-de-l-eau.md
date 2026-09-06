# ADR-022 — La persistance de l'eau

- **Statut** : proposée
- **Session** : S10
- **Remplace** : la fonction que ADR-007 §3 assignait à `condense` / `restore` — « persistance hors
  caméra ». La décision d'ADR-007 n'est pas réécrite ; celle-ci lui succède et dit pourquoi.
- **Résout** : ADR-007 §5.3 (« format exact de `CondensedState` → ADR à écrire ») ;
  SPEC-004 §10.2 ; la configuration L22 signalée par S09 entre `CondensedState` et `CoastalState`
- **Dépend de** : ADR-003, ADR-010, ADR-012, ADR-013, ADR-021, SPEC-005
- **Produit** : invariant **I-17**

---

## 1. Décision

Trois énoncés, dont le premier commande les deux autres.

> **1. Aucun état de la couche δ n'est jamais sérialisé** — ni pour le hors caméra, ni pour une
> sauvegarde, ni pour le réseau, ni pour un rechargement de niveau. *(Invariant I-17.)*
>
> **2. `condense` et `restore` n'échangent pas un état, ils échangent une graine.** `condense` est
> une opération de **l'outil de cuisson** ; `restore` est une opération de **l'exécution**. Le type
> qu'ils partagent est le `SeedState` de §3, qui généralise le `CoastalState` de SPEC-005 §6 et
> remplace le `CondensedState` de SPEC-004 §10.2.
>
> **3. L'état persistant de l'eau tient en trois choses** : `T_sim`, le journal des événements W
> encore vivants, et les volumes entiers des nœuds V modifiés par rapport à leur valeur d'auteur.
> Rien d'autre. C'est vrai d'une sauvegarde, d'une reconnexion, d'un redémarrage de serveur et
> d'un rejeu de harnais — les quatre situations ont la même réponse, et c'est le signe qu'elle est
> la bonne.

---

## 2. La question était mal posée, et le corpus le disait déjà

### 2.1 Une contradiction interne à S01, restée neuf sessions

Dans ADR-007 §3, les deux signatures sont introduites par ce commentaire :

```
// persistance hors caméra (ADR-001, architecture_globale §9)
CondensedState condense();
void           restore(const CondensedState&);
```

Dans ADR-013 §6, écrit la **même session**, sur la même question `§9` :

> « Autrement dit : **il n'existe pas de “simulation ralentie hors caméra”**. Le repli hors caméra
> n'est pas une simulation dégradée, c'est un changement de couche. Cela supprime toute la
> question `§9` “quelle méthode mathématique pour la simulation hors caméra”. »

Le mécanisme a été dissous ; **la signature écrite pour lui est restée**. Pire, ADR-007 §5.3 a
inscrit « format exact de `CondensedState` → ADR à écrire (persistance hors caméra) » : une tâche
créée pour servir un besoin qui n'existait déjà plus, et qui a traversé neuf sessions, deux revues
croisées et une spécification d'interfaces au statut « dernier document avant l'écriture de code ».

C'est la leçon **L35** une seconde fois, une session après qu'elle a été écrite pour un cas
identique (`drain_outgoing_events`) : une décision se propage vers la prose, qui l'explique, et pas
vers les signatures, qui n'ont l'air de rien affirmer.

### 2.2 Pourquoi deux audits ne l'ont pas vue

La revue croisée S05 a confronté les vingt ADR entre eux ; la revue S08 a confronté les cinq SPEC.
Ni l'une ni l'autre n'a relevé ce point, alors qu'il est visible en rapprochant deux paragraphes.

L'explication tient à la forme, pas à l'attention : **le point vivait dans une liste « ce qui reste
ouvert »**. Un audit vérifie ce qui est *affirmé* ; un point reporté se lit comme une lacune connue
et suivie, c'est-à-dire comme quelque chose dont on sait déjà qu'il n'est pas résolu. Personne ne
va vérifier qu'une question ouverte a encore un objet.

C'est une lacune de méthode, pas un accident, et elle est reportée dans `notes/LECONS.md`.

### 2.3 Ce qui doit survivre à la destruction d'un domaine, couche par couche

| Couche | À conserver | Pourquoi |
|---|---|---|
| **B** | **rien** | I-02 — le fond ne stocke rien, il est recalculé à partir de `T_sim` |
| **W** | le journal des événements vivants, 45 o pièce | ADR-003 §3 — `advance(t)` est une fonction pure du journal et de `t` |
| **δ perturbatif** | **rien** | naît à δ = 0 (ADR-013 §3) · destruction gratuite (I-12) · aucune autorité (I-04) |
| **δ substitutif tenant la masse d'un nœud V** | l'entier `i64`, rendu au nœud | ADR-010 §6 — le transfert δ→V est déjà spécifié, et il rend une masse, pas un champ |
| **δ substitutif établissant un train de vagues** | **rien à capturer** | son état vient d'une donnée **cuite**, pas d'une capture d'exécution — ADR-013 §4, SPEC-005 §6 |
| **`F`, `A`** | **rien** | ADR-014 §2.3 — ce qui sort de la cascade est perdu, sans conséquence ; l'écume permanente est re-dérivée de W |
| **V** | `volume_ml` des nœuds modifiés | ADR-010 §7 |

### 2.4 Quatre confirmations indépendantes

Une seule aurait suffi à rendre la thèse plausible ; quatre la rendent difficile à contourner, et
elles ne partagent aucune prémisse.

1. **I-04.** δ n'a aucune autorité gameplay. Perdre son état ne peut, par construction, changer
   aucune issue de jeu.
2. **ADR-021 §3.2, argument de fermeture.** δ ne contient, *par définition de `λ_cut`*, que ce qui
   est plus court que `λ_cut` : du court, du local et du bref. Il n'y a pas de phénomène durable à
   y perdre.
3. **I-12.** Créer et détruire un domaine est visuellement gratuit — c'est la propriété que toute
   l'architecture défend et que ADR-001 a été conçue pour offrir. Un état de δ qu'il faudrait
   sérialiser la contredirait frontalement.
4. **SPEC-003 §8**, et c'est la plus convaincante parce qu'elle vient d'un document écrit pour tout
   autre chose. Le harnais rejoue **une session de jeu entière** à partir de
   `(T_sim, descripteurs de région, journal d'événements, trajectoires)`. Si une session se rejoue
   à l'identique sans aucun état δ, alors l'état δ ne fait pas partie de l'état du monde. Ce n'est
   pas une analogie : c'est une démonstration, et elle était déjà écrite.

### 2.5 Invariant produit

> **I-17 — Aucun état de la couche δ n'est jamais sérialisé.** Ni sur disque, ni sur le réseau, ni
> dans une sauvegarde, ni dans un mécanisme de repli. Ce qui traverse une frontière de persistance
> est toujours l'un des trois : `T_sim`, un événement W, un volume entier. Une proposition qui
> demande à écrire un champ de δ quelque part est refusée sans discussion de détail.

L'invariant vaut mieux que la décision seule : il rend la question inutile à reposer. Sans lui, la
demande reviendra — « juste pour ce cas-là », « juste pour le débogage » — et elle reviendra sous
une forme qui aura l'air raisonnable.

### 2.6 La contre-épreuve, et le cas qui ne passe pas

Une thèse qui n'a pas été mise à l'épreuve n'est pas acquise. Le cas qui résiste est celui du
domaine **substitutif**, et il est réel.

Un domaine substitutif n'est **pas** gratuit à recréer : 40 s d'établissement depuis rien
(ADR-013 §4), ou quelques secondes depuis une graine — 4,4 à 8 s selon l'encadrement établi en S08
(écart E02). Or ADR-012 §4 **rang 5** détruit les domaines non focaux pour tenir le budget, et
ADR-012 §5 engage toute décision de dégradation pour « au moins 30 frames », soit **1 s à 30 Hz**.

```
fenêtre d'engagement de la dégradation           :  1 s
coût de restauration d'un domaine substitutif    :  4,4 à 8 s   (depuis une graine)
                                                    40 s        (depuis rien)
```

Un déferlement non focal peut donc être détruit puis redemandé **quatre à huit fois plus vite qu'il
ne se rétablit**. Le joueur qui balaie la caméra le long d'une côte verrait des plages apparaître
en retard, ou pire, apparaître à demi établies.

**Décision.** Le rang 5 d'ADR-012 §4 ne s'applique pas aux domaines substitutifs. S'ils doivent
céder, leur hystérésis se dimensionne sur **leur temps de restauration**, pas sur une constante de
30 frames. La règle générale, qui déborde ce cas :

> Une dégradation dont la fenêtre d'engagement est plus courte que le coût de restauration de ce
> qu'elle détruit est un **générateur de pompage**, et le pompage est plus visible que la
> dégradation qu'on cherchait à éviter (ADR-012 §5 le dit déjà pour la manette de qualité).

Cela ne contredit pas I-17 : la restauration se fait depuis une **graine cuite**, jamais depuis une
capture d'exécution. Le cas qui résistait renforce la décision au lieu de l'affaiblir — il dit
seulement que l'ordonnanceur doit connaître le coût de ce qu'il détruit.

---

## 3. `SeedState` : ce que `condense` et `restore` échangent réellement

### 3.1 Deux noms pour un mécanisme

| Écrit en | Sous le nom | Rôle décrit | Forme supposée |
|---|---|---|---|
| S04, SPEC-004 §10.2 | `CondensedState` | « persistance hors caméra », reporté | sérialisation d'un état de solveur |
| S06, SPEC-005 §6 | `CoastalState` | amorcer une zone de déferlement sans attendre 40 s | **condition initiale 2D**, 77 Ko |

Les deux répondent à la même question : *comment amener un domaine dans un état non trivial sans le
simuler depuis zéro ?* Écrits à deux sessions d'intervalle, sous deux noms, dans deux documents.
C'est la configuration de **L22**, et la résolution qu'elle prescrit est de ne pas arbitrer entre
les deux mais de remonter au mécanisme manquant.

Il est déjà à moitié écrit : SPEC-005 §6 a trouvé la bonne forme — **une condition initiale 2D, pas
un volume 3D figé** — et l'a trouvée pour la bonne raison, un calcul de volume de données
(197 Mo par plage contre 1,2 Mo). Ce qui manquait, c'est de voir que cette forme n'a rien de côtier.

> **Décision.** Le type est **`SeedState`**. `CoastalState` en devient un cas d'emploi
> (`kind = Cotier`) et `CondensedState` disparaît.

### 3.2 La structure

```cpp
enum class SeedKind : uint8_t {
    Cotier,     // zone de déferlement — SPEC-005 §6
    Bassin,     // grand plan d'eau intérieur avec un état de ballottement établi
    Auteur      // condition initiale voulue par un concepteur : une ruine inondée au chargement
};

struct SeedParams {                  // ce SOUS QUOI la graine a été cuite
    half     hs, tp, theta;          // état de mer incident
    uint8_t  tide_phase;             // index de phase, pas une valeur continue
    vec3     g_eff_dir;  half g_eff_mag;
    uint8_t  liquid_id;
};

struct SeedState {
    uint32_t     size;               // extension par ajout — SPEC-004 §1.4
    SeedId       id;                 // sha256 du contenu — SPEC-005 §7.3
    SeedKind     kind;
    SeedParams   params;

    uint16_t     nx, ny;             // grille 2D
    float        dx_seed;            // 0,5 m pour le côtier — SPEC-005 §6
    half         nominal_mass_t;     // masse nominale, en tonnes — cf. §3.5

    span<const half> h, u, v, roller;   // hauteur, vitesses, intensité de rouleau
};
```

Volumes inchangés par rapport à SPEC-005 §6, la généralisation ne coûtant rien : **77 Ko par
état**, 1,2 Mo par plage pour seize états, 60 Mo pour cinquante plages avant compression.

### 3.3 `condense` est une opération d'outil, et l'interface doit l'empêcher d'être autre chose

`condense` capture ce que l'outil de cuisson vient de simuler pour en faire une graine
(SPEC-005 §7.1). Ce n'est **jamais** une opération d'exécution. La laisser sur `IFluidSolver`, où
tout hôte y a accès, garantit qu'elle sera un jour appelée en jeu « juste pour essayer ».

Suivant **L19** — rendre l'interdit inexprimable plutôt que l'interdire :

```cpp
// Obtenue UNIQUEMENT par un hôte de cuisson. Un hôte de jeu n'en reçoit jamais de pointeur.
class ISeedProducer {
public:
    virtual Result condense(SeedState* out) const = 0;
};

// Sur IFluidSolver, il ne reste que la moitié qui a lieu à l'exécution :
//   virtual Result restore(const SeedState&) = 0;
```

Un hôte de jeu ne peut pas condenser : il n'a pas le type. La règle n'a pas à être écrite dans une
consigne de revue, ce qui est toujours le dernier recours et jamais le premier.

### 3.4 Une graine est un actif cuit, et cela referme l'écart E07

SPEC-004 §10.2 énonçait la contrainte suivante sur `CondensedState` : *« il doit se relire sur une
machine différente, donc pas de disposition mémoire brute »*. Cette contrainte est exactement juste
— **pour un actif cuit**, et dépourvue de sens pour une condensation en mémoire d'un domaine qui
sort du champ de la caméra. Elle était le signe, écrit dès S04, que le champ décrivait une donnée de
pipeline sans qu'on l'ait vu.

Elle referme aussi l'écart **E07** de la revue S08. Une graine est produite en faisant tourner δ,
qui n'est **jamais D1** (SPEC-003 §2) : deux machines ne produiront pas le même octet. Cela n'a
aucune importance, parce qu'une graine n'est pas un calcul reproductible, c'est un **actif
identifié par l'empreinte de son contenu** — un producteur désigné, un `bake_manifest`, une
promotion explicite. Les deux résolutions se rejoignent sans avoir été conçues ensemble, ce qui est
le meilleur indice qu'elles sont justes.

### 3.5 Trois règles d'emploi

**Interpoler les paramètres, jamais les champs (I-09).** Entre deux graines, on interpole `Hs`,
`Tp` et la phase de marée, jamais `h`, `u`, `v`. SPEC-005 §6 le disait déjà pour le côtier : deux
conditions initiales moyennées produisent une mer **plus calme que les deux**, exactement comme deux
champs de houle indépendants font chuter `Hs` de 29 % (L05).

**Une graine porte les paramètres sous lesquels elle a été cuite, et `restore` refuse au-delà d'un
écart.** C'est le **seul endroit du système où un seuil de tolérance physique existe**, et le
contraste avec ADR-013 §3 mérite d'être noté : là-bas, la question « quelle erreur de précalcul est
acceptable ? » s'était **dissoute**, parce qu'un domaine préparé en palier T2 ne contient aucune
information physique — δ y vaut identiquement 0, et il n'y a rien à transporter. Une graine, elle,
en contient. Le seuil est donc réel, et il est à calibrer (banc B4).

**La masse est autoritaire, la graine ne l'est pas.** Un domaine substitutif couplé à un nœud V
reçoit sa masse du nœud (ADR-010 §6, transfert explicite) ; la graine ne fournit que la **forme** de
l'écoulement, renormalisée sur `nominal_mass_t`. Sans cette règle, amorcer un compartiment depuis
une graine créerait ou détruirait de l'eau — et la couche V est justement celle où la conservation
est exacte et entière.

---

## 4. Ce que l'eau met dans une sauvegarde

Personne n'avait posé la question, et elle sera posée — probablement tard, par l'équipe qui écrit
le format de sauvegarde, et avec une échéance.

### 4.1 Quatre situations, une seule réponse

| Situation | Ce qu'il faut transmettre ou écrire |
|---|---|
| Sauvegarde / rechargement | `T_sim`, descripteurs de région, événements W vivants, nœuds V modifiés |
| Arrivée en cours de partie | *idem* — ADR-003 §3 le disait déjà : « rien à transférer sinon `T_sim`, les descripteurs de région et la liste des événements W encore vivants » |
| Reconnexion | *idem* |
| Redémarrage de serveur | *idem* |

**Le fichier de sauvegarde et la charge utile d'une arrivée en cours de partie sont le même objet.**
Ce n'est pas une coïncidence heureuse mais une conséquence d'ADR-003 : dès lors que l'état du monde
se réduit à un temps et à un journal, il n'y a qu'une manière de le transmettre, et le destinataire
— disque, réseau, harnais — n'y change rien.

Trois conséquences pratiques, toutes gratuites :

- **un seul format à écrire, à versionner et à tester** ;
- **un seul cas de non-régression** : le harnais rejoue déjà exactement cet objet (SPEC-003 §8), ce
  qui teste la sauvegarde sans qu'aucun test de sauvegarde ne soit écrit ;
- un défaut dans l'un est un défaut dans l'autre, donc il se trouve deux fois plus vite.

### 4.2 La structure

```cpp
struct WaterPersistentState {
    uint32_t   size;
    uint32_t   format_version;

    SimTime    t_sim;
    span<const RegionDescriptor> regions;     // état de mer par région — ADR-004 §2.2
    span<const WaveEvent>        events_alive; // ttl non échu, ORIGINE `Serveur` UNIQUEMENT
                                                 // SPEC-006 §3.1, 50 o pièce — cf. note S13
    span<const VNodeDelta>       v_nodes;      // cf. §4.3
};

struct VNodeDelta {                 // 20 octets
    uint64_t node_id;
    int64_t  volume_ml;
    uint8_t  liquid_id;
    uint16_t flags;
};
```

> **Note corrective (S13, écarts E11 et E04).** Ce champ n'était pas filtré. `WaveEvent` porte
> depuis SPEC-006 §3.1 un `EventOrigin` à trois valeurs, dont deux — `AnticipationLocale` et
> `TransductionLocale` — sont **locales, cosmétiques et non répliquées** (ADR-021 §3). Les écrire
> aurait trois effets : emporter des événements que le serveur n'a jamais eus ; mêler au tri par `id`
> des identifiants qui ne sont pas des `server_seq` ; et surtout **rendre le cas C19 faux**, puisque
> ces événements viennent d'un solveur δ, qui n'est jamais D1 — le hash différerait par intermittence
> et le banc conçu comme l'argument phare d'I-17 échouerait pour une cause étrangère à I-17.
>
> `events_alive` ne retient donc que l'origine `Serveur`. Ce n'est pas une restriction ajoutée mais
> **appliquée** : le §1 de cet ADR l'imposait déjà, en désignant par « W » ce qui est répliqué.
> *(La taille unitaire passe par ailleurs de 45 à **50 octets**, écart E04 : ≈205 Ko au pire au lieu
> de 180.)*

**Ordres de grandeur.** Les événements vivants sont bornés par le budget de paquets du profil
(`paquets_W_max = 4096`, ADR-012 §3), un événement se développant en plusieurs paquets : au pire
≈**180 Ko**, en pratique bien moins. Les nœuds V dominent donc, et à 20 octets pièce, cent mille
nœuds modifiés font **2 Mo**. Une sauvegarde d'eau est un petit objet, et il faut le dire
maintenant : sans ce chiffre, quelqu'un dimensionnera son format en supposant qu'un système d'eau
coûte cher à sauvegarder, et concevra un mécanisme de découpage dont personne n'a besoin.

### 4.3 On enregistre l'écart à la valeur d'auteur, jamais l'état complet

Un nœud V d'auteur — une citerne, une piscine de niveau — a une valeur initiale qui vient des
données du niveau. La sauvegarde n'écrit que les nœuds **dont le volume diffère de cette valeur**.

C'est la règle de source de vérité unique de SPEC-005 §1 appliquée à la persistance, et le motif est
le même que celui de **L25** : la donnée d'auteur reste la vérité, la sauvegarde est une **entrée
supplémentaire** et jamais une retouche du produit. Écrire l'état complet ferait qu'une correction
d'équilibrage sur le niveau — agrandir une citerne, déplacer une piscine — serait silencieusement
écrasée par toutes les sauvegardes existantes.

**Ce qui borne la taille d'une sauvegarde est le TTL d'ADR-010 §7**, et c'est là son rôle réel : les
nœuds créés par le jeu (flaques, inondations sans conséquence) sont retirés après trente minutes
sans visite et reconvertis en mouillage de surface. Sans ce mécanisme, chaque flaque jamais revisitée
d'un monde persistant resterait dans l'état du monde indéfiniment. Le TTL n'est pas un nettoyage
cosmétique : c'est **la borne supérieure de la persistance de l'eau**, et il devrait être présenté
comme tel dans toute discussion d'équilibrage qui proposerait de l'allonger.

### 4.4 Sauvegarder provoque un règlement δ→V

Si un nœud est gelé et sa masse remise à un domaine substitutif au moment où l'on sauvegarde
(ADR-010 §6), il n'y a **rien à écrire** : la masse n'est plus dans un entier, elle est dans un
champ que I-17 interdit d'écrire.

**Règle.** Une demande de persistance force le transfert δ→V avant d'écrire. Le domaine rend
`M'`, l'écart `M' − M` est journalisé comme dans le cas normal, et ce qui est écrit est un entier.
Un joueur qui sauvegarde pendant qu'une coursive s'inonde retrouve la coursive au même volume, à la
perte contrôlée près — laquelle est mesurée, pas subie.

Corollaire à ne pas manquer : **une sauvegarde est un point de synchronisation**, au même titre
qu'un franchissement de frontière de domaine. Elle ne peut pas être prise à un instant arbitraire
d'un tick ; elle se prend entre deux ticks, après `end_tick` (SPEC-004 §3).

### 4.5 Ce qui n'est jamais écrit

La liste est plus utile que son complément, parce que c'est elle qu'on essaiera d'allonger :

| Jamais écrit | Pourquoi |
|---|---|
| Tout champ de δ | **I-17** |
| L'état de B | I-02 — recalculé depuis `T_sim` |
| Les paquets W | dérivés du journal d'événements par `advance(t)`, fonction pure — ADR-003 §3 |
| Les champs `F` et `A` | cascades transitoires ; l'écume permanente est re-dérivée de W — ADR-014 §2.3 |
| La polyligne de déferlement, les graines, la bibliothèque côtière | données **cuites**, identifiées par empreinte — SPEC-005 §7.3 ; elles appartiennent à la version du jeu, pas à la partie |
| Le signal de traversabilité | entièrement dérivé de ce qui précède — SPEC-006 §2.6 |

### 4.6 Monde persistant : le système d'eau ne décide pas de la propriété

Dans un monde partagé et persistant, il n'y a pas de « fichier de sauvegarde » mais un état de monde
tenu en continu. La réponse ne change pas — les trois mêmes choses — mais **qui les tient** est une
question d'infrastructure, pas d'eau.

Position du système d'eau, la même qu'ADR-018 §1 sur la navigation : il **publie et accepte** un
`WaterPersistentState`, il n'a pas d'opinion sur qui le stocke, à quelle granularité de partition ni
sous quelle politique de propriété. Il fournit en revanche ce qu'il faut pour que la question soit
décidable : l'état est **additif par nœud et par événement**, donc partitionnable par région sans
coordination.

Ce qui relève réellement d'une décision humaine — et qui est signalé, non tranché : la durée de vie
d'un nœud V rattaché à un objet appartenant à un joueur absent depuis des mois. C'est une politique
de monde, avec des conséquences de stockage et de gameplay, et elle n'appartient pas à l'équipe eau.

---

## 5. La couche V est la seule persistance vraie — et la seule que le serveur exécute

### 5.1 Ce que le corpus impliquait sans le dire

I-10 énonce : « Le serveur ne simule pas d'eau. Il tient des enregistrements d'événements dont il
sait calculer analytiquement l'amplitude. **Il n'exécute ni W ni δ.** »

W et δ, nommément. **Pas V.** Et l'omission n'est pas un oubli, elle est nécessaire : la couche V
porte des conséquences de jeu directes — une coursive qui s'inonde, une citerne qui se vide, un
compartiment qui fait chavirer un navire. Une donnée de cette nature ne peut pas être locale à un
client.

Rapprochons ce que V est, d'après ADR-010 : arithmétique **entière** en millilitres, pas fixe de
100 ms, débits quantifiés avec report de reste, aucune masse créée ni perdue, résultat
reproductible. C'est terme pour terme la définition d'une grandeur autoritaire au sens d'**I-15** :
tous les participants peuvent la calculer à l'identique à partir de données répliquées.

> La forme de la couche V n'était donc pas un choix de commodité d'implémentation. C'est la forme
> qu'une couche doit avoir pour être **exécutée par le serveur et autoritaire**, et ADR-010 l'avait
> trouvée sans énoncer cette raison-là.

Conséquence pratique à signaler à l'équipe serveur : **le serveur charge des données cuites.** Il
lui faut au minimum les `shape_lut` (ADR-010 §2) pour convertir un volume en hauteur, sans quoi il
ne peut ni décider d'un débordement ni évaluer une ligne de flottaison. Un serveur « sans assets »
n'est pas une option.

> **Note corrective (S13, écart E05).** Cette liste est incomplète. SPEC-006 §2.6 fait en outre
> évaluer au serveur le **signal de traversabilité** et rendre autoritaires les franchissements de
> seuil, ce qui exige la **bathymétrie** et les **champs de courant C1 cuits** en plus des
> `shape_lut`. Chacun des deux documents était juste, aucun n'était complet, et c'est une contrainte
> de déploiement (angle mort A77) qu'une équipe serveur lira dans l'un **ou** dans l'autre.

### 5.2 Correction de l'invariant I-03

I-03 énonce : « **B et W répliqué sont déterministes.** Bit à bit, entre plateformes. »

Or SPEC-003 §2, écrite en S03, place explicitement **V** dans le régime **D1 — exact,
inter-plateforme** :

| Régime | Portée | Ce qu'on compare |
|---|---|---|
| **D1 — exact, inter-plateforme** | B, W répliqué, **V** | hash |

Les deux documents ne disent pas la même chose, et c'est SPEC-003 qui a raison : sans déterminisme
inter-plateforme de V, un serveur Linux et un client Windows divergeraient sur le volume d'un
compartiment, c'est-à-dire sur une issue de jeu. ADR-010 §4 avait d'ailleurs pris toutes les
dispositions nécessaires — entiers, report de reste, ordre fixé.

> **I-03 est amendé pour inclure V.** Un invariant ne se change que par un ADR explicite ; c'en est
> un. La formulation retenue est portée dans `01_INVARIANTS.md` avec la date et le renvoi ici.

Écart de gravité 3, sans conséquence sur aucune décision — mais il portait sur un **invariant**,
c'est-à-dire sur le document qu'on cite pour refuser une proposition. Un invariant incomplet finit
par autoriser ce qu'il devait interdire.

---

## 6. Conséquences sur les documents existants

| Document | Ce qui change |
|---|---|
| `01_INVARIANTS.md` | **I-17** ajouté ; **I-03** amendé pour inclure V |
| ADR-007 §3, §5.3 | note corrective : `condense`/`restore` ne servent pas la persistance hors caméra ; la tâche « ADR à écrire » est close par celui-ci |
| ADR-012 §4 | note corrective : le rang 5 ne s'applique pas aux domaines substitutifs (§2.6) |
| SPEC-004 §4 | `condense` quitte `IFluidSolver` pour `ISeedProducer` ; `restore` prend un `SeedState` |
| SPEC-004 §10.2 | point ouvert **résolu** |
| SPEC-005 §6 | `CoastalState` devient `SeedState` de `kind = Cotier` ; renvoi ici |
| SPEC-003 | un cas de non-régression à ajouter (§6.1) |
| SPEC-006 | **rien** — et c'est un contrôle passé : aucun des quatre canaux poussés n'a d'état à persister, tous étant dérivés de ce qui précède |

### 6.1 Le cas de non-régression que cette décision rend possible

Puisque la sauvegarde et la charge utile d'arrivée en cours de partie sont le même objet (§4.1), une
seule assertion couvre les deux :

> **C19 — aller-retour de persistance.** Simuler `N` secondes, écrire un `WaterPersistentState`,
> repartir d'un système neuf restauré depuis cet objet, simuler `M` secondes de plus. Le hash de B
> et de W répliqué à `t = N + M` doit être **identique** à celui d'une simulation continue de
> `N + M` secondes.

Le test est en régime **D1**, donc binaire et exécutable en mode `check` — moins de 60 s, à chaque
commit (SPEC-003 §4). Il n'aurait pas été possible si un état de δ figurait dans la sauvegarde :
δ n'est jamais D1 (SPEC-003 §2), et l'assertion aurait dû être statistique, donc faible.

**C'est le bénéfice secondaire d'I-17**, et il vaut d'être noté au titre de L06 : une bonne décision
d'architecture rend gratuites des opérations qui étaient coûteuses. Ici, elle rend *vérifiable* une
propriété qui, autrement, ne l'aurait pas été.

---

## 7. Ce qui reste ouvert

1. **Seuil de tolérance sur les paramètres d'une graine** (§3.5) — jusqu'à quel écart de `Hs`, de
   `Tp` ou de phase de marée un `restore` reste-t-il acceptable ? Banc **B4**. C'est le seul seuil
   de tolérance physique du système, et il est réel parce qu'une graine contient de la physique.
2. **Durée de vie d'un nœud V rattaché à un objet d'un joueur absent depuis des mois.** Politique de
   monde, avec des conséquences de stockage et de gameplay. **N'appartient pas à l'équipe eau** —
   signalé, non tranché.
3. **Granularité de partition** de l'état persistant dans un monde à très grande échelle. L'état
   est additif par nœud et par événement, donc partitionnable par région sans coordination ; le
   découpage retenu relève de l'infrastructure.
4. ~~**Représentation binaire** — boutisme, alignement, versionnement du format.~~
   → **S11 : doublon**, porté par [SPEC-004 §10.1](../specs/SPEC-004-interfaces.md). La formulation
   d'origine — « comme partout » — était elle-même le symptôme.
5. **Une observation à confirmer, pas une question.** Une sauvegarde survit à une mise à jour du
   jeu : ne contenant aucun état de δ ni aucune donnée cuite — seulement un temps, des événements
   et des entiers — elle se recharge sur une version où les graines et la bibliothèque côtière ont
   changé. Un **rejeu** de harnais, lui, ne le survit pas : il exige la même version d'actifs pour
   être bit à bit. Les deux usages du même objet n'ont donc pas la même tolérance aux versions, et
   il vaut mieux l'écrire maintenant que le découvrir quand une campagne de non-régression cassera
   sur un changement de plage.
