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

Session : S682 — **en cours**. En autonomie vers la v2 ; K3, 4.14 ; le relais au rivage (ADR-271, note de S680).

**Ce que la session fait.** **La sortie à droite** (`enable_right_outlet`, avec les bords ouverts de S446) :

- une particule qui franchit le bord droit n'est plus retenue par le domaine, elle est retirée ;
- son volume (`dx³/8`) est compté par rangée `j` et au total (`right_outlet`) ;
- éteinte par défaut ; refusée avec les gouttes et la zone des colonnes (leurs tableaux par particule).

L'entrée (poser des particules pour un volume donné) est la brique suivante.

**Contrôles du plan** (ADR-266, ADR-267, ADR-268, ADR-272)

- **témoin** : sans la sortie, au bit (tous les essais d'APIC) ; le même bassin, le bord ouvert sans la sortie (les particules
  s'entassent contre le bord).
- **instrument** : un bassin à 0,4 m qui se vide par son bord droit ouvert à 0,1 m/s pendant 1 s, à 5 cm. Le volume compté contre deux
  lectures :
  - le compte des particules (exact, au bit) ;
  - le flux de la face, `Σ u·dx²` sur les faces mouillées, intégré sur le pas.

  Ce que rendrait chaque hypothèse :
  - une sortie juste rend le compte exact et le flux à mieux que 10 % ;
  - une particule retenue s'entasse au bord : la densité de la dernière colonne dépasse celle d'une colonne pleine ;
  - un volume compté sans retrait (ou l'inverse) casse le compte exact.
- **calcul** (ce script) : ≈ 4.00 L sortent, 256 particules. La surface baisse de 2.0 cm (5 %
  de `h`) : la borne du flux à **10 %**.
- **ADR** : ADR-271, ADR-272 (sans objet : aucune interface placée dans la maille n'est jugée ici).
- **pièges** :
  - `walls` impose la vitesse du bord à toutes les faces de droite, celles d'air comprises (la projection ne les lit pas) ;
  - le retrait par échange avec la dernière particule (l'ordre change : le compte, non) ;
  - la marge de la borne du domaine (`1e-3·dx`).

**Critères, écrits avant.**

1. Sans la sortie, tout au bit (le banc et les essais d'APIC).
2. Le compte exact : particules au départ = particules restantes + retirées, et le volume compté = retirées × `dx³/8`.
3. Le volume sorti à moins de **10 %** du flux de la face intégré.
4. Aucune accumulation : la dernière colonne ne dépasse pas la densité d'une colonne pleine (8 par maille mouillée).

### Plan

- [x] **P1** — jeton ; plan.
- [ ] **P2** — la sortie ; l'essai ; (1)–(4).
- [ ] **P3** — preuve ; rituel.

### Notes de reprise
