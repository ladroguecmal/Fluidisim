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

Session : S704 — **terminée**. En autonomie, sans arrêt. S703 : à pose égale, les données de SGN avancent le retournement de 0,089 s. Au
plan de 5 m, la crête de la 3D est à 0,139 m au-dessus du niveau, celle de SGN à 0,150 m. **Le juge (le tout-3D à 2,5 cm) est-il juste ?**

**Ce que la session fait.** L'onde de départ (a = 0,15 m, d = 0,5 m, x₁ = 3,4 m), seule, sur un fond plat de 8 m, pendant 0,8 s. On y lit
la plus haute hauteur d'eau à cinq plans (3,4 ; 4,0 ; 4,5 ; 5,0 ; 5,4 m), dans trois calculs :
- APIC 3D à 2,5 cm, le juge ;
- APIC 3D à 1,25 cm, deux rangées ;
- SGN.

La hauteur de la 3D est lue **par le volume** des particules de la tranche `|x − plan| < dx/2`, non par la plus haute particule : sa
résolution est d'un quantum sur `dx × largeur`, 0,8 mm à 2,5 cm.

**Contrôles du plan** (ADR-266, ADR-267, ADR-268, ADR-276, ADR-277)

- **témoin** : la même onde, la même fonction, à deux résolutions. Seul `dx` change, et la largeur suit pour garder le coût (ADR-276 D2 :
  la largeur ne change rien à une onde plane, entre deux murs).
- **instrument** : la crête aux cinq plans. Ce que rendrait chaque hypothèse :
  - le juge amortit : à 1,25 cm, la crête au plan de 5 m plus haute qu'à 2,5 cm, au-delà de 3 mm (quatre fois la résolution) ;
  - SGN s'écarte de la 3D : les deux résolutions d'accord, sous SGN ;
  - les deux à la fois : entre les deux.
- **calcul** : la résolution de lecture, `quantum / (dx · largeur)` = (0,025³ / 8) / (0,025 × 0,1) = 0,78 mm ; à 1,25 cm sur deux
  rangées, (0,0125³ / 8) / (0,0125 × 0,025) = 0,78 mm. Le coût : mesuré au premier pas, montré.
- **ADR**, et comment chacun est tenu (ADR-277 D1) :
  - ADR-276 D1 : l'état initial est construit dans l'essai, depuis une seule fonction, l'onde de S693 (x₁ = 3,4 m) ;
  - ADR-273 D1 : sans objet, aucun raccord ici.
- **pièges** :
  - le mur de droite, à 8 m : l'onde ne l'atteint pas en 0,8 s (≈ 2 m parcourus) ;
  - SGN périodique sur 40 m, les plans lus à la face, moyenne des deux mailles.

**Critères, écrits avant.**

1. Les crêtes aux cinq plans pour les trois calculs, et l'attribution selon l'instrument.
2. Le volume de la 3D tenu (le nombre de particules constant).

### Plan

- [x] **P1** — jeton ; plan.
- [x] **P2** — l'essai ; (1)–(2).
- [x] **P3** — preuve ; rituel.

### Notes de reprise
- **L'instrument corrigé avant toute attribution** : la tranche d'une maille (`|x − plan| < dx/2`) a donné 0,29 m au plan de 5 m à 2,5 cm. C'était le regroupement passager des particules, retenu par le maximum dans le temps ; le pas restait à 10 ms. La tranche passe à 10 cm aux deux résolutions : le lissage de la crête est de `(k·w)²/3` ≈ 7·10⁻⁴, soit 0,1 mm, et la résolution de 0,2 mm.
- **P2 fini** — (1) le juge n'amortit pas (1,25 cm : 0,143 m ; 2,5 cm : 0,149 m ; SGN 0,150 m) : SGN 5 % au-dessus de la 3D convergente ; (2) les particules tenues.
