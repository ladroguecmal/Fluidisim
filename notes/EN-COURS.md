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

---

## Session en cours

Session : S256 — en cours
Agent : Claude Opus 5, Claude Code ; fichiers, git, cargo, Python et GPU local disponibles.
Entrée (2026-09-16 22:13) : premier verdict de l'utilisateur sur R1 — « la mer est trop lisse, on
dirait un lac ». Master propre 029cfd5, copie unique, jeton libre, maillons 1. R1 passe devant la
suite automatique (S254).

Protocole (REVUE-VISUELLE §4) : consigner, classer, **confirmer par une mesure** avant de toucher
à B. Hypothèse à éprouver : la recette de B s'arrête à `4 fp` (aucune onde sous 3,5 m) ; la pente
quadratique moyenne `mss = (2π)⁴m4/g²` exige une coupure (SPEC-001 §1 bis), et la nôtre serait
bien en dessous de la mer réelle. Référence indépendante : Cox & Munk (1954), `mss ≈ 0,003 +
5,12·10⁻³·U` (±0,004) ; vent minimal soutenant `Hs = 1,5 m` par Pierson–Moskowitz (mer pleinement
développée, `Hs ≈ 0,21 U²/g`). **Critère écrit avant mesure** : défaut confirmé si la `mss` de B est
inférieure à la moitié de la borne basse de Cox–Munk à ce vent minimal. Sinon, l'attribution se
tourne vers l'habillage (lumière, ciel) et on le dit.

Si confirmé : ADR et remède physique, pas un réglage — la **queue du même spectre** (au-delà de
`4 fp`, même densité absolue) rendue en **pentes par pixel** (normales seulement, hauteur et
requêtes de jeu inchangées), filtrée par l'empreinte du pixel comme ADR-148. Réception : queue
contre l'intégrale analytique, GPU contre référence CPU, coût GPU eau mesuré, rendus R2 aux mêmes
poses envoyés à l'utilisateur.

### Plan

- [x] **P1** — amorce, jeton et plan seuls.
- [x] **P2** — verdict consigné au registre R1, classement provisoire, protocole de mesure ;
  formules Cox–Munk et Pierson–Moskowitz citées dans SPEC-001 (I-14).
- [x] **P3** — mesure : `mss` de la recette cuite (32 composantes) et du spectre continu coupé à
  `4, 8, 16, 24, 32 fp` ; verdict confirmé ou non.
- [ ] **P4** — ADR-155 et protocole de réception du remède (si confirmé).
- [ ] **P5** — cœur : cuisson de la queue spectrale, même densité absolue, essais contre l'analytique.
- [ ] **P6** — hôte : queue en pentes par pixel, filtre d'empreinte, référence CPU.
- [ ] **P7** — réception GPU/CPU, coût, rendus R2 aux poses R1, envoi à l'utilisateur.
- [ ] **P8** — rituel §6 : liste du projet fini (2.1, 8.9, 8.10), registre, journal, jeton.

### Notes de reprise

P3 : `examples/rugosite_b.rs`. mss cuite 0,00753, continue 0,00752 (accord indépendant) ; 8/16/24/32/57 fp
→ 0,0115/0,0155/0,0179/0,0195/0,0229. Cox–Munk 0,0437–0,0459, seuil 0,0199 : **confirmé**. La queue
JONSWAP seule plafonne à 52 % de l'observé : le remède d'ADR-155 est un premier pas, le modèle de
spectre court (équilibre f⁻⁴, capillaires) reste à nommer comme manque.
