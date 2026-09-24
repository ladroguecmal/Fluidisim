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

Session : S340 — **en cours**. **Porte B** : le verdict R16, puis le critère 2 sur la production, cas 1 et 2 ;
chemin de la v1 ([ADR-174](../docs/adr/ADR-174-arbitrages-du-2026-09-19.md) D4), porte en cours de §3 bis.
Agent : Claude Opus 5.5, application desktop ; fichiers, git, cargo, carte réelle, accès web.
Entrée — **verdict R16** de l'utilisateur sur les images de S339 : *« Tout parrait bon visuellement »*. Critère 3
d'[ADR-175](../docs/adr/ADR-175-architecture-d-execution-de-delta-en-3d.md) §4 : reçu. Restent, pour la porte B,
les cas 1 et 2 du critère 1 **sur la production** — critère 2, *« production contre référence, sur les mêmes cas :
écart de hauteur sous 3 mm, pente et phase publiées »* ; S305 n'a mesuré que le cas 3
([CUVE-GPU-S305](../docs/validation/CUVE-GPU-S305.md) §7). Le critère 4, le coût, relève de la porte C.

**Ce que la session doit rendre possible.** La porte B reçue, si les deux cas tiennent. **Ce qui contraint le
cas 1** : la production n'évalue que le fond de B — des composantes progressives, en eau profonde —, et le cas 1
de S297 couple la référence à une onde stationnaire analytique en eau finie (`L` = `h` = 2 m, `k·h` = π). Le fond
du cas 1 sera donc **la même onde stationnaire faite de deux composantes de B** opposées, pour la référence comme
pour la production ; l'écart de ce fond à l'analytique est publié, et la référence sur ce fond est rejouée contre
HOS.

Critères, écrits avant le code :
1. **R16** consigné : critère 3 reçu — registre, preuve, feuille de route, file.
2. **Un fond de B par ses composantes** (`Background::from_components`) : refus d'une liste vide, allocation
   demandée à l'hôte comme `configure` ; essai du cœur.
3. **Cas 2 sur la production** : une composante de B le long de `x` (direction (1, 0) exacte), une crête initiale
   invariante en `y`, `ny` = 8, 2 s. La production reste **invariante en `y`** à quatre ulps du repos près, et
   suit la référence **sous 3 mm** ; pente publiée.
4. **Cas 1 sur la production** : `ny` = 1, géométrie de S297 (murs, boîte de 2,25 m), 5 et 10 cm, `nx` 32, 64 et
   128 si la durée le permet, une période. Publiés : l'écart du fond de B à l'onde analytique ; la référence sur ce
   fond contre HOS (tolérances de S253 : profil < 2 %, harmonique < 20 % à 128) ; **la production contre la
   référence, sous 3 mm**, pente et phase.
5. Si 3 et 4 tiennent : **porte B reçue** (critères 1 à 3 ; le 4 à la porte C) — feuille de route, file, liste.

### Plan

- [x] **P1** — jeton, plan seul.
- [x] **P2** — le verdict R16 : critère 3 reçu ; critère 1.
- [x] **P3** — `Background::from_components` et son essai ; critère 2.
- [x] **P4** — le cas 2 sur la production ; critère 3.
- [x] **P5** — le cas 1 : fond de B stationnaire, chaînon et HOS sur la référence.
- [x] **P6** — le cas 1 : production contre référence ; critère 4.
- [ ] **P7** — preuve ; la porte B, si elle tient ; critère 5.
- [ ] **P8** — rituel.

### Notes de reprise
- **P3, critère 2 tenu.** `Background::from_components` et `ComponentsError` (vide, non fini, allocation) ;
  essai `two_opposed_components_make_a_standing_wave_s340` : deux composantes de `a/2`, opposées, quart de tour —
  `a·cos(k·x)·cos(ω·t)` à 2·10⁻⁵ près à 0 et à T/2, refus vérifiés. Cœur : 500 réussis, 14 ignorés.
- **P4, critère 3 tenu** (`--delta3d-cas2`, 32×8×36 à 25 cm, repos 8 m, 400 pas de 5 ms, 64 cycles). Houle de B
  d'une seule direction, `x` exact, 4 m ; crête initiale de 10 cm invariante en `y`. **À 5 cm de houle** : carte
  contre référence **1,02·10⁻⁶ m** au pire sur 2 s (3 mm exigés), quadratique 4,6·10⁻⁷, pente 3,7·10⁻⁶ ;
  invariance en `y` de la carte **9,5·10⁻⁷ m, 1,00 ulp du repos** (critère : 4), du cœur 3,4·10⁻⁸. **À 10 cm**, le
  premier essai : au micron jusqu'à 1,25 s, puis 5,1 mm à 1,5 s, 8,3 mm à 2 s, invariance perdue (6·10⁻³) — la
  surface franchit le centre de maille à 12,5 cm : **A297**, l'horizon de S298. Seule l'amplitude change entre
  les deux. Le cas retenu est donc celui de S305 : sous le seuil, A297 entière.
- **P5, fait** (`delta3d_mobile -- coupled-b`, 16 min). **Chaînon** — fond de B contre analytique, à 0 et T/4 : η
  identique à 3,7·10⁻⁹ m ; **dans l'eau**, u, w, du/dt, p à 4,4 % de leur maximum, au bas du domaine — eau profonde
  contre profondeur finie, flux de B au fond 8,5·10⁻³ m/s à 5 cm ; **au-dessus du plan moyen**, u et du/dt à 33 %,
  w et p à 6 % — le prolongement borné d'ADR-154 contre le prolongement analytique. **Référence sur ce fond contre
  HOS** (32 / 64 / 128) : 5 cm, profil 1,646 / 1,034 / **0,984 %**, harmonique 2,694 / 1,566 / **1,231 %** ; 10 cm,
  1,426 / 1,161 / **1,069 %** et 3,072 / 1,996 / **1,353 %** — décroissants, **tolérances de S253 tenues** (2 %,
  20 %), six fois moins bien qu'au fond analytique (0,148 / 0,178 %) : l'écart est celui du fond, non du solveur.
- **P6, critère 4 tenu** (`--delta3d-cas1`, `ny` = 1, fond de B de P5, 1 604 pas de 1 ms, 64 cycles). Carte contre
  référence, écart de hauteur au pire — 5 cm : 3,9·10⁻⁷ / 1,3·10⁻⁶ / **1,4·10⁻⁶ m** (32 / 64 / 128) ; 10 cm :
  1,8·10⁻⁵ / 4,2·10⁻⁶ / **8,2·10⁻⁵ m**. Jamais le millimètre ; amplitude modale à 3,7·10⁻⁷ m au plus : même phase.
  **Pente** : 8,5·10⁻⁶ à 1,7·10⁻⁴ à 5 cm ; à 10 cm, 4,4·10⁻⁴ / 2,3·10⁻⁴ / **8,9·10⁻³** — 82 µm sur une maille de
  1,6 cm, au pas 1 530 ; au-dessus des 5·10⁻⁴ de S260 que S305 citait pour information ; non attribué — candidat
  A297, la surface totale franchissant sans cesse des centres de maille. Durées : 14 à 68 s, 400 s à 128.

