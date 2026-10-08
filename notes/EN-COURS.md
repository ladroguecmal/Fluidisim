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

Session : S708 — **en cours**. En autonomie, sans arrêt ; session longue (ADR-279 D3). S707 : APIC tasse ses particules sous la crête
(+3,8 %), et le compte n'en donne pas la surface. **Le volume d'APIC par sa surface, contre le compte.**

**L'instrument.** Le volume de la surface reconstruite d'APIC (`distance()`, la distance signée aux centres des mailles, l'eau où φ < 0) :
`V_φ = Σ clamp(½ − φ/dx, 0, 1)·dx³` sur les mailles non solides. Par colonne, la même somme sur la verticale donne la hauteur de la
surface. On le compare à `V_n = n · quantum`, le volume que comptent les raccords.

**Les essais, dans l'ordre ; chacun a ses critères, écrits avant lui.**

| essai | ce qu'il mesure | critères |
|---|---|---|
| **E1** | l'eau au repos (le semis, 4 m, 1 s) : `V_φ / V_n` au départ et à 1 s | l'étalon : `V_φ / V_n` constant à 10⁻³ sur 1 s (l'eau au repos ne doit rien changer) ; sa valeur, le décalage propre de la reconstruction |
| **E2** | l'onde plate de S707 (10 m, 1,6 s) : `V_φ(t) / V_φ(0)` aux photos ; la crête par la surface, contre la crête par le compte | rapporté ; la croissance de la crête comptée (0,136 → 0,178 m) relue par la surface |
| **E3** | le tout-3D de S690 (le montage sans raccord, 4 s, le déferlement) : `V_φ(t) / V_φ(0)` à chaque quart de seconde | rapporté et attribué : sous 1 % de dérive, le compte est un bon témoin global de la masse et seule la répartition locale diffère ; au-delà, la masse « au bit » des raccords est à reformuler |

**Contrôles du plan** (ADR-266, ADR-267, ADR-268, ADR-276, ADR-277, ADR-279)

- **témoin** : E1 l'est pour E2 et E3 (le décalage propre de la reconstruction). E2 et E3 sont lus contre leur propre départ.
- **instrument** : `V_φ`. Ce que rendrait chaque hypothèse :
  - APIC garde son volume géométrique : `V_φ(t)/V_φ(0)` à 1 % près ;
  - APIC le dérive : une pente, ou un saut au déferlement.
- **calcul** : la résolution de `V_φ`. Une erreur d'un dixième de maille sur la surface, rapportée à 0,5 m d'eau, fait 0,5 % : la mesure
  vaut pour des dérives de ≥ 1 %. Le coût : ≈ 1 + 3 + 13 min.
- **ADR**, et comment chacun est tenu (ADR-277 D1) :
  - ADR-279 D1 : la mesure précède toute tolérance nouvelle ;
  - ADR-276 D1 : E3 passe par la même fonction que le tout-3D (`Large::Aucun`), le volume dans l'enregistrement.
- **pièges** :
  - les mailles solides (l'escalier du fond) sont exclues ;
  - les gouttes en l'air comptent dans `V_φ`, comme dans le compte ;
  - φ est celui du dernier pas.

**Critères de la session.** E1 tenu ; E2 et E3 mesurés et attribués.

### Plan

- [x] **P1** — jeton ; plan.
- [x] **P2** — E1.
- [x] **P3** — E2.
- [ ] **P4** — E3.
- [ ] **P5** — preuve ; rituel.

### Notes de reprise
- **P2 fini (E1)** — **tenu** : au repos, `V_φ / V_n` = 0,99967 au départ et à 1 s ; la reconstruction rend le compte à 3·10⁻⁴.
- **P3 fini (E2)** — l'onde plate : `V_φ/V_φ(0)` 0,996 (0,4 s), 0,989 (1,0 s), **0,981 (1,6 s)** — APIC perd ≈ 1,2 %/s de volume géométrique en mouvement, à compte constant. La crête par la surface est stable (0,149 → 0,133 → 0,137 m) : la croissance comptée (0,178 m) était du tassement.
