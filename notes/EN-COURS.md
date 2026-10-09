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

Session : S725 — **en cours**. En autonomie, sans arrêt (l'utilisateur dort) ; session longue. LOD-ETAPE-3-S722, **B3 : le raccord de la
boîte**. Saint-Venant troué (B1) et APIC à quatre bords (B2) sont réunis.

**Ce que la session fait.** `relais_boite.rs`, `RelaisBoite`. À chaque pas, pour chaque face du trou :
1. l'état de la colonne 3D voisine (le niveau par la surface, les vitesses moyennes) ;
2. le flux complet de Rusanov contre la maille active de Saint-Venant, donné à Saint-Venant (`pas_avec_flux_trou`) ;
3. `F₀/h` devient la vitesse du bord d'APIC (bornée par la célérité), et l'eau qui entre est posée ;
4. le pas d'APIC ;
5. le bilan de la face (cédé contre reçu et rendu) : l'écart va à la maille active, et la masse est exacte par construction.

Les sorties en y sont comptées par face (`y_outlet_step`), et le flux de Rusanov rendu public (`flux_rusanov`).

*Note* : le code a été écrit avant ce plan ; les critères ci-dessous le précèdent, avant tout essai.

**Les essais ; chacun a ses critères, écrits avant lui.** Saint-Venant 3 m × 3 m (`dx` = 2,5 cm), 0,4 m d'eau, fond plat ; la boîte de
1 m × 1 m au milieu (≈ 205 000 particules).

| essai | ce qu'il juge | critères |
|---|---|---|
| **E1** | le repos, 1 s | la vitesse maximale d'APIC et de Saint-Venant sous 1 mm/s ; la masse à 10⁻¹² ; `|η|` de Saint-Venant sous 1 mm |
| **E2** | une bosse de 2 cm (rayon 0,4 m) qui traverse la boîte, 1,2 s, contre le Saint-Venant entier | la masse à 10⁻¹² ; **ce que la boîte réfléchit** (l'écart de η en arrière de la boîte, x de 0,2 à 0,9 m) sous **10 %** de l'amplitude ; l'écart en aval rapporté (la 3D est dispersive, Saint-Venant non) |

**Contrôles du plan** (ADR-266, ADR-267, ADR-268, ADR-273, ADR-276, ADR-277, ADR-280, ADR-281, ADR-282)

- **témoin** : E1, le repos ; E2, le Saint-Venant entier, le même pas. La réflexion se juge en arrière de la boîte, où la 3D et Saint-Venant
  ne diffèrent que par le raccord.
- **instrument** : la masse (Saint-Venant hors du trou, les particules, les réservoirs), recalculée à chaque pas ; l'écart de η dans des
  bandes (ADR-281 D2 : ils peuvent échouer).
- **calcul** :
  - la bosse : `kd` ≈ 1,6 au rayon 0,4 m pour 0,4 m d'eau, donc dispersive. D'où un écart en aval attendu, et la réflexion jugée seule ;
  - le coût : ≈ 1 min pour E1, ≈ 3 min pour E2.
- **ADR**, et comment chacun est tenu (ADR-277 D1) :
  - ADR-273 D1 : le raccord se juge contre le même Saint-Venant, sans trou ;
  - ADR-280 D1 : la surface ;
  - ADR-282 : la fermeture par l'outil.
- **pièges** :
  - les signes des quatre faces dans le bilan ;
  - le niveau d'une colonne qui sèche (le plancher de `dx/4`) ;
  - la pose à droite reste celle du réseau (S683).

### Plan

- [x] **P1** — jeton ; plan.
- [ ] **P2** — E1, E2.
- [ ] **P3** — preuve ; fermeture (le lot S723–S725).

### Notes de reprise
