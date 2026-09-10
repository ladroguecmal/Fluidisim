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

Session : S143 — terminée
Agent : Claude Code (Opus 5 ; fichiers, git et cargo disponibles)
Objectif : **A210**. Le crate porte deux constantes homonymes qui ne se déduisent pas l'une de
l'autre, et le contrat « ce qui est comparé à `max_slope` est une pente réelle » ne vit que dans
deux commentaires et deux essais. Un troisième champ pourrait écrire
`slope > medium.max_slope` sans que rien ne l'arrête — c'est ce que les deux premiers ont fait
pendant soixante sessions. Défaut de **dispositif**, pas de calcul.

### Plan

- [x] **P1** — état réel, jeton, **plan déclaré et committé seul**.
- [x] **P2** — peser les trois réparations **avant** d'en écrire une, et écrire la pesée. Type
      porteur, essai générique, entrée d'invariant : coût, ce que chacune attrape, ce qu'elle
      laisse passer. La question qui décide est *qu'est-ce qui aurait arrêté le défaut de S141*,
      pas *qu'est-ce qui est le plus propre*.
- [x] **P3** — construire ce que la pesée retient. Étape courte : si la réparation ne tient pas
      en un quart d'heure, c'est qu'elle est plus lourde que le défaut.
- [x] **P4** — **vérifier qu'elle attrape le défaut** : réintroduire la faute de S141 dans une
      copie de travail jetable et constater l'échec. Une garde qu'on n'a pas vue échouer ne
      garde rien — c'est la moitié qu'ADR-082 vérifie pour chaque nom de refus.
- [x] **P5** — ADR et livrable ; porter l'invariant s'il y en a un.
- [x] **P6** — rituel de fin (§6), jeton rendu, fusion `--ff-only`.

### Notes de reprise

Départ 66c192f = master ; worktree `886155`. 273 tests/cinq ignorés.

Les trois voies, telles que S142 les a laissées :
- **type porteur** — un `RealSlope(f32)` au lieu d'un `f32` nu ; le plus solide, le plus
  intrusif : il traverse `Medium`, `composition`, `mixed_water`, `bound_pressure` ;
- **essai générique** — praticable, mais **il faut un trait commun aux champs, qui n'existe
  pas**. Les deux `sample` ont pourtant la même signature : à vérifier avant de conclure ;
- **entrée d'invariant** — la moins chère, la plus oubliable.

Ce que S142 a noté et qui oriente : **I-14 a tenu soixante sessions parce qu'un essai le
vérifiait**, pas parce qu'il était écrit. Une entrée d'invariant seule ne suffira pas.

Piège à éviter : construire le plus beau des trois. Le défaut réel est qu'une **troisième
implémentation** de la même comparaison puisse naître sans mesurer son rapport. La bonne question
n'est pas « comment exprimer le contrat » mais « qu'est-ce qui échouerait le jour où quelqu'un
l'oublie ».

Second piège : un trait commun inventé pour l'occasion, que rien d'autre n'utilise, est une
surface publique sans lecteur — exactement ce qu'ADR-082 refuse. S'il n'a qu'un usage, l'écrire
côté essais plutôt que côté bibliothèque.

P2-P5 : pesée écrite avant d'écrire du code, deux gardes, vérification par réintroduction des
fautes, retrait des deux essais dupliqués, ADR-097, invariant I-18, CONTRAT-PENTE-S143, L226.
273 tests/cinq ignorés — le même compte qu'à l'entrée.

Ce que la pesée a retourné, et qui vaut pour la prochaine fois : **le type porteur ne garde rien**
dès lors qu'un acteur légitime — ici l'hôte — doit pouvoir construire la valeur. Le constructeur
public est la porte, et l'auteur pressé la prend. Ne pas y revenir sans un argument neuf.

Manquement de la session, attrapé par le compte de tests et non par la relecture : le livrable
annonçait « les deux essais spécifiques sont remplacés » alors qu'ils étaient toujours là — 275 au
lieu de 273. Corrigé en P4b. Vérifier le **décompte** après toute annonce de remplacement.

Pour S144 sans relire : A208 dit que le refus rendu quand l'emprise consomme le budget désigne la
pente — `Slope` ou `Steepness` — alors que ce que l'hôte peut changer est **l'emprise** publiée
(ENVELOPPE-PRESSION-S140 §4, facteur ×10,9 mesuré, non borné). ADR-082 exige qu'un nom de refus
désigne ce qu'il faut revoir ; celui-ci désigne le contraire. Deux réparations évoquées et aucune
tranchée : nommer le cas (`Footprint`), ou publier le rapport des deux enveloppes pour que
l'appelant voie sa propre marge. Attention : `bound_pressure::Prepared` retient déjà la borne
resserrée ; publier aussi la L1 rendrait le rapport calculable sans rien mesurer.
