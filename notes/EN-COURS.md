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

Session : S134 — terminée
Agent : Claude Code (Opus 5 ; fichiers, git et cargo disponibles)
Objectif : S133-1 — trois sessions ont buté sur la même limite : l'ajout incrémental n'est
exact que si la source s'insère **en dernier**. Établir ce que coûterait de s'en affranchir,
et décider — y compris décider de la garder.

### Plan

- [x] **P1** — état réel, jeton, plan seul.
- [x] **P2** — poser les voies et les chiffrer avant d'en préférer une. Quatre, et la
      quatrième n'était pas dans les notes de S133 :
      1. **accumuler en `f64`** puis arrondir : réduit l'écart sans le supprimer — l'ordre
         compte toujours, plus finement. Et cela déplacerait tous les résultats publiés.
      2. **sommation compensée** par nœud : même nature, même défaut.
      3. **sommation exacte** : indépendante de l'ordre par construction, coût à mesurer.
      4. **stocker la contribution de chaque source séparément**, et recomposer dans l'ordre
         canonique à chaque admission. L'identité devient exacte **quelle que soit la
         position**, sans refaire les réponses modales — au prix d'une mémoire proportionnelle
         au nombre de sources. C'est la seule voie qui rende la condition inutile.
- [x] **P3** — ADR-090 : la condition reste, faute d'un prix acceptable pour la lever,
      et devient une contrainte d'usage écrite.
- [x] **P4** — contrainte portée dans la doc de `admit` et `extend_into`.
- [x] **P5** — sonde conservée comme test, borne large, commutativité à deux termes figée.
- [x] **P6** — livrable, rituel de fin, fusion `--ff-only`.

### Notes de reprise

Départ 8269d2f = master, trois copies coïncidentes.

Ce que la condition coûte aujourd'hui, et qu'il faut peser : rien tant que les identifiants
croissent — un compteur d'hôte suffit — et une préparation complète sinon. Ce n'est donc pas
une faute de calcul, c'est une **contrainte d'usage non écrite**. La question n'est pas
« comment la lever » mais « vaut-elle son prix, et l'hôte sait-il qu'elle existe ».

Chiffres de référence : préparation 5,00 ms par segment à 224×128 ; pools de coefficients
14 336 slots, environ 630 ko chacun ; le contrôleur en tient deux, et ADR-089 en demande deux
de plus le temps d'une transition.

Piège à éviter : mesurer le coût en temps de la voie 4 et oublier son coût en mémoire, qui est
le vrai. À 8 sources, elle multiplierait par 8 des pools déjà comptés en mégaoctets.

P2-P6 : ADR-090, ORDRE-S134, journal, index, README, REPRISE, jeton rendu, ff-only.
264 tests/cinq ignorés. Aucun code de calcul modifié — la session refuse de construire.

Le piège de la session, et il aurait été coûteux : ma première sonde mesurait la sensibilité à
l'ordre sur des valeurs **synthétiques**, amplitudes réparties sur six décades. Elle donnait
1,5e-2 d'écart relatif à 64 termes, ce qui aurait fait conclure à un défaut de justesse et
justifié de renouveler toutes les références du projet. Sur les vraies contributions modales,
l'écart est de 5,6e-7 à 7,1e-6. **Une sonde synthétique mesure le régime qu'on lui donne.**

Pour S135 sans relire : S134-1 est la transaction mixte. ADR-086 §"Ce que cette décision ne fait
pas" dit exactement où elle s'arrête — l'admission n'est pas coordonnée entre couches. Le montage
mixte compose B, impacts et pression (ADR-077) ; admettre une source de pression pendant qu'une
requête mixte est en cours n'a pas de sémantique définie. Commencer par établir ce qui est
observable : les emprunts Rust interdisent-ils déjà le cas problématique, comme ils l'ont fait
pour `Unchanged` en S130 ?
