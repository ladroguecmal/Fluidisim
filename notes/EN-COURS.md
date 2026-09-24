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

Session : S356 — **en cours**. **Rendu 1 (ADR-191) : les crêtes de B — l'écume et la lumière qui les traverse.**
Agent : Claude Opus 5.5, application desktop ; fichiers, git, cargo, carte réelle, accès web.
Entrée — ADR-191 : une session de rendu, une de physique ; la part de l'eau du rendu du moteur maison s'écrit en
module, depuis l'afficheur. **Ce qui est déjà su** : au verdict R14 (S308), l'utilisateur a nommé le travail
optique prioritaire — ciel, exposition, absorption, **diffusion aux crêtes**, **écume**, hautes fréquences du reflet ;
S307 en a fait l'ordre (RENDU-ECART-S307 §6) : la couleur du corps d'eau (faite), **la lumière des crêtes, pilotée
par le jacobien de CWM déjà calculé à chaque pixel**, puis **l'écume, couverture de Monahan & O'Muircheartaigh —
0,42 % à `U₁₀` ≈ 7,8 m/s —, réflectance effective de Koepke 0,22**.

**Le point délicat** : le rendu filtre les ondes courtes avec l'empreinte du pixel ; un seuil fixe sur le jacobien
ferait disparaître l'écume au loin. Le seuil porte donc sur la variable normalisée `s = (J − 1)/σ(h)`, `σ(h)`
l'écart-type du jacobien au filtrage du pixel ; `s_t` se tire de la loi de `s` à pleine résolution, pas d'un choix.

Critères, écrits avant le code :
1. **La couverture** : sur CPU, loi de `s` pour la mer de `--meilleur` (bande et queue, pleine résolution, plusieurs
   instants et lieux) ; `s_t` tel que `P(s < s_t) = W(U₁₀)` ; écart à la gaussienne et incertitude publiés.
2. **L'écume** (`--ecume`), module WGSL séparé : couverture par pixel tirée de `s_t` et de `σ(h)`, bord adouci,
   réflectance 0,22 éclairée comme l'eau. Sans l'option, les images de `--revue-mer` **au bit**. Avec : couverture
   recalculée sur CPU aux empreintes de 0, 0,5 et 2 m, à ± 30 % de `W`.
3. **La lumière des crêtes** (`--cretes`), même module : masque de compression tiré du même jacobien, diffusion vers
   l'avant, couleur de diffusion dérivée du corps d'eau (ADR-177) ; ses deux paramètres **à calibrer par R19**
   (I-14). Sans l'option, au bit.
4. **Images et coût** : quatre états (base, écume, crêtes, les deux) aux quatre poses de R14 ; surcoût GPU mesuré.
5. **R19** préparée, références demandées ; preuve, file, feuille de route, liste 8.4.

### Plan

*Amendé à 00 h 55, sur la demande de l'utilisateur : « Peut être godot ou unreal serait envisageable car rendu toujours
pas convaincant », puis le choix de **Godot 4** — [ADR-192](../docs/adr/ADR-192-le-rendu-de-l-eau-dans-godot-4.md). Les
images et la revue R19 **dans l'afficheur** perdent leur objet : le jugement se fera dans Godot. Le coût se mesure
quand même, le module restant la référence à porter.*

- [x] **P1** — jeton, plan seul.
- [x] **P2** — la loi de `s` et le seuil ; critère 1.
- [x] **P3** — l'écume au rendu ; critère 2.
- [x] **P4** — la lumière des crêtes ; critère 3.
- [x] **P5** — la décision de l'utilisateur (ADR-192) et le coût du module ; critère 4 réduit au coût.
- [x] **P6** — preuve, file, feuille de route, liste, index ; critère 5 sans R19.
- [ ] **P7** — rituel.

