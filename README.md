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
117 ADR,228 angles,265 leçons,18 invariants,6 SPEC,23 cas.
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
**S171 :** source par flux partagés : conservation à l’arrondi avec flux exacts aux
bornes, précision locale toujours sensible au réseau. Sept tests,128 évolutions ;
A225 traitée sur véhicule1D, A50 partielle. Suite : reconstruire aussi le fond Q.
Voir [SOURCE-FLUX-PARTAGES-S171](docs/validation/SOURCE-FLUX-PARTAGES-S171.md).
**S172 :** fond et source reconstruits conjointement avec le même état initial.
Quatre nouveaux tests,88 simulations et4 témoins ; conservation à l’arrondi et
identité discrète reçues. A50 reste partielle ; suite : fond évoluant dans le temps.
Voir [FOND-RECONSTRUIT-S172](docs/validation/FOND-RECONSTRUIT-S172.md).
**S173 :** fond mobile préservé avec une source intégrée cohérente. Trois nouveaux tests,
quatre rejoués,160 simulations et8 témoins ; bilan fermé, erreur de transport mesurée.
Suite : réévaluer le fond moins souvent que le solveur. A50 reste partielle.
Voir [FOND-MOBILE-S173](docs/validation/FOND-MOBILE-S173.md).
**S174 :** cadence du fond séparée du pas solveur. Le budget interpolé ferme, mais
son écart au flux analytique persiste àcadence fixée. Six tests,192 simulations et
24 témoins ; suite : assemblage avec la frontière autonome. A50 partielle.
Voir [CADENCE-FOND-S174](docs/validation/CADENCE-FOND-S174.md).
**S175 :** frontière autonome assemblée au fond décimé, sortie de crête reçue.
Huit tests,288 simulations et54 témoins ; effet du bord distingué du transport et
des budgets. A50 partielle ; suite : bilan B4 et prochain lot de construction.
Voir [FRONTIERE-FOND-DECIME-S175](docs/validation/FRONTIERE-FOND-DECIME-S175.md).
**S176 :** bilan B4 consolidé ; contrôles1D reçus, B4 complet encore non reçu.
Prochain lot : fournisseur des dérivées du fond B dans la bibliothèque.
Voir [BILAN-B4-S176](docs/validation/BILAN-B4-S176.md). Documentation seule, sans nouveau test.
**S177 :** le fond B fournit ses dérivées et sa pression en profondeur dans la
bibliothèque, avec évaluation par lot sans publication partielle. Huit nouveaux tests,
307 réussis/cinq ignorés ; conformité historique préservée. B profond linéaire seulement.
Voir [FOURNISSEUR-B-S177](docs/validation/FOURNISSEUR-B-S177.md) et
[ADR-113](docs/adr/ADR-113-fournisseur-differentiel-du-fond.md).
**S178 :** le fond B fournit son gradient de pression, son Laplacien et sa source
volumique continue à soustraire. Six nouveaux tests ;313 réussis/cinq ignorés,
contre-épreuve de l’advection et condensats historiques préservés. A50 reste partielle.
Voir [SOURCE-B-S178](docs/validation/SOURCE-B-S178.md) et
[ADR-114](docs/adr/ADR-114-source-continue-du-fond-profond.md).
**S179 :** dérivées profondes des impacts radiaux et composition B+un impact reçues,
y compris au centre. Six nouveaux tests ;319 réussis/cinq ignorés, références historiques
préservées. A50 partielle ; suite : pression forcée W.
Voir [DIFFERENTIEL-W-S179](docs/validation/DIFFERENTIEL-W-S179.md),
[ADR-115](docs/adr/ADR-115-differentiel-radial-et-composition.md).

**S180 :** pression forcée et dérivées profondes reçues, y compris au démarrage et
à l'extinction. Cinq nouveaux tests ;324 réussis/cinq ignorés, références historiques
préservées. A50 partielle ; suite : composition du fond, des impacts et des pressions.
Voir [DIFFERENTIEL-PRESSION-S180](docs/validation/DIFFERENTIEL-PRESSION-S180.md),
[ADR-116](docs/adr/ADR-116-differentiel-de-pression-forcee.md).

**S181 :** fond, impacts et pressions fournissent ensemble leurs dérivées et leur
source, avec contrôle du contexte et de l'instant. Quatre nouveaux tests ;328 réussis,
cinq ignorés, références inchangées. Suite : recevoir ce consommateur dans le cycle vivant.
Voir [COMPOSITION-DIFFERENTIELLE-S181](docs/validation/COMPOSITION-DIFFERENTIELLE-S181.md),
[ADR-117](docs/adr/ADR-117-composition-differentielle-mixte.md).

