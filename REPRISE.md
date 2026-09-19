# REPRISE â€” Ã  lire en premier, en entier

Passation active du systÃ¨me de gestion de l'eau. L'amorce unique est [AGENTS.md](AGENTS.md).
Ce fichier fait foi sur les mÃ©moires privÃ©es. Il contient les rÃ¨gles prÃ©sentes, pas les bilans
successifs : l'histoire vit dans [JOURNAL](notes/JOURNAL.md), les preuves dans les documents liÃ©s.
**Refonte S227, 2026-09-13** : ancienne version intÃ©grale conservÃ©e dans Git Ã  `dfd1507`.

## Jeton de session

```
JETON            : occupÃ©
Battement        : 2026-09-19 20:51 +02:00
Agent            : Claude Opus 5, application desktop (fichiers, git, cargo, outils locaux, carte rÃ©elle RTX 5070 Laptop)
Session en cours : S301 â€” pas couplÃ© complet rÃ©sident sur la carte, un seul device, reÃ§u contre le cÅ“ur
DerniÃ¨re session : S300 â€” fond B Ã©valuÃ© sur la carte et second membre couplÃ© reÃ§us contre le cÅ“ur
Session suivante : S301 â€” porte B : advection, bandes de couplage et Ã©ponge sur la carte, puis la surface mobile et sa surface publiÃ©e (ADR-175 D7). La surface mobile demandera un arbitrage : le cÅ“ur **refait** le pas avec un affinage quand la porte d'ADR-144 Ã©choue, ce que la production n'a pas le droit de faire (D3 : dÃ©gradation dÃ©clarÃ©e) â€” nouvel ADR probable.
Maillons        : 0 â€” capacitÃ© S300 : fond B Ã©valuÃ© sur la carte depuis les seuls paramÃ¨tres publiÃ©s et second membre couplÃ© qui en dÃ©coule, reÃ§us Ã  2,6Â·10â»â¶ du cÅ“ur, consommÃ©s par cinq bancs, preuve DELTA3D-FOND-GPU-S300
```

**Avant de dÃ©cider d'une reprise, vÃ©rifier les copies et branches selon AGENTS.md.** Le jeton
est versionnÃ© et local Ã  une branche : il ne constitue pas un verrou global.

| Ã©tat trouvÃ© | action |
|---|---|
| libre | prendre le jeton, Ã©crire Agent et le plan, committer le plan seul |
| occupÃ©, battement < 2 h | signaler la session active et s'arrÃªter |
| occupÃ©, battement ancien, ou interrompu | reprise Ã  chaud : [EN-COURS](notes/EN-COURS.md) |
| archivÃ© | rejoindre la branche vivante dÃ©signÃ©e ; ne pas travailler ici |

Une interruption explicitement signalÃ©e par l'utilisateur permet la reprise Ã  chaud mÃªme si le
battement est rÃ©cent. Le battement n'est pas une preuve de vie. Lire l'horloge dans un appel
sÃ©parÃ©, puis reporter sa valeur Ã  chaque commit d'Ã©tape (L237). Une seule session Ã©crit Ã  la fois.

## File active du projet

