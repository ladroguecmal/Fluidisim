# REPRISE — à lire en premier, en entier

Passation active du système de gestion de l'eau. L'amorce unique est [AGENTS.md](AGENTS.md).
Ce fichier fait foi sur les mémoires privées. Il contient les règles présentes, pas les bilans
successifs : l'histoire vit dans [JOURNAL](notes/JOURNAL.md), les preuves dans les documents liés.
**Refonte S227, 2026-09-13** : ancienne version intégrale conservée dans Git à `dfd1507`.

## Jeton de session

```
JETON            : occupé
Battement        : 2026-09-18 18:35 +02:00
Agent            : Codex, GPT-6 (fichiers, git, cargo, Python, GPU local, accès web)
Session en cours : S271
Dernière session : S270 — bande du fond aux frontières reçue sur courant uniforme (ADR-165), 478 tests
Session suivante : S271 — houle progressive traversante, référence indépendante et perturbations induites
Maillons        : 0

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
Ne pas recopier leurs suivis ici. [Liste du projet fini](docs/LISTE-PROJET-FINI.md) : ce que l'ambition complète contient, cochée à la demande de l'utilisateur. Dernier audit global : [BILAN-GLOBAL-S227](docs/registres/BILAN-GLOBAL-S227.md).

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

État à S249 : B/W et un afficheur existent ; **filtre lointain B/sillage intégré** (ADR-148,
[réception et limites](docs/validation/COUPURE-S249.md), impacts non filtrés) ; scène multi-sources dessinée, visibilité au bit, composée
et admise par le cœur (ADR-142) ; **la boucle d'image de l'hôte n'alloue rien** pour le code du projet,
la pile verrouillée 133 fois par image, constantes (ADR-145). Le poste dominant du budget de l'hôte est
`ModalPressure::sample` dans la préparation du sillage — **87 %** de ses 3,1 ms, contre 0,44 ms de
GPU eau (S242). Le système a désormais un **parallélisme déterministe** (ADR-146), qui vaut **×2,6**
sur ce poste **hors ligne** ; le chemin d'image reste à un fil tant qu'un vivier persistant n'existe
pas, ce qui demanderait `unsafe` dans l'hôte (A278). Côté δ, **la carte du coût est faite** (S244) : écritures
disjointes 67 à 73 % du pas, réductions 12-13 %, itérations doublant par raffinement, et
**286,2 ms par pas à 32 768 mailles** — 143 fois le budget. Le parallélisme est **fermé** pour cette
boucle (125 µs par fil contre 21,7 de pass) ; **la multigrille est le seul levier dont le gain croît
avec la taille** (A276, [carte](docs/validation/COUT-DELTA-S244.md)). δ MAC x-z possède une **surface géométriquement mobile** reçue contre
l'onde stationnaire HOS d'ordre 3 (surface graphe, sans 3D, cavité, scénario B3 ni I-05 complet), **couplée depuis S253 à un fond B/W linéaire** (ADR-152/153), **B de production prolongé au-dessus du plan moyen depuis S254** (ADR-154 ; W : A286) ; sa
pression f32 s'arrête à sa précision représentable (ADR-143) et **tient la tolérance physique de S199
ou se déclare dégradée** (ADR-144) — tenue jusqu'à 8 192 mailles, et **à 32 768** par le repli multigrille (ADR-147) suivi, depuis S252, d'un affinage de divergence (ADR-151 ; β du gradient multigrille corrigé, A285) ; plancher
des lignes à fantôme de surface non borné (A274). V : plans orientés et restauration locale reçus.
Les réceptions et limites courantes sont dans la feuille de route. Le nombre de tests ne mesure
pas la couverture des intentions, et une exécution locale ne reçoit pas le multiplateforme.

L'inventaire Git se recalcule : `python outils/etat_projet.py` (Python standard, sans réseau).
Ses nombres mesurent des fichiers et des modifications, **pas du temps ni des capacités**.

## 5. Ce qui ne se décide pas ici

- Les cinq arbitrages d'[ADR-027](docs/adr/ADR-027-les-cinq-arbitrages-tranches.md) sont tranchés.
- 2 % pour B4 (ADR-120), 60 Hz / eau 2 ms (ADR-125), hôte GPU séparé (ADR-130) et sources de son
  verrou déjà autorisées : ne pas redemander ces accords. Un dépassement qualifie l'implémentation
  et s'éprouve sur la combinaison des techniques (ADR-131).
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
7. Relire **toute la file active**. La recommandation du dernier bilan doit être portée par
   `Session suivante`, ou explicitement écartée au journal. Les autres sujets gardent leur
   déclencheur. Une suite locale n'efface jamais δ, V, B2, bathymétrie ou multiplateforme (A211).
8. Appliquer la règle des deux maillons précisée ci-dessous, puis fermer/synchroniser les copies
   selon **AGENTS.md**, sans recopier sa procédure ici.

### Deux maillons — critère révisé S227 sur demande d'audit

`Maillons` compte les sessions successives sans **capacité reçue**. Remise à zéro seulement si
le journal nomme : **ce qui devient possible**, **le chemin qui le consomme**, **la preuve**.
Une correction d'intégrité effectivement reproduite puis testée compte. Une décision qui lève
un blocage compte si elle nomme le lot de construction désormais exécutable. Un commentaire,
un banc isolé, un simple ajout dans `src` ou un ADR sans effet aval ne suffisent pas.

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

## 9. Limites du dispositif

L'état des branches, copies et remotes se **constate** avec Git. Aucun distant à l'ouverture de
S227 : la création ou publication distante reste une action d'infrastructure à autoriser.
Le jeton versionné ne verrouille pas plusieurs copies atomiquement ; garder les vérifications
Git de l'amorce. A215 reste ouverte. Les mémoires privées ne constituent jamais une passation.
