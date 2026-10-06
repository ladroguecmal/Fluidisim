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

Session : S520 — **en cours**. En autonomie, **4.13, le sillage mesuré** : S517 n'a pas pu lire l'angle du sillage de la coque dans δ
(trois instruments) ; S519 a montré que l'instrument de S517 ne lit pas Kelvin même sur la théorie, et a éprouvé le **bord d'Airy** (19,98°
sur la théorie, 19,98° sur W). S517 lisait entre 1,7 et 4,2 λ₀ ; le bord d'Airy demande 4–6 λ₀ — un domaine plus long, abordable depuis
S518 (le pas de la carte à 9,2 ms sur 786 000 mailles).

**Ce que la session fait.** (a) **La référence de la coque** : la même réponse linéaire exacte en temps (`outils/reference_sillage.py`),
la source étant la pression hydrostatique de la coque sur son empreinte (un rectangle de 4 × 1,6 m, `p = ρ g d`, `d` = 0,488 m, son
tirant) menée à 3 m/s ; l'instrument (le bord d'Airy, fenêtre 4–6 λ₀, rayons issus du **centre** de la source) éprouvé sur elle. (b) **La
coque dans δ** : le banc du sillage (`--lineaire-sillage`) sur 104 × 56 × 4 m (416 × 224 × 16 mailles de 25 cm, 1,49 M), 30 s, le même
instrument, mêmes rayons, mêmes distances, sur la surface de la carte.

**Ordre de grandeur, calculé.** λ₀ = 5,76 m à 3 m/s : fenêtre 23,1–34,6 m derrière le centre ; le transitoire du départ vers `U·T/2` =
45 m ; parcours 85,5 m (rampe de 3 s) ; rayons jusqu'à 22,2 m de l'axe à 40°. Une origine des rayons déplacée de 2 m (l'étrave de S517)
déplace l'angle de 1,2° à 29 m : la même origine des deux côtés. Coût : ≈ 3 000 pas de ≈ 15–25 ms.

**Critères, écrits avant.** (1) Sur la référence de la coque, le bord d'Airy lit 19,47° à 1° (sinon l'instrument ne vaut pas pour cette
source, et rien n'est conclu de δ). (2) **4.13, le sillage mesuré** : le bord d'Airy sur δ à 2° de 19,47°. (3) Le pas reste borné et fini
(élévation maximale publiée), son coût publié.

### Plan

- [x] **P1** — jeton, plan seul.
- [x] **P2** — la référence de la coque, l'instrument ; (1).
- [ ] **P3** — la coque dans δ ; (2), (3).
- [ ] **P4** — preuve ; liste 4.13 ; rituel.

### Notes de reprise
- **P2 fini — (1) manqué.** La référence de la coque (rectangle 4 × 1,6 m, `ρ g d`, coupure de Nyquist de δ), trois grilles : le bord
  d'Airy lit **16,40 / 16,40 / 16,36°** à 3 m/s (par λ₀ : 16,3 / 16,1 / 16,3 / 16,4 / 16,6° de 2 à 7 λ₀ — stable). Balayage de la vitesse
  sur la référence : 19,1° à 1,5 m/s, 14,4° à 2, 21,7° à 2,5, 16,4° à 3, 19,0° à 4 — **l'angle lu sur une coque de 4 m oscille avec
  L/λ₀** (les ondes d'étrave et de poupe interfèrent) ; 19,47° n'est pas la cible de la théorie à 3 m/s. Les vitesses qui lisent Kelvin
  sont hors de portée de δ (1,5 m/s : λ₀ = 1,44 m, 6 mailles ; 4 m/s : λ₀ = 10 m, `kh` = 2,4 par 4 m de fond, fenêtre au bord du
  transitoire). Comme déclaré, rien n'est conclu de δ contre 19,47° à 3 m/s.
- **Critère nouveau, écrit avant de lancer δ** (ADR-222 : la cible contredite par la mesure est remplacée) : **(2') le bord d'Airy sur δ à
  1° de celui de la théorie de la même coque, 16,40°**, fenêtre 4–6 λ₀, rayons issus du centre ; et par fenêtre d'1 λ₀ de 3 à 6 λ₀, à
  1,5°. (3) inchangé. 4.13 « le sillage mesuré » s'entend alors : le sillage de δ est celui de la théorie linéaire de sa coque.
- **P3 (en cours)** — le banc exporte sa surface (`SORTIE=`), `reference_sillage.py delta <fichier>` y applique l'instrument figé. Calcul
  lancé : `calculs/20261006-110837-sillage-delta-s520` (416 × 224, 30 s, 3 m/s ; sortie `calculs/delta_s520.bin`). **À la reprise** : lire
  `sortie.log` (élévation maximale, coût), puis `python outils/reference_sillage.py delta calculs/delta_s520.bin` → (2') contre 16,40°.

