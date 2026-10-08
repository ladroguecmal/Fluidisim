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

Session : S709 — **en cours**. En autonomie, sans arrêt ; session longue (ADR-279 D3). S708 : à compte exact, APIC perd ≈ 1,3 %/s de
volume géométrique en mouvement. **Le remède : la projection de densité** (Kugelstadt et al. 2019), en option.

**Ce que la session fait.** `apic3d_densite.rs`, à la fin de chaque pas, après la séparation :
- la densité aux centres des mailles, par les poids trilinéaires ;
- le gradient conjugué de la pression, sorti de `project` tel quel (`pcg`), sur `Δq = max(ρ − 1, 0)`, avec la surface à `q = 0` ;
- chaque particule déplacée de `∇q`, borné à un quart de maille ; les vitesses ne changent pas.

`Large::AucunDensite` est le tout-3D avec la projection, un mode nommé (ADR-277 D2).

**Les essais, dans l'ordre ; chacun a ses critères, écrits avant lui.**

| essai | ce qu'il juge | critères |
|---|---|---|
| **E0** | le défaut, au bit : `pcg` est un déplacement de code | le banc de non-régression du rituel tenu (les empreintes, la scène `--v1`) |
| **E1** | le repos (le semis, 4 m, 1 s), avec la projection | la vitesse maximale au plus trois fois celle du témoin sans projection (6,8·10⁻⁶ m/s) ; `V_φ/V_n` constant à 10⁻³ |
| **E2** | l'onde plate (10 m, 1,6 s), avec la projection | `V_φ/V_φ(0)` à 1,6 s à moins de **0,3 %** (sans : −1,87 %) ; la crête par la surface rapportée |
| **E3** | le tout-3D de S690, avec la projection | `V_φ/V_n` à 2,5 s à moins de 0,3 % de sa valeur au départ (sans : −3,2 %) ; le retournement et l'air rapportés (le juge nouveau) ; le coût mesuré contre 777 s |

**Contrôles du plan** (ADR-266, ADR-267, ADR-268, ADR-276, ADR-277, ADR-279)

- **témoin** : S708, les mêmes essais sans la projection, par les mêmes fonctions.
- **instrument** : `V_φ`, étalonné en S708 E1 (3·10⁻⁴) ; le déplacement maximal par pas. Ce que rendrait chaque hypothèse :
  - le tassement est la cause de la perte de volume : avec la projection, `V_φ` tenu ;
  - une autre cause : `V_φ` dérive encore.
- **calcul** : aucun nombre neuf. Le coût : une résolution de plus par pas. On attend 20 à 40 % de plus, et on le mesure.
- **ADR**, et comment chacun est tenu (ADR-277 D1) :
  - ADR-279 D1, par E3 : le juge nouveau sera mesuré, non supposé ;
  - ADR-276 D2 : chaque essai ne change que la projection, contre S708 ;
  - ADR-277 D2 : `Large::AucunDensite`.
- **pièges** :
  - le tableau `p` de la pression est sauvé et rendu autour de la projection ;
  - les gouttes sont exclues ;
  - les étiquettes sont celles de la reconstruction du pas, avant l'advection (une demi-maille au plus) ;
  - le défaut doit rester au bit.

**Critères de la session.** E0 à E2 tenus ; E3 mesuré. Si E2 échoue, on cherche la cause et la session s'arrête quand elle est nommée.

### Plan

- [x] **P1** — jeton ; plan.
- [x] **P2** — E0, E1.
- [x] **P3** — E2.
- [ ] **P4** — E3.
- [ ] **P5** — preuve ; rituel.

### Notes de reprise
- **P2 fini** — E0 : le banc au bit (les empreintes inchangées). E1 : au repos, avec la projection, 1,5·10⁻⁵ m/s (sans 7,0·10⁻⁶), `V_φ/V_n` 0,99967 → 0,99969, déplacement max 2,2 µm. Tenus.
- **E2, premier passage : échoue par excès** — `V_φ/V_φ(0)` +7,4 % à 1,6 s, la crête par la surface à 0,185 m. Cause : la correction d'un seul côté (ρ > 1) sur une densité bruitée dilate l'eau à chaque pas. **Second passage**, les mêmes critères : à l'intérieur, `ρ − 1` dans les deux sens ; à la surface, l'excès seul.
- **P3 fini (E2, second passage)** — **tenu** : `V_φ/V_φ(0)` 0,9993 (0,4 s), 0,9987 (1,0 s), 0,9990 (1,6 s), contre 0,981 sans projection. La crête par la surface est stable (0,136 → 0,140 m), surface et compte d'accord à 2 mm. Le coût : 196 s contre 131 s (+50 %).
- **E3 : échoue, et la vague ne plonge plus.** `V_φ/V_n` passe de 0,9987 à 0,9924 en 0,25 s, puis tient à 0,991 jusqu'à 4 s (sans projection : 0,941). Mais il n'y a **ni retournement ni air** en 4 s, alors que la vague doit plonger (Grilli, S₀ ≈ 0,23). Deux témoins, une cause chacun : **E3a**, sans correction aux mailles de surface (le plongeon revient-il ?) ; **E3b**, l'excès seul près des parois solides (le saut du départ disparaît-il ?).
