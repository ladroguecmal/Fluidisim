# REPRISE — à lire en premier, en entier

Passation active du système de gestion de l'eau. L'amorce unique est [AGENTS.md](AGENTS.md).
Ce fichier fait foi sur les mémoires privées. Il contient les règles présentes, pas les bilans
successifs : l'histoire vit dans [JOURNAL](notes/JOURNAL.md), les preuves dans les documents liés.
**Refonte S227, 2026-09-13** : ancienne version intégrale conservée dans Git à `dfd1507`.

## Jeton de session

```
JETON            : occupé
Battement        : 2026-09-20 22:14 +02:00
Agent            : Claude Opus 5, application desktop (fichiers, git, cargo, outils locaux, carte réelle RTX 5070 Laptop, accès web)
Session en cours : S314 — **ordre B** : une primitive de W qui garde la **direction**, le **spectre** et la **phase** de ce qui sort d'un domaine δ, et les cinq essais qui la jugent
Dernière session : S313 — **ordre A rendu** : la loi du résidu est `u₃₂·activité/√N`, l'instrument voit une fuite de bilan à 10⁻¹³ m³/pas et **ne voit pas** une fuite d'état (T1 et T2 non redondantes), **T2 est tenue sur 10 s**, trois seuils proposés ([preuve](docs/validation/PLANCHER-BILAN-S313.md), [ADR-181](docs/adr/ADR-181-conservation-transfert-oriente-et-ordre-du-lot-2.md))
Session suivante : selon la file active, à la fin de S314
Maillons        : 0 — capacité S313 : le dépôt sait dire si un écart de conservation est numérique ou physique, connaît la **portée** de son instrument, et l'a éprouvé sur un défaut qu'il s'est donné ; consommateur : les ordres B à E, qui auraient tous reçu leurs bancs sur un critère dont A304 disait qu'il ne mesurait rien
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
Ne pas recopier leurs suivis ici. [Liste du projet fini](docs/LISTE-PROJET-FINI.md) : ce que l'ambition complète contient, cochée à la demande de l'utilisateur. Dernier audit global : [BILAN-GLOBAL-S293](docs/registres/BILAN-GLOBAL-S293.md) — où l'avancement bloque.

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
`S<n> P<k> — …`, moins d'un quart d'heure par étape. Déclarer un découpage si nécessaire.

## 3. Où est la connaissance — lecture à froid

Après l'amorce Git et la prise du jeton, lire dans cet ordre :

1. La **dernière entrée seulement** de [JOURNAL](notes/JOURNAL.md).
2. [Index](docs/00_INDEX.md), pour choisir les sources utiles.
3. [Invariants](docs/01_INVARIANTS.md), puis [ADR-001](docs/adr/ADR-001-decomposition-en-couches.md).
4. [Feuille de route](docs/FEUILLE-DE-ROUTE.md), [file active](docs/registres/QUESTIONS-OUVERTES.md#file-active)
   et [méthode](notes/METHODE.md).
5. Les ADR, spécifications et leçons **du lot choisi** ; recherche ciblée dans [LECONS](notes/LECONS.md).

Ne pas relire le journal, les leçons ou tous les ADR intégralement à chaque reprise. Une reprise
à chaud suit uniquement EN-COURS et le diff. Les preuves d'un lot se lisent avant de le modifier.

## 4. Où en est le projet

État au 2026-09-20 (S308). **Depuis S308, les chantiers se regroupent en trois systèmes** — A haute mer superficielle, B volumique 3D, C couplage — par décision de l'utilisateur ([ADR-178](docs/adr/ADR-178-strategie-en-trois-systemes-physiques.md)) ; les portes restent vraies, leurs priorités changent. Trajectoire et état par jalon : [FEUILLE-DE-ROUTE](docs/FEUILLE-DE-ROUTE.md) ;
travaux : [file active](docs/registres/QUESTIONS-OUVERTES.md#file-active). En bref :

- **J1** partiel : B+W sur GPU dans l'hôte séparé, scène multi-sources admise par le cœur, GPU eau
  1,74 ms, mer jugée par l'utilisateur (R7 accepté). **S304–S308 : la mer a ses asymétries** (ADR-176) et
  sa couleur dérivée de ses sources (ADR-177). **R14 reçu : sa troisième image est la référence
  interne provisoire de l'océan**, et le lot optique est clos — S308 a montré que le contraste
  local manquant est **spatial**, hors de portée de toute courbe. Manquent le CPU sous 2 ms
  (A278), un objet pilotable et la seconde cible.
- **δ reçu en 2D** — surface mobile couplée à B/W contre HOS, frontières, rendu en direct.
  **S295–S296** : référence 3D à surfaces linéaire et mobile reçue, identique au bit à la 2D quand `ny = 1`.
  **Porte B ouverte** : δ en 3D selon [ADR-175](docs/adr/ADR-175-architecture-d-execution-de-delta-en-3d.md),
  référence CPU couplée reçue S297, frontières et fond spectral réel reçus S298. Une mer à la fois
  résolue et étalée n'entre pas dans un banc CPU : le critère 3 d'ADR-175 §4 demande la production
  GPU. **S299–S301** l'ont construite : le pas couplé entier tourne sur la carte (0,84 ms à
  64 cycles sur 27 648 mailles), publie sa surface, se diagnostique en différé, et suit la
  référence jusqu'à l'horizon de prévisibilité **de la référence elle-même** (≈ 1,2 s, A297).
  **S302 : la scène tourne** — 30 × 28 m sur la mer étalée, une onde la traverse, rendu en direct à
  197 Hz depuis la seule surface publiée. **R11 : δ reçu sans artefact et sans raccord visible —
  mais S308 l'a nommé : ce raccord n'a jamais été mesuré. Aucun bilan de masse, de quantité de
  mouvement ni d'énergie à l'interface, et le couplage est à sens unique (A302, lots 1 et 2).**
- **Ordonnanceur** (porte A) : décide qu'un domaine vit et avec quel budget ; ni plusieurs
  candidats, ni déplacement, ni dégradation automatique.
- **V** : noyau reçu (C12, géométrie orientée, restauration), sans articulation avec δ.
- **Solides** : rien dans le système ; faces coupées en 2D seulement, `body.rs` statique. Porte D,
  devenue les lots 3 et 4 d'ADR-178 D7.
- Dernier audit : [BILAN-GLOBAL-S293](docs/registres/BILAN-GLOBAL-S293.md) — le projet bloquait sur
  l'ordre de ses propres travaux. **Confrontation S308** :
  [TROIS-SYSTEMES-S308](docs/registres/TROIS-SYSTEMES-S308.md) — ce qui existe pour A, B et C,
  les six interfaces manquantes, le banc de la piscine essai par essai, l'ordre en sept lots.

L'inventaire Git se recalcule : `python outils/etat_projet.py` (Python standard, sans réseau).
Ses nombres mesurent des fichiers et des modifications, **pas du temps ni des capacités**. `--check`
vérifie la navigation et les plafonds des documents d'état.

## 5. Ce qui ne se décide pas ici

- Les cinq arbitrages d'[ADR-027](docs/adr/ADR-027-les-cinq-arbitrages-tranches.md) sont tranchés.
- 2 % pour B4 (ADR-120), 60 Hz (ADR-125), hôte GPU séparé (ADR-130) et sources de son verrou
  déjà autorisées : ne pas redemander ces accords. **Arbitrages du 2026-09-19**
  ([ADR-174](docs/adr/ADR-174-arbitrages-du-2026-09-19.md)) : machine de référence = ce poste ;
  temps de l'eau au service de l'objectif, profil de travail eau ≤ 4 ms GPU et ≤ 2 ms CPU, dont δ
  ≤ 2 ms GPU ; v1 = porte D franchie ; aucun dépôt distant. Un dépassement qualifie
  l'implémentation et s'éprouve sur la combinaison des techniques (ADR-131).
- Une réduction d'ambition demande une décision explicite de l'utilisateur (ADR-127).
- Les faits d'intégration non constatés et les actions d'infrastructure restent distincts des
  décisions techniques : ne pas inventer terrain, format réseau, personnes ou dépôt distant.

## 6. Rituel de fin de session — dernière étape du plan

1. Écrire **une entrée concise** au journal : entrées, changements, preuves et limites, non-fait,
   prochaine capacité visée, arbitrages réels. Ne pas la dupliquer dans les points d'entrée.
2. Enregistrer les nouveaux angles morts avec sévérité, ou actualiser ceux qui couvrent déjà le
   défaut. Une leçon nouvelle doit être généralisable ; aucune obligation d'en produire.
3. Transformer chaque action annoncée non réalisée en point daté de la file, avec déclencheur.
4. Actualiser les lignes touchées de la feuille de route et de la file active **en remplacement
   de leur état périmé**. Les états antérieurs restent dans Git et le journal.
5. Corriger les renvois affectés et relire les invariants touchés. Un ADR reçoit seulement une
   note factuelle datée ; une décision remplacée exige un nouvel ADR. Mettre les nouveaux
   documents utiles dans l'index ; vérifier les décomptes seulement s'ils sont encore affichés.
6. Mettre à jour jeton, session, battement et état ; cocher le rituel avant son commit.
7. Relire **toute la file active**. **`Session suivante` se prend dans la porte en cours** que
   désigne [FEUILLE-DE-ROUTE §3 bis](docs/FEUILLE-DE-ROUTE.md), ou dans la demande de
   l'utilisateur : la suite qu'une session déclare n'est qu'une proposition. Un même point de
   file ne porte pas une troisième session consécutive si aucun critère « reçu si » d'une porte
   n'a avancé (S294, A211). La recommandation du dernier bilan est portée ou écartée au journal.
   Une suite locale n'efface jamais δ, V, B2, bathymétrie ou multiplateforme.
8. Appliquer la règle des deux maillons précisée ci-dessous, puis fermer/synchroniser les copies
   selon **AGENTS.md**, sans recopier sa procédure ici.

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
**Plafonds (S294)** : une ligne de la file active ≤ 90 mots — état présent, déclencheur, lien ;
une section de jalon de la feuille de route ≤ 450 mots ; l'histoire va au journal et aux
preuves. `python outils/etat_projet.py --check` vérifie plafonds et navigation au rituel.

## 9. Limites du dispositif

L'état des branches, copies et remotes se **constate** avec Git. Aucun distant à l'ouverture de
S227 : la création ou publication distante reste une action d'infrastructure à autoriser.
Le jeton versionné ne verrouille pas plusieurs copies atomiquement ; garder les vérifications
Git de l'amorce. A215 reste ouverte. Les mémoires privées ne constituent jamais une passation.
