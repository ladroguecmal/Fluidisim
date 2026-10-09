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

Session : S727 — **terminée**. En autonomie, sans arrêt (l'utilisateur dort) ; session longue. LOD-ETAPE-3-S722, **B4a : un corps dans la
boîte**. La boîte est fixe ; une sphère la traverse. Son déplacement avec le corps (B4b) vient ensuite.

**L'essai.** Une sphère de rayon 8 cm, son centre à la surface (z = 0,4 m, à demi immergée), tirée à 0,3 m/s selon x pendant 1,5 s :
- **le témoin** : un APIC entier de 2 m × 2 m (80 × 80 × 24, ≈ 820 000 particules), aux murs fermés ;
- **la boîte** : un APIC de 1 m × 1 m au milieu, dans un Saint-Venant de 2 m × 2 m troué au même endroit, aux mêmes murs.

Les deux ont les mêmes murs : les vagues qui y rebondissent sont les mêmes. Seul le dehors de la boîte change, la 3D contre Saint-Venant.

**Critères, écrits avant** (ADR-283 D1 : les vagues d'un corps sont courtes, et Saint-Venant les porte mal ; on juge ce qui compte en jeu,
la 3D près du corps) :
1. la force sur la sphère : son écart moyen au témoin, sur 0,2 à 1,5 s, sous **10 %** de sa moyenne ;
2. la surface dans la boîte à 1,0 s : l'écart quadratique moyen au témoin sous **20 %** de la plus haute vague du témoin ;
3. la masse à 10⁻¹².

**Contrôles du plan** (ADR-266, ADR-267, ADR-268, ADR-273, ADR-276, ADR-277, ADR-280, ADR-281, ADR-283)

- **témoin** : la même 3D, entière (ADR-273 D1). Les murs sont les mêmes ; la boîte et ses particules aussi, au même réseau.
- **instrument** :
  - la force sur le corps (`body_force`), à chaque pas ;
  - la surface par colonne (φ), dans la boîte ;
  - la masse.
  Tous peuvent échouer (ADR-281 D2). Ce que rendrait chaque hypothèse :
  - si le raccord renvoie peu des vagues courtes, la force et la surface sont près du témoin ;
  - s'il les réfléchit, la surface dans la boîte s'en écarte, la force moins.
- **calcul** :
  - le témoin : ≈ 820 000 particules, ≈ 17 min pour 1,5 s ;
  - la boîte : ≈ 205 000 particules, ≈ 4 min.
- **ADR**, et comment chacun est tenu (ADR-277 D1) :
  - ADR-283 D1 : l'état de départ de Saint-Venant est le niveau que lit la boîte ; le régime est celui du jeu, et la limite est nommée ;
  - ADR-276 D1 : la même construction pour les deux, la sphère et le réseau ;
  - ADR-282 : la fermeture par l'outil.
- **pièges** :
  - le repère : la boîte commence à (0,5 ; 0,5) m dans le témoin ;
  - les particules hors de la sphère au départ ;
  - le même pas pour les deux (le plus petit).

### Plan

- [x] **P1** — jeton ; plan.
- [x] **P2** — l'essai ; (1)–(3).
- [x] **P3** — preuve ; fermeture.

### Notes de reprise
- **P2 fini** — **tenu** : la force à 2,0 %, la surface à 1,7 %, la masse 3,9·10⁻¹⁵. B4a acquis.