**S182 :** dérivées et source préservées lors des mises à jour, admissions,
renouvellements et restaurations. Trois nouveaux tests ;331 réussis/cinq ignorés,
références inchangées, aucun correctif d'exécution nécessaire. Suite : mesurer le coût.
Voir [CYCLE-DIFFERENTIEL-S182](docs/validation/CYCLE-DIFFERENTIEL-S182.md).

**S183 :** le coût du consommateur différentiel est mesuré, conditions publiées avant
les chiffres. Il vaut 3,0 à 4,3 fois le chemin de surface aux mêmes entrées, médiane
~3,4, et **le même facteur pour chaque couche** : il suit ce qui est publié, 31 scalaires
contre 10. Préparation et actualisation inchangées ; zéro allocation après `seal()`.
Aucun ADR, aucun budget. Deux trouvailles : **A226**, un refus porté par un point fait
payer le lot entier ; **L263**, un axe ne mesure son effet que s'il dépasse ce qu'il
transporte. Suite : la consommation perturbative.
Voir [COUT-DIFFERENTIEL-S183](docs/validation/COUT-DIFFERENTIEL-S183.md).

**S184 :** la consommation est mesurée, pas seulement la production. La source coûte
**~2100 fois** le pas de solveur qu'elle alimente. La décimation spatiale est plafonnée
à `r = 2` par le contenu de la source, la cadence temporelle est l'axe bon marché, et
une récurrence de phase sur réseau ne retirerait que 12–15 % — le coût est le volume de
sortie, pas la trigonométrie. Aucun ADR. **A227** : le fournisseur a la forme d'une
requête, pas d'un champ. **L264** : mesurer la part avant d'optimiser le parcours.
Suite : l'erreur de cadence en 3D.
Voir [CONSOMMATION-S184](docs/validation/CONSOMMATION-S184.md).

**S185 :** l'erreur de la cadence temporelle est mesurée en 3D, et les trois modes de
réemploi séparés — le corpus n'avait mesuré que celui qui exige de connaître l'avenir.
Le **maintien est d'ordre un**, l'extrapolation et l'interpolation d'ordre deux ;
l'extrapolation est causale et presque gratuite, donc le maintien n'est jamais le bon
choix. Une période de latence vaut un facteur 2,6 sur la cadence. Aucun ADR.
**A228** : réemployer une source en la maintenant coûte un ordre entier, et rien ne le
disait. **L265** : un contrôle qui relie deux mesures attrape ce qu'aucune ne montre —
il a trouvé un défaut d'indexation que deux tables plausibles cachaient.
Suite : composer l'erreur spatiale et l'erreur temporelle.
Voir [CADENCE-3D-S185](docs/validation/CADENCE-3D-S185.md).

**S186 :** l'erreur spatiale et l'erreur temporelle sont composées sur un seul véhicule,
et **la loi de composition dépend du mode de réemploi** : le **maximum** pour les deux
modes causaux — les seuls dont un runtime dispose — la quadratique pour l'interpolation.
Un budget conjoint est donc licite, et la règle est d'égaliser les deux erreurs prises
seules puis de s'arrêter ; l'axe bon marché est gratuit jusqu'à la parité. Espace et
temps sont le même opérateur **par axe** : rapport de constantes 2,27 et 3,05 pour les
trois axes d'une interpolation trilinéaire. Aucun ADR.
**A229** : dégrader un axe peut réduire l'erreur totale — jusqu'à −17 % — donc un réglage
à un axe à la fois trouve un optimum faux. **A230** : « points par longueur d'onde » n'est
pas un critère pour une source échantillonnée en profondeur, où le contenu présent est 5 à
16 fois plus lisse que la coupure de la recette. **L266** : deux erreurs mesurées
séparément ne se composent pas ; leur somme est une enveloppe, jamais une prédiction.
117 ADR,230 angles,266 leçons,18 invariants,6 SPEC,23 cas.
Suite : le réseau gradué en profondeur — l'erreur vient d'une tranche sur quatorze.
Voir [COMPOSITION-ERREURS-S186](docs/validation/COMPOSITION-ERREURS-S186.md).

