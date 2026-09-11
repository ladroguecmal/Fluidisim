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
  01_INVARIANTS.md     les 18 règles non négociables
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
État vérifié S154 : **296 tests réussis, cinq ignorés ; 105 ADR, 212 angles, 18 invariants,
6 spécifications, 23 cas canoniques, 2 bancs sur 11 partiellement exécutés.** Détail par couche et par bloc :
[BILAN-S145](docs/registres/BILAN-S145.md).
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
A210 est traitée en S143 : deux gardes exécutables — l'une sur ce que les champs calculent, l'autre sur ce que le
crate contient — et l'**invariant I-18**. La pesée a écarté le type porteur, qui paraissait la garde la plus
solide : l'hôte devant pouvoir construire la valeur, la garde serait une bosse et non un mur (L226). Les deux
gardes ont été **vues échouer** sur la faute de S141 avant d'être acceptées.
A208 est traitée en S144, et **aucune des deux réparations qu'elle proposait n'a été prise** : l'une attribuerait
une cause que la bibliothèque ne peut pas établir, l'autre était périmée par la session suivante (L227). Le refus
distingue désormais trois causes — un paramètre inutilisable, un champ vraiment trop raide au point, et une
enveloppe qui refuse ce que la pente ne contredit pas. Six essais ont changé d'attente : cinq exerçaient le
majorant en croyant exercer la pente.
Le bilan est refait en S145, celui de S69 ayant 76 sessions : **`W` a cessé de ne pas exister** — impacts,
pressions, journal rejouable et service vivant, 9 768 lignes reçues par 275 essais — pendant que le repère
annonçait le contraire. `δ` reste 1D, `V` nul, et **zéro banc sur onze a été exécuté**, exactement comme en S69.
Le projet est à ~90 % comme corpus de conception et **~30 % comme système**.
**B1 a été lancé en S146** — le premier banc exécuté du projet — et la recommandation portée par le jeton a donc
fonctionné. Décision : **32 composantes**, les trois critères mesurables convergeant (coût ×8,3, dispersion de
`Hs` ×2,0, aucune différence pour un objet de côté ≤ 30 m). Le banc renverse une intuition : **augmenter le
nombre de composantes ne rend pas la mer plus juste, il la rend moins prévisible**, et A187 — un écart de 6,6 %
inexpliqué depuis quatre-vingts sessions — était une réalisation à 3 σ sur une graine unique.
**S147 : S63-1 close** sur la dispersion W déjà reçue. [ADR-100](docs/adr/ADR-100-spectre-de-fond-et-bande-explicite.md)
décide un candidat JONSWAP à bande explicite ; **A212 reste partielle**, constructeur à écrire.
L'instrument montre que normaliser Hs masque la perte des moments gouvernant les vitesses.
Suite S148 : S147-1, configuration spectrale et cuisson reproductible, puis B+W/pente/rejeu.
100 ADR,212 angles,18 invariants,6 spécifications,23 cas, **1 banc partiellement exécuté sur11**.
Voir [l'index](docs/00_INDEX.md) et [la passation](REPRISE.md) pour l'état détaillé.

**S148 : candidat spectral construit**, [ADR-101](docs/adr/ADR-101-cuisson-du-fond-spectral.md).
Cuisson explicite sans libm, moments/pic et B+impact reçus ; gravité vérifiée en composition.
282 tests réussis/cinq ignorés,101 ADR,212 angles,18 invariants,6 SPEC,23 cas.
Suite S149 : S148-1, transport de recette et cycle hôte spectral ; A212 reste partielle.
**S149 : recette spectrale transportée**, [ADR-102](docs/adr/ADR-102-transport-recette-spectrale.md).
[Cycle hôte reçu](docs/validation/CYCLE-SPECTRAL-S149.md) après destruction des sources,
comparaison directe et hashes debug/release identiques. S148-1 close ; A212 partielle.
Suite S150 : S149-1, W4/sillage puis B2.
**S150 : mouvement et charge raccordés au sillage**, [ADR-103](docs/adr/ADR-103-mouvement-charge-sillage.md).
[Réception](docs/validation/TRAJET-SILLAGE-S150.md) : virage, arrêt du forçage, transport
et B+W comparés à une référence raffinée. W4 reste partiel.
Suite S151 : S150-1, alimentation progressive par le mouvement hôte puis B2.
**S151 : sillage alimenté progressivement**, [ADR-104](docs/adr/ADR-104-emission-progressive-sillage.md).
[Réception](docs/validation/EMISSION-SILLAGE-S151.md) : saturation puis reprise sans
doublon, champ identique au trajet complet. S150-1 close sur le contrat borné ; W4 partiel.
Suite S152 : S151-1, comparaison B2.
**S152 : premier volet B2 exécuté**, [résultats](docs/validation/BANC-B2-S152.md).
Profil512 reçu pour couvrir les sources2–4m sur80m/60s ;256 suffit pour5/6m.
Coût et restauration30s mesurés. B2 reste partiel, technologie et lambda_cut ouverts.
Suite S153 : énergie et transport à60s (S152-1).
**S153 : bilan énergétique60s reçu pour la source4m**, [rapport](docs/validation/ENERGIE-B2-S153.md).
L'énergie sortie de80m est retrouvée dans120m ; pas de dissipation de3,516 %.
Suite S154 : extension aux quatre autres sources B2 ; verdict global toujours partiel.
**S154 : cinq bilans d'impact à60s reçus**, [rapport](docs/validation/ENERGIE-BANDE-B2-S154.md).
Énergie retrouvée sur des collecteurs adaptés ; profils hôte80m conservés.
Suite S155 : B2 sillage prolongé (S154-1), verdict global encore partiel.
**S155 : la fenêtre de 16 s n'était pas une limite numérique**, [mesure](docs/validation/HORIZON-MODAL-S155.md),
[ADR-106](docs/adr/ADR-106-horizon-d-observation-et-duree-de-forcage.md). 16 000 000 µs, c'est 2^24 :
la borne venait de la représentation. Horizon d'observation porté à64s, durée de forçage gardée à16s ;
budget écrit à la place de la constante. L'âge n'est pas gratuit pour autant — omega en f32 fait dériver
la phase linéairement en temps (A213). Suite S156 : bilan d'un sillage prolongé, S155-1.
106 ADR,213 angles,232 leçons,18 invariants,6 SPEC,23 cas.
**S156 : le sillage prolongé revient par l'autre bord**, [mesure](docs/validation/SILLAGE-DOMAINE-S156.md),
[ADR-107](docs/adr/ADR-107-le-domaine-d-un-sillage-se-deduit-de-sa-recette.md). Le bilan énergétique se
conserve au bit près, et c'est vide : la rotation des modes le garantit. La validité **spatiale** est bornée
par deux mécanismes indépendants — le pas angulaire borne le rayon, le pas radial borne la durée par
périodicité — et laquelle mord dépend de l'instant. Verdict B2 volet sillage : **partiel et négatif à 60 s**,
prix mesuré à l'appui. Suite S157 : établir la loi en durée, S156-1.
107 ADR,214 angles,234 leçons,18 invariants,6 SPEC,23 cas.
**S157 : il n'y a pas de loi, et c'est le résultat**, [mesure](docs/validation/LOI-DUREE-S157.md),
[ADR-108](docs/adr/ADR-108-pas-de-garde-fou-sans-tolerance-declaree.md). La dégradation d'un sillage
hors domaine est **graduelle** : l'instant limite hérite de la tolérance qu'on choisit, et le plan
d'expérience à produit réduit constant liait deux variables qu'il prétendait séparer. Pas de garde-fou —
encoder un seuil gèlerait une tolérance que personne n'a spécifiée ; une estimation conservatrice est
publiée à sa place. Suite S158 : sortir la mesure de sa dégénérescence, ou demander la tolérance, S157-1.
108 ADR,214 angles,237 leçons,18 invariants,6 SPEC,23 cas.
**S158 : la question était dans le mauvais ordre**, [inventaire](docs/validation/TOLERANCE-SILLAGE-S158.md),
[ADR-109](docs/adr/ADR-109-le-repliement-est-une-infidelite-pas-une-faute.md). Avant de chercher une
tolérance, demander ce que l'erreur casse : elle est déterministe et identique chez tous, donc **infidélité
et non faute**, et les consommateurs qui lisent une **borne** y sont insensibles d'un facteur huit mille.
Le juge de fidélité est B4, bloqué ailleurs. Suite S159 : le facteur 2,5 qui revient partout, S158-1.
109 ADR,214 angles,239 leçons,18 invariants,6 SPEC,23 cas.
**S159 : six copies de travail, quatre jetons « libres » contradictoires**,
[inventaire](docs/registres/COPIES-S159.md), [ADR-110](docs/adr/ADR-110-une-copie-de-travail-se-ferme.md).
L'une croyait la dernière session être **S44** : l'ouvrir aurait recréé cent quinze sessions parallèles.
Avance rapide d'abord, suppressions ensuite ; six copies ramenées à trois, aucune histoire perdue, et la
procédure de fermeture écrite dans l'amorce. Suite S160 : le facteur 2,5, S158-1.
112 ADR,225 angles,252 leçons,18 invariants,6 SPEC,23 cas.
**S162 : diagnostic de Stokes et portée de B4 corrigée.**
[Résultats](docs/validation/ADDITIVITE-PROFONDE-S162.md),
[ADR-112](docs/adr/ADR-112-la-superposition-independante-ne-recoit-pas-le-couplage.md).
La cambrure intervient dans les interactions profondes au second ordre. Mais la superposition
indépendante mesurée en S161 ne reçoit pas le résidu couplé prévu : aucun critère de bascule
n'est validé. 299 tests/cinq ignorés, plus un test d'exemple. Suite S163 : recevoir ce couplage.
**S163 : résidu couplé intégré et reçu en 1D.**
[Résultats](docs/validation/RESIDU-COUPLE-S163.md). La reconstruction retrouve le même schéma total
à l'arrondi ; retirer les couplages physiques, numériques ou la source fait échouer la comparaison,
même quand la masse reste conservée. S162-1 réalisée sur véhicule ; aucun seuil ni B4 complet.
299 tests/cinq ignorés, plus cinq nouveaux tests d'exemple. Suite S164 : fond prescrit instationnaire.
**S164 : fond analytique variable en temps reçu sur le véhicule 1D.**
[Résultats](docs/validation/FOND-PRESCRIT-S164.md). Les incréments discrets du fond retrouvent
le total ; la dérivée continue converge, l'omission temporelle reste fausse. Quatre nouveaux
tests d'exemple, S163 rejoué et 78 exécutions release reçus. Suite S165 : frontière locale.

**S165 :** frontière locale exercée sur le véhicule 1D ; témoin exact aux étages reçu,
bord fond seul insuffisant quand le résidu extérieur n'est pas nul. 54 montages et cinq
nouveaux tests propres reçus. Suite : fermeture sans référence globale, entrée et sortie
distinctes. Voir [FRONTIERE-LOCALE-S165](docs/validation/FRONTIERE-LOCALE-S165.md).

**S166 :** frontière autonome construite sur le véhicule subcritique 1D : entrée prescrite,
sortie issue de l'intérieur. Six nouveaux tests et huit S165 reçus ; 128 évolutions.
Le bord ajoute peu d'erreur, mais le calcul intérieur amortit le fond exact. Suite :
préserver ce fond. Voir [BORD-AUTONOME-S166](docs/validation/BORD-AUTONOME-S166.md).

**S167 :** fond analytique exact préservé et perturbation ajoutée reçue sur le véhicule 1D.
La source physique d'un fond inexact reste nécessaire. Cinq nouveaux tests et45 évolutions ;
suite : fermer le volume total avec moyennes et flux cohérents.
Voir [FOND-PRESERVE-S167](docs/validation/FOND-PRESERVE-S167.md).

**S168 :** volume total fermé à l'arrondi par moyennes de cellules et flux intégrés
indépendamment. Cinq nouveaux tests et 30 évolutions reçus. Suite : réunir ce calcul
et la frontière autonome. Voir [VOLUME-MOYEN-S168](docs/validation/VOLUME-MOYEN-S168.md).
**S169 :** frontière autonome et résidu conservatif assemblés : fond variable intact,
sortie et volume reçus ensemble sur le véhicule1D. Quatre nouveaux tests et60 évolutions.
Suite : source interpolée sur réseau grossier (A50).
Voir [ASSEMBLAGE-AUTONOME-S169](docs/validation/ASSEMBLAGE-AUTONOME-S169.md).
**S170 :** source sur réseau grossier mesurée : raffiner le solveur ne corrige pas son
injection artificielle. Quatre nouveaux tests et48 évolutions reçus. Suite : source
calculée par différence de flux partagés.
Voir [SOURCE-DECIMEE-S170](docs/validation/SOURCE-DECIMEE-S170.md).