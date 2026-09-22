# ADR-187 — La méthode refondue sur l'analyse de S321 : un rituel en deux parties, une lecture bornée, des protections plutôt que des leçons, des réceptions reproductibles

- **Statut : actée**, S321, 2026-09-22, **sur demande explicite de l'utilisateur** — *« réorganiser
  ou refaire des principes des points améliorables »* —, dans l'autonomie technique déléguée (S71).
- **Appuyée sur** [BILAN-GLOBAL-S321](../registres/BILAN-GLOBAL-S321.md), qui en donne les mesures.
- **Modifie** : [REPRISE](../../REPRISE.md) §3, §4, §6 et §8 ; [METHODE](../../notes/METHODE.md) ;
  [EN-COURS](../../notes/EN-COURS.md) ; `outils/etat_projet.py`.
- **Laisse entiers** : [AGENTS](../../AGENTS.md) — amorce, jeton, copies de travail — ; la règle des
  deux maillons ; les portes ; [ADR-027](ADR-027-les-cinq-arbitrages-tranches.md),
  [127](ADR-127-ambition-complete-construction-progressive.md), [174](ADR-174-arbitrages-du-2026-09-19.md),
  [178](ADR-178-strategie-en-trois-systemes-physiques.md), [184](ADR-184-seconde-representation-en-parallele.md)
  et [186](ADR-186-apic-seconde-representation.md) ; le périmètre.

## 1. Pourquoi

Cinq audits ont précédé celui-ci (S69, S145, S198, S227, S293). Chacun a trouvé un défaut de
pilotage ou d'accumulation, et chacun l'a corrigé **en ajoutant une règle**. Aucune n'a été retirée.
Le bilan de S321 mesure où cela mène :

- le coût de la méthode suit le **nombre de sessions** — 17,5 par jour, 38 minutes en médiane, un
  rituel de 6 à 11 fichiers à chacune —, pas le travail ;
- les documents d'état **regrossissent** après chaque plafond — `EN-COURS` 93 → 1 686 lignes en 27
  sessions, la file active +57 % en vingt ;
