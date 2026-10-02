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
- **Ce fichier ne porte que la session en cours** ([ADR-187](../docs/adr/ADR-187-methode-refondue-s321.md)
  D3). À la clôture, ce qui doit survivre des notes va à la preuve ou au journal ; la session
  suivante remplace ensuite toute la section. Aucune section d'archive, 300 lignes au plus :
  `outils/etat_projet.py --check` le vérifie. Notes de S301 à S320 : `git show 78622a19:notes/EN-COURS.md`.

---

## Session en cours

Session : S438 — **en cours**. Demande de l'utilisateur (2026-10-02) : *« Continue »* — la suite déclarée : **C7d-3a**, localiser le
défaut d'A320 près de la surface ([preuve](../docs/validation/MER-S369.md) §9).

**Ce que la session trouve en entrant.** À 25 cm, sous la houle de 4 m (16 mailles par longueur d'onde), le taux dépend de la place
du repos dans la maille (0,106 sur une face, 0,036 au centre, sous 7,5 cm) ; sous la houle de 8 m (32 mailles), rien au-delà de
Benjamin-Feir. **Une hypothèse écartée avant tout code** : une quasi-résonance de triades, ouverte par l'écart entre la dispersion
discrète de δ et celle, exacte, de B — en eau profonde, les triades sont loin de résonner (un écart d'ordre `Ω/2`) ; quelques pourcents
de dispersion n'y suffisent pas. **L'hypothèse retenue** : le taux de Benjamin-Feir passe par la réponse liée du second ordre — l'onde
`2K`, collée à la surface (`e^{2Kz}` : 32 cm sous la houle de 4 m, **à peine plus d'une maille de 25 cm**) ; mal résolue, elle fausse
le coefficient cubique, et sa valeur dépend de la place de la surface dans la maille. Si c'est cela, **l'excès est une fonction du seul
nombre de mailles par longueur d'onde de la houle**, `λ_B/dx`.

**Ce que la session fait.** L'épreuve de cette échelle, sans code nouveau : la houle de **8 m à 50 cm** et celle de **2 m à 12,5 cm**
(16 mailles par longueur d'onde, comme 4 m à 25 cm), à la même cambrure (`ak` = 0,118), repos sur une face et au centre ; germe à la
longueur d'onde de la houle.

**Critères, écrits avant.** **L'échelle est confirmée** si, aux deux nouvelles paires (16 mailles), le taux sur une face est au moins
2,5 fois Benjamin-Feir **et** au moins deux fois celui du centre — comme à 4 m et 25 cm. **Réfutée** si l'une des deux paires reste sous
1,5 fois Benjamin-Feir aux deux places. Autrement, indécis. Confirmée, la suite proposée est un remède de production : ne coupler δ qu'aux
composantes de B qu'il résout (`λ ≥ 32·dx`), les plus courtes restant portées par B seul — à concevoir et à éprouver, non à décider ici.

### Plan

- [x] **P1** — jeton, plan seul.
- [x] **P2** — mesures : 8 m à 50 cm, 2 m à 12,5 cm ; deux places ; critères.
- [ ] **P3** — preuve ; A320 ; registres ; suite.
- [ ] **P4** — rituel.

### Notes de reprise
- **P2** — `ak` = 0,118, germe de 1 mm à `2K`, masque 7, 95 s ; taux de la bande 35–59 s (et, entre parenthèses, la phase linéaire) :
  **8 m à 50 cm** (16 mailles) : face **0,042** (2,2 fois Benjamin-Feir, 0,0193 ; 0,059 en fin), centre 0,010 (0,5 fois) ;
  **2 m à 12,5 cm** (16 mailles) : face 0,073 — fenêtre saturée, le domaine refuse à 55 s (phase linéaire 10–30 s : **0,207**, 5,4 fois
  Benjamin-Feir, 0,0386), centre **0,074** (1,9 fois) ; **8 m à 25 cm** (32 mailles), germe à `2K` : 0,0085 · 0,010 (sous
  Benjamin-Feir). **Verdict, tel qu'écrit : indécis** — ni « au moins 2,5 fois sur une face » à la fenêtre écrite (2,2 ; saturée), ni la
  réfutation (une paire sous 1,5 fois aux deux places). Le motif « face ≫ centre » tient aux trois paires à 16 mailles (rapport 2,8 à
  4,2) ; mais l'échelle en `λ_B/dx` n'est pas propre : la houle de 4 m à 12,5 cm (32 mailles, sur une face, 6 cm, S436) montait à
  3,1 fois. La cause n'est pas trouvée en une session : la règle déclarée en S437 s'applique — le constat s'écrit, C7d-3b vient.

