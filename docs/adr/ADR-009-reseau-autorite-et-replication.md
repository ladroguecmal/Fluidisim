# ADR-009 — Réseau : réplication d'événements, pas de champs

- **Statut** : proposée
- **Session** : S01
- **Résout** : `zones_ouvertes §2, §30`
- **Dépend de** : ADR-001, ADR-003, ADR-005, ADR-008

---

## 1. Décision

Le serveur ne réplique **aucun champ d'eau**. Il réplique cinq choses, toutes petites :

| Donnée | Volume | Fréquence | Nature |
|---|---|---|---|
| `T_sim` | 8 o | 1 Hz + resync | horloge autoritaire |
| Grille `HydroSample` | quelques Ko/région | au streaming, versionnée | statique / météo lente |
| Événements W | 40 o | à l'occurrence | source de perturbation |
| État des nœuds V | 16 o/nœud | 1–5 Hz, delta | volumes finis |
| Hash de conformité | 8 o | 1 Hz | détection de divergence (ADR-003 §2.4) |

Tout le reste est **dérivé identiquement** chez chaque participant.

## 2. Événement W

```
struct WaveEvent {                  // 40 octets
    id           : u64              // (server_seq) — ordre total, déduplication
    frame_id     : u32
    origin_local : vec3<f16>  + cell : u60 morton   // position via HydroGrid
    t_birth      : u64              // µs, dans T_sim
    kind         : u8               // impact, sillage, explosion, effondrement, transduction δ
    energy       : f16              // J/m ou J selon kind
    dir          : f16[2]
    lambda       : f16
    ttl_hint     : f16
}
```

Chaque client développe l'événement en paquets d'ondes par une fonction **pure et déterministe**
`expand(WaveEvent, T_sim) → paquets`. Aucun état de paquet ne transite jamais sur le réseau.

**Débit attendu** dans une zone chargée (combat naval, 20 événements/s) : 800 o/s par joueur
intéressé. Négligeable devant le trafic d'entités.

## 3. Autorité et anti-triche

> **Correction (S05, écart R03).** Cette section supposait que le client émettait une demande
> d'événement issue de sa transduction δ→W, et que le serveur la plafonnait par une cause connue.
> Le raisonnement contenait sa propre réfutation : **si le serveur connaît la cause, il peut émettre
> l'événement lui-même**, et le chemin client → serveur est inutile.
>
> C'est la décision retenue (ADR-021 §3). Le serveur émet depuis la cause ; la transduction client
> ne produit que du `W_local` non répliqué. Le plafonnement décrit ci-dessous **n'a plus lieu
> d'être** — le paramètre `K` survit côté serveur comme réglage d'équilibrage, plus comme garde-fou.
>
> Bénéfice de sécurité : l'angle mort A16 n'est plus atténué, il **disparaît**, faute de chemin
> d'énergie du client vers le monde répliqué.

Le seul chemin par lequel un client peut faire naître un événement W est la transduction δ→W
(ADR-005 §3). Un client modifié pourrait donc y injecter une énergie arbitraire — et comme W est
autoritaire sur le gameplay (ADR-008), cela reviendrait à laisser un joueur déclencher un tsunami.

**Décision** : la transduction ne crée pas d'événement répliqué directement. Elle produit une
*demande*, que le serveur borne :

```
E_admise = min( E_demandée , K · E_cause )
E_cause  = énergie cinétique de l'objet responsable, connue du serveur
K        = 0,3  (fraction maximale transmise à la surface)  — à calibrer
```

Si aucune cause serveur n'est identifiable, l'événement est **purement local** : il reste dans le
W du client demandeur, non répliqué, et n'a aucune conséquence gameplay. Une éclaboussure sans
cause est un effet visuel.

Corollaire : la couche W a deux populations, `W_répliqué` et `W_local`. Elles s'additionnent au
rendu ; seule `W_répliqué` est lue par la flottabilité et par les forces sur les joueurs.

## 4. Le serveur n'a pas de caméra

`architecture_globale §9` hiérarchise la conservation hors caméra par la visibilité. Ce critère
n'existe pas côté serveur. La hiérarchie serveur est :

