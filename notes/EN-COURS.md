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

Session : S729 — **en cours**. En autonomie, sans arrêt ; session longue. LOD-ETAPE-3-S722, **B5 : le déclencheur de présence**. La boîte
naît quand un corps touche l'eau, le suit, meurt quand il en sort.

**Ce que la session fait.**
- **`RelaisBoite::naitre`** : au milieu d'un Saint-Venant entier, une boîte naît de son état :
  - les colonnes par `birth_from_columns` (S707) ;
  - la vitesse de chaque colonne, celle de Saint-Venant ;
  - **le niveau réglé sur ce que la 3D lit** (ADR-283 D1) : une première naissance mesure le biais de lecture, une seconde pose ce qu'il
    faut pour que la 3D lise le niveau de Saint-Venant. L'écart est tenu dans la masse (`reste`), rendu à la mort.
- **`RelaisBoite::mourir`** : la boîte rend à Saint-Venant, colonne par colonne, sa surface (φ) et sa quantité de mouvement
  (`SaintVenant2D::fermer_trou`). Ce qui reste (les dettes, les réservoirs, le compte contre la surface, `reste`) est réparti également sur
  les mailles du trou : la masse est exacte.
- **Le déclencheur** : la boîte naît quand le bas du corps passe sous le niveau ; elle meurt quand il en est sorti depuis 0,2 s
  (l'hystérésis).

**L'essai et ses critères, écrits avant.** La sphère de B4 (rayon 8 cm) descend de 15 cm au-dessus de l'eau à 0,3 m/s jusqu'à mi-immersion
(0,5 s), avance à 0,3 m/s (1,5 s), remonte et sort (0,6 s), puis l'eau seule (0,6 s) ; 3,2 s en tout. Saint-Venant fait 3 m × 2 m ; le
témoin est un APIC entier aux mêmes murs.
1. la boîte naît une fois, meurt une fois ;
2. la force sur la sphère dans l'eau : l'écart moyen au témoin sous **10 %** ;
3. la masse à 10⁻¹², à la naissance et à la mort comprises ;
4. aucun choc de niveau : le niveau moyen de Saint-Venant autour du trou, juste avant et juste après la naissance comme la mort, à
   **1 mm** près.

**Contrôles du plan** (ADR-266, ADR-267, ADR-268, ADR-273, ADR-276, ADR-277, ADR-280, ADR-281, ADR-283)

- **témoin** : le même corps, le même mouvement, dans un APIC entier (ADR-273 D1).
- **instrument** : la force, la masse, le niveau autour du trou, recalculés à chaque pas (ADR-281 D2).
- **calcul** : le témoin, ≈ 1,2 million de particules sur 3,2 s, ≈ 40 min ; la boîte, quelques minutes.
- **ADR**, et comment chacun est tenu (ADR-277 D1) :
  - ADR-283 D1 : le niveau lu, réglé à la naissance ;
  - ADR-280 D1 : la surface à la mort ;
  - ADR-275 D1 : l'hystérésis.
- **pièges** :
  - les particules dans la sphère, à la naissance (une sphère en partie dans l'eau) : elles sont retirées et comptées dans `reste` ;
  - le corps hors de l'eau n'a pas de boîte ; sa force est nulle dans les deux ;
  - la naissance alloue une boîte (en jeu, une réserve ; ici, l'essai la prépare d'avance).

### Plan

- [x] **P1** — jeton ; plan.
- [ ] **P2** — naître, mourir, le déclencheur ; l'essai ; (1)–(4).
- [ ] **P3** — preuve ; fermeture.

### Notes de reprise
