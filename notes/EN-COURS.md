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

Session : S693 — **en cours**. En autonomie vers la v2 ; le LOD de simulation (ADR-275), étape 1 : **les deux raccords ensemble**.

**Ce que la session fait.**

- **La zone de colonnes et la sortie à droite, permises ensemble.** S682 les refusait pour « leurs tableaux par particule » ; la zone de
  colonnes n'en a aucun (ses champs sont ceux de la grille) : le refus était trop prudent.
- **`RelaisRivage`** :
  - le bord gauche d'APIC réglable par l'appelant (il le remettait à zéro) ;
  - le volume de la 3D pris par `total_volume` (les particules, l'eau des colonnes, leurs soldes) ; sans colonnes, le même nombre
    qu'avant, au bit.
- **La vague de S647, 3D réduite.** APIC 3D sur `[5,0 ; 10,775]` m :
  - au large, la zone de colonnes de S650 (0,6 m), son bord gauche poussé par Saint-Venant ;
  - au rivage, le relais de S690.

**Contrôles du plan** (ADR-266, ADR-267, ADR-268, ADR-273, ADR-274)

- **témoin** : le relais au rivage seul (S690 : 2,624 s, 9,963 m ; l'air à 2,777 s, 10,325 m) et le tout-3D (2,642 s, 9,988 m) ;
  S684 inchangé (au bit, sans colonnes).
- **instrument** : les lecteurs de S647–S648 ; la masse (la 3D + Saint-Venant du rivage + réservoir − dette − le volume entré par la
  gauche). Ce que rendrait chaque hypothèse :
  - si les deux raccords coexistent, le même retournement que S690, à 0,02 s et 0,15 m (le relais au large de S650 en était à 0,004 s
    et 0,15 m à 5 cm) ;
  - un conflit entre la zone de colonnes et la sortie, une masse qui dérive ou une colonne de bord vidée.
- **calcul** (ce script) : l'eau de la 3D passe de 4.31 à 1.81 m² par mètre de largeur, ÷ 2.4 particules. Le calcul attendu :
  ≈ 5.7 min au lieu de 13,6 (mesuré au premier pas, ADR-274 D1).
- **ADR** : ADR-271, ADR-273 D2 (une seule source : Saint-Venant du rivage part du niveau des particules), ADR-274, ADR-275.
- **pièges** :
  - S650 pose le fond plat à z = 0 sous les colonnes (l'eau à 0,5 m, non 0,55 m) : la géométrie est celle de S647 décalée de 5 cm ;
  - le bord gauche ouvert et le bord droit de sortie au même pas ;
  - la dette et le réservoir de la sortie, à côté des soldes des colonnes.

**Critères, écrits avant.**

1. Sans colonnes, au bit : S684 inchangé.
2. Le premier retournement à moins de **0,02 s** et **0,15 m** du tout-3D ; l'air enfermé après lui, en avant.
3. La masse à 10⁻¹² près en relatif ; la dette sous un quantum.
4. Rapportés : le temps de calcul contre S690, les particules.

### Plan

- [x] **P1** — jeton ; plan.
- [ ] **P2** — les deux raccords ; l'essai ; (1)–(4).
- [ ] **P3** — preuve ; rituel.

### Notes de reprise