**S187 :** la session venait construire un réseau d'échantillonnage **gradué** en
profondeur ; elle a trouvé en chemin que le réseau du dépôt posait son dernier nœud
**hors** du bloc, et que l'ancrer vaut **jusqu'à un facteur six** — gratuitement, sans
changer un seul nœud — là où la graduation ne vaut que 1,4. Le mécanisme est mesuré et non
supposé : la métrique est un **maximum**, il vit sur la tranche la plus haute, et un nœud
posé là supprime le terme dominant ; la contre-épreuve horizontale, où la source ne pique
pas, ne donne que −2,5 %, −40 % puis **+1,3 %**. D'où **ADR-118** : un réseau
d'échantillonnage s'ancre sur ses frontières, gradue son pas selon la courbure du contenu,
et s'arrête quand son axe cesse d'être le plus grossier. Gains à erreur égale : −37,5 % de
nœuds contre l'isotrope `r = 2`, −78,4 % contre `r = 4`, erreur divisée par 2,44 à nœuds
identiques contre `r = 8`.
**A231** : le réseau débordait, et toutes les erreurs spatiales de S186 valent donc pour un
réseau inutilement mauvais. **L267** : une campagne qui balaie une résolution à convention
de placement fixée mesure la convention autant que la résolution — quatre sessions avaient
balayé le ratio sans questionner où les nœuds se posaient.
118 ADR,231 angles,267 leçons,18 invariants,6 SPEC,23 cas.
Suite : ancrer le réseau et rejouer la composition de S186 dessus.
Voir [RESEAU-GRADUE-S187](docs/validation/RESEAU-GRADUE-S187.md).

**S188 :** la grille de composition de S186 est **rejouée sur le réseau ancré** d'ADR-118,
à nombre de nœuds identique et avec le critère de jugement déjà déclaré. **Le verdict ne
change pas mode par mode** — le maximum pour les deux modes causaux — et il est **mieux
satisfait** : corriger le placement des nœuds a resserré la loi au lieu de la casser.
Le rejeu livre surtout ce que S186 ne pouvait pas dire : **pourquoi** la loi tient. La
métrique ajoutée avant la mesure — la tranche qui porte le maximum — vaut la plus haute
partout, et **39 cases jugées sur 39** voient les deux maxima au même endroit. L'ancrage a
changé la magnitude de l'erreur spatiale, jusqu'à 3,7 fois, mais pas l'endroit de son
maximum, qui est une propriété du contenu et non du réseau.
**A232** : la loi du maximum n'est donc valide que **tant que les deux maxima coïncident**,
et rien ne le disait. **L268** : un rejeu qui confirme n'est pas un rejeu inutile — il
transforme une coïncidence en condition, à condition d'avoir déclaré d'avance la métrique
qui distingue les explications.
Conversion mesurée : **27 nœuds ancrés valent 125 nœuds débordants** à erreur égale. Et le
point de parité entre axe spatial et axe temporel se déplace d'un facteur ~3, donc
l'optimum va vers plus de décimation spatiale et moins de réduction de cadence.
118 ADR,232 angles,268 leçons,18 invariants,6 SPEC,23 cas.
Suite : la composition sur réseau gradué, seul endroit connu où les deux maxima se séparent.
Voir [COMPOSITION-ANCREE-S188](docs/validation/COMPOSITION-ANCREE-S188.md).

**S189 :** la loi de composition mesurée depuis S186 **n'était pas une loi**. Mesurée sur
le réseau **gradué** que recommande ADR-118 — le seul qui sépare les deux pics d'erreur —
la loi du maximum est **rejetée** pour le mode maintien, alors qu'elle tient sur un réseau
ancré. Le mécanisme, dérivé avant la mesure et vérifié : les deux champs d'erreur
s'**additionnent maille par maille**, à 10 % près et exactement dans les cas dégénérés.
Ce que la norme maximum affichait n'était donc que la **position relative de leurs pics** —
et ces pics ne coïncident **jamais** à la maille, 0 cas sur 78, ce qui corrige l'inférence
de S188 sans toucher à ses mesures.
D'où **ADR-119** : le budget d'erreur conjoint se **borne par la somme** — la seule forme
jamais dépassée sur trois géométries de réseau et trois sessions — le maximum n'est pas une
estimation portable, et la règle « égaliser les deux axes puis s'arrêter » est abandonnée.
Un budget conjoint reste licite ; c'est sa répartition qui tombe.
**A233** : cette borne est lâche d'un facteur 2,3, et aucune estimation plus serrée ne tient
sur toutes les géométries. **L269** : une loi mesurée en norme n'est pas une loi — trois lois
candidates qui se partagent les cas sont le signe qu'aucune n'est la bonne.
119 ADR,233 angles,269 leçons,18 invariants,6 SPEC,23 cas.
Suite : l'additivité locale avec une projection de pression — la seule limite qui menace
l'ensemble.
Voir [COMPOSITION-GRADUEE-S189](docs/validation/COMPOSITION-GRADUEE-S189.md).


