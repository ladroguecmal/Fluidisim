# Travail en cours — journal d'intention

> **Pourquoi ce fichier existe.** Une session coupée par une limite d'usage n'a *aucune* occasion
> d'écrire « j'ai été interrompue ». Tout dispositif de passation qui suppose une action au moment
> de l'arrêt est donc inutile. Seule survit une déclaration faite **avant** le travail.
>
> Ce fichier déclare ce qui va être fait, avant de le faire. Git enregistre ce qui a effectivement
> été fait. L'écart entre les deux est exactement ce qui a été interrompu.

---

## Reprise à chaud — procédure

À suivre lorsque l'état ci-dessous n'est pas `terminée`. Cinq minutes ; **ne pas lire tout le
dépôt** — la lecture complète (`REPRISE.md`) ne sert qu'au démarrage à froid.

1. **Lire l'état et le plan** de la session en cours, plus bas.
2. `git log --oneline -15` — **ce qui est committé est fait**, définitivement. Ne pas le refaire.
3. `git status --short` — les fichiers modifiés non committés appartiennent à l'étape marquée
   `[>]`. C'est elle qui a été interrompue, et elle seule.
4. `git diff` — **lire avant de décider**. Deux issues, pas trois :
   - **compléter** l'étape, si le diff est cohérent et si la thèse déclarée dans le plan est
     claire ;
   - **annuler** l'étape (`git restore <fichiers>`), si le diff est incohérent ou
     incompréhensible.

   Ne jamais laisser un état intermédiaire non tranché, et écrire dans le journal lequel des deux
   a été choisi.
5. **Lire les notes de reprise** de la session interrompue. C'est là que vivent les chiffres déjà
   calculés, les décisions prises mais pas encore écrites et les impasses déjà explorées —
   l'information la plus coûteuse à reproduire, et la seule que git ne conserve pas.
6. Reprendre au premier `[ ]`, ou à `[>]` si l'étape a été complétée.
7. **Prévenir l'utilisateur** : la session précédente a probablement été coupée avant d'avoir pu
   rendre compte de son travail. Résumer ce qu'elle avait fait — il ne l'a peut-être jamais vu.

---

## Règles pour la session qui travaille

- **Déclarer le plan complet avant la première modification**, et le committer seul. C'est
  l'écriture anticipée : sans elle, une interruption ne laisse aucune trace d'intention.
- **Aucune étape ne dépasse une quinzaine de minutes de travail.** Si elle est plus grosse, la
  découper. C'est la seule prophylaxie réelle contre une coupure — pas un confort d'organisation.
- Marquer `[>]` **avant** de commencer une étape. Basculer `[x]` **en dernière action avant le
  commit de cette étape**, jamais après : le commit doit contenir à la fois le travail et la case
  cochée, sinon l'historique ment dans un sens ou dans l'autre. Un `[x]` sans commit est un
  mensonge que la session suivante paiera ; un commit sans `[x]` fera refaire du travail déjà fait.
- **Un commit par étape**, message `S<n> P<k> — <description>`. Le plan et le journal git disent
  alors la même chose de deux façons indépendantes ; si l'un est faux, l'autre le révèle.
- Déposer dans **Notes de reprise** tout ce qui n'est pas encore dans un fichier : un chiffre
  calculé, une décision prise, une impasse explorée. **Une impasse est aussi précieuse qu'un
  résultat** — sans elle, la session suivante la réexplore intégralement.
- **Le rituel de fin (`REPRISE.md` §6) est lui-même une étape du plan.** Une session interrompue
  laisse ainsi cette étape visiblement non cochée, ce qui dit à la suivante exactement ce qui
  manque.

---

## Session en cours

Session : S158 — en cours
Agent : Claude Code (Opus 5 ; fichiers, git et cargo disponibles)
Objectif : S157-1, A214. S157 a laissé deux voies. Je prends la **seconde** — la tolérance — et
non la première, pour une raison qui se vérifie : sortir le plan d'expérience de sa dégénérescence
demanderait de faire varier `cutoff` et `sigma` séparément, donc de comparer des formes spectrales
différentes ; or S157 a montré que l'instant limite hérite de la tolérance choisie. **Mesurer plus
finement une quantité dont la définition dépend d'une convention non écrite ne rapporterait rien.**

ADR-028 : il n'y a personne à qui demander. Une question de conception est à moi. « Demander la
tolérance » veut donc dire la **dériver**, avec provenance — I-14 interdit un nombre qui
n'appartiendrait ni à une formule citée ni à un banc qui le fixe.

### Plan

- [x] **P1** — état réel, jeton, plan déclaré et committé seul.
- [x] **P2** — inventaire des **consommateurs** d'un champ de sillage et de ce que chacun lit
      réellement : `eta`, sa dérivée temporelle, la vitesse, la normale. Pour chacun, la plus
      petite erreur qui change une grandeur déjà décidée ailleurs. La réponse est là, pas dans
      une mesure de plus.
