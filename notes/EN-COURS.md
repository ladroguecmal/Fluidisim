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

Session : S684 — **en cours**. En autonomie vers la v2 ; K3, 4.14 ; le relais au rivage (ADR-271), l'étape 1 de la conception.

**Ce que la session fait.** Le module `relais_rivage.rs` : `RelaisRivage` tient APIC 3D (bord droit ouvert, sa sortie et son entrée)
et Saint-Venant 2D côte à côte, à la même maille et au même nombre de rangées. À chaque pas :

1. l'état du bord 3D, le niveau `η` (la plus haute particule de la dernière colonne + `dx/4`) et la vitesse `u` moyenne, nourrit le
   bord gauche de Saint-Venant (`h_e = η − z` de sa première maille) ;
2. Saint-Venant fait son pas et rend son flux (S680) ;
3. APIC reçoit ce flux comme vitesse de son bord droit, et fait entrer le reflux (S683) ;
4. ce qu'APIC a laissé sortir (S682) contre ce que Saint-Venant a pris est gardé comme une dette par rangée.

La masse se compte : Saint-Venant + particules × quantum + réservoir − dette.

**Contrôles du plan** (ADR-266, ADR-267, ADR-268, ADR-272)

- **témoin** : APIC seul sur les mêmes plages (S678 : de 0,21 à 0,58 m/s).
- **instrument** : l'eau au repos 2 s, la vitesse maximale des deux côtés, la masse. Ce que rendrait chaque hypothèse :
  - un raccord juste rend **moins de 1 cm/s partout** et la masse exacte ;
  - des niveaux mal accordés (`h_e` pris à un autre fond) font couler un flux au raccord, quelques cm/s ;
  - une dette qui grandit trahit un échange mal compté.
- **calcul** (ce script, qui asserte trois places au moins) :
  - 1:3, eau 0.4 m, maille 0.05 m : le raccord à 3.03 mailles de fond, la ligne d'eau à 0.10 de maille ;
  - 1:3, eau 0.4 m, maille 0.025 m : le raccord à 3.07 mailles de fond, la ligne d'eau à 0.20 de maille ;
  - 1:10, eau 0.3 m, maille 0.05 m : le raccord à 3.01 mailles de fond, la ligne d'eau à 0.10 de maille ;
  - 1:10, eau 0.3 m, maille 0.025 m : le raccord à 3.02 mailles de fond, la ligne d'eau à 0.20 de maille ;
  - 1:3, eau 0.31 m, maille 0.05 m : le raccord à 3.23 mailles de fond, la ligne d'eau à 0.70 de maille ;
  - 1:3, eau 0.31 m, maille 0.025 m : le raccord à 3.13 mailles de fond, la ligne d'eau à 0.40 de maille ;
- **ADR** : ADR-271, ADR-272 D1.
- **pièges** :
  - le bord de Saint-Venant prend un seul état extérieur pour toutes les rangées (la moyenne) : exact au repos et sur une côte
    uniforme, non en général (noté) ;
  - le pas de temps commun, le plus petit des deux ;
  - le niveau du bord 3D, sans particule dans la colonne, pris au fond.

**Critères, écrits avant.**

1. Sur les six plages, l'eau au repos 2 s : la vitesse maximale sous **1 cm/s**, des deux côtés.
2. La masse : Saint-Venant + particules × quantum + réservoir − dette, constante à 10⁻¹² près en relatif.
3. Les essais d'APIC et de Saint-Venant inchangés.

### Plan

- [x] **P1** — jeton ; plan.
- [ ] **P2** — `relais_rivage.rs` ; l'essai ; (1)–(3).
- [ ] **P3** — preuve ; liste 4.14 ; rituel.

### Notes de reprise