**S190 — 2026-09-12 : tolérance B4 fixée à 2 % par l'utilisateur**, ADR-120.
Attente du critère close ; volet d'échantillonnage source reçu sur le véhicule S185–S189.
Profil de banc choisi : **14×14×8 nœuds gradués, extrapolation 80 ms** ; budget
spatial+temporel+réserve **1,800653 %**, erreur composée+réserve **1,161371 %**.
**12 couples reçus / 114 refusés**, omission de S refusée à 100 %. Réduction des
évaluations de source **13,46 fois** sur la fenêtre, pas un gain CPU mesuré.
Deux release et une debug identiques, empreinte `0x4b479c21a520cd7b` ; workspace
**331 tests réussis, cinq ignorés**. Bibliothèques et supports inchangés.
**B4 complet reste à recevoir** : projection/surface/frontières, substitutif intégral,
forces et perception. A50 partielle ; N universel et seuil de bascule non déduits du 2 %.
**Suite S191 : S190-1 (reprend S189-1)**, projection et profil sous le seuil fixé.
File plurielle des autres chantiers portée par REPRISE §6.7 pour A211.
**120 ADR, 233 angles, 269 leçons, 18 invariants, 6 SPEC, 23 cas** ; aucun banc complet.
Voir [réception B4 à 2 %](docs/validation/B4-TOLERANCE-S190.md) et [ADR-120](docs/adr/ADR-120-b4-tolerance-de-deux-pour-cent.md).


**S191 — 2026-09-12 : profil B4 reçu avec projection**, ADR-121.
**S190-1/S189-1 réalisées sur véhicule de banc** : projecteur D/G reçu contre matrice
indépendante, 3 tests debug/release. Profil **14×14×8 / extrapolation 80 ms** conservé :
budget **1,371947 %**, borne avec résidu **1,374540 %**, erreur avec réserves
**0,948047 %**, sous les **2 % inchangés**. **16 couples reçus / 4 refusés**, 160 ms
refusé partout ; 20384 évaluations de source, réduction13,46 sur la fenêtre, pas un coût CPU.
Deux campagnes release identiques, empreinte **0x52d7645d4548ebb2** ; divergence
normalisée max4,922e-8, pression max44 itérations. Tests workspace331/cinq ignorés
reçus S190 non rejoués ; bibliothèques et supports historiques inchangés.
**ADR-121** : projection fixe linéaire ; borne algébrique de composition avec résidu,
la somme seule d'ADR-119 reste une enveloppe mesurée. **L270**, aucun nouvel angle.
**Suite S192 : S191-1**, construire une tranche2D à surface libre et recevoir une onde
de gravité avant comparaison perturbatif/total. Bords algébriques S191, surface libre
et B4 complet non reçus ; A50 partielle. File plurielle relue, autres chantiers conservés.
**121 ADR,233 angles,270 leçons,18 invariants,6 SPEC,23 cas.**
Voir [réception projetée S191](docs/validation/PROJECTION-B4-S191.md).


**S192 — 2026-09-12 : tranche x-z à surface libre linéaire reçue contre Airy.**
S191-1 réalisée : fond imperméable, dispersion finie/profonde et hauteur évolutive.
Trois profondeurs0,25/2/8 m, quatre grilles, cinq périodes. À128×64 : hauteur
au plus1,647737 %, vitesse au plus1,732796 %, sous2 %. Raffinements espace/temps
proches de l'ordre2 ; dérive d'énergie maximale4,111837e-6 relatif.
Trois tests debug/release ; deux campagnes release identiques **0x4fc690d4ac035bf7**.
Workspace331/cinq ignorés reste le reçu S190, non rejoué. Bibliothèques inchangées.
**B4/A50 partiels** : source S191 non branchée, comparaison intégrale, forces et
perception absentes ; A216/A217 ouvertes. Aucun choix δ. Ne pas additionner ces
écarts Airy au budget source S191, mesuré sur une autre référence.
**Suite S193 : S192-1**, conditions de surface non linéaires dispersives reçues contre
Stokes, ordre en amplitude explicite, avant branchement/comparaison perturbatif-total.
File plurielle relue et actualisée ; aucun autre chantier effacé. Aucun ADR/angle/leçon
nouveau : **121 ADR,233 angles,270 leçons,18 invariants,6 SPEC,23 cas**.
Voir [surface libre S192](docs/validation/SURFACE-LIBRE-2D-S192.md) et [mesures](docs/validation/SURFACE-LIBRE-2D-S192-MESURES.md).


