# REPRISE — à lire en premier, en entier

Passation active du système de gestion de l'eau. L'amorce unique est [AGENTS.md](AGENTS.md).
Ce fichier fait foi sur les mémoires privées. Il contient les règles présentes, pas les bilans
successifs : l'histoire vit dans [JOURNAL](notes/JOURNAL.md), les preuves dans les documents liés.
**Refonte S227, 2026-09-13** : ancienne version intégrale conservée dans Git à `dfd1507`.
**Révision S321, 2026-09-22** ([ADR-187](docs/adr/ADR-187-methode-refondue-s321.md)) : lecture
bornée, rituel en deux parties ; version précédente à `2f275cd2`.

## Jeton de session

```
JETON            : occupé
Battement        : 2026-10-09 08:39 +02:00
Agent            : Claude Code (Opus 5.5), application de bureau, **au poste** (fichiers, git, cargo, Python, RTX 5070 Laptop ; Godot non utilisé) — branche `poste`
Session en cours : S731 — la cinquantième revue de méthode (S726–S730) et le lot ; plan dans [EN-COURS](notes/EN-COURS.md)
Dernière session : S730 — Le raccord du rivage au-delà du jet (R43 reçu) ([journal](notes/JOURNAL.md)). Avant : S729 (La boîte qui naît et meurt avec le corps)
Session suivante : S731 — la cinquantième revue de méthode (S726–S730) et le lot ; puis ADR-284 D3, la place des frontières tirée de la prédiction
Maillons        : 1
Registres       : dernier lot S728 (ADR-213 D3) ; le prochain au plus tard en S731
```

**Avant de décider d'une reprise, vérifier les copies et branches selon AGENTS.md.** Le jeton
est versionné et local à une branche : il ne constitue pas un verrou global.

| état trouvé | action |
|---|---|
| libre | prendre le jeton, écrire Agent et le plan, committer le plan seul |
| occupé, battement < 2 h | signaler la session active et s'arrêter |
| occupé, battement ancien, ou interrompu | reprise à chaud : [EN-COURS](notes/EN-COURS.md) |
| archivé | rejoindre la branche vivante désignée ; ne pas travailler ici |

Une interruption explicitement signalée par l'utilisateur permet la reprise à chaud même si le
battement est récent. Le battement n'est pas une preuve de vie. Lire l'horloge dans un appel
séparé, puis reporter sa valeur à chaque commit d'étape (L237). Une seule session écrit à la fois.

## File active du projet