[Questions ouvertes â€” file active](docs/registres/QUESTIONS-OUVERTES.md#file-active) porte les
travaux, Ã©tats et dÃ©clencheurs. [FEUILLE-DE-ROUTE](docs/FEUILLE-DE-ROUTE.md) porte seule les jalons.
Ne pas recopier leurs suivis ici. [Liste du projet fini](docs/LISTE-PROJET-FINI.md) : ce que l'ambition complÃ¨te contient, cochÃ©e Ã  la demande de l'utilisateur. Dernier audit global : [BILAN-GLOBAL-S293](docs/registres/BILAN-GLOBAL-S293.md) â€” oÃ¹ l'avancement bloque.

## 1. Ce qu'est ce projet

SystÃ¨me d'eau temps rÃ©el pour un jeu de trÃ¨s grande Ã©chelle. Les intentions d'origine sont dans
[docs/sources](docs/sources/systeme_eau_architecture_globale.md), non modifiables.
La conception s'Ã©crit en Markdown ; le cÅ“ur et le harnais en Rust sans dÃ©pendance externe.
L'hÃ´te GPU sÃ©parÃ© `viewer/` utilise les dÃ©pendances autorisÃ©es et verrouillÃ©es en S210/S211.
Images de banc locales autorisÃ©es (ADR-124) ; pas de page HTML ni d'artefact publiÃ©.

**Ambition complÃ¨te**, construction progressive : Î´ gÃ©nÃ©ral, V, inondations complexes et grande
Ã©chelle restent obligatoires (ADR-127). Une prioritÃ© ou un budget ne rÃ©duit jamais le pÃ©rimÃ¨tre.

## 2. RÃ´le et mÃ©thode

Autonomie technique dÃ©lÃ©guÃ©e par l'utilisateur (S71). FranÃ§ais, concis, factuel ; le fond va dans
les fichiers. **Il n'y a pas d'autres Ã©quipes** : les dÃ©veloppeurs observent, les dÃ©cisions
internes nous appartiennent (ADR-028). Ne pas transformer un fait externe inconnu en hypothÃ¨se
acquise. **Depuis S254, l'utilisateur supervise les rendus visuels** : lui demander des rÃ©fÃ©rences
rÃ©elles quand un rendu doit Ãªtre jugÃ©, selon [REVUE-VISUELLE](docs/validation/REVUE-VISUELLE.md). La mÃ©thode de choix du lot et de validation vit dans [METHODE](notes/METHODE.md).

Plan avant travail dans [EN-COURS](notes/EN-COURS.md), committÃ© seul ; une Ã©tape par commit
`S<n> P<k> â€” â€¦`, moins d'un quart d'heure par Ã©tape. DÃ©clarer un dÃ©coupage si nÃ©cessaire.

## 3. OÃ¹ est la connaissance â€” lecture Ã  froid

AprÃ¨s l'amorce Git et la prise du jeton, lire dans cet ordre :

1. La **derniÃ¨re entrÃ©e seulement** de [JOURNAL](notes/JOURNAL.md).
2. [Index](docs/00_INDEX.md), pour choisir les sources utiles.
3. [Invariants](docs/01_INVARIANTS.md), puis [ADR-001](docs/adr/ADR-001-decomposition-en-couches.md).
4. [Feuille de route](docs/FEUILLE-DE-ROUTE.md), [file active](docs/registres/QUESTIONS-OUVERTES.md#file-active)
   et [mÃ©thode](notes/METHODE.md).
5. Les ADR, spÃ©cifications et leÃ§ons **du lot choisi** ; recherche ciblÃ©e dans [LECONS](notes/LECONS.md).

Ne pas relire le journal, les leÃ§ons ou tous les ADR intÃ©gralement Ã  chaque reprise. Une reprise
Ã  chaud suit uniquement EN-COURS et le diff. Les preuves d'un lot se lisent avant de le modifier.

## 4. OÃ¹ en est le projet

Ã‰tat au 2026-09-19 (S300). Trajectoire et Ã©tat par jalon : [FEUILLE-DE-ROUTE](docs/FEUILLE-DE-ROUTE.md) ;
travaux : [file active](docs/registres/QUESTIONS-OUVERTES.md#file-active). En bref :

- **J1** partiel : B+W sur GPU dans l'hÃ´te sÃ©parÃ©, scÃ¨ne multi-sources admise par le cÅ“ur, GPU eau
  1,74 ms, mer jugÃ©e par l'utilisateur (R7 acceptÃ©) ; manquent le CPU sous 2 ms (A278), un objet
  pilotable et la seconde cible.
- **Î´ reÃ§u en 2D** â€” surface mobile couplÃ©e Ã  B/W contre HOS, frontiÃ¨res, rendu en direct.
  **S295â€“S296** : rÃ©fÃ©rence 3D Ã  surfaces linÃ©aire et mobile reÃ§ue, identique au bit Ã  la 2D quand `ny = 1`.
  **Porte B ouverte** : Î´ en 3D selon [ADR-175](docs/adr/ADR-175-architecture-d-execution-de-delta-en-3d.md),
  rÃ©fÃ©rence CPU couplÃ©e reÃ§ue S297, frontiÃ¨res et fond spectral rÃ©el reÃ§us S298. Une mer Ã  la fois
  rÃ©solue et Ã©talÃ©e n'entre pas dans un banc CPU : le critÃ¨re 3 d'ADR-175 Â§4 demande la production
  GPU. **S299** en a construit le premier Ã©tage â€” opÃ©rateur, prÃ©conditionneur et projection Ã 
  travail bornÃ© â€” et **S300** le second : la carte Ã©value B elle-mÃªme et en tire le second membre
  couplÃ©, lÃ  oÃ¹ le mÃªme Ã©chantillonnage sur CPU coÃ»terait 93 Ã  148 ms par pas. Restent
  l'advection, les bandes, l'Ã©ponge, la surface mobile et sa surface publiÃ©e, les diagnostics,
  la scÃ¨ne et la revue.
- **Ordonnanceur** (porte A) : dÃ©cide qu'un domaine vit et avec quel budget ; ni plusieurs
  candidats, ni dÃ©placement, ni dÃ©gradation automatique.
- **V** : noyau reÃ§u (C12, gÃ©omÃ©trie orientÃ©e, restauration), sans articulation avec Î´.
- **Solides** : rien dans le systÃ¨me (porte D, scÃ¨ne-tÃ©moin de la v1).
- Dernier audit : [BILAN-GLOBAL-S293](docs/registres/BILAN-GLOBAL-S293.md) â€” le projet bloquait sur
  l'ordre de ses propres travaux.

L'inventaire Git se recalcule : `python outils/etat_projet.py` (Python standard, sans rÃ©seau).
Ses nombres mesurent des fichiers et des modifications, **pas du temps ni des capacitÃ©s**. `--check`
vÃ©rifie la navigation et les plafonds des documents d'Ã©tat.

## 5. Ce qui ne se dÃ©cide pas ici

- Les cinq arbitrages d'[ADR-027](docs/adr/ADR-027-les-cinq-arbitrages-tranches.md) sont tranchÃ©s.
- 2 % pour B4 (ADR-120), 60 Hz (ADR-125), hÃ´te GPU sÃ©parÃ© (ADR-130) et sources de son verrou
  dÃ©jÃ  autorisÃ©es : ne pas redemander ces accords. **Arbitrages du 2026-09-19**
  ([ADR-174](docs/adr/ADR-174-arbitrages-du-2026-09-19.md)) : machine de rÃ©fÃ©rence = ce poste ;
  temps de l'eau au service de l'objectif, profil de travail eau â‰¤ 4 ms GPU et â‰¤ 2 ms CPU, dont Î´
  â‰¤ 2 ms GPU ; v1 = porte D franchie ; aucun dÃ©pÃ´t distant. Un dÃ©passement qualifie
  l'implÃ©mentation et s'Ã©prouve sur la combinaison des techniques (ADR-131).
- Une rÃ©duction d'ambition demande une dÃ©cision explicite de l'utilisateur (ADR-127).
- Les faits d'intÃ©gration non constatÃ©s et les actions d'infrastructure restent distincts des
  dÃ©cisions techniques : ne pas inventer terrain, format rÃ©seau, personnes ou dÃ©pÃ´t distant.

## 6. Rituel de fin de session â€” derniÃ¨re Ã©tape du plan

1. Ã‰crire **une entrÃ©e concise** au journal : entrÃ©es, changements, preuves et limites, non-fait,
   prochaine capacitÃ© visÃ©e, arbitrages rÃ©els. Ne pas la dupliquer dans les points d'entrÃ©e.
2. Enregistrer les nouveaux angles morts avec sÃ©vÃ©ritÃ©, ou actualiser ceux qui couvrent dÃ©jÃ  le
   dÃ©faut. Une leÃ§on nouvelle doit Ãªtre gÃ©nÃ©ralisable ; aucune obligation d'en produire.
3. Transformer chaque action annoncÃ©e non rÃ©alisÃ©e en point datÃ© de la file, avec dÃ©clencheur.
4. Actualiser les lignes touchÃ©es de la feuille de route et de la file active **en remplacement
   de leur Ã©tat pÃ©rimÃ©**. Les Ã©tats antÃ©rieurs restent dans Git et le journal.
5. Corriger les renvois affectÃ©s et relire les invariants touchÃ©s. Un ADR reÃ§oit seulement une
   note factuelle datÃ©e ; une dÃ©cision remplacÃ©e exige un nouvel ADR. Mettre les nouveaux
   documents utiles dans l'index ; vÃ©rifier les dÃ©comptes seulement s'ils sont encore affichÃ©s.
6. Mettre Ã  jour jeton, session, battement et Ã©tat ; cocher le rituel avant son commit.
7. Relire **toute la file active**. **`Session suivante` se prend dans la porte en cours** que
   dÃ©signe [FEUILLE-DE-ROUTE Â§3 bis](docs/FEUILLE-DE-ROUTE.md), ou dans la demande de
   l'utilisateur : la suite qu'une session dÃ©clare n'est qu'une proposition. Un mÃªme point de
   file ne porte pas une troisiÃ¨me session consÃ©cutive si aucun critÃ¨re Â« reÃ§u si Â» d'une porte
   n'a avancÃ© (S294, A211). La recommandation du dernier bilan est portÃ©e ou Ã©cartÃ©e au journal.
   Une suite locale n'efface jamais Î´, V, B2, bathymÃ©trie ou multiplateforme.
8. Appliquer la rÃ¨gle des deux maillons prÃ©cisÃ©e ci-dessous, puis fermer/synchroniser les copies
   selon **AGENTS.md**, sans recopier sa procÃ©dure ici.

### Deux maillons â€” critÃ¨re rÃ©visÃ© S227, resserrÃ© S294

`Maillons` compte les sessions successives sans **capacitÃ© reÃ§ue**. Remise Ã  zÃ©ro seulement si
le journal nomme : **ce qui devient possible**, **le chemin qui le consomme**, **la preuve**.
Une correction d'intÃ©gritÃ© effectivement reproduite puis testÃ©e compte. Une dÃ©cision qui lÃ¨ve
un blocage compte si elle nomme le lot de construction dÃ©sormais exÃ©cutable. Un commentaire,
un banc isolÃ©, un simple ajout dans `src` ou un ADR sans effet aval ne suffisent pas.
**Depuis S294** : une capacitÃ© compte si elle fait avancer un critÃ¨re Â« reÃ§u si Â» d'une porte de
Â§3 bis, ou l'Ã©tat d'un point de la [liste du projet fini](docs/LISTE-PROJET-FINI.md). Une
optimisation consommÃ©e qui ne franchit aucun critÃ¨re de porte ne remet pas le compteur Ã  zÃ©ro :
S289, S290 et S291 l'avaient fait trois fois de suite sur le mÃªme fil (BILAN-GLOBAL-S293 M1).

Sinon, incrÃ©menter. Ã€ deux maillons, choisir dans la file un lot faisant avancer une capacitÃ©,
et comparer sa prioritÃ© aux reliquats. Un troisiÃ¨me maillon demande une justification explicite
au journal. MÃªme avec du code produit, **avant une troisiÃ¨me session sur le mÃªme sujet**, vÃ©rifier
si sa suite dÃ©bloque encore l'usage visÃ© ou relÃ¨ve dÃ©sormais d'un approfondissement diffÃ©rable.
La demande actuelle de l'utilisateur prime toujours sur la suite automatique.

## 7. Session interrompue

ProcÃ©dure unique dans [EN-COURS](notes/EN-COURS.md). Les commits attestent le travail fini ; le
diff appartient Ã  l'Ã©tape `[>]`. Le lire et complÃ©ter ou annuler cette Ã©tape, sans toucher Ã 
un travail Ã©tranger. Les notes de reprise conservent mesures, dÃ©cisions et impasses coÃ»teuses.

## 8. Tenue de la connaissance

ADR et sources conservÃ©s. Aucune valeur physique sans provenance (I-14). Distinguer dÃ©cidÃ©,
construit, reÃ§u et intÃ©grÃ©, ainsi que rÃ©solu, dissous, partiel et ouvert par dÃ©cision.
Chaque information a un porteur : journal pour l'histoire, feuille de route pour les capacitÃ©s,
file active pour les travaux, documents de validation pour les preuves, index pour les liens.
REPRISE ne grandit pas d'un compte rendu Ã  chaque session.
**Plafonds (S294)** : une ligne de la file active â‰¤ 90 mots â€” Ã©tat prÃ©sent, dÃ©clencheur, lien ;
une section de jalon de la feuille de route â‰¤ 450 mots ; l'histoire va au journal et aux
preuves. `python outils/etat_projet.py --check` vÃ©rifie plafonds et navigation au rituel.

## 9. Limites du dispositif

L'Ã©tat des branches, copies et remotes se **constate** avec Git. Aucun distant Ã  l'ouverture de
S227 : la crÃ©ation ou publication distante reste une action d'infrastructure Ã  autoriser.
Le jeton versionnÃ© ne verrouille pas plusieurs copies atomiquement ; garder les vÃ©rifications
Git de l'amorce. A215 reste ouverte. Les mÃ©moires privÃ©es ne constituent jamais une passation.
