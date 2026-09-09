# ADR-086 — Admission dynamique des sources de pression

- **Statut : actée**, S130, 2026-09-10, autonomie technique S71.
- **Prolonge :** journal de pression ADR-075, contrôleur de publication ADR-078.
- **Résout :** S129-1 — l'admission pendant la vie d'une publication.

## Problème

Le contrôleur d'ADR-078 emprunte un journal figé. Changer l'admission oblige à le libérer,
donc à perdre la publication en cours et à tout reconstruire. ADR-078 l'avait écrit comme une
limite assumée, en ajoutant la contrainte que devrait respecter la suite : « une future
transaction d'admission ne devra pas présenter l'ancien champ comme représentant le journal
modifié ».

C'est exactement le risque. Admettre une source change ce que le champ *devrait* être ; si le
recalcul échoue, ou s'il n'a pas lieu, l'appelant tient un champ qui ne correspond plus au
journal, sans que rien ne le lui dise.

## Décision

**1. Le contrôleur emprunte le journal mutablement.** `Controller::new` prend
`&'v mut Journal`. Ce n'est pas un détail d'implémentation : c'est ce qui rend impossible, et
non simplement improbable, qu'un tiers modifie le journal pendant qu'une publication en dépend.
L'invariant « jamais `Unchanged` sur un journal différent » est alors tenu par le compilateur.

**2. `admit(source)` est une transaction**, dont l'issue est l'une des trois suivantes, et
jamais un état intermédiaire :

| issue | journal | champ publié |
|---|---|---|
| la source était déjà là, à l'octet près | inchangé | **conservé**, il reste exact |
| admission et recalcul réussissent | source ajoutée | **republié** au même instant |
| refus, à n'importe quelle étape | **rendu à son état antérieur** | conservé |

Le troisième cas est celui qui demande du travail. `admit_authenticated` peut réussir puis le
recalcul échouer — pool insuffisant, débordement numérique, contexte incompatible. Le journal
contiendrait alors une source que le champ ignore. **Le contrôleur revient en arrière** : la
source est retirée à la position exacte où elle venait d'être insérée.

**3. Le retour en arrière n'ouvre pas de retrait public.** `Journal` reçoit une opération
`pub(crate)` qui défait la dernière insertion, à sa position connue — pas une suppression
arbitraire par identifiant. Un retrait libre demanderait de décider ce que deviennent les
publications qui référencent la source, question que cette décision n'ouvre pas.

**4. La saturation est un état terminal pour ce contrôleur, et c'est dit.** Quand le journal
est plein, `admit_authenticated` conserve la source en attente et rend `Full`. Or
`from_journal` refuse tout journal dont l'attente est non vide : à partir de là, le contrôleur
**ne peut plus changer d'instant non plus**, seulement continuer à servir sa publication
courante. Ce n'est pas un défaut à corriger ici : c'est la conséquence cohérente d'une attente
non résolue, et la résolution passe par un pool élargi, donc par la libération du contrôleur.
La transaction le signale explicitement plutôt que de le laisser découvrir au premier `update`.

**5. `Unchanged` ne coûte rien, et reste exact.** Réadmettre une source identique ne déclenche
aucun recalcul : le journal n'a pas changé, donc le champ non plus. C'est le seul cas où une
admission ne coûte pas une préparation complète.

## Ce que cette décision ne fait pas

Elle ne résout pas la saturation : élargir le pool demande `copy_into` sur un stockage plus
grand, donc de libérer le contrôleur. Elle ne retire aucune source publiée, ne réordonne rien,
et ne touche ni aux formats WPRS/WPJR ni à l'époque.

Elle ne rend pas l'admission atomique **vis-à-vis d'un montage mixte** : la requête mixte
d'ADR-077 compose plusieurs vues, et rien ici ne coordonne l'admission de plusieurs couches.
La consigne de S129-1 demandait précisément de recevoir doublons, conflits et saturation sur le
chemin pression *avant* de revendiquer une transaction mixte ; c'est ce que fait cette décision,
et elle s'arrête là.

Aucun coût nouveau n'est certifié : une admission qui recalcule coûte une préparation, et cette
préparation est déjà mesurée depuis S118.

## Réception

[ADMISSION-PRESSION-S130](../validation/ADMISSION-PRESSION-S130.md). Les trois issues sont
exercées séparément, y compris le retour en arrière après un recalcul refusé — vérifié en
comparant le journal et le champ **avant et après** l'échec, et non seulement le code d'erreur.