**S193 — 2026-09-12 : surface non linéaire dispersive reçue contre Stokes**, ADR-122.
S192-1 réalisée : conditions de Zakharov exactes, développement en amplitude sur le
relèvement de S192, bande spectrale à convolution tronquée, RK4. À kh=6,2832 et M=3 :
harmonique liée à **0,4555 %** de Stokes, décalage de fréquence à **1,6454 %**, sous 2 %.
Profil sur vingt périodes **0,33 %** à M=3 contre **21,3 %** au modèle linéaire.
Ordres mesurés **2** en profondeur discrète, **4** en temps ; bande identique au bit de
Q=8 à Q=16 ; énergie au plus 2,616146e-9. Quatre tests debug/release, deux campagnes
release identiques **0x41fc3b13793bee10**.
**ADR-122** : l'ordre trois est retenu, l'ordre deux refusé — il rend le bon profil mais
**la moitié** du décalage de fréquence, et la fraction captée dépend du régime (0,663 à
kh=1,5708). Vérifier un profil ne suffit pas à recevoir un schéma tronqué en amplitude.
**A217 perd son manque structurel, pas son objet** : aucun couplage de deux trains n'est
mesuré, ADR-112 intact. Source S191 non branchée, aucune addition au budget 1,374540 % ;
A216 inexpliquée, forces et perception non reçues, A50/B4 partiels, aucun choix δ, fond
plat, surface graphe. Workspace **331 réussis / cinq ignorés rejoués** en debug, identiques au reçu S190.
**Trois trouvailles hors protocole** : la faible profondeur non linéaire **n'a aucun
oracle** ici, la borne d'Ursell étant une falaise mesurée (**A234**) ; la contre-épreuve
à amplitude négligeable a trouvé un biais d'estimateur de 1,1125e-7 dû à une condition
initiale bâtie sur la fréquence du continu (**A236**) ; et la dérive de volume prédite en
a^(M+1) vaut de l'arrondi, deux termes s'annulant identiquement au mode nul (**L273**).
**Suite S193-1 : couplage de deux trains**, écart entre la somme des évolutions et
l'évolution de la somme, contre-épreuve M=1 exactement nulle. File active relue et
renommée S193 (A185), quatre ancres repointées ; aucun autre chantier effacé.
**A234, A235, A236, A237** et **L271, L272, L273** :
**122 ADR,237 angles,273 leçons,18 invariants,6 SPEC,23 cas**.
Voir [surface libre non linéaire S193](docs/validation/SURFACE-LIBRE-NL-S193.md), [mesures](docs/validation/SURFACE-LIBRE-NL-S193-MESURES.md) et [ADR-122](docs/adr/ADR-122-l-ordre-en-amplitude-d-un-vehicule-non-lineaire.md).


