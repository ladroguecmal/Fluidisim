# Fluidisim — Système de gestion de l'eau

Connaissance projet du système d'eau temps réel. Ce dépôt est la **mémoire** du système : il
existe pour que le travail survive au changement de conversation, de session et de personne.

## Entrer

- **Vous reprenez le projet** (autre session, autre compte, **autre agent**, autre personne) →
  **[`REPRISE.md`](REPRISE.md)**, à lire en entier avant toute action.
- **Vous cherchez une décision ou une donnée** → [`docs/00_INDEX.md`](docs/00_INDEX.md)

## Organisation

```
AGENTS.md              **l'amorce** — le seul texte, quel que soit l'agent qui lit
CLAUDE.md              renvoi d'une ligne vers AGENTS.md (Claude Code lit ce nom-là)
REPRISE.md             passation : rôle, état, rituel de fin de session, jeton
docs/
  00_INDEX.md          point d'entrée, état d'avancement, arbitrages en attente
  01_INVARIANTS.md     les 17 règles non négociables
  adr/                 décisions d'architecture, numérotées, jamais réécrites
  specs/               références chiffrées et signatures d'interfaces
  registres/           angles morts, traçabilité des questions sources
  validation/          harnais, cas canoniques, plan de benchmark
  sources/             documents d'intention d'origine, non modifiés
notes/
  METHODE.md           protocole de conception
  LECONS.md            enseignements généralisables
  JOURNAL.md           historique des sessions et points de reprise
  EN-COURS.md          plan de la session en cours, déclaré avant le travail
code/                  le harnais et deux δ d'essai — Rust, sans dépendance moteur
```

## Règles de tenue

- **Un ADR n'est jamais réécrit.** Une décision qui change fait l'objet d'un nouvel ADR qui
  remplace explicitement le précédent. L'historique du raisonnement a autant de valeur que la
  conclusion.
- **Aucun nombre sans provenance** : formule citée dans `SPEC-001`, ou étiquette « à calibrer »
  avec le banc qui le fixera.
- **Les documents de `docs/sources/` ne sont pas modifiés.** Leur relecture critique vit dans
  `registres/`.
- Chaque session ajoute une entrée à `notes/JOURNAL.md` avant de se clore, et exécute le rituel de
  fin décrit dans [`REPRISE.md`](REPRISE.md) §6.
- **Une seule session travaille à la fois** : le jeton en tête de `REPRISE.md` en tient le compte,
  avec quatre états — `libre`, `occupé`, `interrompu`, `archivé`.
- **Une seule amorce, [`AGENTS.md`](AGENTS.md)**, quel que soit l'agent — Claude, ChatGPT, Codex,
  un autre modèle, ou une personne. Les autres noms de fichier d'amorce sont des **renvois d'une
  ligne**, jamais des copies : deux amorces qui se ressemblent divergeront, et chaque agent suivra
  alors la sienne (**L137**).
- **Avant de regarder le jeton : `git worktree list` et `git branch -a`.** Le jeton est un fichier
  **versionné** : il est propre à une branche et à une copie de travail, et ne dit rien de ce qui
  se passe ailleurs. Le dépôt a forké **trois fois** par ce mécanisme — voir
  [`docs/registres/FORK-S22-S26.md`](docs/registres/FORK-S22-S26.md). Ces deux commandes sont le
  seul dispositif qui ait tenu.
- **Le plan se déclare avant le travail**, dans [`notes/EN-COURS.md`](notes/EN-COURS.md), et se
  commit seul. Une étape par commit, message `S<n> P<k> — …`. Une session coupée par une limite
  d'usage n'a aucune occasion d'écrire qu'elle s'arrête : seule une déclaration antérieure survit.
- **Reprendre après une interruption** : la procédure est en tête de `notes/EN-COURS.md`. Cinq
  minutes, sans relire le dépôt.

## Où en est le projet

