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
