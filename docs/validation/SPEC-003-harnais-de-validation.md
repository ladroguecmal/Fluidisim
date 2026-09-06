# SPEC-003 — Harnais de validation

- **Statut** : proposée — élément unique du chemin critique
- **Session** : S03
- **Dépend de** : ADR-003 (déterminisme), ADR-020 (bibliothèque autonome)
- **Conditionne** : les onze bancs de `PLAN-BENCHMARK.md` et l'ensemble des seuils « à calibrer »

---

## 1. Objet et critère de réussite

Le harnais doit rendre les bancs **exécutables, comparables et à l'épreuve des régressions**. Il
n'est pas un outil de test : c'est l'instrument de mesure du projet, et la qualité des décisions
qui suivent est plafonnée par la sienne.

Critère de réussite, en trois lignes :

1. la batterie déterministe tourne en **moins de 60 secondes** sur un poste de développement, sans
   GPU, à chaque commit ;
2. n'importe quel résultat publié est **reproductible** à partir du seul triplet
   `(scénario, build, matériel)` ;
3. le harnais est capable de **contredire** une décision d'architecture — en particulier ADR-001
   par le banc B4.

Un harnais qui ne peut que confirmer est un harnais inutile.

---

## 2. Trois régimes de déterminisme — et le piège du régime intermédiaire

| Régime | Portée | Ce qu'on compare | Verdict |
|---|---|---|---|
| **D1 — exact, inter-plateforme** | B, W répliqué, V | hash | binaire |
| **D2 — reproductible sur la même machine** | δ, avec graine imposée | trace complète | binaire, en local |
| **D3 — statistique** | δ entre machines, GPU, ordonnancements différents | invariants et distributions | enveloppes |

Le régime **D2 est celui qu'on oublie**, et son absence rend le débogage impossible.

Un solveur δ ne sera jamais D1 (ADR-001 §1.3). Il n'y a en revanche aucune raison qu'il ne soit
pas D2 : même binaire, même machine, même graine, même ordre de parallélisation ⇒ même résultat.
Cela demande trois disciplines, à imposer dès le premier jour parce qu'elles sont très coûteuses à
rétroporter :

