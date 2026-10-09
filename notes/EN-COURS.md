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

Session : S715 — **terminée**. En autonomie, sans arrêt ; session longue. Le LOD reprend (LOD-ETAPE-2-S705, N2). S707 : renée depuis le
compte de ses particules, la 3D perdait 6 % d'amplitude, car APIC tasse ses particules sous la crête et le compte n'est pas la surface.
**La renaissance par la surface.**

**Ce que la session fait.** Le montage de l'onde plate (S707, 10 m, 1,6 s, la même fonction) :
- **`Renaissance::DepuisLaSurface`** : à 0,4 s, chaque colonne est réduite à la hauteur de sa surface reconstruite (φ). Les particules
  sont reposées à la densité nominale jusqu'à elle, avec le profil vertical de SGN tiré de `ū` lissé, comme en S707. Le compte des
  particules change (le tassement disparaît), le volume que voit la 3D est gardé ;
- **la lecture** se fait par la surface (ADR-280 D1) : le profil de la surface reconstruite, moyen sur les rangées. On en tire la
  comparaison intégrale de S707 (le décalage, le facteur, l'écart), contre la 3D ininterrompue.

**Les essais, dans l'ordre ; chacun a ses critères, écrits avant lui.**

| essai | ce qu'il juge | critères |
|---|---|---|
| **E1** | la renaissance par la surface, contre la 3D ininterrompue | à 0,4 s (juste après), le plancher : décalage sous 1 cm, facteur à 0,5 %, `V_φ` continu à 0,3 % ; à 1,0 et 1,6 s : décalage sous **5 cm**, facteur à **2 %** |
| **E2** (si E1 tient) | la naissance depuis SGN (l'état de SGN à 0,4 s), lue par la surface | rapporté et attribué (S704 : SGN garde 0,150 m, la 3D se pose plus bas) |

**Contrôles du plan** (ADR-266, ADR-267, ADR-268, ADR-273, ADR-276, ADR-277, ADR-280)

- **témoin** : la 3D ininterrompue, par la même fonction ; S707 E2 (par le compte : facteur 0,935 à 1,0 s).
- **instrument** : le profil de la surface reconstruite, étalonné en S708 (3·10⁻⁴), et la comparaison intégrale. Son plancher est mesuré
  à 0,4 s (ADR-280 D1). Ce que rendrait chaque hypothèse :
  - le tassement était la cause de S707 : E1 tient ;
  - la perte de la structure verticale (le profil de SGN) compte aussi : un écart demeure.
- **calcul** : ≈ 3 min par passage, deux passages.
- **ADR**, et comment chacun est tenu (ADR-277 D1) :
  - ADR-273 D1 : entre deux copies de la 3D d'abord (E1), puis depuis SGN (E2) ;
  - ADR-280 D1 : la surface pour la renaissance et pour la lecture ;
  - ADR-276 D2 : E1 ne change, contre S707 E2, que la hauteur de la réduction.
- **pièges** :
  - la masse se compte désormais en volume de surface, non en particules ;
  - la hauteur par colonne et par rangée, non moyennée sur les rangées, pour la repose.

**Critères de la session.** E1 tenu, ou son échec nommé ; E2 rapporté.

### Plan

- [x] **P1** — jeton ; plan.
- [x] **P2** — E1.
- [x] **P3** — E2.
- [x] **P4** — preuve ; rituel.

### Notes de reprise
- **P2 fini (E1)** — **tenu** : décalage 1,1 / −2,9 / −0,5 cm, facteur 0,996 / 0,992 / 0,999 aux photos de 0,6, 1,0 et 1,6 s (S707 par le compte : 0,935 à 1,0 s). Particules 271 952 → 270 652. `V_φ` −0,07 % à 0,6 s, −0,31 % à 1,6 s. **Réserve** : la photo de 0,4 s lit un φ reconstruit avant la renaissance (φ ne se refait qu'au pas suivant) ; son « plancher » était trivial, et la mesure vaut à partir de 0,6 s.
- **P3 fini (E2)** — née de SGN : la phase juste (sous 4 cm), le facteur 1,098 → 1,065 → 1,040 ; la crête de SGN se repose vers celle de la 3D. N2 acquis.