[Questions ouvertes — file active](docs/registres/QUESTIONS-OUVERTES.md#file-active) porte les
travaux, états et déclencheurs. [FEUILLE-DE-ROUTE](docs/FEUILLE-DE-ROUTE.md) porte seule les jalons.
Ne pas recopier leurs suivis ici. [Liste du projet fini](docs/LISTE-PROJET-FINI.md) : ce que l'ambition complète contient, demandée par l'utilisateur, tenue à chaque changement d'état ; son décompte est vérifié par l'outil. Dernier audit global : [BILAN-GLOBAL-S321](docs/registres/BILAN-GLOBAL-S321.md) — code, documents et méthode ; la méthode refondue (ADR-187).

## 1. Ce qu'est ce projet

Système d'eau temps réel pour un jeu de très grande échelle. Les intentions d'origine sont dans
[docs/sources](docs/sources/systeme_eau_architecture_globale.md), non modifiables.
La conception s'écrit en Markdown ; le cœur et le harnais en Rust sans dépendance externe.
L'hôte GPU séparé `viewer/` utilise les dépendances autorisées et verrouillées en S210/S211.
Images de banc locales autorisées (ADR-124) ; pas de page HTML ni d'artefact publié.

**Ambition complète**, construction progressive : δ général, V, inondations complexes et grande
échelle restent obligatoires (ADR-127). Une priorité ou un budget ne réduit jamais le périmètre.

## 2. Rôle et méthode

Autonomie technique déléguée par l'utilisateur (S71). Français, concis, factuel ; le fond va dans
les fichiers. **Il n'y a pas d'autres équipes** : les développeurs observent, les décisions
internes nous appartiennent (ADR-028). Ne pas transformer un fait externe inconnu en hypothèse
acquise. **Depuis S254, l'utilisateur supervise les rendus visuels** : lui demander des références
réelles quand un rendu doit être jugé, selon [REVUE-VISUELLE](docs/validation/REVUE-VISUELLE.md). La méthode de choix du lot et de validation vit dans [METHODE](notes/METHODE.md).

Plan avant travail dans [EN-COURS](notes/EN-COURS.md), committé seul ; une étape par commit
`S<n> P<k> — …`, moins d'un quart d'heure par étape. Déclarer un découpage si nécessaire. **Le plan
déclare ses entrées et comment il les vérifie** (S480 : en S472, une mer lisait le détail d'une autre,
et trois séries de mesures étaient fausses). Un calcul de plus de dix minutes se lance par
`python outils/calcul.py lancer` (sortie dans `calculs/`, trace dans [CALCULS](notes/CALCULS.md)) :
le dossier temporaire d'une conversation disparaît avec elle.

## 3. Où est la connaissance — lecture à froid

Après l'amorce Git et la prise du jeton, lire dans cet ordre — ≈ 70 Ko, pas davantage
([ADR-187](docs/adr/ADR-187-methode-refondue-s321.md) D2) :

0. [BOUSSOLE](BOUSSOLE.md) — déjà lue par l'amorce — et le [tableau de bord](docs/registres/TABLEAU-DE-BORD.md) (généré) :
   la liste à 100 %, campagne par campagne.
1. La **dernière entrée seulement** de [JOURNAL](notes/JOURNAL.md).
2. [Invariants](docs/01_INVARIANTS.md), puis [ADR-001](docs/adr/ADR-001-decomposition-en-couches.md) §2.
3. Le [plan de complétion](docs/registres/PLAN-COMPLETION-S475.md) et la conception de la campagne en cours ; dans la
   [file active](docs/registres/QUESTIONS-OUVERTES.md#file-active), les **décisions** en tête. Les décisions en vigueur, ADR par
   ADR : [DECISIONS-EN-VIGUEUR](docs/registres/DECISIONS-EN-VIGUEUR.md) (généré) ; les anomalies ouvertes :
   [ANOMALIES-OUVERTES](docs/registres/ANOMALIES-OUVERTES.md) (généré).
4. [Méthode](notes/METHODE.md), **ses protections actives d'abord**.
5. Pour le lot choisi : ses ADR, spécifications et preuves, **avant de le modifier**.

**Se consultent, ne se lisent pas** : l'[index](docs/00_INDEX.md), dont la carte par système est en
tête ; la feuille de route et la file entières ; le journal, les leçons, les angles morts. Une
reprise à chaud suit uniquement EN-COURS et le diff.

## 4. Où en est le projet

**Depuis S480, l'état courant ne s'écrit plus ici** : il est dans le [tableau de bord](docs/registres/TABLEAU-DE-BORD.md),
régénéré par `python outils/tableau_de_bord.py` (la liste, point par point, campagne par campagne, et l'historique du décompte), et
la suite dans le jeton ci-dessus. Ce qui suit est l'état au 2026-09-24 (S351), gardé pour l'histoire de la v1 ; il ne fait plus
foi sur l'état présent.

État au 2026-09-24 (S351), en bref ; le détail par jalon et par porte est dans la
[feuille de route](docs/FEUILLE-DE-ROUTE.md) et ne se recopie pas ici.

- **Stratégie** : trois systèmes — A haute mer (B+W), B volumique 3D (δ), C couplage — et sept
  lots ([ADR-178](docs/adr/ADR-178-strategie-en-trois-systemes-physiques.md)) ; v1 = portes A, B, C
  et D franchies ([ADR-174](docs/adr/ADR-174-arbitrages-du-2026-09-19.md) D4).
- **A** stabilisé : mer jugée par l'utilisateur, GPU eau 1,74 ms ; CPU hors profil (A278).
- **B** : δ 3D reçu, **porte B reçue** (S340) — référence CPU, production GPU sur les trois cas de cuve, scène
  jugée (R16) ; seconde représentation **APIC**
  retenue ([ADR-186](docs/adr/ADR-186-apic-seconde-representation.md)), cavité reçue sur un banc
  2D (S320), temps de pincement non convergé à trois mailles (S326) ; raccord aux colonnes à masse
  exacte, frontière non reçue (S325–S354, A316 : la densité des particules n'y est pas tenue).
- **C** : compteurs reçus (lot 1) ; retour δ → W reçu en ordres A à D, **ordre E bloqué par A289**.
- **Porte D** — lots 3 et 4 — **reçue sur la référence CPU** (S338, verdict R15,
  [preuve](docs/validation/PORTE-D-S333.md) §9) : solide immergé quelconque, fixe ou en mouvement (S324–S330) ;
  corps rigide du jeu sur B, C10 tenu, qui pilote sa coque dans δ sans que δ le pilote (S331–S333) ; A317
  corrigé (S335) ; rayonnement mesuré par δ (S336) ; bord de δ invisible (S337). Restent W derrière la requête
  du corps, la coque dans la production de δ. **Porte C reçue sur le banc** (S348, 1,92 ms au 99ᵉ centile par
  image, 30 Hz). **Porte A reçue sur le banc** (S351 : le rang 1, deux domaines servis ensemble). **Les quatre
  portes sont reçues : la v1 au sens d'ADR-174 D4**, chacune sur son banc ou sa référence, pas encore réunies en une
  scène vivante ([feuille de route](docs/FEUILLE-DE-ROUTE.md), « La v1 »). Depuis : la liste entière (ADR-190).
  **V** : noyau reçu, sans articulation avec δ.
- Liste du projet fini : **3 validés, 73 partiels, 44 absents** sur 120 — actualisée en entier en S350, 8.5 en S359, 2.7 en S362, 8.6 en S365, 7.1 en S367, 4.21 en S369, 5.4 en S372, 5.10 en S375, 5.5 en S378, 8.4 en S380, 4.3 en S386, 4.16 en S393, 4.9 en S396, 9.2 en S401, 9.3 en S405, 4.10 en S408.

L'inventaire se recalcule : `python outils/etat_projet.py` (Python standard, sans réseau). Ses
nombres mesurent des fichiers et des modifications, **pas du temps ni des capacités**. `--check`
tient les contrôles que nomme la table des protections de [METHODE](notes/METHODE.md).

## 5. Ce qui ne se décide pas ici

- Les cinq arbitrages d'[ADR-027](docs/adr/ADR-027-les-cinq-arbitrages-tranches.md) sont tranchés.
- 2 % pour B4 (ADR-120), 60 Hz (ADR-125), hôte GPU séparé (ADR-130) et sources de son verrou
  déjà autorisées : ne pas redemander ces accords. **Arbitrages du 2026-09-19**
  ([ADR-174](docs/adr/ADR-174-arbitrages-du-2026-09-19.md)) : machine de référence = ce poste ;
  temps de l'eau au service de l'objectif, profil de travail eau ≤ 4 ms GPU et ≤ 2 ms CPU, dont δ
  ≤ 2 ms GPU ; v1 = portes A à D franchies ; aucun dépôt distant. Un dépassement qualifie
  l'implémentation et s'éprouve sur la combinaison des techniques (ADR-131).
- Une réduction d'ambition demande une décision explicite de l'utilisateur (ADR-127). **Après la v1,
  l'objectif est la liste du projet fini entière**, ses points validés au périmètre final
  ([ADR-190](docs/adr/ADR-190-apres-la-v1-la-liste-entiere.md), décision du 2026-09-24). **Le rendu, 2026-09-25**
  ([ADR-191](docs/adr/ADR-191-le-rendu-realiste-un-module-du-moteur.md), [ADR-192](docs/adr/ADR-192-le-rendu-de-l-eau-dans-godot-4.md)) :
  le rendu final de l'eau se fait dans **Godot 4**, nos nuanceurs portés dans son langage ; l'afficheur reste le banc ;
  une session de rendu, une de physique.
- Les faits d'intégration non constatés et les actions d'infrastructure restent distincts des
  décisions techniques : ne pas inventer terrain, format réseau, personnes ou dépôt distant.

## 6. Rituel de fin de session — dernière étape du plan

En deux parties ([ADR-187](docs/adr/ADR-187-methode-refondue-s321.md) D1). La première se fait
**toujours** ; la seconde **seulement si l'état a changé**. Une session qui n'a rien changé
n'écrit pas dans les registres.

**Outillé depuis S480** : `python outils/rituel.py fin --session Snnn --suivante "…" [--maillons "…"] [--lot]`
vérifie les points 1 et 2 (cases, entrée de journal), régénère les registres générés, libère le jeton (point 4), coche le rituel et
lance le point 5 ; il échoue sans rien écrire si un point manque, et ne committe pas. **Depuis S483** (ADR-222 D3), il lance
d'abord le banc de non-régression (`outils/non_regression.py`, ≈ 2 min ; empreintes dans `docs/validation/EMPREINTES.md`) et
s'arrête s'il échoue ; un changement voulu réinscrit les empreintes (`--inscrire`) et le journal le dit. Le journal et la suite restent écrits par la session.

**Toujours**

1. `EN-COURS` : cases cochées ; ce qui doit survivre des notes, versé à la preuve ou au journal.
   La session suivante remplacera toute la section (D3).
2. Journal : **une entrée de vingt lignes au plus** — entrée, fait, preuve, limites, non-fait,
   suite proposée, arbitrages réels. Ne pas la dupliquer dans les points d'entrée.
3. Chaque action annoncée non réalisée devient un point daté de la file, avec déclencheur.
4. Jeton : session, battement, maillons (ci-dessous), `Session suivante`. Elle se prend dans la
   demande de l'utilisateur, sinon dans la **porte en cours** que désigne
   [FEUILLE-DE-ROUTE §3 bis](docs/FEUILLE-DE-ROUTE.md) : la suite qu'une session déclare n'est
   qu'une proposition. Un même point de file ne porte pas une troisième session consécutive si
   aucun critère « reçu si » d'une porte n'a avancé (S294, A211). Une suite locale n'efface jamais
   δ, V, B2, bathymétrie ou multiplateforme.
5. `python outils/etat_projet.py --check` sans erreur ; cocher le rituel avant son commit ; fermer
   ou synchroniser les copies selon **AGENTS.md**, sans recopier sa procédure ici.

**Seulement si l'état a changé** — une capacité reçue ou perdue, un critère de porte, un point de
la liste, une décision de l'utilisateur, un défaut bloquant —, et **par lots de trois sessions**
([ADR-213](docs/adr/ADR-213-accelerer-tolerance-plafond-rituel-bancs.md) D3) : la troisième session depuis le
dernier lot (ligne `Registres`) fait les points 6 à 8 pour tout ce qui a changé depuis ; plus tôt seulement
pour une porte reçue ou perdue ou un angle mort nouveau de sévérité 3. Une décision de l'utilisateur va
toujours, tout de suite, dans la preuve et l'ADR qui la porte :

6. Feuille de route, file active, liste : les lignes touchées, **en remplacement** de leur état
   périmé. Les états antérieurs restent dans Git et le journal.
7. Index, carte par système, pour tout document nouveau utile ; renvois affectés corrigés ;
   invariants touchés relus. Un ADR reçoit seulement une note factuelle datée ; une décision
   remplacée exige un nouvel ADR.
8. Angle mort nouveau s'il est de sévérité 2 ou plus, ou bloquant ; leçon **seulement** si elle
   crée ou change une protection de [METHODE](notes/METHODE.md).

### Deux maillons — critère révisé S227, resserré S294

`Maillons` compte les sessions successives sans **capacité reçue**. Remise à zéro seulement si
le journal nomme : **ce qui devient possible**, **le chemin qui le consomme**, **la preuve**.
Une correction d'intégrité effectivement reproduite puis testée compte. Une décision qui lève
un blocage compte si elle nomme le lot de construction désormais exécutable. Un commentaire,
un banc isolé, un simple ajout dans `src` ou un ADR sans effet aval ne suffisent pas.
**Depuis S294** : une capacité compte si elle fait avancer un critère « reçu si » d'une porte de
§3 bis, ou l'état d'un point de la [liste du projet fini](docs/LISTE-PROJET-FINI.md). Une
optimisation consommée qui ne franchit aucun critère de porte ne remet pas le compteur à zéro :
S289, S290 et S291 l'avaient fait trois fois de suite sur le même fil (BILAN-GLOBAL-S293 M1).

Sinon, incrémenter. À deux maillons, choisir dans la file un lot faisant avancer une capacité,
et comparer sa priorité aux reliquats. Un troisième maillon demande une justification explicite
au journal. Même avec du code produit, **avant une troisième session sur le même sujet**, vérifier
si sa suite débloque encore l'usage visé ou relève désormais d'un approfondissement différable.
La demande actuelle de l'utilisateur prime toujours sur la suite automatique.

## 7. Session interrompue

Procédure unique dans [EN-COURS](notes/EN-COURS.md). Les commits attestent le travail fini ; le
diff appartient à l'étape `[>]`. Le lire et compléter ou annuler cette étape, sans toucher à
un travail étranger. Les notes de reprise conservent mesures, décisions et impasses coûteuses.

## 8. Tenue de la connaissance

ADR et sources conservés. Aucune valeur physique sans provenance (I-14). Distinguer décidé,
construit, reçu et intégré, ainsi que résolu, dissous, partiel et ouvert par décision.
Chaque information a un porteur : journal pour l'histoire, feuille de route pour les capacités,
file active pour les travaux, documents de validation pour les preuves, index pour les liens.
REPRISE ne grandit pas d'un compte rendu à chaque session.
**Plafonds (S294, S321)** : une ligne de la file active ≤ 90 mots — état présent, déclencheur,
lien ; une section de jalon de la feuille de route ≤ 450 mots ; `EN-COURS` ≤ 300 lignes, sans
archive ; une entrée de journal ≤ 20 lignes de texte. L'histoire va au journal et aux preuves.
`python outils/etat_projet.py --check` les vérifie au rituel.

## 9. Limites du dispositif

L'état des branches, copies et remotes se **constate** avec Git. Aucun distant à l'ouverture de
S227 : la création ou publication distante reste une action d'infrastructure à autoriser.
Le jeton versionné ne verrouille pas plusieurs copies atomiquement ; garder les vérifications
Git de l'amorce. A215 reste ouverte. Les mémoires privées ne constituent jamais une passation.