**S194 — 2026-09-12 : le couplage de deux trains est mesuré**, ADR-123, et **A217 est close**.
S193-1 réalisée : somme des évolutions contre évolution de la somme, sur le véhicule S193
inchangé. L'écart est la réponse à un forçage croisé explicite, en deux parts de mécanismes
distincts qui vivent sur des modes différents : part croisée de pente **1,0041** et
**stationnaire** (aucune triade résonante en eau profonde, redémontré), part de train de
pente **2,0178** et **croissante d'un facteur 4,1**. Loi `écart/A ≈ α s + β s² N`,
`α = 1,302602`, `β = 5,898728`, résidu à **1,82 %** de l'écart maximal.
**Livrable — la frontière des 2 %** : la superposition indépendante tient au moins vingt
périodes sous `s = 0,008`, **5,4 périodes** à `0,0125`, **moins d'une** à `0,014`. Le domaine
existe en (cambrure × durée) mais le levier de la durée est étroit. À `s = 0,0125`, S193
recevait un train **unique** à `0,4555 %` : facteur **quarante** à cambrure égale.
Écart insensible au couple (**13 %** sur quatre géométries, contra-propagation comprise) et
**8,6 fois plus fort** vers le rivage quand le désaccord de triade tombe de 4,9.
**A217 close** : la variable est la cambrure, plus deux variables qu'A217 ignorait — la
durée et le désaccord de triade. `M=2` sous-estime `β` d'un facteur **3,4** : argument
indépendant pour ADR-122. `s=0,1` par train à `M=3` est **hors domaine** (énergie
`4,643·10⁻³`), publié et exclu par la règle déclarée.
**Deux réfutations publiées** : le contrôle de non-artefact du protocole est non tenu et ne
pouvait pas l'être — il confond convergence et artefact, le rapport des déplacements valant
`3,81` donc l'ordre deux, refait en ordre **1,93** et résidu **0,47 %** (**A239**, **L274**) ;
et la cause soupçonnée, une condition initiale en `b₂` du continu, est **fausse**, testée en
retirant le terme (**L275**). Résultat de méthode : le **maximum d'un résidu ne converge
pas**, ordres `−0,79 / −0,37 / +0,61` (**A238**).
Huit tests debug/release, deux campagnes release identiques **0x4bc0934d630c2c50**.
`water-core` et le support S193 inchangés ; workspace **rejoué : 331 réussis / cinq
ignorés**, identiques au reçu. Restent ouverts : **`n` sources** (**A240**, les paires croissent en
`n²`), obliquité, A216, forces et perception, source S191 non branchée, fond plat.
**Suite S195 — à instruire** : `n` sources, ou la correction croisée quadratique dont
ADR-123 chiffre déjà le gain. File active relue et renommée S194 ; A217 retirée de sa ligne.
**A238, A239, A240** et **L274, L275** :
**123 ADR,243 angles,279 leçons,18 invariants,6 SPEC,23 cas**.
Voir [couplage S194](docs/validation/COUPLAGE-DEUX-TRAINS-S194.md), [mesures](docs/validation/COUPLAGE-DEUX-TRAINS-S194-MESURES.md) et [ADR-123](docs/adr/ADR-123-le-domaine-de-validite-de-la-superposition.md).

**S195 :** `n` sources mesurées, **A240 close**, aucun ADR. À cambrure par train fixée —
ce qu'A240 craignait — l'écart croît en **n^0,75**, sous-linéaire, loin du `n²` du
comptage de paires. À cambrure **totale** fixée il **décroît en 1/√n** : répartir une même
mer sur plus de composantes améliore la superposition. ADR-123 se transporte à `n` sources
dans le sens favorable. Sept réceptions sur dix ; les trois autres tenaient au jeu de
phases, que le banc avait réfuté avant la campagne — c'est la **fonctionnelle** qui sépare
les régimes. **A241** : les harmoniques croisées retombent sur les modes de train et cette
part gouverne la loi à grand `n`. **L276** : une variable de protocole peut être réfutée
par le véhicule avant d'être mesurée. Session **reprise après interruption** d'un autre
compte ; P3a complétée, pas annulée.
Voir [`n` sources S195](docs/validation/SOURCES-MULTIPLES-S195.md).

**S196 :** *(verdict renversé par S197 — lire la suite)* le repli des harmoniques croisées est **séparé** de ce qu'il accompagnait, par
un montage de parité — trains impairs, donc sommes et différences paires, donc aucun
repli, par arithmétique. Verdict : il **déplace** la loi sans la **gouverner**, un tiers
du chemin et pas plus ; la thèse de S195 était trop forte. La moitié « limite » d'A241 est
close — l'exposant sature à `−0,52` dès `n ≈ 4`. Le protocole demandait `>0,20` ou
`<0,10` d'écart : mesuré `0,131`, **ni l'un ni l'autre**, ce qu'une prédiction déclarée
d'avance rend impossible à maquiller. **A242** : la dérive d'énergie, employée comme
critère de domaine par trois sessions, **ne détecte pas la sous-résolution** — un résultat
faux d'un facteur cinq passait 65× sous le seuil. **L277** : un invariant conservé ne dit
rien de ce qui est résolu. Aucun ADR.
Voir [repli des croisées S196](docs/validation/REPLI-CROISEES-S196.md).

