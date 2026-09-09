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

Session : S132 — terminée
Agent : Claude Code (Opus 5 ; fichiers, git et cargo disponibles)
Objectif : S131-1 — une admission recalcule tout le champ alors que la superposition modale
est linéaire. Mesurer si un chemin incrémental est **exact** et ce qu'il rapporte, **puis**
décider. Renoncer est une issue légitime.

### Plan

- [x] **P1** — état réel, jeton, plan seul.
- [x] **P2** — mesurer avant de décider (L209, L210), sans toucher la production :
      1. l'identité en bits, selon que la source s'insère **en fin** ou **au milieu** de
         l'ordre canonique — l'accumulation se fait par nœud, segment après segment, donc
         seule la première position préserve l'ordre d'addition ;
      2. ce que l'incrémental ne peut pas reprendre en l'état : la **pression modale cumulée**
         n'est pas stockée dans le `Slot`, or la puissance en dépend ;
      3. le gain réel, qui vaut au mieux le rapport du nombre de sources.
- [x] **P3** — ADR-088 : incremental si et seulement si insertion finale, resultat toujours celui de la voie directe.
- [x] **P4** — pression cumulée dans le `Slot`, `add_segments`, branchement conditionnel dans `admit`.
- [x] **P5** — champ identique à la voie directe dans les deux ordres d'arrivée ; test vérifié comme témoin.
- [x] **P6** — 5,03 ms au lieu de 35,06 à sept segments publiés ; recopie 28,5 µs.
- [x] **P7** — livrable, rituel de fin, fusion `--ff-only`.

### Notes de reprise

Départ 357b052 = master, trois copies coïncidentes.

Ce que la lecture de `prepare_segments` établit déjà :

- L'accumulation est **par nœud, boucle sur les segments** dans l'ordre de l'itérateur, avec
  `+=` en f32. Une source insérée en dernier laisse donc l'ordre d'addition inchangé ;
  au milieu, il change, et l'identité en bits n'est plus acquise.
- L'énergie et la puissance sont sommées en Kahan **sur les nœuds** : reparcourir les nœuds
  dans le même ordre les reproduit à l'identique.
- L'énergie ne dépend que de `total` (la réponse cumulée), qui est stockée dans le `Slot`.
  **La puissance dépend aussi de la pression modale cumulée, qui ne l'est pas.** La reprendre
  exigerait de refaire `ModalPressure::new` pour chaque segment — c'est-à-dire l'essentiel du
  coût — ou d'agrandir le `Slot` de deux `f32`.
- `phase_safe` et l'enveloppe ne dépendent que des nœuds et des slots : inchangés.

Piège à éviter : conclure sur le gain sans compter ce que la structure grossit ni ce que
l'identité conditionnelle impose à l'appelant. Le consommateur — un hôte qui admet une source
en cours de jeu — n'est pas une boucle par trame ; 12,2 ms occasionnels ne sont peut-être pas
un problème à résoudre.

P2 : deux mesures, et la premiere version de la sonde etait trop faible. Avec **deux** segments,
l addition f32 est commutative : la sonde concluait que l ordre n a aucune importance. Avec
trois, le verdict est net — insertion en fin : identique en bits, 0 point sur 8 different ;
au milieu : les 8 points different.
Cout de la preparation lineaire en segments : x1,98 a n=2, x3,91 a n=4, x7,12 a n=8 (224x128),
soit 4,85 ms par segment. Un incremental ramenerait toute admission au cout de n=1.
Decision : incremental **si et seulement si** la source s insere en dernier, sinon recalcul.
Le resultat est alors toujours celui de la voie directe — sans quoi deux hotes ayant admis les
memes sources dans un ordre different auraient des champs differents, et I-03 ne survivrait pas.
Prix : le Slot doit porter la pression modale cumulee (+18 % sur les pools) car la puissance en
depend et la reconstituer couterait ce qu on cherche a eviter.

P4-P7 : ADR-088, INCREMENTAL-S132, journal, index, README, REPRISE, jeton rendu, ff-only.
262 tests/cinq ignorés, hachages inchangés. Aucun angle ni leçon nouveaux — la session applique
L211 (témoin) et la discipline de mesure de L209/L210.

Deux écueils évités, à retenir :
1. J'ai failli ajouter deux méthodes publiques au contrôleur pour instrumenter un banc. Retiré :
   les chiffres qui décidaient étaient déjà accessibles par l'API publique (`Prepared::build` sur
   des chemins de 1 à 8 segments). Ne pas élargir une API pour une mesure.
2. `prepare` exige un chemin **contigu** ; une suite de segments identiques est refusée. La sonde
   de coût mesure donc une trajectoire réelle découpée.

Pour S133 sans relire : S132-1 veut faire repartir la reconstruction d'ADR-087 des coefficients
déjà calculés. Il faudrait transporter les slots d'un pool à l'autre (`copy_from_slice` suffit,
les pools ont la même taille si la recette est la même) puis appeler `add_segments` pour la seule
source reprise. Condition d'ordre : la source en attente doit avoir l'identifiant le plus grand,
ce qui n'est **pas** garanti — le vérifier avant, et retomber sur la reconstruction complète
sinon, exactement comme `admit` le fait déjà.