- [ ] **P3** — convertir en tolérance sur l'observable de S157, l'excès de champ proche. Sans
      cette conversion, une tolérance dérivée reste inutilisable : les deux ne parlent pas de la
      même chose.
- [ ] **P4** — ADR fixant la tolérance et sa provenance, puis relecture des durées de S157 **à
      cette tolérance** — les mesures existent, aucune campagne nouvelle si elle tombe dedans.
- [ ] **P5** — le garde-fou, désormais constructible, avec son test témoin ; ou le constat motivé
      qu'il ne l'est toujours pas.
- [ ] **P6** — livrable, rituel de fin, fusion `--ff-only`.

### Notes de reprise

Départ 7d474cd = master, trois copies coïncidentes, rien en attente.
298 tests/cinq ignorés, 108 ADR, 214 angles, 237 leçons, 18 invariants.

Pistes de provenance repérées avant de commencer, à vérifier et non à croire :
- **I-08 et SPEC-001** : ulp d'un f32 à distance `d` vaut `d·2⁻²³`, soit 0,49 mm au bord du
  domaine de 4096 m. C'est un **plancher de représentation**, pas une tolérance — le confondre
  avec une tolérance serait exactement l'erreur qu'I-14 vise.
- **budget de pente** : `max_slope`, le facteur L1 de 1,7950713 (ADR-094) et la cambrure limite
  de Stokes 0,4488 (SPEC-001 §4). La pente est ce que lisent le rendu **et** la flottabilité.
- **seuil de B2** : 1e-4 E0 depuis S153, déjà employé sur les bilans d'énergie.
- **ADR-008** flottabilité, **I-13** le rendu ne pilote pas la physique, **I-12** un domaine
  perturbatif est visuellement gratuit.

Danger principal de cette session, à énoncer pour pouvoir s'y prendre : **inventer un nombre qui
sonne juste**. Un « 2 mm » plausible et sans provenance vaudrait moins que l'absence de garde-fou
d'ADR-108, parce qu'il aurait l'autorité d'une décision. Si aucune dérivation ne tient, la
conclusion correcte est qu'A214 reste ouverte et que la session l'aura montré une seconde fois.

P2 — l'inventaire des consommateurs donne plus que prevu, et deplace la question.

**Premier resultat, mesure et non argumente : les consommateurs qui lisent une borne sont
immunises.** La recurrence rephase les modes ; elle ne change pas l'amplitude des coefficients.
Enveloppe de pente et energie, radial 128 contre 512, angulaire 512 :

| recette | 8 s | 60 s |
|---|---|---|
| enveloppe, 128 | 5,578484852e-3 | 5,620170385e-3 |
| enveloppe, 512 | 5,578064360e-3 | 5,585675128e-3 |
| ecart relatif | 7,5e-5 | **6,2e-3** |
| energie, ecart relatif | 2,5e-4 | **4,2e-4** |

A 60 s, la ou le champ **echantillonne** en champ proche se trompe d'un facteur 48 (S156), les
grandeurs **bornees** se trompent de 0,6 % et 0,04 %. Rapport de sensibilite : environ 8000.
Consequence directe : le declencheur d'ecume, qui passe par `slope_envelope` et non par un
echantillon (`bound_pressure` ligne 663, `base.steepness*PI + slope_envelope`), **ne voit pas la
recurrence**. Le budget de pente d'ADR-080 non plus. L'admissibilite non plus.

**Deuxieme resultat, celui qui deplace la question.** L'erreur de recurrence est **deterministe et
identique chez tous les participants** — I-03 le garantit, et c'est le meme calcul partout. Donc
elle ne peut produire ni desynchronisation, ni divergence de replique, ni inegalite entre joueurs.
I-15 : une grandeur derivee est autoritaire si tous la calculent a l'identique ; c'est le cas, y
compris avec l'erreur. **L'autorite est preservee ; seule la fidelite souffre.**
Cela change la nature du garde-fou cherche depuis A214 : il ne protegerait pas d'une faute, il
protegerait d'une **invraisemblance**. Ce n'est pas la meme urgence, et ce n'est pas le meme juge.

**Troisieme resultat : le juge existe et n'a pas siege.** Le seul dispositif du corpus qui statue
sur une fidelite est **B4** — « a partir de quel rapport la decomposition additive devient-elle
visiblement fausse ? », protocole avec « perception en double aveugle », et sa valeur de depart
0,35*Hs est explicitement provisoire. B4 est concu pour produire exactement le type de nombre
qu'A214 reclame, et il est bloque : il demande la reference substitutive integrale, qui n'existe
pas.

Ce que l'inventaire ne trouve pas, et il faut le dire : **aucun consommateur n'a de seuil declare
que l'erreur de recurrence pourrait franchir.** Le declencheur d'ecume a bien un seuil chiffre —
cambrure de Stokes 1/7, SPEC-001 §3 — mais il lit une borne, donc il est immunise ; et les pentes
du sillage mesure (5,6e-3) sont de toute facon a deux ordres de grandeur du seuil.
