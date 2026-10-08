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

Session : S695 — **en cours**. En autonomie, sans arrêt. Le relais au large de S693, nourri par SGN (S694) au lieu de Saint-Venant.

**Ce que la session fait.** `deux_raccords_s693` reçoit le porteur du large : Saint-Venant (S693), ou SGN 1D. SGN est sur fond plat,
périodique, 40 m, l'onde à 3,4 m ; le large est plat jusqu'au pied (5,696 m), et le raccord est à 5,0 m. Le bord gauche d'APIC reçoit sa
vitesse moyenne à 5,0 m, comme en S650.

**Contrôles du plan** (ADR-266, ADR-267, ADR-268, ADR-273, ADR-274)

- **témoin** : le raccord du large à 1,0 m (S693). L'onde y naît dans la 3D et le porteur ne porte rien d'elle : 2,582 s, 9,888 m. C'est
  la référence du côté d'APIC (ADR-273 D1). Le tout-3D (2,642 s) garde l'écart propre au montage du large (≈ 0,04 s, non départagé en
  S693).
- **instrument** : le premier retournement. Ce que rendrait chaque hypothèse :
  - si SGN porte l'onde comme APIC, le témoin à 0,02 s près ;
  - si le défaut est ailleurs (le raccord lui-même), 2,52 s comme avec Saint-Venant ;
  - si SGN corrige trop, au-delà du témoin.
- **calcul** : aucun nombre neuf. Les références sont mesurées (S693 : 2,524 s avec Saint-Venant, 2,582 s au témoin). Le coût : celui de
  S693, 10,3 min (SGN 1D, quelques secondes).
- **ADR** : ADR-273, ADR-274, ADR-275.
- **pièges** :
  - SGN sur fond plat ne voit pas la pente au-delà du pied : les ondes réfléchies vers le large manquent (le relais est à sens unique,
    comme S650) ;
  - une seule source : l'onde de SGN et celle de la 3D, la même (x₁ = 3,4 m, ADR-273 D2) ;
  - le pas de SGN, sous-divisé dans le pas d'APIC.

**Critères, écrits avant.**

1. Avec SGN : le premier retournement à moins de **0,02 s** et **0,15 m** du témoin (2,582 s, 9,888 m).
2. L'air enfermé après lui, en avant ; la masse à 10⁻¹² ; la dette sous un quantum.
3. Rapporté : l'écart au tout-3D, qui reste à départager.

### Plan

- [x] **P1** — jeton ; plan.
- [ ] **P2** — le porteur SGN ; l'essai ; (1)–(3).
- [ ] **P3** — preuve ; rituel.

### Notes de reprise