**S197 :** audit de résolution, **A242 close**. Le nombre de niveaux verticaux n'entre dans
le véhicule que par le **symbole de dispersion**, précalculé — le défaut se calcule donc en
forme fermée, sans rien simuler, et l'audit est bon marché. À la résolution employée depuis
S193, ce symbole se trompait de 9 % sur la bande peuplée de S194, 44 % de S195, **112 %** de
S196. **ADR-123 tient** — sa table mesurée est convergée dès `K=256`, déplacement ≤ 3,3 % —
et reçoit une note de confirmation. **A240 tient.** **Le verdict de S196 tombe** :
son écart pair/impair de `0,131` vaut **`0,005`** à résolution convergée, ce qui est la
prédiction *réfutante* de S196 lui-même. Le repli n'explique rien de mesurable.
**L278** : une erreur systématique ne s'annule dans une comparaison que si les deux côtés la
portent également — ce n'est pas son ampleur qui décide, c'est sa répartition. Aucun ADR.
Voir [audit de résolution S197](docs/validation/AUDIT-RESOLUTION-S197.md).

**S198 :** à la demande de l'utilisateur, **ce qui ralentit le projet** — mesuré, puis
corrigé. Dernière session ayant ajouté du code d'exécution par couche : **B S181, W S182,
δ S161, V jamais**. δ est arrêtée depuis 37 sessions, V n'a jamais commencé en 198, et
**S190–S197 n'ont produit aucune ligne de bibliothèque** pour 4 509 lignes de bancs. Le
mécanisme : **33 sessions sur 38** ont pris pour sujet le reliquat de la précédente — A211
l'avait nommé en S145, mais son remède avait corrigé le *canal* sans toucher à l'*auteur*.
Quatre correctifs appliqués, dont la **règle des deux maillons** et `outils/velocite.sh`,
qui recalcule tout et fait foi contre les documents. **L279** : corriger le canal ne sert à
rien si l'auteur est en conflit d'intérêt. **A243** : un corpus produit du travail de corpus.
Aucun ADR. Suite : **B3/δ**, choisie par la règle et non par le chaînage.
Voir [bilan de vélocité S198](docs/registres/BILAN-VELOCITE-S198.md).

**S199 — 2026-09-13 : premier noyau δ MAC x-z en bibliothèque.**
Claude construit P1–P3 ; Codex termine la passation P4/P5 sans remesure.
Lac au repos reçu exactement ; filtre spatial passé sur fond plat (ordre1,947),
échoué au fond coupé (≈0,90). **Candidat non éligible B3**, aucune famille éliminée.
339 tests réussis/cinq ignorés, empreinte0x0ad3f695685ca27a : reçus Claude P3.
**A244** : le test de mémoire ne voit pas les allocations du pas ; plafond
d'itérations sans budget temporel, refus/capacités et f64 à mettre en conformité.
**Suite S200 : S199-1**, corriger ces contrats dans la bibliothèque ; **S199-2**
conserve la reconstruction des flux ouverts avant surface mobile. Compteur **0**,
δ a avancé ; B4/A50 partiels, seuil2 % acquis. A217 reste close (S194).
123 ADR,244 angles,279 leçons,18 invariants,6 SPEC,23 cas. Aucun ADR nouveau.
Voir [candidat δ S199](docs/validation/CANDIDAT-DELTA-S199.md).

**S200 — 2026-09-13 : pas δ sans allocation, refus numériques atomiques.**
Clones supprimés, sauvegardes préallouées, mémoire comptée à sa précision réelle.
Compteur global avec témoin : zéro allocation au premier pas, nominal/dégradé et refus.
Après débordement, u/w/p restaurés et récupération identique à un noyau neuf.
Caps restreint ses promesses (référentiel général faux, bornes inconnues absentes).
**342 tests réussis, cinq ignorés** ; trois nouveaux tests aussi en release.
Filtre S199 inchangé : **0x0ad3f695685ca27a**, fond coupé toujours≈ordre0,90.
**A244 partielle** : pression f64 expérimentale et budget temporel non reçus.
**Suite S201 : S200-1**, recevoir ces deux contrats ; S199-2 flux ouverts conservée.
Compteur0, δ avance. B3 non admissible, B4/A50 partiels, seuil2 % inchangé.
123 ADR,244 angles,279 leçons,18 invariants,6 SPEC,23 cas ; aucun ADR nouveau.
Voir [contrats δ S200](docs/validation/CONTRATS-DELTA-S200.md).

