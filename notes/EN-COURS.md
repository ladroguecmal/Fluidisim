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

Session : S722 — **en cours**. En autonomie, sans arrêt (l'utilisateur dort). L'étape 2 du LOD est faite : N1, N2, M1 et la vague de bout en
bout (S707–S720) ; R43 attend son verdict. Le déclencheur au plus simple (D1) est remis : sur les plages de référence, l'onde touche la
bande dès le départ, et il ne se juge pas utilement sans la décision « faut-il la 3D ? », que l'utilisateur a voulu remettre.
**La conception de l'étape 3 : la 3D rallumée autour d'un corps** (ADR-275 D2).

**Ce que la session fait.** Le registre `LOD-ETAPE-3-S722` : les pièces, une par session au plus, chacune avec son essai.

**Contrôles du plan** (ADR-266, ADR-267, ADR-268, ADR-273, ADR-277)

- **témoin** : sans objet (une conception).
- **instrument** : chaque pièce nomme son essai, et d'abord son essai entre deux copies du même solveur (ADR-273 D1).
- **calcul** : le coût d'une boîte de 3D de 1 m × 1 m, à 2,5 cm, sur 0,5 m d'eau : 40 × 40 × 20 mailles d'eau × 8 particules ≈ 256 000
  particules. C'est l'ordre du tout-3D de S690 : il est montré et mesuré dans la pièce qui l'emploie.
- **ADR**, et comment chacun est tenu (ADR-277 D1) :
  - ADR-275 D2 ;
  - ADR-273 D1, pièce par pièce ;
  - ADR-280 D1 : la surface à la naissance et à la mort ;
  - ADR-282 : la fermeture par l'outil.
- **pièges** : un raccord sur quatre côtés n'est pas deux raccords de plus. Les coins, où deux côtés se touchent, ont leur propre essai.

**Critères.** Le registre, chaque pièce avec son essai et son critère ; la note sur ADR-275.

### Plan

- [x] **P1** — jeton ; plan.
- [ ] **P2** — le registre ; la note.
- [ ] **P3** — la fermeture (`fermer.py`).

### Notes de reprise