- les erreurs qui reviennent **reviennent avec leur leçon écrite** (L362, L237, l'encodage) : 371
  leçons, une par session, aucune chargée au moment utile ;
- ce qui a tenu, ce sont les consignes **qu'un outil vérifie** (plafonds de S294, battement de S309) ;
- les réceptions obtenues sur des bancs **ne sont protégées par rien** contre la régression.

L'ancien banc `simufluid`, même architecture, abandonné avec un état de 7 199 lignes, avait conclu
de lui-même : *le manque n'était pas une nouvelle règle, mais le chargement de la bonne protection
au moment utile.* Cette décision en tire la conséquence : **retirer, remplacer par des contrôles,
et charger peu mais au bon moment.**

## 2. Décisions

**D1 — Un rituel en deux parties.** *Toujours*, à la fin de chaque session : `EN-COURS` coché puis
purgé, journal en vingt lignes au plus, jeton, `etat_projet.py --check`. *Seulement si l'état a
changé* — une capacité, un critère de porte, un point de la liste, une décision de l'utilisateur,
un défaut bloquant : feuille de route, file active, liste, index, notes datées d'ADR, chacun **en
remplacement**. Une session qui n'a rien changé de tout cela n'y touche pas.

**D2 — La lecture à froid est bornée.** Amorce, REPRISE, dernière entrée du journal, invariants,
ADR-001 §2, feuille de route §3 bis, décisions et lignes de la porte en cours dans la file,
méthode — ≈ 70 Ko au lieu de 155. L'index, la feuille de route entière et la file entière **se
consultent** : ce sont des cartes, pas des lectures.

**D3 — `EN-COURS` ne porte que la session en cours.** Procédure, règles, plan, notes de reprise.
Les notes d'une session close vont à sa preuve ou au journal, puis sortent du fichier — Git les
garde. Au plus 300 lignes, aucune section d'archive : l'outil le vérifie.

**D4 — Des protections plutôt que des leçons.** [METHODE](../../notes/METHODE.md) porte une table
courte de **protections actives** — dix-sept, chacune avec le moment où elle s'applique et, quand il
existe, le contrôle qui la tient. Elle se lit à froid. [LECONS](../../notes/LECONS.md) devient une
archive consultée par recherche. **Une leçon ne s'écrit que si elle crée ou change une protection** ;
sinon, le fait va au journal ou à la preuve.

**D5 — Une erreur qui revient se corrige par un contrôle, pas par une consigne** (L19, L349).
`etat_projet.py --check` vérifie désormais aussi : la taille et l'absence d'archive d'`EN-COURS`,
l'encodage des fichiers suivis, l'absence de fichiers produits versionnés, l'accord du décompte de
la liste avec ses points, la section « Reproduire » des preuves nouvelles. La construction se tient
à **zéro avertissement**, pour qu'un avertissement neuf se voie.

**D6 — Une réception se reproduit.** Toute preuve ouverte à partir de S321 commence par une section
**« Reproduire »** : commit, commandes exactes, valeurs attendues, durée. Une capacité qui passe d'un
banc au système — APIC au raccord, par exemple — y entre **avec ses essais**, et les chiffres de sa
réception deviennent des assertions, lentes et ignorées par défaut s'il le faut.

**D7 — Un fil, une preuve.** Un fil de plusieurs sessions sur une même capacité enrichit **une**
preuve par sections datées, dont l'en-tête dit l'état présent, au lieu d'en ouvrir une par session.
Les preuves existantes ne sont ni renommées ni fusionnées.

## 3. Ce que cette décision ne fait pas

- Elle **ne retire aucune exigence de physique** : critère avant la mesure, référence indépendante,
  seuil jamais relevé pour faire passer, précision rapportée à l'usage — tout reste.
- Elle **n'efface aucune histoire** : journal, leçons, angles morts et traçabilité des questions
  restent, consultés par recherche.
- Elle **ne reformate pas le code** (`rustfmt` changerait 144 fichiers du cœur et ferait perdre
  `git blame`) et ne déplace aucun banc.
- Elle **ne change pas l'ordre des lots** : il appartient à l'utilisateur (ADR-178 D7, ADR-184).

## 4. Comment la défaire

Chaque décision se défait seule, et la mesure qui la justifierait est nommée.

| | se défait si | geste |
|---|---|---|
| D1 | une session coupée après la partie « toujours » laisse un document d'état faux | ajouter le cas manquant à la liste « si l'état a changé », plutôt que rendre tout le rituel obligatoire |
| D2 | une session fautive n'avait pas lu ce qu'il fallait | ajouter **ce** document à la lecture, pas la liste entière |
| D3 | une note purgée manque à une reprise | la verser à la preuve avant la purge ; relever le plafond si une session légitime le dépasse |
| D4 | une erreur couverte par une protection revient | la protection est mal placée ou sans contrôle : la corriger, ne pas écrire une leçon de plus |
| D5 | un contrôle rejette à tort | corriger le contrôle et ajouter le contre-exemple à ses essais |
| D6 | la section « Reproduire » ne suffit pas à reproduire | c'est une faute de la preuve, pas de la règle |
| D7 | une preuve unique devient illisible | la scinder par capacité, pas par session |

## 5. Ce qui devient faux si elle est mal lue

**« Deux parties » ne veut pas dire « pas de registre ».** Un changement d'état s'inscrit dans la
session où il a lieu — c'est précisément la seconde partie. Ce qui disparaît, c'est l'écriture
**sans** changement d'état.

**« Des protections plutôt que des leçons » ne veut pas dire « plus de leçon ».** Une découverte
généralisable qui crée une protection s'écrit, et la protection avec elle.

**« Lire peu » ne s'applique pas au lot.** Les ADR, preuves et spécifications du lot choisi se
lisent avant de le modifier, comme avant.