- toute réduction parallèle est **déterministe** (ordre fixé, pas d'accumulation dans l'ordre
  d'arrivée des tâches) ;
- toute source d'aléa est un PRNG à état entier semé par le scénario, jamais une horloge ni une
  adresse mémoire ;
- le nombre de fils de travail ne change pas le résultat, seulement sa vitesse.

Sans D2, un bug qui apparaît une fois sur cinquante n'est jamais reproduit, donc jamais corrigé.
Avec D2, il l'est toujours.

---

## 3. Le scénario

Un scénario est un **fichier texte court, lisible et comparable en diff**. C'est l'unité de travail
du harnais, et tout le reste en découle.

```toml
[scenario]
id          = "C07-sillage-eau-peu-profonde"
duree_s     = 40.0
t_sim_debut = 1735689600000000       # µs — fixe, donc la houle est identique à chaque fois
graine      = 20260905

[region]
hs = 0.8 ; tp = 5.5 ; theta = 0.0 ; u10 = 7.0
bathymetrie = "sha256:9f3c…"          # référence par empreinte de contenu, jamais par chemin

[[acteur]]
type = "coque"; archetype = "vedette_12m"
trajectoire = [[0,0,0,0.0], [40,400,0,0.0]]   # t, x, y, cap

[solveur]
delta = "candidat_A" ; dx = 0.10 ; lambda_cut = 4.0

[assertions]
angle_sillage      = { ref = "arcsin(1/Fr_h)", tolerance_deg = 2.0 }
derive_masse_par_s = { max = 0.001 }
budget_p99_ms      = { max = 2.0 }
allocations        = { max = 0 }
```

Trois règles de forme, chacune corrigeant un défaut observé partout :

- **Les assertions vivent dans le scénario**, pas dans le code du harnais. Un scénario est donc
  auto-suffisant et lisible par quelqu'un qui n'a pas le code sous les yeux.
- **Les données sont référencées par empreinte de contenu.** Un chemin de fichier casse la
  reproductibilité dès la première réorganisation du dépôt.
- **`t_sim_debut` est explicite.** B étant fonction du temps absolu, un scénario qui démarrerait à
  « maintenant » ne serait pas rejouable. C'est la conséquence directe et non évidente d'ADR-003.

---

## 4. Modes d'exécution

| Mode | Rendu | δ | Usage | Durée cible |
|---|---|---|---|---|
| `check` | non | non | batterie D1 : B, W, V, hashes, allocations | < 60 s pour tout le lot |
| `physics` | non | oui | batterie physique, convergence, conservation | minutes |
| `perf` | non | oui | mesure de coût sur matériel de référence | minutes |
| `capture` | oui | oui | séquences pour comparaison perceptuelle | lent, rare |
| `replay` | option | oui | rejeu d'une session de jeu enregistrée (§8) | variable |
| `starve` | non | oui | budget réduit, validation de la dégradation (§9) | minutes |

Le mode `check` ne demande **ni GPU ni rendu ni assets lourds**. C'est ce qui permet de le faire
tourner à chaque commit, et c'est le seul mode dont la vitesse est un objectif de conception.

---

## 5. Comparer : trois outils, trois usages

### 5.1 L'oracle est un solveur lent, pas la version précédente

Comparer une version à la précédente ne dit jamais **laquelle est juste**. Le harnais a besoin d'un
oracle indépendant.

> **Décision : écrire un solveur de référence délibérément lent, simple et non optimisé**, en 1D
> et 2D, dont la seule qualité est d'être manifestement correct.

Quelques centaines de lignes. Il ne tournera jamais dans le jeu. Il sert à :

- fournir la solution de convergence quand aucune forme analytique n'existe ;
- arbitrer entre deux solveurs candidats qui divergent ;
- distinguer « différent » de « faux » — distinction que rien d'autre ne permet.

Là où une solution analytique existe, elle prime sur l'oracle. Voir
[`CAS-CANONIQUES.md`](CAS-CANONIQUES.md) : douze des seize cas ont une référence fermée.

### 5.2 Iso-qualité, pas iso-résolution

Le piège méthodologique majeur de la campagne de solveurs.

Comparer deux candidats à `dx` égal mesure leur coût par cellule, pas leur rendement. Un solveur
peut être deux fois plus cher par cellule et rester le meilleur choix s'il atteint la même qualité
à `dx` deux fois plus grand — soit seize fois moins de travail (SPEC-001 §2.2).

**Protocole imposé pour B3 :**

```
1. choisir un cas canonique et une métrique d'erreur unique
2. pour chaque candidat, régler dx (et ses paramètres internes) jusqu'à atteindre
   la même erreur cible, à 5 % près
3. alors seulement, comparer coût, latence et mémoire
4. répéter à trois niveaux d'erreur cible : grossier, nominal, fin
```

Une campagne conduite à `dx` fixe désignera très probablement le mauvais candidat, et l'erreur
sera irréversible parce qu'elle sera étayée par des chiffres.

### 5.3 Comparaison perceptuelle : mesurer d'abord le bruit du jury

Protocole en double aveugle : même trajectoire de caméra, même graine, même encodage, ordre
randomisé, choix forcé avec option « aucune différence », motif de la réponse consigné.

**Contrôle obligatoire : la paire nulle.** Chaque session inclut, à l'insu du jury, au moins une
paire composée deux fois de la même séquence. Le taux de « différences » détectées sur ces paires
est le **plancher de bruit du jury**.

Un résultat n'est retenu que s'il dépasse nettement ce plancher. Sans ce contrôle, un jury motivé
trouve des différences partout et le banc produit du bruit présenté comme une donnée. C'est le
contrôle le plus souvent omis et le seul qui rende les autres interprétables.

Taille de jury minimale : 8 personnes, dont au moins 3 hors de l'équipe eau.

---

## 6. Métriques — et les six façons de se tromper en mesurant

| Métrique | Définition retenue |
|---|---|
| `cpu_p50`, `cpu_p99` | contribution du système d'eau par tick, en régime établi |
| `gpu_p50`, `gpu_p99` | **horodatages GPU**, jamais des timers CPU |
| `pire_frame_a_froid` | maximum sur les 300 premières frames, rapporté **séparément** |
| `latence_echantillon` | âge de la donnée **au moment de son usage**, distribution complète |
| `derive_masse` | (masse mesurée − attendue)/attendue, par seconde |
| `derive_energie` | idem ; une croissance signale une instabilité, pas une imprécision |
| `reflexion_frontiere` | amplitude renvoyée / incidente à l'éponge — cible < 1 % |
| `allocations` | nombre d'allocations après l'initialisation — doit valoir 0 (I-06) |
| `ecart_hash` | divergence de B entre plateformes — doit valoir 0 (I-03) |
| `taux_gaspillage` | domaines créés n'ayant jamais dépassé 2 % de l'écran (ADR-013) |

### Les pièges

1. **La moyenne.** Une moyenne à 2 ms avec un centile 99 à 9 ms produit un jeu qui saccade et un
   tableau de bord qui rassure. *Toute* décision se prend sur le p99.
2. **Chronométrer le GPU avec le CPU.** On mesure la soumission, pas l'exécution. Écart courant :
   un ordre de grandeur.
3. **Jeter la mise en route sans le dire.** Les premières frames contiennent compilation de
   shaders, allocations et caches froids. Il faut les écarter du régime établi **et** les publier
   à part : ce sont exactement les hoquets que le joueur voit au chargement.
4. **Mesurer avec la synchronisation verticale active.** On ne mesure alors plus rien.
5. **Mesurer sur une machine au repos.** Le jeu se fera concurrence à lui-même. Prévoir un profil
   « chargé » qui réserve une partie des cœurs et du GPU.
6. **Moyenner une latence.** La latence est une queue de distribution. La grandeur utile pour la
   flottabilité est l'âge de la donnée à l'instant où elle est lue, pas la profondeur du pipeline.

---

## 7. Intégration continue

| Cadence | Contenu | Coût | Blocant ? |
|---|---|---|---|
| À chaque commit | mode `check`, batterie D1, allocations | < 60 s | **oui** |
| Nocturne | batterie physique complète + `perf` sur matériel de référence | ≈1 h | oui, sur seuils durs |
| Hebdomadaire | hashes multi-plateformes, `starve`, rejeu de sessions | ≈4 h | alerte |
| Par jalon | comparaison perceptuelle | session humaine | revue |

### 7.1 Seuils de performance : le vrai adversaire est la dérive

Une comparaison naïve avec la veille produit des alertes en permanence : la variance d'exécution
sur une même machine atteint couramment quelques pour cent.

**Règle retenue** — un banc de performance échoue si l'une des deux conditions est remplie :

```
a) médiane glissante sur 5 exécutions  >  médiane de référence + 3 σ mesuré
b) valeur absolue  >  plafond du profil          (ADR-012 §3)
```

Et surtout, une troisième surveillance, qui n'est pas un seuil mais une **tendance** :

> Une dérive de 1 % par semaine est invisible à chaque commit. Elle vaut **+14 % par trimestre** et
> **+68 % sur un an**. C'est ainsi que les budgets se perdent : jamais d'un coup.

Le harnais conserve donc une série temporelle par métrique et par scénario, et signale toute pente
significative sur 30 jours, indépendamment du franchissement d'un seuil.

### 7.2 Bisection automatique

Une exécution en mode `check` coûtant quelques secondes et étant D1, la recherche dichotomique du
commit fautif est immédiate et doit être automatique. Sur un changement de hash, le harnais
désigne le commit responsable sans intervention.

### 7.3 Ce qui est archivé avec chaque résultat

`scénario (empreinte)` · `commit` · `identifiant matériel` · **`version du pilote graphique`** ·
`système` · `options de compilation` · `nombre de fils` · `horodatage`.

La version de pilote est celle qu'on oublie, et c'est celle qui explique la moitié des sauts de
performance et des divergences de hash inexpliqués.

---

## 8. Rejeu de sessions réelles — le meilleur rapport valeur/effort

ADR-003 et ADR-009 ont une conséquence que personne n'avait cherchée : **l'état complet de l'eau
d'une session de jeu tient dans presque rien.**

```
T_sim initial + descripteurs de région + journal d'événements W (50 o pièce — corrigé en S13, E04)
+ trajectoires des acteurs + trajectoire de caméra
```

Une heure de jeu chargé représente quelques centaines de kilo-octets. Une session de test peut donc
être **enregistrée puis rejouée dans le harnais**, sans le jeu.

Ce que cela apporte :

- les scénarios les plus utiles ne sont plus imaginés, ils sont **récoltés** ;
- un défaut signalé par un testeur devient un cas de non-régression en une commande ;
- les campagnes de test de jeu alimentent gratuitement la base de bancs.

À instrumenter dès les premières versions jouables : un enregistrement qui n'a pas été fait ne se
rattrape pas.

---

## 9. Deux batteries que personne ne pense à écrire

### 9.1 Dégradation (`starve`)

> Le chemin de dégradation est le code le moins testé et le plus exécuté : c'est celui qui tourne
> sur la configuration minimale, c'est-à-dire chez la majorité des joueurs.

Chaque scénario est rejoué à **100 %, 50 %, 25 % et 10 %** du budget nominal. Assertions :

- aucune assertion physique ne casse — la qualité baisse, rien ne rompt ;
- aucun domaine ne dépasse son budget individuel (I-05) ;
- la manette de qualité `q` ne change pas plus de **N fois par minute** (anti-pompage,
  ADR-012 §5) ;
- la surface reste continue : aucune discontinuité de hauteur au-delà d'un seuil entre deux
  frames consécutives.

### 9.2 Saturation

Tempêtes d'événements aléatoires, tous les acteurs au même endroit, création et destruction
massives de domaines. On ne cherche pas un chiffre, on cherche la **falaise** : le point où le
système cesse de dégrader proprement et commence à casser. Ce point doit être connu et documenté,
pas découvert en production.

---

## 10. Construction par étapes

| Étape | Contenu | Débloque |
|---|---|---|
| **H1** | lecteur de scénario, hôte harnais, mode `check`, hashes, compteur d'allocations | la CI par commit, dès le premier jour de code |
| **H2** | collecte de métriques, `perf`, série temporelle, seuils | B7, la surveillance de dérive |
| **H3** | cas canoniques analytiques, mode `physics` | B1, B2, B9 |
| **H4** | oracle lent 1D/2D, convergence, protocole iso-qualité | **B3 et B4** |
| **H5** | `capture` + protocole perceptuel avec paire nulle | B4, B6, B11 |
| **H6** | `replay` et `starve` | non-régression permanente |

**H1 doit exister avant la première ligne du solveur**, pas après. C'est le seul point de cette
spécification qui ne se rattrape pas : un système écrit sans harnais ne se laisse pas
instrumenter ensuite (ADR-020 §1).

---

## 11. Ce qui reste ouvert

1. Format exact du fichier de scénario — TOML proposé pour sa lisibilité en diff ; sans importance
   tant qu'il est textuel et versionné.
2. Le harnais doit-il embarquer un rendu de référence minimal pour le mode `capture`, ou
   celui-ci passe-t-il par l'hôte moteur ? Le second est plus fidèle, le premier plus rapide et
   plus stable. Proposition : hôte moteur, avec un profil de capture figé et versionné.
3. Politique de conservation des séries temporelles et des captures (volume de stockage).
4. Qui possède le harnais ? Il ne doit appartenir ni à l'équipe eau seule — juge et partie — ni à
   une équipe d'outillage détachée du domaine. Proposition : propriété eau, revue par
   l'assurance qualité technique.
