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

Session : S161 — terminée
Agent : Claude Code (Opus 5 ; fichiers, git et cargo disponibles)
Objectif : **débloquer B4**, seule voie ouverte selon ADR-109, et qu'A214 attend désormais seule.
B4 est **le juge de l'architecture** : « à partir de quel rapport `|δ|/Hs` la décomposition
additive d'ADR-001 devient-elle visiblement fausse ? ». Il exige une **référence substitutive
intégrale** que le dépôt n'a pas, et c'est ce blocage-là qu'il faut instruire — pas le contourner.

### Plan

- [x] **P1** — état réel, jeton, **plan déclaré et committé seul**.
- [x] **P2** — **dire ce qui bloque exactement.** B4 compare trois choses — surface, forces sur la
      coque, perception en double aveugle — et une seule est hors de portée d'une session. Le
      blocage annoncé est la référence intégrale : établir ce que le dépôt possède déjà
      (`shallow.rs`, `dispersif.rs`, l'oracle croisé de S37) et ce qui manque vraiment.
- [x] **P3** — **peser une version 1D avant de l'écrire.** Un contre-exemple 1D suffirait à
      infirmer l'additivité ; son absence ne la validerait pas. Dire lequel des deux verdicts un
      B4 en 1D pourrait rendre — et si c'est encore B4 ou un banc différent qui mérite son nom.
- [x] **P4** — si la voie tient : mesurer. Superposition linéaire contre solution non linéaire sur
      la même scène, en balayant le rapport d'amplitude, jusqu'à trouver la bascule ou montrer
      qu'elle n'apparaît pas dans la plage accessible.
- [x] **P5** — verdict, livrable, et ADR **seulement** si une décision en sort. La valeur de départ
      d'ADR-001 est 0,35·Hs : la confirmer, la déplacer ou la laisser est une décision.
- [x] **P6** — rituel de fin (§6, sept points), jeton rendu. Copie principale : rien à refermer.

### Notes de reprise

Départ fb73282 = master, copie principale, arbre propre. 299 tests/cinq ignorés.
110 ADR, 215 angles, 242 leçons, 18 invariants, 6 SPEC, 23 cas.

Ce que le protocole demande, mot pour mot (`PLAN-BENCHMARK` §B4) : même scène simulée deux fois,
(a) perturbative B+W+δ, (b) **substitutive intégrale de référence à résolution élevée** ; puis
comparaison de la surface, des forces sur la coque, et de la perception en double aveugle. Décision
attendue : seuil de bascule, valeur de départ **0,35·Hs**, ou remise en cause d'ADR-001.
**Ajout S04** : avant toute conclusion sur l'architecture, écarter le terme source incomplet
(A50) — une exécution avec `S` non dégradé, une avec `S` tronqué, l'écart mesure la sensibilité.

Piège nommé par S146, et il vaut ici plus qu'ailleurs : **un banc dont deux volets sont hors de
portée reste utile s'il dit lesquels.** Le contraire — annoncer « B4 débloqué » sans réserve —
ferait porter au corpus un renvoi faux sur le banc qui juge l'architecture.

Second piège, propre à cette session : **écrire un solveur**. La tentation est réelle et elle
mangerait dix sessions. Ce qui est demandé est de dire ce qu'il faut, pas de le construire — et
de regarder d'abord si une référence suffisante existe déjà.

Troisième piège : confondre « infirmer ADR-001 » et « mesurer un seuil ». B4 doit pouvoir
l'infirmer ; une mesure qui ne peut rendre qu'un seuil n'est pas B4.

P2-P6 : B4-DEBLOCAGE-S161, sonde `additivite_b4.rs`, `configure_bosses`, ADR-111, note corrective
sur ADR-001 §3.3, A216, A217, L243. 299 tests inchangés.

Ce que la session a appris, et qui vaut pour la suivante : **le blocage hérité n'en était pas un.**
La référence existait depuis S36 ; il manquait trois lignes. Ouvrir un blocage coûte une lecture,
le supposer coûte des sessions.

Deux pièges évités, à ne pas réexplorer :
- `dispersif.rs` **ne peut pas** servir de référence : il est linéaire, la superposition y est
  vraie par construction, l'écart serait nul quel que soit le rapport ;
- le pas CFL dépend de la hauteur maximale, donc de la présence de la seconde perturbation : les
  trois simulations d'un même point n'avancent pas par la même suite de pas. Contrôlé à pas imposé,
  la part numérique vaut 0 à 2 % — l'artefact existe, il est négligeable ici.

Pour S162 sans relire : **A217** demande quelle variable gouverne l'additivité en eau profonde,
là où la profondeur ne joue plus. La réponse plausible est la **cambrure**, et rien ne le vérifie.
Le blocage annoncé est qu'aucun solveur du dépôt n'est à la fois **non linéaire et dispersif** —
`shallow.rs` est non linéaire mais non dispersif, `dispersif.rs` est dispersif mais linéaire.
**Commencer par vérifier ce blocage-là**, exactement comme S161 a vérifié le précédent : de quoi
a-t-on besoin au juste, et qu'est-ce qui existe déjà qui y ressemble ? Une réponse négative,
argumentée, est un résultat — elle dirait que ce volet de B4 attend une couche non écrite.
À défaut, **A216** est peu coûteuse : la sonde existe, il s'agit de balayer `A_B/h` entre 0,02 et
0,10 pour voir comment le coefficient passe de 0,95 à 0,24.
