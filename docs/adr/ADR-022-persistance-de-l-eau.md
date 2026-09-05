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
