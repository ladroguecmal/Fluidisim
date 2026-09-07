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

```
Session          : S49
État             : terminée
Agent            : Codex (git et cargo disponibles)
Objectif         : Raffiner les fenêtres C22 shallow avec réutilisation des oracles (S48-1).
```

### Plan

- [x] **P1** — reprise, copies, jeton et plan seul.
- [x] **P2** — fenêtres prédéclarées 100–1600, 200–3200, 400–6400 ; réutilisation en mémoire des oracles et mesures ; tests du filtrage.
- [x] **P3** — mesurer avec deux oracles suffisamment fins, vérifier stabilité et coût, documenter les limites et tester.
- [x] **P4** — rituel de fin : journal, leçons, actions, index, décomptes, passation et jeton libre.

### Notes de reprise

Départ master 8395ca0 propre. Copies anciennes contrôlées ; aucun travail concurrent détecté.
Socle lu dans cette conversation ; S48 et son registre de mesures relus. Aucun seuil changé.
Les champs restent en mémoire, sans sérialisation (I17). Toutes les fenêtres seront rapportées.

P2 : mode c22-shallow-fenetres, trois fenêtres fixes, sept champs et deux oracles calculés une fois en mémoire. Trois tests ciblés verts, compilation release réussie.

P3 : oracles 51200/102400, écart 5,207593646e-10 ; sept grilles séparées.
Fenêtre 400–6400 : p=1,849839 / 1,960632 / 2,011665, toujours non stable car
4*d1>d0. Trois fenêtres sans verdict, 382,716 s sans tests concurrents. 105 tests
réussis, deux ignorés. Refus CLI et mode historique vérifiés. MESURES-C22-S49 écrit.

P4 : journal, L171, aucun nouvel angle mort (175), S48-1 close, S49-1 ouverte.
Index et passation à jour : suite S50 S44-1. Décomptes inchangés : 47 ADR, 17 invariants,
6 SPEC, 14 registres, 23 cas. Jeton libre.