État S138 : la construction est actée par ADR-053. Le noyau B+W dispose d'un journal rejouable,
d'impacts radiaux et de requêtes communes en lot. Le renouvellement numérique est testé
jusqu'à 16 secondes sur un scénario borné ; rétention durable et système complet restent à
construire. Le contrôleur à deux pools assure désormais la bascule après succès et signale
l'expiration. LiveWater admet désormais les commandes en publiant ensemble journal et champs ;
une commande bloquée reste visible. La sauvegarde WLIV conserve cette attente et permet une
reprise sur des buffers plus grands. Le scénario hôte complet est testé et mesuré : 3 impacts
sur 64 points en 0,64 ms médiane locale. Une première réponse physique à une pression mobile
est construite hors runtime, puis étendue à un profil gaussien localisé avec bilan global vérifié.
Les trajectoires avec virage conservent désormais les interférences et le bilan de travail.
Une enveloppe refuse les requêtes hors bornes ; le virage est comparé par raffinement sur
[-8,12]² m et 0–8 s, sans garantie continue. La préparation dispose d'un chemin sur pool hôte.
Potentiel, pente et vitesses sont vérifiés. Le candidat modal à phases entières passe la
réception locale : 246 tests réussis, cinq ignorés ; conformité interplateforme encore ouverte.
La superposition sur pool passe le virage et le découpage, pour toutes les grandeurs.
La cuisson gaussienne possède une recette versionnée, sur pool et sans libm.
Le demi-spectre et les coefficients préparés coûtent 19,800 ms pour 64 points, médiane locale.
Les lots publient après succès intégral ; le parcours par tuiles, plus lent, est rejeté.
Sinus/cosinus partagent leur réduction avec identité locale, sans gain global établi.
Une résolution112×80 passe le virage densifié : lot64 à11,305 ms médian local, usage limité à cette fixture.
Puissance candidate reçue : bilans spatial et temporel, extinction, aux deux résolutions (S104).
Le candidat lie désormais ses requêtes au contexte complet et à son instant préparé.
Une préparation refusée conserve la publication précédente (S105).
Une requête monde commune compose désormais B et la pression, avec contrôle de pente et lot atomique (S106).
Cycle B+pression reçu sur le virage : médianes locales29,37 ms (128²) et16,28 ms (112×80), sans budget cible certifié.
Source de pression immuable et codec WPRS V1 construits (ADR-074), virage196 octets.
Journal de pression construit : doublons, conflits, attente en saturation et reprise explicite (ADR-075).
Instantané mémoire WPJR et restauration construits, attente conservée (ADR-076).
Reprise complète reçue :1152 points-temps identiques en bits, cycles locaux28,4/17,0 ms (S111).
Champ multisource construit, interférences et travail total conservés (S112).
Deux sources reçues contre référence f64 raffinée :224×128 et256×128 (S113).
Préparation+requête pression64 :48,29/56,13 ms médianes locales ; B et codecs exclus.
Reprise multisource reçue :2944 points-temps identiques en bits, refus transactionnels (S114).
Cycle depuis WPRS jusqu’à B+pression :47,95/55,55 ms médians locaux, cuisson exclue.
Requête commune B+impacts+pressions construite : B unique, normale issue des pentes totales,
refus transactionnels et réductions aux chemins antérieurs reçus (ADR-077, S115).
Montage mixte reçu sur8670 points-temps du modèle profond, cycles50,05/57,97 ms (S116).
Contrôleur pression à deux pools construit : publication sur succès, instant exact,
champ précédent conservé au refus ; journal figé (ADR-078, S117).
Cycle hôte temporel mixte reçu par le contrôleur : douze instants non monotones,3468 points-temps
identiques en bits à la voie directe par recette, cycle48,82/56,06 ms,`Unchanged`0,1 µs (S118).
Deux horizons distincts constatés — la pression publie plus loin que la validité des impacts (A194) ;
la position d'un bloc de mesure fabriquait un écart de coût qui n'existe pas (A195, L207).
Horizon effectif et annonce du montage mixte construits : avant toute publication, l'hôte connaît
la fenêtre servable et ce que la requête fera d'un instant, par une implémentation unique partagée
avec la requête elle-même (ADR-079, S119). Annonce 23 ns contre12,6 ms de préparation évitée.
A194 résolue ; A195 corrigée et vérifiée par une mise en régime avant la première mesure.
Les points sont annonçables à leur tour : `admits` compose les prédicats de domaine posés dans les
trois couches, `slope_floor` donne la part de l'enveloppe de pente indépendante des points (ADR-080,
S120). Filtrer coûte ~40 ns par point contre35,60 ms pour le lot que l'atomicité ferait perdre.
L'inventaire a montré que deux des trois conditions n'avaient besoin d'aucune borne : leur prédicat
exact est quatre ordres de grandeur moins cher que l'évaluation (L209).
Un refus de champ ne confond plus la limite physique et la limite numérique : `NotRepresentable`
dit que la bibliothèque ne peut pas représenter le champ, `Steepness` reste un verdict que l'appelant
peut lever (ADR-081, S121). La cible reçue était le mauvais étage — le bloc visé n'a aucune entrée qui
l'atteigne, sur 673 884 échantillons — et la sonde a désigné le site voisin, lui démontrable (L210).
Chaque borne de construction d'un champ d'impact nomme désormais le paramètre à revoir, et les trois
bornes couplées — portée, régime, résolution — sont nommées comme des relations (ADR-082, S122). La carte
du couloir d'acceptation est mesurée : ondes du mètre à la dizaine de mètres, rayon d'autant plus petit que
l'onde est courte. Le test d'atteignabilité de chaque nom a montré qu'un refus attribué à l'énergie venait
en réalité de la longueur d'onde (L211).
Le couloir a été confronté aux impacts du jeu : sur onze cas couvrant six ordres de grandeur en taille,
**un seul se construit à la portée voulue** (ADR-083, S123). La portée d'un champ d'impact vaut5,09 λ, soit
une dizaine de fois la taille de l'objet — limite venue de la table de Bessel, non de la physique (A201).
Et la longueur d'onde qui pilote tout le candidat n'est reliée à aucune propriété de l'objet (A200,
sévérité1) : le contrat `λ = α·b` est acté, α restant à calibrer.
La limite de portée est levée : le domaine de Bessel passe de64 à2048 par développement asymptotique,
borne **mesurée** et fixée par la précision de la phase en `f32` (ADR-084, S124). Le facteur32 espéré ne
se produit pas — une seconde borne, `Resolution`, prend aussitôt le relais (L213) — mais avec `N =256`,
déjà permis depuis ADR-060, **neuf cas de jeu sur onze** atteignent enfin leur portée, contre un seul.
Coût mesuré en S125 : environ ×4 à points identiques et ×9 pour le montage étendu comparé au montage
initial. [ADR-085](docs/adr/ADR-085-profils-radiaux-selon-le-domaine.md) conserve N64 par défaut ;
N128/N256 se dimensionnent explicitement au domaine commun du service. A202 traitée ; A203 suit
la réception physique à grande portée, réalisée en S126 sur les deux fixtures à âge0–4 s.
État vérifié S142 : **273 tests réussis, cinq ignorés ; 96 ADR, 211 angles, 17 invariants,
6 spécifications, 23 cas canoniques.**
[Réception S126](docs/validation/RECEPTION-ETENDUE-S126.md) :1350 points-temps reçus contre
oracle indépendant, erreur normalisée<=4,44e-7. La fenêtre4 s reçoit surtout des queues aux
distances64/128 m ; elle ne valide pas encore un paquet transporté au loin. Suite S127 :
S126-1/A204, dimensionner portée et horizon ensemble. Bibliothèque inchangée ; les256 tests
restent ceux vérifiés en S125, deux nouvelles campagnes release avec assertions passent en S126.
[Transport S127](docs/validation/TRANSPORT-ETENDU-S127.md) : N256/R80/horizon48 admis ;
99,985 % de l'énergie de référence est entre32 et80 m à48 s. Rayon moyen53,42 m ;
2187 points-temps du candidat reçus. A204 traitée sur cette fixture, bilan cinétique total
du candidat encore ouvert en S127, reçu en S129 ci-dessous. Cycle hôte reçu en S128.
[Cycle transporté S128](docs/validation/CYCLE-TRANSPORTE-S128.md) : service B+W N256/R80/horizon48,
TTL4 conservé,1280 points-temps identiques après renouvellement et reprise. Refus atomiques reçus.
Renouvellement+requête64 ~0,96 ms médian local, un impact ; restauration~2,47 µs, sauvegarde289 octets.
Deux campagnes release et debug reçus, production inchangée.
[Bilan candidat S129](docs/validation/BILAN-CANDIDAT-ETENDU-S129.md) : cinétique et énergie totale
du candidat N256/R80/48 reçues contre S127, écart total maximal9,045e-7 E0. Interférences
conservées ;99,985214 % de E0 entre32 et80 m à48 s. **S128-1 réalisée sur fixture.**
Suite complète259/cinq ignorés.
Les sources de pression s'admettent désormais pendant qu'une publication est en cours : le contrôleur
emprunte le journal mutablement, et `admit` republie un champ qui lui corresponde ou rend le journal à
son état antérieur (ADR-086, S130). La saturation est reçue comme état terminal du contrôleur.
La sortie de saturation est reçue et chiffrée : `required_capacity` annonce ce qu'il faut pour que la copie
et la reprise aboutissent, l'élargissement se fait **sans interrompre le service** (0,1 µs) et seule la
reconstruction prive l'hôte de champ — une préparation,12,21/13,41 ms, incompressible (ADR-087, S131).
Une admission ajoute désormais la source au champ publié au lieu de tout recalculer — mais **seulement**
quand elle s'insère en dernier, seul cas où l'ordre d'addition `f32` est préservé ; sinon elle recalcule
(ADR-088, S132). Le résultat est celui de la voie directe dans les deux cas, sans quoi deux hôtes ayant admis
les mêmes sources dans un ordre différent auraient des champs différents. Coût ramené au prix d'un segment :
5,03 ms au lieu de35,06 à sept segments publiés.
Sortir d'une saturation n'interrompt plus le service : `extend_into` lit le contrôleur en place et en
construit un second sur le journal élargi, en repartant des coefficients publiés, pendant que l'ancien
continue de servir (ADR-089, S133). La fenêtre sans champ de S131 **disparaît** au lieu de raccourcir, et
le coût tombe à6,21 ms contre19,11 pour construire le même journal à trois sources.
La condition d'exactitude incrémentale a été pesée et **conservée** : sur les contributions réelles, l'ordre
des segments ne déplace le champ que de5,6e-7 à7,1e-6 — du bruit d'arrondi — tandis que s'en affranchir
rendrait toutes les références depuis S113 non reproductibles. Elle devient une contrainte d'usage écrite :
des identifiants croissants donnent le chemin rapide, sinon le même champ plus lentement (ADR-090, S134).
La « transaction mixte » n'était pas ce qui manquait : les emprunts interdisent déjà d'admettre pendant une
requête, chaque couche est transactionnelle, et la cause est commune aux deux journaux. Ce que personne n'avait
constaté, c'est qu'**aucune admission n'est annulable** — ce qui rend un coordinateur irréalisable. L'admissibilité
s'annonce donc des deux côtés, et l'hôte vérifie avant de modifier quoi que ce soit (ADR-091, S135).
Un objet qui entre dans l'eau donne enfin ses deux nombres au modèle : la **longueur d'onde se dérive** de la
forme que le candidat engendre — `λ = 3,35·b`, bornée à [3,35 ;6,11]·b — et l'énergie reste à calibrer mais sa
borne suit une loi exacte, `E_max = K·ρ·g·λ⁴·s²` (ADR-092, S136). A200, sévérité1, est traitée. Les conclusions
de S123 s'en trouvent changées : cinq cas de jeu sur onze tiennent dans le couloir au lieu d'un seul.
Le renvoi « à calibrer B2 », que trois décisions recopiaient depuis S77, est **faux** : B2 choisit la technologie
de W et B10 mesure la cavité — aucun banc ne mesurait la source d'onde d'un impact. La calibration passe à B10,
étendu de deux métriques observables et doté d'un critère de réussite qui peut **réfuter** le modèle au lieu de
l'absorber (ADR-093, S137). Un renvoi non vérifié ferme la question au lieu de la laisser ouverte (L217).
L'audit des renvois montre un corpus **cohérent** sur ses identifiants — 93 ADR, 217 leçons, 204 angles, aucun
trou ni renvoi cassé — mais que le correctif de la session précédente était **incomplet** : six ADR portaient le
renvoi erroné, trois seulement avaient été corrigés (S138, L218). Et `max_slope`, qui décide de l'admissibilité de
tout champ, est renvoyé à un banc qui ne le mesure pas alors que sa limite physique est déjà dérivable (A205).
`max_slope` se dérive, et la question n'était pas celle qu'on croyait : la limite physique est déjà dans
SPEC-001 §4 — cambrure limite de Stokes, `πH/λ = 0,4488` — et ce qui manquait est le **rapport entre la borne L1
du modèle et la pente réelle**, mesuré constant à **1,7950713** (ADR-094, S139). Mais le budget de pente
**additionne trois grandeurs de natures différentes** — une pente exacte, une borne L1 de facteur 1,795, une
enveloppe de facteur inconnu : aucun seuil unique n'y est physiquement juste, et c'est pourquoi le nombre était
resté sans provenance (L220). Le seuil de 0,1 n'admet aujourd'hui que 12,4 % de la pente physique.
Le facteur de la pression, lui, **n'est pas une constante** : il se décompose en un facteur de forme, borné par
2 et éliminable sans coût, et un facteur d'alignement que rien ne borne — 3,03 sur un spectre réaliste, 16,7 sur
une emprise étroite (ADR-095, S140). Le budget devient homogène par la **nature** de ses termes : la même
grandeur majorée, le meilleur majorant exact de chacun, la marge résiduelle mesurée (L222).
S139-1 est **faite** (S141) : chaque terme du budget consomme le meilleur majorant exact de sa pente réelle, et
`BREAKING_SLOPE = π/7` est publiée avec sa provenance. Le champ limite admis est désormais **exactement à la
cambrure de Stokes** — 0,448799 mesuré, contre 12,4 % de cette valeur jusqu'à S139. Quatre hachages de campagne
se sont déplacés, chacun prédit puis vérifié ; le harnais H1 est inchangé.
A209 est traitée dès la session suivante (S142) : le champ modal a lui aussi une constante — **1,701591** — et il
compare désormais la pente réelle. `max_slope` a un seul sens dans le crate, et les **deux** champs placent leur
champ limite à la cambrure de Stokes. Le champ n'est pas retiré : ADR-059 le conserve exprès.
Suite : A210 — rien n'oblige un futur champ à mesurer son rapport avant de comparer à `max_slope`, et le contrat
ne vit que dans deux commentaires et deux essais.
96 ADR,211 angles,17 invariants,6 spécifications,23 cas.
Voir [l'index](docs/00_INDEX.md) et [la passation](REPRISE.md) pour l'état détaillé.