**S201 — 2026-09-13 : B visible, première image CPU locale.**
Caméra/rayons et PPM640×360 depuis Background::eval/JONSWAP N32, sans dépendance.
Deux instants et un plan témoin inspectés ; zéro rayon non résolu, tolérance3 mm.
T12 empreinte **a52ff81902b150c3**, t13 **1df02ffb7c202b32**, environ12 s/image
sur cette machine : référence d'observation, aucun budget temps réel reçu.
Cinq tests exemple réussis ;342/cinq ignorés reste le reçu bibliothèque S200.
**ADR-124 actée sur instruction utilisateur** : images locales permises, puis
budget image/coût par bloc, puis δ en effets bornés et V au besoin gameplay.
*⚠ S204 : lecture corrigée par ADR-127 — ordre de construction, pas réduction.*
**Suite S202 : S201-1**, budget image. S200-1/S199-2 reportées, pas closes.
B4/A50 partiels ; seuil numérique2 % acquis, aucune perception déclarée reçue.
124 ADR,244 angles,279 leçons,18 invariants,6 SPEC,23 cas. Compteur0 : décision δ/V.
Voir [image B S201](docs/validation/IMAGE-B-S201.md), [ADR-124](docs/adr/ADR-124-image-budget-et-effets-bornes.md).

**S202 — 2026-09-13 : budget60 images/s, eau2 ms par image, ADR-125.**
Choix explicite utilisateur. `cost_per_block_ms` branché sur le pas complet via
horloge injectée ; inconnu avant mesure, aucune allocation ni physique modifiée.
Un bloc de banc=un domaine x-z, pas une maille ou un bloc3D fictif.
32×16 convergé≈0,59 ms médiane,64×32≈4,79 ms : trop cher même isolé.
**343 tests réussis, cinq ignorés**, quatre tests intégration aussi en release.
Le coût mesuré ne garantit ni qualité ni délai ; A244 reste partielle, B3 non admis.
Rendu CPU S201 hors ligne, aucun budget GPU reçu.2 % physique reste acquis.
**Suite S203 : S202-1**, impact visible W, emprise/observateur explicites, puis part
δ éventuellement nécessaire ; V au besoin gameplay selon ADR-124.
*⚠ S204 : corrigé par ADR-127 — δ général et V obligatoires.*
125 ADR,244 angles,279 leçons,18 invariants,6 SPEC,23 cas ; compteur0 (δ avance).
Voir [budget image S202](docs/validation/BUDGET-IMAGE-S202.md), [ADR-125](docs/adr/ADR-125-budget-image-60hz-deux-ms.md).

**S203 — 2026-09-13 : un impact W visible, ADR-126 ; la mer S201 n'est pas composable.**
Plancher de pente L1 de B 0,6082 à Hs1,5 > π/7 : `compose` refuse chaque point de la mer
S201 (A245, gravité1 ; ADR-062/094/095 corrigés par note datée). Scène à Hs0,5, impact
λ3,35 m/E164 J. ADR-126 : emprise d'image reçue par ses coutures, R ≥ 15,5 λ,
A ≥ 96√(λ/g), N ≥ 256. Images +1/+3/+6 s : zéro pixel différent hors emprise.
Coût B+W 14 µs/pt, table radiale 2,7 ms/impact contre 2 ms (A247). Bibliothèque inchangée.
**Suite S204 : A245.** 126 ADR,247 angles,281 leçons,18 invariants,6 SPEC,23 cas.
Voir [impact W S203](docs/validation/IMPACT-W-S203.md), [ADR-126](docs/adr/ADR-126-emprise-d-un-impact-visible.md).

**S204 — 2026-09-13 : ambition complète rétablie, ADR-127 corrige ADR-124.**
Clarification de l'utilisateur : *ambition finale complète, construction progressive par
versions de plus en plus capables*. δ général, V, inondations complexes et grande échelle
obligatoires ; effets bornés = étapes ; V-noyau au plus tard avec J2. Budget 60 Hz/eau 2 ms
et seuil 2 % acquis, incompatibilité ⇒ arbitrage explicite. Nouvelle feuille de route.
**Suite S205 : A245.** 127 ADR,248 angles,282 leçons,18 invariants,6 SPEC,23 cas.
Voir [ADR-127](docs/adr/ADR-127-ambition-complete-construction-progressive.md), [feuille de route](docs/FEUILLE-DE-ROUTE.md).

**S205 — 2026-09-13 : la mer de référence se compose, ADR-128 ; A245 close.**
Budget de pente = perturbations seules ; raideur de B publiée ; aucun bit publié changé pour un
lot déjà admis. Mer S201 Hs 1,5 composée avec impact, zéro refus. 344 réussis/cinq ignorés.
**Suite S206 : A247.** 128 ADR,249 angles,282 leçons,18 invariants,6 SPEC,23 cas.
Voir [composition mer S205](docs/validation/COMPOSITION-MER-S205.md), [ADR-128](docs/adr/ADR-128-le-budget-de-pente-borne-les-perturbations.md).
