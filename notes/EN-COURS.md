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

Session : S144 — en cours
Agent : Claude Code (Opus 5 ; fichiers, git et cargo disponibles)
Objectif : **A208**, ouverte depuis S140 et **trois fois reportée**. À champ identique, l'emprise
publiée décide de la part de budget de pente consommée — ×10,9 mesuré, sans borne — et le refus
rendu, `Slope` ou `Steepness`, désigne la **pente**, c'est-à-dire la seule chose que l'hôte n'a
pas à changer. ADR-082 exige qu'un nom de refus désigne ce qu'il faut revoir.

### Plan

- [x] **P1** — état réel, jeton, **plan déclaré et committé seul**.
- [x] **P2** — établir ce qui est **calculable au moment du refus**, avant d'imaginer un nom.
      La question décisive : la bibliothèque peut-elle distinguer « champ vraiment raide » de
      « emprise étroite » ? S140 dit que la pente réelle d'une pression ne se calcule pas, elle
      se cherche — si c'est vrai ici, un refus `Footprint` serait un nom qu'on ne peut pas
      justifier, et ADR-082 refuse autant un nom faux qu'un nom vague.
- [ ] **P3** — peser les réparations à la lumière de P2, et écrire la pesée. Nommer le cas ;
      publier de quoi calculer sa marge ; rendre la marge observable sur les points de
      l'appelant. Le critère reste celui de S143 : **qu'est-ce qui aurait aidé quelqu'un qui
      se fait refuser sans comprendre**.
- [ ] **P4** — construire ce que la pesée retient, étage court, tests verts.
- [ ] **P5** — **vérifier que ça sert** : reprendre le cas mesuré en S140 — emprise de 0,01 λ
      posée sur un zéro, facteur ×10,9 — et montrer que l'appelant voit désormais ce qui le
      fait refuser. Une aide qu'on n'a pas vue aider ne vaut pas mieux qu'une garde qu'on n'a
      pas vue échouer (S143).
- [ ] **P6** — ADR, livrable, rituel de fin (§6), jeton rendu, fusion `--ff-only`.

### Notes de reprise

Départ 597eca7 = master ; worktree `886155`. 273 tests/cinq ignorés, 18 invariants.

Ce qui est établi et n'est pas à remesurer (ENVELOPPE-PRESSION-S140) :
- le conservatisme de la pression se décompose en un facteur de **forme**, borné par 2 et
  **déjà retiré** depuis S141 par `slope_envelope_tight()`, et un facteur d'**alignement** que
  rien ne borne, qui dépend de l'emprise choisie par l'hôte ;
- à emprise large le facteur d'alignement vaut 1,0000 sur une case ; à 0,01 λ posée sur un zéro
  du champ, il vaut 10,89 et continue de croître ;
- sur un spectre gaussien réaliste il vaut 1,8522, stable en résolution.

`bound_pressure::Prepared` retient déjà la borne resserrée et l'expose par `slope_envelope()`.
La L1 reste disponible sur `Field::slope_envelope()`. Le rapport des deux ne dit que la
**forme** — déjà retirée : le publier n'apprendrait donc rien sur l'alignement, qui est le sujet
d'A208. À vérifier en P2 plutôt qu'à supposer, mais c'est la piste qui rend la réparation (b)
douteuse.

Piège à éviter : inventer `Footprint` parce que le nom est joli. Un refus doit être **décidable**
au moment où il est rendu ; si la bibliothèque ne peut pas distinguer les deux causes, le nom
ment, et ADR-082 refuse un nom qui ment autant qu'un nom vague.

Second piège : trois reports ont déjà eu lieu. Si la conclusion honnête est « rien à faire ici »,
la dire et **fermer** A208 avec son motif, plutôt que la reporter une quatrième fois.
