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

Session : S728 — **en cours**. En autonomie, sans arrêt (l'utilisateur dort) ; session longue. LOD-ETAPE-3-S722, **B4b : la boîte qui suit
le corps**.

**Ce que la session fait.**
- **`Apic3::shift_x`** (`apic3d_deplacement.rs`) : chaque particule recule d'une maille. Celles qui sortent derrière meurent (leur volume
  et leur quantité de mouvement rendus par rangée). La colonne de devant naît d'un volume donné par rangée, chaque sous-colonne emplie à la
  même hauteur (S707). Le corps recule d'une maille dans le repère de la boîte.
- **`SaintVenant2D::deplacer_trou_x`** : la colonne de derrière redevient active avec l'état donné ; celle de devant est gelée.
- **`RelaisBoite::suivre_x`** :
  - la colonne de derrière rend à Saint-Venant sa surface (φ, ADR-280 D1) et sa quantité de mouvement ;
  - la colonne de devant naît de l'état de Saint-Venant ;
  - les écarts (le compte contre la surface derrière, le donné contre le posé devant) vont aux dettes des faces voisines : la masse est
    exacte.

**L'essai et ses critères, écrits avant.** La sphère de B4a (rayon 8 cm, à demi immergée), tirée à 0,3 m/s pendant **3 s** (0,9 m, plus que
la boîte). La boîte de 1 m × 1 m la suit : elle avance d'une colonne chaque fois que le corps dépasse son milieu d'une maille. Saint-Venant
fait 3 m × 2 m. Le témoin est un APIC entier de 3 m × 2 m (≈ 1,2 million de particules), aux mêmes murs.
1. La force sur la sphère : l'écart moyen au témoin, de 0,2 à 3,0 s, sous **10 %** ;
2. la surface autour du corps à 2,5 s (la boîte où elle est alors) : l'écart quadratique moyen sous **20 %** de la plus haute vague du témoin ;
3. la masse à 10⁻¹².

**Contrôles du plan** (ADR-266, ADR-267, ADR-268, ADR-273, ADR-276, ADR-277, ADR-280, ADR-281, ADR-283)

- **témoin** : le même corps dans la 3D entière (ADR-273 D1) ; B4a pour le raccord fixe (2,0 % et 1,7 %).
- **instrument** : la force à chaque pas ; la surface par colonne (φ) dans la fenêtre de la boîte ; la masse à chaque pas. Ce que rendrait
  chaque hypothèse :
  - le déplacement est sans faute : comme B4a ;
  - une naissance ou une mort brusque : un saut de force à chaque avancée, et des rides derrière la boîte.
- **calcul** :
  - le corps avance de 0,9 m, soit 36 colonnes, une avancée tous les ≈ 83 ms ;
  - le témoin : ≈ 1,2 million de particules, ≈ 25 min ; la boîte : ≈ 6 min.
- **ADR**, et comment chacun est tenu (ADR-277 D1) :
  - ADR-283 D1, D2 : l'état lu ; les constantes de Saint-Venant dans ses volumes ;
  - ADR-280 D1 : la surface à la mort ;
  - ADR-282 : la fermeture par l'outil.
- **pièges** :
  - la fenêtre de comparaison suit la boîte (son origine change à chaque avancée) ;
  - la dette des faces de gauche reçoit l'écart de la mort, celle de droite l'écart de la naissance ;
  - le trou ne doit pas approcher le bord de Saint-Venant (trois mailles).

### Plan

- [x] **P1** — jeton ; plan.
- [ ] **P2** — le déplacement ; l'essai ; (1)–(3).
- [ ] **P3** — preuve ; fermeture.

### Notes de reprise