### Notes de reprise
- **P2, critère 1 tenu — avec une hypothèse contredite.** `--meilleur --ecume-loi` (`rendu_cretes.rs`) : bande 64,
  queue 60, M = 2, retard −0,20 ; `U₁₀` = 7,787 m/s (Pierson–Moskowitz depuis `Hs` 1,5 m), **W = 0,421 %**.
  1,2 million d'échantillons (2 km, huit instants) : `s` de moyenne 0,011, écart-type 1,004, **queue basse plus
  lourde que la gaussienne** — `s_t` = −2,311 [−2,319 ; −2,305] contre −2,635 ; `J` jamais négatif (min 0,215),
  `σ` moyen 0,206. **Le seuil unique ne tient pas la couverture** quand le rendu filtre : 1,55 × W à 10 cm
  d'empreinte, 1,77 à 0,5 m, 1,83 à 2 m, 2,02 à 8 m. **Remède** : un seuil par empreinte, 14 empreintes
  `2^(i−7)` m (7,8 mm à 64 m), interpolé en `log₂ h`, 1,2 s de calcul (un fil par empreinte, 160 000 tirages) ;
  sur un **tirage indépendant**, couverture 0,93 / 0,97 / 1,03 / 1,02 / 1,00 / 1,04 / 1,03 × W à 0 / 1,2 cm / 5 cm /
  35 cm / 1,4 / 5,6 / 22 m.
- **P3 et P4, un seul commit** : le module `water_cretes.wgsl` les porte ensemble. Concaténé avant `water.wgsl`
  (`gpu.rs`) ; seuils et paramètres dans cinq vecteurs de plus de l'uniforme (224 → 304 octets) ; `σ²` de la bande
  au sommet (`Cwm.v`, sortie `sigma2`), de la queue filtrée au fragment (`TailMoments.variance`) ; `fwidth(s)` en
  flot uniforme pour le bord antialiasé. **Options éteintes : les quatre poses de `--revue-mer` identiques au bit**
  au binaire d'avant (empreintes 0x6f8a…1761, 0x8e33…4281, 0xdd86…7a2f, 0x7d28…316a).
- **P3, ce que l'image a dit.** Réflectance 0,22 (Koepke) sous `E/π` = gain 2 : luminance ≈ 0,41, **grise**, plus
  sombre que le ciel reflété près de l'horizon — invisible en pose rasante. Koepke est une moyenne sur tout le
  mouton ; l'écume fraîche est mesurée vers 0,55 (Whitlock et al. 1982) : `--ecume=0,55`, taches blanches. Pas de
  partage cœur/frange publié trouvé (Monahan & Lu 1990 nomment les stades A et B, sans rapport chiffré) : **R19
  montre les deux**. L'écume se place sur la crête de la grande houle, où la surface se comprime le plus.
- **P4.** Première teinte, celle du corps d'eau (`SEA_R0`, B/G 10,9) : taches **bleu électrique**, artificielles.
  Remplacée par la **transmission de l'eau pure sur une crête**, `exp(−a·Hs)`, `a` de Pope & Fry (mêmes absorptions
  qu'ADR-177) : (0,599 ; 0,919 ; 0,986), un cyan clair ; force 0,15, exposant 4, à calibrer par R19 ; effet discret.
- **P5.** Godot 4.4.1 (et 4.2, 4.4) présent dans Téléchargements ; Unreal absent (lanceur Epic, RealityScan). ADR-192 ;
  notes datées d'ADR-191 ; file, feuille de route §3 ter, liste 8.1, REPRISE §5, index. **Coût du module**
  (`--cretes-bench`, 1280 × 720, 120 images, secteur 96 % avant et après) : médiane GPU de l'eau, référence 2,106 ms
  sans, +0,018 écume, +0,012 crêtes, +0,026 les deux ; rasante 2,095, +0,010, +0,008, +0,012. Le calcul de `σ` et de
  `fwidth(s)`, fait sans condition, est dans les deux états : non séparé.

