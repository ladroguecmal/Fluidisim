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

Session : S730 — **en cours**. En autonomie ; session longue. **R43 reçu** (ADR-284) : réaliste et cohérent, sauf au raccord du rivage, où
le jet de la 3D retombe contre un mur de Saint-Venant, avec une poche d'air. **La question** : placé au-delà du point où le jet retombe
(ADR-284 D2), le raccord du rivage efface-t-il ce mur, et à quel coût ?

**Ce que la session fait.**
- `deux_raccords_porteur` prend la place du raccord du rivage, `x_f` ; 10,775 m reste le défaut, au bit.
- **L'instrument du mur** (ADR-284 D4), à chaque pas : `J(t)`, le niveau de Saint-Venant à sa première maille (moyenne des rangées), moins
  le niveau de la 3D à sa dernière colonne (fond + l'épaisseur d'eau lue par φ, `volume_surface_s708`).
- `x_f` = **12,0 m** : 0,30 m au-delà du rivage au repos (11,696 m). La 3D couvre la plage où le jet retombe (≈ 10,4–10,8 m) et la lame qui
  remonte, jusqu'à sa mort à 3,2 s (M1) ; Saint-Venant prend le sable sec au-delà.

**Les essais, et leurs critères écrits avant.**
- **E1 — de bout en bout** (`BoutEnBout`, 5 s), `x_f` = 10,775 m (le montage de R43) puis 12,0 m :
  1. **le mur** : le plus grand `J` sur [2,4 s ; 3,2 s] à 12,0 m sous **1 cm**, et sous le quart de celui de 10,775 m. Le plancher de bruit
     de l'instrument (ADR-280 D1) est le plus grand `|J|` avant l'arrivée de l'onde (t < 1,5 s) ; le critère n'est pas plus fin que lui ;
  2. le retournement à 0,15 m et 0,1 s de celui de 10,775 m (ADR-278 D2 ; l'amont ne change pas) ;
  3. la masse à 10⁻¹² ;
  4. le coût rapporté ; s'il dépasse deux fois celui de 10,775 m, la lame mince en 3D (S712) est la suite ;
  5. le film à 1/30 s, et l'image de l'instant de la capture de l'utilisateur, à lui envoyer.
- **E2 — le tout-3D** (`AucunJusqua5`), `x_f` = 12,0 m : le nouveau témoin. Il rapporte le retournement, l'air, la remontée, `J`, le coût et
  le film, contre S717 (2,637 s, 9,988 m ; 1 191 s). Critères : le retournement à 0,15 m et 0,1 s de S717 ; la masse à 10⁻¹² ; `J` sous 1 cm.

**Contrôles du plan** (ADR-266, ADR-273, ADR-277, ADR-278, ADR-280, ADR-281, ADR-283, ADR-284)

- **témoin** : le même montage à 10,775 m, mesuré par le même instrument dans le même essai (ADR-273 D1).
- **instrument** : `J` lu par la surface de la 3D (ADR-280 D1), son plancher de bruit mesuré avant l'onde ; affiché au fil du calcul
  (ADR-281 D1).
- **calcul** :
  - E1 : deux fois ≈ 5 min ;
  - E2 : ≈ 20 à 25 min.

  La 3D grandit de 431 à 480 colonnes (+11 %), sur du sable presque sec.
- **ADR** :
  - ADR-284 D2 et D4 ;
  - ADR-283 D1 : Saint-Venant part du niveau que lit la 3D, comme avant ;
  - ADR-278 D2 : les tolérances du juge.
- **pièges** :
  - le raccord sur du sable sec : Saint-Venant à `h` = 0, la vitesse imposée au bord d'APIC bornée par la célérité (S690) ;
  - la lame mince en 3D fait tomber le pas (S712) ;
  - `x_f` change `nx` : tout ce qui dépend de 431 colonnes (la mort, le film) le lit de `nx`.

### Plan

- [x] **P1** — jeton ; ADR-284 ; plan.
- [x] **P2** — `x_f` et l'instrument du mur ; E1 ; (1)–(5).
- [x] **P3** — E2, le tout-3D à 12,0 m.
- [ ] **P4** — preuve ; images ; fermeture.

### Notes de reprise
- **P2 et P3, dans un même lancement** (le filtre d'`essai.py` est une sous-chaîne : le nom de E1 est contenu dans celui de E2, les deux ont
  tourné, E2 d'abord). **E1 tenu** : le mur 281,5 mm (à 2,934 s, R43) → 0,0 mm (le bruit 0,69 → 0,00 mm) ; le retournement 2,569 s / 9,863 m
  → 2,588 s / 9,913 m ; la masse 1,6·10⁻¹⁵ ; le coût 291 → 308 s (1,06×). La remontée 0,3714 → 0,3364 m. **E2 tenu** : le tout-3D à 12,0 m,
  le retournement 2,620 s / 9,938 m (S717 : 2,637 s / 9,988 m), l'air 2,804 s / 10,375 m, la masse 1,2·10⁻¹⁶, le mur 0, la remontée
  0,3547 m à 3,942 s, 1 152 s. De bout en bout contre le tout-3D, la remontée passe de +3,3 cm (S718) à −1,8 cm.