1. **dans le volume d'intérêt d'au moins un joueur** → événement maintenu, réplication active ;
2. **hors intérêt mais conséquence gameplay possible** (dérive vers une zone habitée, chargement
   d'un joueur imminent) → événement maintenu, non répliqué ;
3. **hors intérêt, sans conséquence** → événement retiré quand son amplitude estimée passe sous
   le seuil, par une formule analytique — le serveur ne simule rien.

Le serveur ne fait donc jamais tourner ni δ ni W : il conserve des **enregistrements** dont il sait
calculer l'amplitude à tout instant. C'est ce qui rend la persistance hors caméra gratuite à
grande échelle, contrairement à ce que `§9` laissait craindre.

**Rayon de pertinence** d'un événement : `r = min( r_max , C·√E )`, borné. Un gros événement
intéresse plus de monde. Le routage se fait par cellules `HydroGrid` (ADR-006 §2).

## 5. Prédiction client et resynchronisation

Effet secondaire important : **il n'y a rien à réconcilier sur l'eau**.

Le client qui pilote un bateau calcule ses forces de flottabilité sur B+W, exactement le même
champ que le serveur. L'erreur de prédiction liée à l'eau est nulle à l'erreur d'horloge près
(< 2 cm, ADR-003 §2.1). Le défaut classique — un bateau qui tressaute parce que client et serveur
ne s'accordent pas sur la hauteur de vague — **n'existe pas** dans cette architecture.

Deux joueurs qui se rapprochent n'ont aucune resynchronisation d'eau à faire : ils voient déjà le
même B+W. Seuls leurs δ diffèrent, et δ n'a pas d'autorité. `zones_ouvertes §30`
« resynchronisation lors du rapprochement de deux joueurs » devient sans objet.

## 6. Tolérance de divergence — énoncé exploitable

| Couche | Divergence tolérée |
|---|---|
| B | 0 (hash vérifié) |
| W répliqué | 0 en amplitude/phase ; l'ordre d'arrivée des événements est réconcilié par `id` |
| W local | libre |
| δ | libre |
| V | 0 (entier, ADR-010) |
| Pose des solides | politique réseau générale du projet, inchangée par l'eau |

## 7. Ce qui reste ouvert

1. ~~Valeur de `K` et table `E_cause` par type d'objet.~~
   → **S11 : doublon, et sa nature a changé.** La question est portée par **ADR-021 §7.2**, qui dit
   lui-même qu'elle « change de propriétaire : c'est désormais une donnée d'équilibrage gameplay ».
   `K` n'est plus le garde-fou de sécurité décrit au §3 ci-dessus — ADR-021 §3 a supprimé le
   plafonnement d'une demande client, faute de demande client. Un lecteur de cet ADR seul
   planifierait un travail d'anti-triche là où il ne reste qu'un réglage.
2. ~~Faut-il autoriser un client à *anticiper* localement un événement qu'il vient de causer, avant
   validation serveur ?~~ **Clos.**
   → **S11** : oui, et le mécanisme est spécifié. Deux choses ont changé : la **prémisse est morte** en S05 —
   ADR-021 §3 a supprimé la validation serveur d'une demande client, le serveur ne « borne » donc
   plus rien ; et le mécanisme demandé a été écrit en S09 — SPEC-006 §3.3 traite la réconciliation
   par un **bit de rétractation**, non pour corriger une amplitude mais pour éviter qu'un impact
   anticipé et sa version serveur ne soient joués deux fois à 100–300 ms d'intervalle.
3. Nombre maximal d'événements W actifs par région (protection contre la saturation).
   → **S11** : ce n'est pas une valeur à choisir mais une borne à **dériver** : elle doit s'accorder avec
   `paquets_W_max` (ADR-012 §3, qui compte des *paquets*, pas des événements) et avec le nombre de
   régions actives — donc I-16 interdit de la déclarer. Et le comportement à la borne est déjà
   contraint : **ADR-021 §4** interdit d'élaguer un paquet `W_rep` au-dessus du seuil de pertinence
   gameplay, quel que soit le profil. Une protection contre la saturation ne peut donc pas
   consister à jeter des événements répliqués.
4. Comportement en cas d'horloge client manifestement fausse (triche par décalage temporel) :
   la vérification par hash le détecte, la sanction relève de la politique anti-triche générale.
